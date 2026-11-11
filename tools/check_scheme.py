#!/usr/bin/env python3
"""Check that a PLinOpt L/R/P triple is a matrix multiplication scheme.  Standard library only.

    python3 tools/check_scheme.py schemes/4x4x4_48_201/4x4x4_48_201          # prefix of the _L/_R/_P.sms files
    python3 tools/check_scheme.py --mod 2 schemes/4x4x4_47_mod_2_166/4x4x4_47_mod_2_166
    python3 tools/check_scheme.py --alt schemes/3x3x3_23_ks51/3x3x3_23_ks51   # also ALT x CoB == flat

Conventions (PLinOpt's): the file name starts with MxKxN; L is r x (M K) acting on vec(A), R is r x (K N) on
vec(B), P is (M N) x r giving vec(C), all row-major.  Product t is (L_t . vec A) (R_t . vec B), and
C = P [products].

Small schemes (at most 10^6 terms in the trilinear form) get the full Brent equations, exactly: the trilinear form
sum_t L_t (x) R_t (x) P_t must equal the matrix multiplication tensor coefficient by coefficient.  Larger ones are
run on random integer matrices with exact rational arithmetic (entries up to 10^9); a wrong scheme passes one such
trial with probability below 10^-6 (Schwartz-Zippel), and every run also changes one coefficient and requires the
check to fail.  With --mod p everything is done modulo the prime p.
"""
import argparse, random, re, sys
from fractions import Fraction

def read_sms(path):
    rows = cols = None
    entries = []
    with open(path) as f:
        for line in f:
            s = line.split('#', 1)[0].strip()
            if not s:
                continue
            a = s.split()
            if rows is None:
                rows, cols = int(a[0]), int(a[1])
                continue
            i, j = int(a[0]), int(a[1])
            if i == 0 and j == 0:
                break
            entries.append((i - 1, j - 1, Fraction(a[2])))
    return rows, cols, entries

def as_field(x, p):
    if p is None:
        return x
    return x.numerator % p * pow(x.denominator % p, -1, p) % p

def load(prefix, p):
    mats = {}
    for side in 'LRP':
        rows, cols, ent = read_sms(f'{prefix}_{side}.sms')
        mats[side] = (rows, cols, [(i, j, as_field(v, p)) for i, j, v in ent if as_field(v, p) != 0])
    return mats

def shape_of(prefix):
    m = re.match(r'(\d+)x(\d+)x(\d+)_', prefix.rsplit('/', 1)[-1])
    if not m:
        sys.exit(f'cannot read MxKxN from {prefix}')
    return tuple(int(x) for x in m.groups())

def by_row(n, ent):
    rows = [[] for _ in range(n)]
    for i, j, v in ent:
        rows[i].append((j, v))
    return rows

def by_col(n, ent):
    cols = [[] for _ in range(n)]
    for i, j, v in ent:
        cols[j].append((i, v))
    return cols

def full_brent(m, k, n, L, R, P, p):
    """Exact trilinear form vs the matrix multiplication tensor."""
    r = L[0]
    Lr, Rr, Pc = by_row(r, L[2]), by_row(r, R[2]), by_col(r, P[2])
    T = {}
    for t in range(r):
        for x, a in Lr[t]:
            for y, b in Rr[t]:
                ab = a * b
                for z, c in Pc[t]:
                    key = (x, y, z)
                    v = T.get(key, 0) + ab * c
                    T[key] = v % p if p else v
    bad = 0
    for i in range(m):
        for j in range(k):
            for l in range(n):
                key = (i * k + j, j * n + l, i * n + l)
                if T.pop(key, 0) != 1:
                    bad += 1
    bad += sum(1 for v in T.values() if v != 0)
    return bad

def evaluate(m, k, n, L, R, P, A, B, p):
    r = L[0]
    u = [0] * r; w = [0] * r
    for t, x, a in L[2]:
        u[t] += a * A[x]
    for t, y, b in R[2]:
        w[t] += b * B[y]
    prod = [(u[t] * w[t]) % p if p else u[t] * w[t] for t in range(r)]
    C = [0] * (m * n)
    for z, t, c in P[2]:
        C[z] += c * prod[t]
    return [c % p if p else c for c in C]

def random_check(m, k, n, L, R, P, p, trials, rng):
    wrong = []
    for _ in range(trials):
        hi = (p - 1) if p else 10 ** 9
        A = [rng.randint(0, hi) for _ in range(m * k)]
        B = [rng.randint(0, hi) for _ in range(k * n)]
        C = evaluate(m, k, n, L, R, P, A, B, p)
        ref = [sum(A[i * k + j] * B[j * n + l] for j in range(k)) for i in range(m) for l in range(n)]
        if p:
            ref = [x % p for x in ref]
        wrong.append(sum(1 for a, b in zip(C, ref) if a != b))
    return wrong

def matprod(X, Y):
    """Sparse exact product of two (rows, cols, entries) matrices."""
    Yr = by_row(X[1], Y[2])
    acc = {}
    for i, j, v in X[2]:
        for l, w in Yr[j]:
            acc[(i, l)] = acc.get((i, l), 0) + v * w
    return X[0], Y[1], sorted((i, l, v) for (i, l), v in acc.items() if v != 0)

def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('prefix', help='path prefix of PREFIX_{L,R,P}.sms')
    ap.add_argument('--mod', type=int, default=None, help='check modulo this prime (2 for the F2 schemes)')
    ap.add_argument('--alt', action='store_true', help='also check PREFIX-ALT_* x PREFIX-CoB_* against PREFIX_*')
    ap.add_argument('--trials', type=int, default=2)
    ap.add_argument('--seed', type=int, default=1)
    a = ap.parse_args()
    m, k, n = shape_of(a.prefix)
    p = a.mod
    mats = load(a.prefix, p)
    L, R, P = mats['L'], mats['R'], mats['P']
    r = L[0]
    assert (L[1], R[0], R[1], P[0], P[1]) == (m * k, r, k * n, m * n, r), 'dimensions do not match MxKxN and rank'
    field = f'F_{p}' if p else 'Q'
    Lr, Rr, Pc = by_row(r, L[2]), by_row(r, R[2]), by_col(r, P[2])
    terms = sum(len(Lr[t]) * len(Rr[t]) * len(Pc[t]) for t in range(r))
    print(f'{a.prefix}: <{m}x{k}x{n}:{r}> over {field}, nonzeros L {len(L[2])} R {len(R[2])} P {len(P[2])}, '
          f'{terms} trilinear terms')
    ok = True
    rng = random.Random(a.seed)
    if terms <= 10 ** 6:
        bad = full_brent(m, k, n, L, R, P, p)
        print(f'  Brent equations, all {(m*k)*(k*n)*(m*n)} coefficients: {"EXACT" if bad == 0 else f"{bad} WRONG"}')
        ok &= bad == 0
        if P[2]:
            z, t, c = P[2][0]
            P2 = (P[0], P[1], [(z, t, (c + 1) % p if p else c + 1)] + P[2][1:])
            bad2 = full_brent(m, k, n, L, R, P2, p)
            print(f'  control, one P coefficient changed: {"REJECTED" if bad2 else "NOT REJECTED"}')
            ok &= bad2 > 0
    else:
        wrong = random_check(m, k, n, L, R, P, p, a.trials, rng)
        print(f'  {a.trials} random products, wrong entries per trial {wrong}: {"EXACT" if not any(wrong) else "WRONG"}')
        ok &= not any(wrong)
        z, t, c = P[2][0]
        P2 = (P[0], P[1], [(z, t, (c + 1) % p if p else c + 1)] + P[2][1:])
        wrong2 = random_check(m, k, n, L, R, P2, p, 1, rng)
        print(f'  control, one P coefficient changed: {"REJECTED" if any(wrong2) else "NOT REJECTED"}')
        ok &= any(wrong2)
    if a.alt:
        alt, cob = load(a.prefix + '-ALT', p), load(a.prefix + '-CoB', p)
        same = [matprod(alt['L'], cob['L'])[2] == sorted(L[2]), matprod(alt['R'], cob['R'])[2] == sorted(R[2]),
                matprod(cob['P'], alt['P'])[2] == sorted(P[2])]
        print(f'  ALT_L CoB_L == L, ALT_R CoB_R == R, CoB_P ALT_P == P: {same}')
        ok &= all(same)
    print('  RESULT:', 'PASS' if ok else 'FAIL')
    return 0 if ok else 1

if __name__ == '__main__':
    sys.exit(main())
