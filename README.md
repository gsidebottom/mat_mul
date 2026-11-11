# mat_mul

Matrix multiplication schemes found by Greg Sidebottom in 2026: records for the number of multiplications (the rank)
or for the number of additions, and a few near records.  Every scheme is in the format of
[PLinOpt](https://github.com/jgdumas/plinopt), with a straight-line program for each of its three linear maps, and has
been checked exactly.  Three of them come with Lean 4 proofs that they compute the matrix product.

Status as of 2026-11-11.  "Previous best" is what we knew of when the scheme was found; the rank comparisons use
Alexandre Sedoglavic's table at [fmm.univ-lille.fr](https://fmm.univ-lille.fr/) (its `fmm_sota.json` of 2026-11-09)
and Andrew Perminov's catalog
[FastMatrixMultiplication](https://github.com/dronperminov/FastMatrixMultiplication) (commit 6f8ef61 of 2026-11-09).

## Index

### Fewest operations at a given rank

| Scheme | Format | Operations | Previous best | Valid over | Exponent | Found | Lean proof |
|---|---|---|---|---|---|---|---|
| [3x3x3_23_55](schemes/3x3x3_23_55) | ⟨3×3×3:23⟩ | 55 additions: 13 + 14 + 28 | 56 (Sun, April 2026) | every ring | 2.854050 | 2026-07-05 | [Correct.lean](lean/Mm/Correct.lean), [Matrix.lean](lean/Mm/Matrix.lean) |
| [3x3x3_23_ks51](schemes/3x3x3_23_ks51) | ⟨3×3×3:23⟩ | 51 additions per recursive step: 12 + 13 + 26, in an alternative basis; the basis changes take 4 + 4 + 3 more | ties Stapleton and Perminov's 51 (September 2026, posted first); Nielsen's 52 | every ring | 2.854050 | 2026-10-11 | [Ring51.lean](lean/Mm/Ring51.lean) |
| [4x4x4_48_201](schemes/4x4x4_48_201) | ⟨4×4×4:48⟩ | 201: 197 additions and 4 halvings; 55 + 57 + 89 | 202 (Patel, October 2026); 204 published | rings in which 2 is invertible | 2.792481 | 2026-10-21 | [Correct48_201.lean](lean/Mm/Correct48_201.lean), [Ring_4x4_48_201.lean](lean/Mm/Ring_4x4_48_201.lean) |
| [4x4x4_47_mod_2_166](schemes/4x4x4_47_mod_2_166) | ⟨4×4×4:47⟩ | 166 XORs: 44 + 48 + 74, and a second point 43 + 48 + 75 | 167 (ours); 170 in PLinOpt | characteristic 2 | 2.777294 | 2026-11-08 | none |

### Lowest rank for rectangular formats

| Scheme | Format | Rank | Previous best: table / catalog | Valid over | Exponent (previous) | Operations: additions + scalings | Found | Lean proof |
|---|---|---|---|---|---|---|---|---|
| [13x14x14_1564_rational](schemes/13x14x14_1564_rational) | ⟨13×14×14⟩ | 1564 | 1579 / 1582 | 2 and 5 invertible | 2.813315 (2.816966) | 32,015 + 12,945 | 2026-11-11 | none |
| [13x15x15_1763_rational](schemes/13x15x15_1763_rational) | ⟨13×15×15⟩ | 1763 | 1776 / 1776 | 2, 5 and 17 invertible | 2.809695 (2.812457) | 16,335 + 3,666 | 2026-11-10 | none |
| [14x14x15_1715_rational](schemes/14x14x15_1715_rational) | ⟨14×14×15⟩ | 1715 | 1789 / 1790 | 2, 5 and 17 invertible | 2.797526 (2.813395) | 16,825 + 3,890 | 2026-11-10 | none |
| [14x15x15_1842_rational](schemes/14x15x15_1842_rational) | ⟨14×15×15⟩ | 1842 | 1848 / 1848 | 2, 5 and 17 invertible | 2.800171 (2.801382) | 17,598 + 3,933 | 2026-11-10 | none |
| [15x16x16_2202_rational](schemes/15x16x16_2202_rational) | ⟨15×16×16⟩ | 2202 | 2220 / 2223 | 2, 3 and 5 invertible | 2.797859 (2.800818) | 43,925 + 15,699 | 2026-11-11 | none |
| [13x14x16_1805_ternary](schemes/13x14x16_1805_ternary) | ⟨13×14×16⟩ | 1805 | with integer coefficients: 1806 in the catalog; with rational ones: 1796 / 1796 | every ring | 2.820119 (2.818239 with rational coefficients) | 10,903 + 0 | 2026-11-10 | none |

The ternary 13×14×16 scheme is a record only among schemes with integer coefficients, which work over every ring; a
rational scheme of rank 1796 is known.

### Near records

| Scheme | Format | What it is | Record | Valid over | Exponent | Found | Lean proof |
|---|---|---|---|---|---|---|---|
| [4x4x4_48_201-balanced](schemes/4x4x4_48_201-balanced) | ⟨4×4×4:48⟩ | a second 201: 56 + 56 + 89, 197 additions and 4 halvings | 201 (ties) | rings in which 2 is invertible | 2.792481 | 2026-10-21 | none |
| [4x4x4_47_mod_2_167](schemes/4x4x4_47_mod_2_167) | ⟨4×4×4:47⟩ | 167 XORs: 44 + 49 + 74, in Kauers and Moosbauer's class | 166 | characteristic 2 | 2.777294 | 2026-10-17 | none |
| [12x13x13_1275_rational](schemes/12x13x13_1275_rational) | ⟨12×13×13⟩ | rank 1275; 12,287 additions + 3,144 scalings | 1274 (table and catalog) | 2, 3 and 13 invertible | 2.817157 (record 2.816848) | 2026-11-11 | none |

## Conventions

**Format and rank.** ⟨m×k×n:r⟩ multiplies an m×k matrix A by a k×n matrix B with r multiplications.  The A side
forms one linear combination of A's entries for each product, the B side one of B's entries, and the C side recovers
the entries of C = AB as linear combinations of the products.

**Operations.** Additions are binary additions and subtractions; a negation is free.  Scalings are multiplications or
divisions by a constant other than ±1.  Over F₂ every addition is an XOR.  The counts are those of PLinOpt's
`OpCount.sh` on the programs in each folder, written A side + B side + C side.  For the rectangular schemes the
programs come from one randomized loop of PLinOpt's `optimizer -O 1`, so their counts are upper bounds that more
optimization would lower.

**Exponent.** 3 ln r / ln(mkn), as in the complexity column of fmm.univ-lille.fr: composing the scheme with its two
cyclic rotations gives a scheme for ⟨mkn×mkn×mkn:r³⟩, and recursion on it multiplies N×N matrices in O(N^ω) for this
ω.  For a square format ⟨n×n×n:r⟩ it is log_n r.  Used recursively, a square scheme whose three maps take q
operations multiplies N×N matrices in (1 + q/(r − n²)) N^ω + O(N²) operations, so q fixes the leading constant: 4.93
for the 55, 4.64 for KS-51 (plus O(N² log N) for its basis changes), 7.28 for the 201 and 6.35 for the 166.

**Rings and fields.** Where a scheme works depends on its coefficients:

- integer coefficients (here 0, ±1): every ring;
- coefficients in ℤ[1/2]: every ring in which 2 is invertible, such as ℚ, ℝ, ℂ and F_p for odd p;
- rational coefficients whose denominators have the prime factors p₁, …, p_k: every ring in which those primes are
  invertible; for the finite fields F_p, every prime p other than p₁, …, p_k;
- coefficients in F₂: every ring of characteristic 2, such as F₂, F₄ and F_{2^k}.

In every scheme each product keeps its A factor on the left, so the schemes also work over non-commutative rings, in
particular on block matrices, which is what recursion needs.  The exact Brent equations are what guarantee this; the
rectangular schemes were also run on matrices whose entries are 2×2 integer matrices.

Rank 47 for ⟨4×4×4⟩ is known only in characteristic 2.  In characteristic 0 the best known rank is 48.

## Files

Each folder holds `NAME_L.sms`, `NAME_R.sms` and `NAME_P.sms` and a program `.slp` for each, in PLinOpt's
conventions:

- `L` is r × (mk) and gives the A factor of each product from vec(A), `R` is r × (kn) on vec(B), and `P` is (mn) × r
  and gives vec(C) from the products, all vectors row-major.
- `.sms`: a header `rows cols R` (`M` in the F₂ files), then one line `i j value` per nonzero entry, 1-based, with
  integer or `a/b` values, and a final `0 0 0`.  A first line starting with `#` is a comment.
- `.slp`: one assignment per line; inputs are `i0, i1, …`, outputs `o0, o1, …`, and other names are temporaries.
- KS-51 is in alternative-basis form, in the same layout as PLinOpt's `4x4x4_48_204-16ALT/CoB` files:
  `-CoB_{L,R,P}` are the basis changes, `-ALT_{L,R,P}` the core, and L = ALT_L·CoB_L, R = ALT_R·CoB_R,
  P = CoB_P·ALT_P.  The plain `_L/_R/_P` files of that folder are these products: the scheme in the standard basis,
  whose programs take 62 additions.
- The rectangular schemes are also given as `.json.gz` in the JSON layout of Perminov's catalog (fields `n`, `m`, `u`,
  `v`, `w`; w indexes C transposed).

## Checking

```sh
python3 tools/check_scheme.py schemes/4x4x4_48_201/4x4x4_48_201
python3 tools/check_scheme.py --mod 2 schemes/4x4x4_47_mod_2_166/4x4x4_47_mod_2_166
tools/check_all.sh                              # every scheme, Python standard library only
PLINOPT=/path/to/plinopt tools/check_all.sh     # also PLinOpt's SLPchecker, MMchecker and ALTchecker
```

`check_scheme.py` checks the 3×3 and 4×4 schemes against the Brent equations, exactly: every coefficient of the
trilinear form (729 or 4,096 of them) must be right, modulo 2 for the F₂ schemes.  The rectangular schemes are too big
for that in plain Python, so it runs them on random integer matrices with entries up to 10⁹ in exact rational
arithmetic, where a wrong scheme passes a trial with probability below 10⁻⁶.  Every run also changes one coefficient
and requires the check to fail.  [checks/check_all.log](checks/check_all.log) is the output of the last full run with
PLinOpt.

The rectangular schemes were verified exactly against the Brent equations when they were found.  Two independent
routes were used: an exact modular contraction with a bound on the entries, and an integer contraction after clearing
denominators.  Details and logs are in
[matmul/tomar_line](https://github.com/gsidebottom/logic/tree/master/matmul/tomar_line) of the logic repository.

### Lean proofs

```sh
cd lean
lake exe cache get    # prebuilt Mathlib
lake build
```

Lean 4.33.0 and Mathlib v4.33.0, pinned in `lean-toolchain` and `lake-manifest.json`.  After the Mathlib cache the
build takes about 2 minutes, most of it `Ring_4x4_48_201.lean`.  No proof uses `sorry`, and the build prints the
axioms of each headline theorem.

| Theorem | File | Statement | Axioms |
|---|---|---|---|
| `Matmul55.correct` | [Correct.lean](lean/Mm/Correct.lean) | the 55-addition program's 9 outputs are the entries of AB, over any ring, commutative or not | `propext` |
| `Matmul55.scheme_eq_mul` | [Matrix.lean](lean/Mm/Matrix.lean) | `scheme A B = A * B` for 3×3 matrices in Mathlib's `Matrix` API | the standard three |
| `Ring51.correct` | [Ring51.lean](lean/Mm/Ring51.lean) | KS-51, basis changes included, computes AB over any ring | `propext` |
| `Ring51.rank23_add51_scheme_eq_mul` | [Ring51.lean](lean/Mm/Ring51.lean) | the same as a `Matrix` function | the standard three |
| `Matmul48_201.correct` | [Correct48_201.lean](lean/Mm/Correct48_201.lean) | the 201 programs compute AB over every commutative ring with an `h` such that `2 * h = 1` | `propext`, `Quot.sound` |
| `Ring48_201.rank48_op201_scheme_eq_mul` | [Ring_4x4_48_201.lean](lean/Mm/Ring_4x4_48_201.lean) | `rank48_op201_scheme h A B = A * B` for 4×4 matrices over the same rings | the standard three |

The standard three axioms are `propext`, `Classical.choice` and `Quot.sound`.  The Lean files are copies of
[proof/Mm](https://github.com/gsidebottom/logic/tree/master/proof/Mm) in the logic repository, where their
generators live.  Their docstrings name paths in that repository.  The 201 proofs are about exactly the programs in
[schemes/4x4x4_48_201](schemes/4x4x4_48_201).

## The schemes

### 3x3x3_23_55: 55 additions for ⟨3×3×3:23⟩

23 multiplications and 55 additions, 13 on the A side, 14 on the B side and 28 on the C side, with coefficients 0
and ±1 and no change of basis.  When it was found the record was 56 (Yinqi Sun,
[arXiv:2604.27645](https://arxiv.org/abs/2604.27645), April 2026), after a chain 60, 59, 58, 56 driven by greedy
common-subexpression elimination.

How it was found (2026-07-05): we started from Perminov's 58-addition scheme cn122
([arXiv:2512.21980](https://arxiv.org/abs/2512.21980)), whose de Groote class is `i19w225c4efh` in the
Heule–Kauers–Seidl database, and minimized each side exactly.  The two input sides were done by exact chain
covering: iterative deepening over helper values, on Sun's chain-covering structure.  The output side was done
through the transposition principle: an addition-only map W costs as many additions as its transpose plus (inputs −
outputs), and the transpose of the 9×23 output map is small enough to minimize exactly.  Greedy elimination leaves
one addition on the output side; the exact minimum gives 13 + 14 + 28.  In October 2026 an exact count over every
point of every class in the database showed that this class is the only one that reaches 55.

Paper: [arXiv:2610.27451](https://arxiv.org/abs/2610.27451).  Artifacts: Zenodo,
[doi:10.5281/zenodo.21240904](https://doi.org/10.5281/zenodo.21240904).

### 3x3x3_23_ks51: 51 additions per recursive step for ⟨3×3×3:23⟩

In the alternative-basis model of Karstadt and Schwartz, A and B are transformed to a new basis once, the core scheme
recurses in that basis, and C is transformed back once.  Here the core takes 12 + 13 + 26 = 51 additions per recursive step;
the C side is the transpose of a 12-addition chain plus 23 − 9 = 14.  The basis changes take 4, 4 and 3 additions on
3×3 blocks, O(N² log N) in total.  All three are unimodular integer maps, so the scheme works over every ring.  Used
once, without recursion, the scheme in the standard basis takes 62 additions.

How it was found (2026-10-11): an exact free-basis minimization of each side, whose negative answers are exhaustion
proofs, was swept over the classes of the Heule–Kauers–Seidl database of rank-23 schemes.  Class `i12w211c23fg` has
exact sides 12, 13 and 12 over F₂, the last counted through its transpose, which gives 12 + 13 + (12 + 14) = 51.
An exact signed search then found a sign assignment valid over the integers, and the basis-change cost was reduced to 11 the next day.  The 51 is optimal for this class.  Exactly
four classes of the database reach 51, and none reaches 50.

Stapleton and Perminov posted a 51 first, on a different scheme ([arXiv:2609.32388](https://arxiv.org/abs/2609.32388),
2026-09-26), so this ties the record.  Both are one below Nielsen's 52
([arXiv:2609.06588](https://arxiv.org/abs/2609.06588)).

Paper: [arXiv:2611.12907](https://arxiv.org/abs/2611.12907).  Artifacts: Zenodo,
[doi:10.5281/zenodo.23305910](https://doi.org/10.5281/zenodo.23305910) (all versions).

### 4x4x4_48_201: 201 operations for ⟨4×4×4:48⟩ over ℤ[1/2]

48 multiplications.  The A side takes 53 additions and 2 halvings, the B side 55 and 2, the C side 89 additions: 197
additions and 4 multiplications by 1/2.  No line of the programs is a negation, so the count is the same when sign
changes are charged.  Used recursively it costs (7 + 9/32) N^2.7925 + O(N²) operations.

Previous best: 202 by Ruchit Patel's value-set search
([rp440/matmul-4x4-48-202](https://github.com/rp440/matmul-4x4-48-202), October 2026) and 204 published by Dumas,
Pernet, Sedoglavic, Stapleton and Tichavský (2026), both in the same class.  The class is the member t = 1/4 of Li,
Wang and Hu's one-parameter curve of rational rank-48 classes.

How it was found (2026-10-21): annealing over the class's symmetry orbit for representatives with a low operation
floor and few non-unit coefficients, then a Rust reimplementation of Patel's large-neighbourhood search, seeded from
the matrices alone, with the C side optimized through its transpose.  About twenty representatives reach 201 and none
we tried goes lower.  A SAT encoding shows that no recombination of the programs found by then reaches 200.

Also in PLinOpt as `data/4x4x4_48_201` ([jgdumas/plinopt#23](https://github.com/jgdumas/plinopt/pull/23), merged
2026-10-22), where J.-G. Dumas later compacted the programs.  The files here are the ones we submitted.

### 4x4x4_48_201-balanced: a second 201

201 = 56 + 56 + 89: 54 additions and 2 halvings on each input side, 89 additions on the C side.  Found 2026-10-21 by
the same search, from another representative of the same class.  Its programs have 22
lines that are negations, which are free in this count.  Also in PLinOpt as `data/4x4x4_48_201-balanced`
([jgdumas/plinopt#36](https://github.com/jgdumas/plinopt/pull/36), merged 2026-10-31), where J.-G. Dumas changed signs
within products so that no line is a negation.  The files here are the ones we submitted.  The Lean proofs are about
the 55 + 57 + 89 scheme above, not this one.

### 4x4x4_47_mod_2_166: 166 XORs for ⟨4×4×4:47⟩ over F₂

47 multiplications over F₂ with 44 + 48 + 74 = 166 XORs, and a second scheme of the same class with 43 + 48 + 75
(`4x4x4_47_mod_2_166b_*`).  Previous best: our 167 below, and 170 for PLinOpt's `data/4x4x4_47_mod_2`.

How it was found: the class was reached on the night of 2026-11-07 by restricting Kauers and Moosbauer's ⟨4×4×5:60⟩
scheme over F₂ to ⟨4×4×4⟩ and descending in the flip graph to rank 47.  Its symmetry orbit under GL(4,2)³ sandwiching
was then scanned with Boyar–Peralta-style heuristic programs for each side, the C side through
its transpose.  On 2026-11-08 an exact search over helper values settled the 283 best orbit points, which took one
side from 49 to 48.  Every orbit point the scan rated within 4 of its best was then
settled exactly, and none goes below 166; there is no lower bound for the whole orbit yet.

Also in PLinOpt as `data/4x4x4_47_mod_2_166` and `_166b`
([jgdumas/plinopt#39](https://github.com/jgdumas/plinopt/pull/39), merged 2026-11-10).

### 4x4x4_47_mod_2_167: 167 XORs over F₂

44 + 49 + 74 = 167 XORs, in the class of Kauers and Moosbauer's rank-47 scheme.  Found 2026-10-17 by simulated
annealing over the class's orbit, with moves by elementary matrices scored by Boyar–Peralta, and an exact helper
search that took the expensive side from 50 to 49.  It was the record until the 166.  Also in PLinOpt as
`data/4x4x4_47_mod_2_167` ([jgdumas/plinopt#22](https://github.com/jgdumas/plinopt/pull/22), merged 2026-10-19),
where J.-G. Dumas later compacted the A-side program; the files here are the ones we submitted.

### The rectangular schemes: projection and 2-reduction

Six of the seven start from the square schemes of Khoruzhii, Serafin, Gelß and Pokutta, *Local Improvements to
Trilinear Aggregation* ([khoruzhii/lita](https://github.com/khoruzhii/lita)), as included in Perminov's catalog.  The
steps:

1. **Project.** Delete one index, for example one row of A.  This gives a scheme for the next smaller format;
   products whose A factor becomes zero disappear.
2. **Reduce.** In the projected scheme the Kronecker products of two of the three factors of the products become
   linearly dependent; the scheme is *2-reducible* in LITA's sense.  Each dependency lets one product's third factor
   be folded into the others, which removes that product.
3. **Choose what to remove.** Which products the dependencies remove is a search.  An exchange search over pivot sets
   picks them so that many more products vanish than there are dependencies: 14 dependencies remove 30 products for
   13×14×14, and 17 remove 113 for 14×15×15.

The pivot search runs modulo the prime 1048573.  The reduction is then replayed exactly over ℚ, with the
dependencies lifted from several primes.  The result is verified against the Brent equations by two independent
routes, and a copy with one coefficient changed must be rejected.  Finally an independent program runs it on random
integer matrices.  The method is LITA's, and Perminov's catalog already holds projected LITA schemes; our part is the choice of which
products to remove.  The seventh scheme
is a direct 2-reduction of a ternary catalog scheme by MerlijnW70
([MerlijnW70/fmm-schemes](https://github.com/MerlijnW70/fmm-schemes)).

| Scheme | Source | Deleted | Products left | Dependencies | Products removed | Rank | Found |
|---|---|---|---|---|---|---|---|
| 13x14x14_1564 | LITA ⟨14×14×14:1594⟩ | row 7 of A | 1594 | 14 | 30 | 1564 | 2026-11-10, verified 2026-11-11 |
| 15x16x16_2202 | LITA ⟨16×16×16:2237⟩ | row 9 of A | 2237 | 16 | 35 | 2202 | 2026-11-10, verified 2026-11-11 |
| 14x15x15_1842 | LITA ⟨15×15×15:1977⟩ | row 0 of A | 1955 | 17 | 113 | 1842 | 2026-11-10 |
| 14x14x15_1715 | the 14x15x15 scheme above | inner index 0: column 0 of A, row 0 of B | 1827 | 17 | 112 | 1715 | 2026-11-10 |
| 13x15x15_1763 | the 14x15x15 scheme above | row 7 of A | 1797 | 21 | 34 | 1763 | 2026-11-10 |
| 12x13x13_1275 | LITA ⟨13×13×13:1379⟩ | row 0 of A | 1360 | 15 | 85 | 1275 | 2026-11-10, verified 2026-11-11 |
| 13x14x16_1805 | MerlijnW70's ⟨13×14×16:1806⟩, ternary | none | 1806 | 1 | 1 | 1805 | 2026-11-10 |

Coefficients: the six projected schemes are over ℚ and not dyadic.  After each product is rescaled so that its A and
B factors are primitive integer vectors, all denominators sit on the C side, and each entry of C needs one division
by a constant.

| Scheme | Denominator primes | Largest denominator | Divisions after rescaling | Valid over F_p for |
|---|---|---|---|---|
| 13x14x14_1564 | 2, 5 | 200 | 182, one per entry of C | p ≠ 2, 5 |
| 13x15x15_1763 | 2, 5, 17 | 1088 | 195 | p ≠ 2, 5, 17 |
| 14x14x15_1715 | 2, 5, 17 | 1088 | 210 | p ≠ 2, 5, 17 |
| 14x15x15_1842 | 2, 5, 17 | 1088 | 210 | p ≠ 2, 5, 17 |
| 15x16x16_2202 | 2, 3, 5 | 720 | 240 | p ≠ 2, 3, 5 |
| 12x13x13_1275 | 2, 3, 13 | 117 | 156 | p ≠ 2, 3, 13 |
| 13x14x16_1805 | none (coefficients −1, 0, 1) | 1 | 0 | every p |

Programs, totals over the three sides; the divisions and integer multiplications together are the scalings:

| Scheme | Additions | Divisions (of which by a power of 2) | Integer multiplications | Total |
|---|---|---|---|---|
| 13x14x14_1564 | 32,015 | 11,422 (1,223) | 1,523 | 44,960 |
| 13x15x15_1763 | 16,335 | 2,495 (694) | 1,171 | 20,001 |
| 14x14x15_1715 | 16,825 | 2,680 (792) | 1,210 | 20,715 |
| 14x15x15_1842 | 17,598 | 2,741 (845) | 1,192 | 21,531 |
| 15x16x16_2202 | 43,925 | 13,636 (2,368) | 2,063 | 59,624 |
| 12x13x13_1275 | 12,287 | 2,160 (1,735) | 984 | 15,431 |
| 13x14x16_1805 | 10,903 | 0 | 0 | 10,903 |

The first five are in PLinOpt's pull request [jgdumas/plinopt#40](https://github.com/jgdumas/plinopt/pull/40), opened
2026-11-11.

## Credits

- Khoruzhii, Serafin, Gelß and Pokutta ([LITA](https://github.com/khoruzhii/lita)): the square schemes behind six of
  the rectangular ones, and 2-reducibility, the reduction they use.
- Andrew Perminov's [catalog](https://github.com/dronperminov/FastMatrixMultiplication), where we found LITA's and
  MerlijnW70's schemes, and his 58-addition scheme cn122, the start of the 55.
- Alexandre Sedoglavic's table at [fmm.univ-lille.fr](https://fmm.univ-lille.fr/).
- Heule, Kauers and Seidl's database of rank-23 3×3 schemes; Yinqi Sun's chain covering; Karstadt and Schwartz's
  alternative-basis model; Nielsen's 52; Stapleton and Perminov's 51.
- Dumas, Pernet and Sedoglavic for the rational rank-48 class, made from AlphaEvolve's complex scheme, and Li, Wang
  and Hu for the curve it lies on; Ruchit Patel for the value-set search.
- Kauers and Moosbauer for flip graphs, their ⟨4×4×5:60⟩ scheme and their rank-47 class.
- Jean-Guillaume Dumas and the PLinOpt authors for the format and the checking and optimization tools.
- The searches, checks and write-ups were done with Claude Code (Anthropic).

## License

[Apache License 2.0](LICENSE).
