//! kronexact: exact replay over Q of a kronkill path, and an exact check of every Brent equation.
//!
//! kronkill (the search program, not in this repository) searches projections and 2-reductions of a matrix multiplication scheme
//! modulo one 20-bit prime and writes a path file: per stage the deleted index, per reduction step the
//! direction, the pivot products and the support of the dependencies.  This program redoes the path over the
//! rationals and certifies the result.
//!
//! Prior art: 2-reducibility is LITA's (Khoruzhii, Serafin, Gelss, Pokutta, github.com/khoruzhii/lita).
//!
//! replay: for every step, the dependencies of the Kronecker products K_t (t in the support) are the null space
//!   of their Gram matrix (X_r X_r^T) o (X_c X_c^T).  It is computed modulo three primes just below 2^61 with
//!   the pivots as the free columns, lifted to Q from the first two (Chinese remainder + rational
//!   reconstruction) and compared with the third.  The fold f_s -= sum_j N[j][s] f_{J_j} is done in exact
//!   rational arithmetic (64-bit numerators and denominators, every overflow is a panic).
//! check: with D_u, D_v, D_w the common denominators of the three factors, every one of the
//!   (n0 n1)(n1 n2)(n2 n0) Brent equations sum_t U_t[a] V_t[b] W_t[c] = D_u D_v D_w [target] is evaluated in
//!   128-bit integers, after a proof that no partial sum can overflow.  A copy with one changed coefficient
//!   must fail.  THE CHECK IS THE CERTIFICATE: it does not depend on how the scheme was found, so nothing in
//!   the replay has to be trusted.
//!
//! usage: kronexact check SCHEME.json[.gz]
//!        kronexact replay SOURCE.json[.gz] PATH.json OUT.json [--check-input] [--dense]
//! Layout as in the catalog: u on A (n0 x n1 row-major), v on B (n1 x n2), w on C^T (index k * n0 + i).

use rayon::prelude::*;
use serde_json::{Value, json};
use std::time::Instant;

const RC: [(usize, usize); 3] = [(1, 2), (0, 2), (0, 1)];

// ---------------------------------------------------------------- exact rationals, 64-bit

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.abs()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Q {
    n: i64,
    d: i64,
}

impl Q {
    const ZERO: Q = Q { n: 0, d: 1 };
    fn new(n: i128, d: i128) -> Q {
        assert!(d != 0, "zero denominator");
        let g = gcd(n, d).max(1);
        let (mut n, mut d) = (n / g, d / g);
        if d < 0 {
            (n, d) = (-n, -d);
        }
        Q { n: i64::try_from(n).expect("numerator beyond 64 bits"), d: i64::try_from(d).expect("denominator beyond 64 bits") }
    }
    fn mul(self, o: Q) -> Q {
        Q::new(self.n as i128 * o.n as i128, self.d as i128 * o.d as i128)
    }
    fn sub(self, o: Q) -> Q {
        Q::new(self.n as i128 * o.d as i128 - o.n as i128 * self.d as i128, self.d as i128 * o.d as i128)
    }
    fn is_zero(self) -> bool {
        self.n == 0
    }
    fn show(self) -> Value {
        if self.d == 1 { json!(self.n) } else { json!(format!("{}/{}", self.n, self.d)) }
    }
}

/// "12", "-3/4", "0.25", "1.5/2": exact.
fn parse_q(s: &str) -> Q {
    fn dec(s: &str) -> (i128, i128) {
        let s = s.trim();
        let (neg, body) = match s.strip_prefix('-') {
            Some(r) => (true, r),
            None => (false, s.strip_prefix('+').unwrap_or(s)),
        };
        let (mut num, mut den, mut frac) = (0i128, 1i128, false);
        for ch in body.chars() {
            match ch {
                '0'..='9' => {
                    num = num.checked_mul(10).unwrap().checked_add(ch as i128 - '0' as i128).unwrap();
                    if frac {
                        den = den.checked_mul(10).unwrap();
                    }
                }
                '.' => frac = true,
                _ => panic!("cannot parse the entry {s:?}"),
            }
        }
        (if neg { -num } else { num }, den)
    }
    let mut parts = s.split('/');
    let (a, b) = dec(parts.next().unwrap());
    let (c, d) = parts.next().map(dec).unwrap_or((1, 1));
    assert!(parts.next().is_none(), "cannot parse the entry {s:?}");
    let g1 = gcd(a, b).max(1);
    let g2 = gcd(c, d).max(1);
    Q::new((a / g1).checked_mul(d / g2).unwrap(), (b / g1).checked_mul(c / g2).unwrap())
}

// ---------------------------------------------------------------- schemes over Q, sparse rows

#[derive(Clone)]
struct Scheme {
    n: [usize; 3],
    x: [Vec<Vec<(u32, Q)>>; 3], // per slot, per product: (column, value), columns increasing, values nonzero
}

fn cols_of(n: [usize; 3], s: usize) -> usize {
    match s {
        0 => n[0] * n[1],
        1 => n[1] * n[2],
        _ => n[2] * n[0],
    }
}

impl Scheme {
    fn m(&self) -> usize {
        self.x[0].len()
    }
    fn cols(&self, s: usize) -> usize {
        cols_of(self.n, s)
    }
}

fn entry(v: &Value) -> Q {
    match v {
        Value::Number(x) => parse_q(&x.to_string()),
        Value::String(x) => parse_q(x),
        _ => panic!("unexpected entry {v}"),
    }
}

fn load(path: &str) -> Scheme {
    let text = if path.ends_with(".gz") {
        // no compression crate: the system's gzip
        let o = std::process::Command::new("gzip").args(["-dc", path]).output().unwrap_or_else(|e| panic!("gzip -dc {path}: {e}"));
        assert!(o.status.success(), "gzip -dc {path} failed");
        String::from_utf8(o.stdout).expect("UTF-8")
    } else {
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"))
    };
    let d: Value = serde_json::from_str(&text).expect("JSON");
    drop(text);
    let n: Vec<usize> = d["n"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as usize).collect();
    let n = [n[0], n[1], n[2]];
    let m = d["m"].as_u64().unwrap() as usize;
    let mut x: [Vec<Vec<(u32, Q)>>; 3] = [vec![], vec![], vec![]];
    for (s, key) in ["u", "v", "w"].iter().enumerate() {
        let rows = d[*key].as_array().unwrap();
        assert_eq!(rows.len(), m, "{key} has {} rows, m = {m}", rows.len());
        let c = cols_of(n, s);
        for row in rows {
            let row = row.as_array().unwrap();
            let mut out: Vec<(u32, Q)> = Vec::new();
            if row.first().is_some_and(|e| e.is_object()) || row.is_empty() {
                for e in row {
                    let i = e["index"].as_u64().expect("sparse entry without an index") as usize;
                    assert!(i < c, "sparse index {i} out of range {c}");
                    let q = entry(&e["value"]);
                    if !q.is_zero() {
                        out.push((i as u32, q));
                    }
                }
                out.sort_by_key(|e| e.0);
                assert!(out.windows(2).all(|w| w[0].0 < w[1].0), "a sparse row repeats an index");
            } else {
                assert_eq!(row.len(), c);
                for (i, v) in row.iter().enumerate() {
                    let q = entry(v);
                    if !q.is_zero() {
                        out.push((i as u32, q));
                    }
                }
            }
            x[s].push(out);
        }
    }
    Scheme { n, x }
}

fn save(sc: &Scheme, path: &str, dense: bool, note: Value) {
    let mut d = serde_json::Map::new();
    d.insert("n".into(), json!(sc.n));
    d.insert("m".into(), json!(sc.m()));
    d.insert("z2".into(), json!(false));
    d.insert("note".into(), note);
    for (s, key) in ["u", "v", "w"].iter().enumerate() {
        let c = sc.cols(s);
        let rows: Vec<Value> = sc.x[s]
            .iter()
            .map(|row| {
                if dense {
                    let mut v = vec![json!(0); c];
                    for &(i, q) in row {
                        v[i as usize] = q.show();
                    }
                    Value::Array(v)
                } else {
                    Value::Array(row.iter().map(|&(i, q)| json!({"index": i, "value": q.show()})).collect())
                }
            })
            .collect();
        d.insert((*key).into(), Value::Array(rows));
    }
    std::fs::write(path, serde_json::to_string(&Value::Object(d)).unwrap()).unwrap();
}

/// Delete one index of slot `slot` (0 = I, 1 = J, 2 = K): kronkill's `project`, same order of the survivors.
fn project(sc: &Scheme, slot: usize, index: usize) -> Scheme {
    let n = sc.n;
    assert!(index < n[slot], "index {index} out of range for slot {slot} of {n:?}");
    let keep: Vec<Vec<usize>> = (0..3).map(|t| (0..n[t]).filter(|&j| !(t == slot && j == index)).collect()).collect();
    let (ki, kj, kk) = (&keep[0], &keep[1], &keep[2]);
    let cu: Vec<usize> = ki.iter().flat_map(|&i| kj.iter().map(move |&j| i * n[1] + j)).collect();
    let cv: Vec<usize> = kj.iter().flat_map(|&j| kk.iter().map(move |&k| j * n[2] + k)).collect();
    let cw: Vec<usize> = kk.iter().flat_map(|&k| ki.iter().map(move |&i| k * n[0] + i)).collect();
    let maps: Vec<Vec<u32>> = [cu, cv, cw]
        .iter()
        .enumerate()
        .map(|(s, cs)| {
            let mut mp = vec![u32::MAX; sc.cols(s)];
            for (new, &old) in cs.iter().enumerate() {
                mp[old] = new as u32;
            }
            mp
        })
        .collect();
    let mut n2 = n;
    n2[slot] -= 1;
    let mut x: [Vec<Vec<(u32, Q)>>; 3] = [vec![], vec![], vec![]];
    for t in 0..sc.m() {
        let rows: Vec<Vec<(u32, Q)>> = (0..3)
            .map(|s| {
                let mut r: Vec<(u32, Q)> = sc.x[s][t].iter().filter(|e| maps[s][e.0 as usize] != u32::MAX).map(|e| (maps[s][e.0 as usize], e.1)).collect();
                r.sort_by_key(|e| e.0);
                r
            })
            .collect();
        if rows.iter().all(|r| !r.is_empty()) {
            for (s, r) in rows.into_iter().enumerate() {
                x[s].push(r);
            }
        }
    }
    Scheme { n: n2, x }
}

// ---------------------------------------------------------------- arithmetic modulo q = 2^61 - c

#[derive(Clone, Copy)]
struct Fp {
    q: u64,
    c: u128,
}

const M61: u128 = (1u128 << 61) - 1;

impl Fp {
    #[inline]
    fn red(self, x: u128) -> u64 {
        let y = (x & M61) + (x >> 61) * self.c; // below 2^61 + 2^67 c
        let y = (y & M61) + (y >> 61) * self.c; // below 2^61 + 2^47
        let mut z = y as u64;
        while z >= self.q {
            z -= self.q;
        }
        z
    }
    #[inline]
    fn mul(self, a: u64, b: u64) -> u64 {
        self.red(a as u128 * b as u128)
    }
    fn pow(self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    fn inv(self, a: u64) -> u64 {
        assert!(a % self.q != 0, "inverse of zero modulo {}", self.q);
        self.pow(a % self.q, self.q - 2)
    }
    fn of_i64(self, a: i64) -> u64 {
        (a as i128).rem_euclid(self.q as i128) as u64
    }
    fn of_q(self, v: Q) -> u64 {
        let d = self.of_i64(v.d);
        assert!(d != 0, "a denominator is divisible by {}", self.q);
        self.mul(self.of_i64(v.n), self.inv(d))
    }
}

fn is_prime(n: u64) -> bool {
    fn mulmod(a: u64, b: u64, n: u64) -> u64 {
        (a as u128 * b as u128 % n as u128) as u64
    }
    fn powmod(mut a: u64, mut e: u64, n: u64) -> u64 {
        let mut r = 1;
        while e > 0 {
            if e & 1 == 1 {
                r = mulmod(r, a, n);
            }
            a = mulmod(a, a, n);
            e >>= 1;
        }
        r
    }
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'outer: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        // deterministic for every 64-bit n
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// The `count` largest primes below 2^61.
fn primes(count: usize) -> Vec<Fp> {
    let mut out = Vec::new();
    let mut c = 1u64;
    while out.len() < count {
        let q = (1u64 << 61) - c;
        if is_prime(q) {
            out.push(Fp { q, c: c as u128 });
        }
        c += 2;
    }
    assert!(c < 1 << 20);
    out
}

// ---------------------------------------------------------------- one reduction step, exact

/// The dependencies with the pivots as free columns, modulo f: returns N[j][pos] for pos over order[..np]
/// (order = the support without the pivots, then the pivots).
fn null_mod(sc: &Scheme, d: usize, order: &[usize], k: usize, f: Fp) -> Vec<Vec<u64>> {
    let (r, c) = RC[d];
    let ns = order.len();
    let np = ns - k;
    // Gram matrices of the two other factors on the support, then their entrywise product
    let gram = |s: usize| -> Vec<Vec<u64>> {
        let cs = sc.cols(s);
        let sparse: Vec<Vec<(u32, u64)>> = order.iter().map(|&t| sc.x[s][t].iter().map(|&(i, v)| (i, f.of_q(v))).collect()).collect();
        let mut dense = vec![0u64; ns * cs];
        for (a, row) in sparse.iter().enumerate() {
            for &(i, v) in row {
                dense[a * cs + i as usize] = v;
            }
        }
        (0..ns)
            .into_par_iter()
            .map(|a| {
                (0..ns)
                    .map(|b| {
                        let rb = &dense[b * cs..(b + 1) * cs];
                        let (mut acc, mut cnt) = (0u128, 0);
                        for &(i, v) in &sparse[a] {
                            acc += v as u128 * rb[i as usize] as u128; // below 2^122
                            cnt += 1;
                            if cnt == 32 {
                                acc = f.red(acc) as u128;
                                cnt = 0;
                            }
                        }
                        f.red(acc)
                    })
                    .collect()
            })
            .collect()
    };
    let (gr, gc) = (gram(r), gram(c));
    let mut rows: Vec<Vec<u64>> = gr.into_par_iter().zip(gc.into_par_iter()).map(|(a, b)| a.iter().zip(&b).map(|(&x, &y)| f.mul(x, y)).collect()).collect();
    // forward elimination: every column of the support without the pivots must carry a pivot
    for col in 0..np {
        let i = (col..ns).find(|&i| rows[i][col] != 0).unwrap_or_else(|| panic!("modulo {}: column {col} of {np} has no pivot: the pivot products are not independent on the dependencies", f.q));
        rows.swap(col, i);
        let iv = f.inv(rows[col][col]);
        for v in rows[col][col..].iter_mut() {
            *v = f.mul(*v, iv);
        }
        let (top, rest) = rows.split_at_mut(col + 1);
        let pr = &top[col];
        rest.par_iter_mut().with_min_len(8).for_each(|row| {
            let x = row[col];
            if x != 0 {
                let cf = (f.q - x) as u128;
                for (v, &q) in row[col + 1..].iter_mut().zip(&pr[col + 1..]) {
                    *v = f.red(*v as u128 + cf * q as u128);
                }
                row[col] = 0;
            }
        });
    }
    assert!(rows[np..].iter().all(|r| r.iter().all(|&v| v == 0)), "modulo {}: the Gram matrix has rank above support - pivots", f.q);
    let rows = &rows[..np];
    (0..k)
        .into_par_iter()
        .map(|j| {
            let mut x = vec![0u64; ns];
            x[np + j] = 1;
            for i in (0..np).rev() {
                let row = &rows[i];
                let (mut acc, mut cnt) = (0u128, 0);
                for l in i + 1..ns {
                    if x[l] != 0 {
                        acc += row[l] as u128 * x[l] as u128;
                        cnt += 1;
                        if cnt == 32 {
                            acc = f.red(acc) as u128;
                            cnt = 0;
                        }
                    }
                }
                let a = f.red(acc);
                x[i] = if a == 0 { 0 } else { f.q - a };
            }
            x.truncate(np);
            x
        })
        .collect()
}

/// The rational n/d with |n|, d <= sqrt(M / 2) and n = x d modulo M (M = q0 q1), if there is one.
fn reconstruct(x: u128, m: u128) -> Option<Q> {
    if x == 0 {
        return Some(Q::ZERO);
    }
    let bound = ((m / 2) as f64).sqrt() as i128 - 2;
    let (mut r0, mut r1) = (m as i128, x as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 > bound {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    if t1 == 0 || t1.abs() > bound || gcd(r1, t1) != 1 {
        return None;
    }
    Some(Q::new(if t1 < 0 { -r1 } else { r1 }, t1.abs()))
}

/// One step of a kronkill path: drop the pivot products of direction d, fold their slot-d factors into the others.
fn step(sc: &Scheme, d: usize, pivots: &[usize], support: Option<&[usize]>, ps: &[Fp]) -> (Scheme, Vec<usize>, String) {
    let t0 = Instant::now();
    let m = sc.m();
    let k = pivots.len();
    let mut is_piv = vec![false; m];
    for &j in pivots {
        assert!(j < m && !is_piv[j], "bad pivot {j}");
        is_piv[j] = true;
    }
    let all: Vec<usize> = (0..m).collect();
    let supp = support.unwrap_or(&all);
    let mut in_supp = vec![false; m];
    for &t in supp {
        in_supp[t] = true;
    }
    assert!(pivots.iter().all(|&j| in_supp[j]), "a pivot outside the support");
    let mut order: Vec<usize> = supp.iter().copied().filter(|&t| !is_piv[t]).collect();
    let np = order.len();
    order.extend_from_slice(pivots);
    let nm: Vec<Vec<Vec<u64>>> = ps.iter().map(|&f| null_mod(sc, d, &order, k, f)).collect();
    // lift from the first two primes, compare with the third
    let (f0, f1, f2) = (ps[0], ps[1], ps[2]);
    let modulus = f0.q as u128 * f1.q as u128;
    let q0_inv = f1.inv(f0.q % f1.q);
    let mut height = 1i64;
    let nq: Vec<Vec<Q>> = (0..k)
        .map(|j| {
            (0..np)
                .map(|pos| {
                    let (a0, a1) = (nm[0][j][pos], nm[1][j][pos]);
                    let diff = f1.red(a1 as u128 + (f1.q - a0 % f1.q) as u128);
                    let x = a0 as u128 + f0.q as u128 * f1.mul(diff, q0_inv) as u128;
                    let v = reconstruct(x, modulus).unwrap_or_else(|| panic!("dependency {j}, product {}: no rational of height below 2^60", order[pos]));
                    assert_eq!(f2.of_q(v), nm[2][j][pos], "dependency {j}, product {}: the lift disagrees with the third prime", order[pos]);
                    height = height.max(v.n.abs()).max(v.d);
                    v
                })
                .collect()
        })
        .collect();
    let nnz: usize = nq.iter().map(|r| r.iter().filter(|v| !v.is_zero()).count()).sum();
    // the fold, exact
    let cd = sc.cols(d);
    let fj: Vec<&Vec<(u32, Q)>> = pivots.iter().map(|&j| &sc.x[d][j]).collect();
    let folded: Vec<(usize, Vec<(u32, Q)>)> = (0..np)
        .into_par_iter()
        .filter(|&pos| (0..k).any(|j| !nq[j][pos].is_zero()))
        .map(|pos| {
            let mut row = vec![Q::ZERO; cd];
            for &(i, v) in &sc.x[d][order[pos]] {
                row[i as usize] = v;
            }
            for j in 0..k {
                let w = nq[j][pos];
                if !w.is_zero() {
                    for &(i, v) in fj[j] {
                        row[i as usize] = row[i as usize].sub(w.mul(v));
                    }
                }
            }
            (order[pos], row.into_iter().enumerate().filter(|e| !e.1.is_zero()).map(|(i, v)| (i as u32, v)).collect())
        })
        .collect();
    let mut newd: Vec<Option<Vec<(u32, Q)>>> = vec![None; m];
    for (t, row) in folded {
        newd[t] = Some(row);
    }
    let mut x: [Vec<Vec<(u32, Q)>>; 3] = [vec![], vec![], vec![]];
    let mut dead = Vec::new();
    for t in 0..m {
        if is_piv[t] {
            continue;
        }
        let rd = newd[t].take().unwrap_or_else(|| sc.x[d][t].clone());
        if rd.is_empty() {
            dead.push(t);
            continue;
        }
        for s in 0..3 {
            x[s].push(if s == d { rd.clone() } else { sc.x[s][t].clone() });
        }
    }
    let out = Scheme { n: sc.n, x };
    let msg = format!(
        "direction {d}: {k} dependencies on {} products (lifted from 2 primes of 61 bits, equal modulo a third; {nnz} nonzero coefficients, largest numerator or denominator {height}); {m} -> {} products ({k} pivots, {} left with a zero factor) ({:.1} s)",
        order.len(),
        out.m(),
        dead.len(),
        t0.elapsed().as_secs_f64()
    );
    (out, dead, msg)
}

// ---------------------------------------------------------------- the Brent equations, exact

struct Scaled {
    den: [i128; 3],
    rows: [Vec<Vec<(u32, i128)>>; 3],
    bits: f64,
}

/// Integer rows D_s x_s and a proof that sum_t |U_t[a] V_t[b] W_t[c]| stays below 2^126.
fn scale(sc: &Scheme) -> Scaled {
    let mut den = [1i128; 3];
    let mut mx = [0i128; 3];
    let mut rows: [Vec<Vec<(u32, i128)>>; 3] = [vec![], vec![], vec![]];
    for s in 0..3 {
        for row in &sc.x[s] {
            for &(_, v) in row {
                let g = gcd(den[s], v.d as i128);
                den[s] = (den[s] / g).checked_mul(v.d as i128).expect("the common denominator of a factor exceeds 127 bits");
            }
        }
        rows[s] = sc.x[s]
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&(i, v)| {
                        let y = (den[s] / v.d as i128).checked_mul(v.n as i128).expect("a scaled entry exceeds 127 bits");
                        (i, y)
                    })
                    .collect()
            })
            .collect();
        mx[s] = rows[s].iter().flat_map(|r| r.iter().map(|e| e.1.abs())).max().unwrap_or(0);
    }
    let bits = (sc.m() as f64).log2() + mx.iter().map(|&v| (v.max(1) as f64).log2()).sum::<f64>();
    Scaled { den, rows, bits }
}

/// Number of Brent equations that fail (0 = the scheme is a matrix multiplication scheme over Q), and their count.
fn brent(sc: &Scheme) -> (u64, u64, String) {
    let t0 = Instant::now();
    let z = scale(sc);
    assert!(z.bits < 125.0, "partial sums could reach 2^{:.0}: beyond 128-bit integers", z.bits);
    let target = z.den[0].checked_mul(z.den[1]).and_then(|x| x.checked_mul(z.den[2])).expect("D_u D_v D_w exceeds 127 bits");
    let [n0, n1, n2] = sc.n;
    let (cu, cv, cw) = (n0 * n1, n1 * n2, n2 * n0);
    let mut by_col: Vec<Vec<(u32, i128)>> = vec![vec![]; cu];
    for (t, row) in z.rows[0].iter().enumerate() {
        for &(a, v) in row {
            by_col[a as usize].push((t as u32, v));
        }
    }
    let bad: u64 = (0..cu)
        .into_par_iter()
        .map(|a| {
            let mut acc = vec![0i128; cv * cw];
            for &(t, al) in &by_col[a] {
                let wr = &z.rows[2][t as usize];
                for &(b, be) in &z.rows[1][t as usize] {
                    let ab = al * be;
                    let line = &mut acc[b as usize * cw..(b as usize + 1) * cw];
                    for &(c, ga) in wr {
                        line[c as usize] += ab * ga;
                    }
                }
            }
            let (i, j) = (a / n1, a % n1);
            let mut bad = 0u64;
            for b in 0..cv {
                let (j2, k) = (b / n2, b % n2);
                for c in 0..cw {
                    let (k2, i2) = (c / n0, c % n0);
                    let want = if j == j2 && k == k2 && i == i2 { target } else { 0 };
                    if acc[b * cw + c] != want {
                        bad += 1;
                    }
                }
            }
            bad
        })
        .sum();
    let total = (cu * cv * cw) as u64;
    let msg = format!(
        "{bad} of {total} Brent equations fail; common denominators {:?}, partial sums below 2^{:.0} ({:.1} s)",
        z.den,
        z.bits.ceil(),
        t0.elapsed().as_secs_f64()
    );
    (bad, total, msg)
}

fn factor_small(mut v: i64) -> (Vec<i64>, i64) {
    let mut ps = Vec::new();
    let mut p = 2;
    while p * p <= v && p < 100_000 {
        if v % p == 0 {
            ps.push(p);
            while v % p == 0 {
                v /= p;
            }
        }
        p += 1;
    }
    if v > 1 && v < 10_000_000_000 {
        ps.push(v);
        v = 1;
    }
    (ps, v)
}

fn stats(sc: &Scheme) -> String {
    let mut out = Vec::new();
    let mut all_primes: Vec<i64> = Vec::new();
    for s in 0..3 {
        let nnz: usize = sc.x[s].iter().map(|r| r.len()).sum();
        let frac = sc.x[s].iter().flat_map(|r| r.iter()).filter(|e| e.1.d != 1).count();
        let maxd = sc.x[s].iter().flat_map(|r| r.iter()).map(|e| e.1.d).max().unwrap_or(1);
        let maxn = sc.x[s].iter().flat_map(|r| r.iter()).map(|e| e.1.n.abs()).max().unwrap_or(0);
        let mut dens: Vec<i64> = sc.x[s].iter().flat_map(|r| r.iter()).map(|e| e.1.d).collect();
        dens.sort_unstable();
        dens.dedup();
        for d in dens {
            let (ps, rest) = factor_small(d);
            all_primes.extend(ps);
            if rest > 1 {
                all_primes.push(-rest);
            }
        }
        out.push(format!("{} {nnz} nonzeros, {frac} fractions, largest numerator {maxn}, largest denominator {maxd}", ["u", "v", "w"][s]));
    }
    all_primes.sort_unstable();
    all_primes.dedup();
    let ring = if all_primes.is_empty() {
        "integer"
    } else if all_primes == [2] {
        "dyadic"
    } else {
        "rational"
    };
    format!("{}; denominator primes {:?} -> {ring}", out.join("; "), all_primes)
}

/// The exact check with its negative control.  Returns true when the scheme verifies and the control fails.
fn certify(sc: &Scheme, tag: &str) -> bool {
    let (bad, _, msg) = brent(sc);
    println!("  [{tag}] <{}x{}x{}:{}> {msg}", sc.n[0], sc.n[1], sc.n[2], sc.m());
    let mut c = sc.clone();
    let t = sc.m() / 2;
    let e = c.x[2][t][0];
    c.x[2][t][0] = (e.0, e.1.sub(Q { n: -1, d: 1 }));
    if c.x[2][t][0].1.is_zero() {
        c.x[2][t][0] = (e.0, Q { n: 2, d: 1 });
    }
    let (cbad, _, _) = brent(&c);
    println!("  [{tag}, one coefficient of product {t} changed] {cbad} equations fail -> {}", if cbad > 0 { "REJECTED" } else { "ACCEPTED: THE CHECK IS BROKEN" });
    bad == 0 && cbad > 0
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let mut flag = |key: &str| -> bool {
        match args.iter().position(|a| a == key) {
            Some(i) => {
                args.remove(i);
                true
            }
            None => false,
        }
    };
    let (check_input, dense) = (flag("--check-input"), flag("--dense"));
    let t0 = Instant::now();
    match args.first().map(|s| s.as_str()) {
        Some("check") if args.len() == 2 => {
            let sc = load(&args[1]);
            println!("{}: <{}x{}x{}:{}> loaded ({:.1} s)", args[1], sc.n[0], sc.n[1], sc.n[2], sc.m(), t0.elapsed().as_secs_f64());
            println!("  {}", stats(&sc));
            let ok = certify(&sc, "scheme");
            println!("  RESULT: {}", if ok { "VERIFIES EXACTLY; corrupted copy REJECTED" } else { "DOES NOT VERIFY" });
            std::process::exit(if ok { 0 } else { 1 });
        }
        Some("replay") if args.len() == 4 => {
            let pj: Value = serde_json::from_str(&std::fs::read_to_string(&args[2]).expect("path file")).expect("path JSON");
            let mut sc = load(&args[1]);
            println!("{}: <{}x{}x{}:{}> loaded ({:.1} s); path {} (sequence {:?})", args[1], sc.n[0], sc.n[1], sc.n[2], sc.m(), t0.elapsed().as_secs_f64(), args[2], pj["sequence"].as_str().unwrap_or(""));
            if check_input {
                assert!(certify(&sc, "input"), "the input does not verify");
            }
            let ps = primes(3);
            println!("  primes {:?}", ps.iter().map(|f| f.q).collect::<Vec<_>>());
            for (si, st) in pj["stages"].as_array().unwrap().iter().enumerate() {
                let del = st["delete"].as_str().unwrap();
                if !del.is_empty() {
                    let slot = "IJK".find(&del[..1]).expect("bad deletion");
                    let m_in = sc.m();
                    sc = project(&sc, slot, del[1..].parse().expect("bad deletion"));
                    println!("  stage {si}: delete {del}: {m_in} -> {} products, <{}x{}x{}>", sc.m(), sc.n[0], sc.n[1], sc.n[2]);
                    assert_eq!(sc.m() as u64, st["alive"].as_u64().unwrap(), "survivors differ from kronkill's");
                }
                for (ti, x) in st["steps"].as_array().unwrap().iter().enumerate() {
                    let list = |v: &Value| -> Vec<usize> { v.as_array().unwrap().iter().map(|i| i.as_u64().unwrap() as usize).collect() };
                    assert_eq!(x["m"].as_u64().unwrap() as usize, sc.m());
                    let pivots = list(&x["pivots"]);
                    let support = x.get("support").filter(|v| v.is_array()).map(list);
                    let (y, dead, msg) = step(&sc, x["direction"].as_u64().unwrap() as usize, &pivots, support.as_deref(), &ps);
                    println!("  stage {si} step {ti}: {msg}");
                    assert_eq!(y.m() as u64, x["m_out"].as_u64().unwrap(), "product count differs from kronkill's");
                    assert_eq!(dead, list(&x["also_zero"]), "the products left with a zero factor differ from kronkill's");
                    sc = y;
                }
            }
            assert_eq!(sc.m() as u64, pj["m_out"].as_u64().unwrap());
            println!("  {}", stats(&sc));
            let ok = certify(&sc, "output");
            if ok {
                save(&sc, &args[3], dense, json!({"source": args[1], "path": args[2], "sequence": pj["sequence"], "by": "kronexact replay"}));
                println!("  RESULT: output with {} products VERIFIES EXACTLY; corrupted copy REJECTED; written {} ({:.1} s)", sc.m(), args[3], t0.elapsed().as_secs_f64());
            } else {
                println!("  RESULT: DOES NOT VERIFY; nothing written");
            }
            std::process::exit(if ok { 0 } else { 1 });
        }
        _ => {
            eprintln!("usage: kronexact check SCHEME.json\n       kronexact replay SOURCE.json PATH.json OUT.json [--check-input] [--dense]");
            std::process::exit(2);
        }
    }
}
