# mat_mul

Matrix multiplication schemes found by Greg Sidebottom in 2026: records for the number of multiplications (the rank)
or for the number of additions, and a few near records.  Every scheme in `schemes/` is in the format of
[PLinOpt](https://github.com/jgdumas/plinopt), with a straight-line program for each of its three linear maps, and has
been checked exactly.  Three of them come with Lean 4 proofs that they compute the matrix product.  The rank records
for formats with a dimension above 16 are in `large/` (deletions from LITA's squares) and `products/` (shared-factor
products and deletions from them), as compressed JSON with an exact certificate each.

Status as of 2026-11-21.  "Previous best" is what we knew of when the scheme was found; the rank comparisons use
Alexandre Sedoglavic's table at [fmm.univ-lille.fr](https://fmm.univ-lille.fr/) (its `fmm_sota.json` of 2026-11-09)
and Andrew Perminov's catalog
[FastMatrixMultiplication](https://github.com/dronperminov/FastMatrixMultiplication) (commit 6f8ef61 of 2026-11-09
for the schemes of 2026-11-10 and 11; commit f70078c of 2026-11-16, still its head on 2026-11-20, for those of
2026-11-20).  On 2026-11-15 the table, now published at
[sedoglavic/fmm_digest](https://github.com/sedoglavic/fmm_digest), took up our five schemes of 2026-11-10 and 11
(it lists 1565 for ⟨13×14×14⟩, where our scheme has 1564, and 1763, 1715, 1842 and 2202 for the others) and Arhan
Canli's 708 for ⟨9×11×11⟩.  Every rank record below is still below that table, and the schemes of `large/` and
`products/` are compared with it (commit 8e6ad05).

## Index

### Fewest operations at a given rank

| Scheme | Format | Operations | Previous best | Valid over | Exponent | Found | Lean proof |
|---|---|---|---|---|---|---|---|
| [3x3x3_23_55](schemes/3x3x3_23_55) | ⟨3×3×3:23⟩ | 55 additions: 13 + 14 + 28 | 56 (Sun, April 2026) | every ring | 2.854050 | 2026-07-05 | [Correct.lean](lean/Mm/Correct.lean), [Matrix.lean](lean/Mm/Matrix.lean) |
| [3x3x3_23_ks51](schemes/3x3x3_23_ks51) | ⟨3×3×3:23⟩ | 51 additions per recursive step: 12 + 13 + 26, in an alternative basis; the basis changes take 4 + 4 + 3 more | ties Stapleton and Perminov's 51 (September 2026, posted first); Nielsen's 52 | every ring | 2.854050 | 2026-10-11 | [Ring51.lean](lean/Mm/Ring51.lean) |
| [4x4x4_48_201](schemes/4x4x4_48_201) | ⟨4×4×4:48⟩ | 201: 197 additions and 4 halvings; 55 + 57 + 89 | 202 (Patel, October 2026); 204 published | rings in which 2 is invertible | 2.792481 | 2026-10-21 | [Correct48_201.lean](lean/Mm/Correct48_201.lean), [Ring_4x4_48_201.lean](lean/Mm/Ring_4x4_48_201.lean) |
| [4x4x4_47_mod_2_166](schemes/4x4x4_47_mod_2_166) | ⟨4×4×4:47⟩ | 166 XORs: 44 + 48 + 74, and a second point 43 + 48 + 75 | 167 (ours); 170 in PLinOpt | characteristic 2 | 2.777294 | 2026-11-08 | none |

### Lowest rank for rectangular formats

Grouped by what the coefficients need: integers, which work over every ring; ℤ[1/2], which needs only 2 to be
invertible; and rationals with other denominators.  "Previous best" is Sedoglavic's table / Perminov's catalog when
the scheme was found, and "(ours)" marks a catalog entry that is one of our earlier schemes.  None of these has a
Lean proof.

#### Integer coefficients: valid over every ring

| Scheme | Format | Rank | Previous best: table / catalog | Valid over | Exponent (previous) | Operations: additions + scalings | Found |
|---|---|---|---|---|---|---|---|
| [4x11x14_426_integer](schemes/4x11x14_426_integer) | ⟨4×11×14⟩ | 426 | 429 / 429 | every ring | 2.827747 (2.831025) | 2,655 + 10 | 2026-11-20 |
| [13x14x16_1805_ternary](schemes/13x14x16_1805_ternary) | ⟨13×14×16⟩ | 1805 | with integer coefficients: 1806 in the catalog; with rational ones: 1796 / 1796 | every ring | 2.820119 (2.818239 with rational coefficients) | 10,903 + 0 | 2026-11-10 |

The ⟨4×11×14⟩ scheme has coefficients between −2 and 2 and is below the table and the catalog outright; the catalog's
best integer scheme for the format has rank 429 as well.  The ternary 13×14×16 scheme is a record only among schemes
with integer coefficients; a rational scheme of rank 1796 is known.

#### Coefficients in ℤ[1/2]: valid wherever 2 is invertible

| Scheme | Format | Rank | Previous best: table / catalog | Valid over | Exponent (previous) | Operations: additions + scalings | Found |
|---|---|---|---|---|---|---|---|
| [4x11x13_397_dyadic](schemes/4x11x13_397_dyadic) | ⟨4×11×13⟩ | 397 | 400 / 400 | 2 invertible | 2.827440 (2.830997) | 2,602 + 72 | 2026-11-20 |
| [4x11x16_477_dyadic](schemes/4x11x16_477_dyadic) | ⟨4×11×16⟩ | 477 | 480 / 480 | 2 invertible | 2.821896 (2.824765) | 2,702 + 6 | 2026-11-20 |
| [4x13x14_498_dyadic](schemes/4x13x14_498_dyadic) | ⟨4×13×14⟩ | 498 | 500 / 500 | 2 invertible | 2.827155 (2.828979) | 7,631 + 4,261 | 2026-11-20 |
| [8x11x13_749_dyadic](schemes/8x11x13_749_dyadic) | ⟨8×11×13⟩ | 749 | 750 / 750 | 2 invertible | 2.819570 (2.820138) | 7,502 + 1,606 | 2026-11-20 |
| [9x11x11_705_dyadic](schemes/9x11x11_705_dyadic) | ⟨9×11×11⟩ | 705 | 710 / 708 | 2 invertible | 2.813464 (2.815285) | 7,347 + 2,674 | 2026-11-20 |
| [9x11x16_987_dyadic](schemes/9x11x16_987_dyadic) | ⟨9×11×16⟩ | 987 | 996 / 996 | 2 invertible | 2.807387 (2.811083) | 9,985 + 3,807 | 2026-11-20 |
| [11x13x16_1444_dyadic](schemes/11x13x16_1444_dyadic) | ⟨11×13×16⟩ | 1444 | 1446 / 1446 | 2 invertible | 2.821499 (2.822036) | 9,988 + 457 | 2026-11-20 |

Every denominator in these seven is a power of 2, at most 16, and their programs divide by powers of 2 only.

#### Rational coefficients with other denominators

| Scheme | Format | Rank | Previous best: table / catalog | Valid over | Exponent (previous) | Operations: additions + scalings | Found |
|---|---|---|---|---|---|---|---|
| [9x11x15_953_rational](schemes/9x11x15_953_rational) | ⟨9×11×15⟩ | 953 | 956 / 956 | 2 and 3 invertible | 2.817796 (2.819087) | 10,054 + 3,403 | 2026-11-20 |
| [13x13x14_1477_rational](schemes/13x13x14_1477_rational) | ⟨13×13×14⟩ | 1477 | 1511 / 1511 | 2, 5 and 17 invertible | 2.818050 (2.826838) | 14,475 + 5,231 | 2026-11-20 |
| [13x13x15_1577_rational](schemes/13x13x15_1577_rational) | ⟨13×13×15⟩ | 1577 | 1591 / 1591 | 2, 5 and 17 invertible | 2.818319 (2.821702) | 14,718 + 5,044 | 2026-11-20 |
| [13x14x14_1538_rational](schemes/13x14x14_1538_rational) | ⟨13×14×14⟩ | 1538 | 1579 / 1564 (ours) | 2, 5 and 17 invertible | 2.806902 (2.813315) | 15,078 + 5,447 | 2026-11-20 |
| [13x14x15_1645_rational](schemes/13x14x15_1645_rational) | ⟨13×14×15⟩ | 1645 | 1681 / 1681 | 2, 5 and 17 invertible | 2.807928 (2.816137) | 15,678 + 5,594 | 2026-11-20 |
| [13x15x15_1762_rational](schemes/13x15x15_1762_rational) | ⟨13×15×15⟩ | 1762 | 1776 / 1763 (ours) | 2, 5 and 17 invertible | 2.809482 (2.809695) | 16,036 + 5,424 | 2026-11-20 |
| [14x14x15_1715_rational](schemes/14x14x15_1715_rational) | ⟨14×14×15⟩ | 1715 | 1789 / 1790 | 2, 5 and 17 invertible | 2.797526 (2.813395) | 16,825 + 3,890 | 2026-11-10 |
| [14x15x15_1842_rational](schemes/14x15x15_1842_rational) | ⟨14×15×15⟩ | 1842 | 1848 / 1848 | 2, 5 and 17 invertible | 2.800171 (2.801382) | 17,598 + 3,933 | 2026-11-10 |
| [15x16x16_2200_rational](schemes/15x16x16_2200_rational) | ⟨15×16×16⟩ | 2200 | 2220 / 2223; ours 2202 | 2, 3, 5 and 7 invertible | 2.797528 (2.797859) | 43,330 + 22,473 | 2026-11-20 |

⟨15×16×16⟩ has since gone lower: [15x16x16_2193_rational](large/15x16x16_2193_rational), found 2026-11-21 from LITA's
⟨17×17×17:2723⟩ by four deletions, has rank 2193.  It is listed with the larger formats below and is in `large/` as
compressed JSON without PLinOpt programs; 15x16x16_2200 stays here as the lowest rank we have with programs.

A scheme for ⟨m×k×n⟩ gives one of the same rank for every ordering of m, k and n, by cyclic rotation and
transposition of the trilinear form, so the tables of records index formats by the sorted shape, and every scheme
here is stored in that ordering.  Four of them (13x13x14_1477, 13x13x15_1577, 13x14x15_1645 and 15x16x16_2200) were
found in another ordering and rotated or transposed with [tools/scheme_orient.py](tools/scheme_orient.py); the result
was verified exactly again ([sorted_formats.verify.log](checks/discovery/sorted_formats.verify.log)).

#### Larger formats: deletions from LITA's squares 17³ to 32³

<!-- large:begin -->
106 formats with a dimension above 16 and ⟨15×16×16⟩, each below Sedoglavic's table of 2026-11-15 (by 9 to 723 products, 21,144 in total).  Every scheme is one of LITA's square schemes ⟨k×k×k⟩ (here k = 17, 19, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32) with one or more indices deleted, each deletion followed by LITA's 2-reduction.  The square schemes and the reduction are theirs; the choice of deletions and of the products to drop is ours.

- **Deletions.** `I5` removes row 5 of A, `J5` the inner index 5 (a column of A with the matching row of B) and `K5` column 5 of B, counted from 0 in the scheme as it stands after the earlier deletions.
- **Coefficients.** All are rational; "Invertible" lists the primes that occur in denominators.
- **What is listed.** The lowest rank we certified for each format, and only formats for which no larger format in this list has the same or a lower rank.
- **Form.** Compressed JSON in [large/](large), one folder per scheme, not PLinOpt files: see [Files](#files).  Each was certified by evaluating every Brent equation in exact integer arithmetic ([checks/check_large.log](checks/check_large.log)).
- **No file above 10 MB.** 9 schemes marked "no file" are reported here only: their compressed files are 12 to 17 MB.  Each of their folders holds the paths and the certificate log, which ends with the exact check of the scheme; `tools/kronexact replay` on the paths rebuilds and re-certifies it from LITA's file.

| Scheme | Format | Rank | Table | Gain | From LITA's | Deletions | Invertible | Exponent (table's) | Found |
|---|---|---|---|---|---|---|---|---|---|
| [14x16x17_2278_rational](large/14x16x17_2278_rational) | ⟨14×16×17⟩ | 2278 | 2306 | 28 | ⟨17×17×17:2723⟩ | `I0`, `J0`, `I15`, `I0` | 2, 3, 5, 7 | 2.813045 (2.817490) | 2026-11-21 |
| [14x17x17_2435_rational](large/14x17x17_2435_rational) | ⟨14×17×17⟩ | 2435 | 2456 | 21 | ⟨17×17×17:2723⟩ | `I0`, `I15`, `I0` | 2, 3, 5, 7 | 2.816586 (2.819687) | 2026-11-20 |
| [15x15x17_2286_rational](large/15x15x17_2286_rational) | ⟨15×15×17⟩ | 2286 | 2305 | 19 | ⟨17×17×17:2723⟩ | `I0`, `J0`, `I15`, `J0` | 2, 3, 5, 7 | 2.812801 (2.815811) | 2026-11-21 |
| [15x16x16_2193_rational](large/15x16x16_2193_rational) | ⟨15×16×16⟩ | 2193 | 2202 | 9 | ⟨17×17×17:2723⟩ | `I0`, `J0`, `I15`, `K0` | 2, 3, 5, 7 | 2.796370 (2.797859) | 2026-11-21 |
| [15x16x17_2334_rational](large/15x16x17_2334_rational) | ⟨15×16×17⟩ | 2334 | 2442 | 108 | ⟨17×17×17:2723⟩ | `I0`, `J0`, `I15` | 2, 3, 5, 7 | 2.798464 (2.814786) | 2026-11-20 |
| [15x17x17_2490_rational](large/15x17x17_2490_rational) | ⟨15×17×17⟩ | 2490 | 2622 | 132 | ⟨17×17×17:2723⟩ | `I0`, `I15` | 2, 3, 5, 7 | 2.801383 (2.819887) | 2026-11-20 |
| [15x19x19_3229_rational](large/15x19x19_3229_rational) | ⟨15×19×19⟩ | 3229 | 3260 | 31 | ⟨19×19×19:3633⟩ | `I0`, `I17`, `I0`, `I0` | 2, 3, 5, 7 | 2.819587 (2.822921) | 2026-11-21 |
| [16x16x17_2392_rational](large/16x16x17_2392_rational) | ⟨16×16×17⟩ | 2392 | 2492 | 100 | ⟨17×17×17:2723⟩ | `I0`, `J0` | 2, 3, 5, 7 | 2.785697 (2.800361) | 2026-11-20 |
| [16x17x17_2553_rational](large/16x17x17_2553_rational) | ⟨16×17×17⟩ | 2553 | 2706 | 153 | ⟨17×17×17:2723⟩ | `I0` | 2, 3, 5, 7 | 2.788841 (2.809532) | 2026-11-20 |
| [16x17x18_2864_rational](large/16x17x18_2864_rational) | ⟨16×17×18⟩ | 2864 | 2884 | 20 | ⟨19×19×19:3633⟩ | `I0`, `J0`, `I17`, `J0`, `I0`, `K18` | 2, 3, 5, 7 | 2.810668 (2.813125) | 2026-11-21 |
| [16x17x19_3029_rational](large/16x17x19_3029_rational) | ⟨16×17×19⟩ | 3029 | 3066 | 37 | ⟨19×19×19:3633⟩ | `I0`, `J0`, `I17`, `J0`, `I0` | 2, 3, 5, 7 | 2.812548 (2.816808) | 2026-11-21 |
| [16x18x19_3102_rational](large/16x18x19_3102_rational) | ⟨16×18×19⟩ | 3102 | 3204 | 102 | ⟨19×19×19:3633⟩ | `I0`, `J0`, `I17`, `I16` | 2, 3, 5, 7 | 2.802171 (2.813447) | 2026-11-21 |
| [16x19x19_3293_rational](large/16x19x19_3293_rational) | ⟨16×19×19⟩ | 3293 | 3408 | 115 | ⟨19×19×19:3633⟩ | `I0`, `I17`, `I0` | 2, 3, 5, 7 | 2.805375 (2.817265) | 2026-11-20 |
| [17x17x18_2923_rational](large/17x17x18_2923_rational) | ⟨17×17×18⟩ | 2923 | 3012 | 89 | ⟨19×19×19:3633⟩ | `I0`, `J0`, `I17`, `K0`, `J0` | 2, 3, 5, 7 | 2.797904 (2.808419) | 2026-11-21 |
| [17x17x19_3106_rational](large/17x17x19_3106_rational) | ⟨17×17×19⟩ | 3106 | 3165 | 59 | ⟨19×19×19:3633⟩ | `I0`, `J0`, `I17`, `J0` | 2, 3, 5, 7 | 2.801492 (2.808048) | 2026-11-21 |
| [17x18x18_2982_rational](large/17x18x18_2982_rational) | ⟨17×18×18⟩ | 2982 | 3031 | 49 | ⟨19×19×19:3633⟩ | `I0`, `J0`, `I17`, `K0` | 2, 3, 5, 7 | 2.786298 (2.791974) | 2026-11-21 |
| [17x18x19_3160_rational](large/17x18x19_3160_rational) | ⟨17×18×19⟩ | 3160 | 3337 | 177 | ⟨19×19×19:3633⟩ | `I0`, `J0`, `I17` | 2, 3, 5, 7 | 2.788984 (2.807847) | 2026-11-20 |
| [17x19x19_3354_rational](large/17x19x19_3354_rational) | ⟨17×19×19⟩ | 3354 | 3633 | 279 | ⟨19×19×19:3633⟩ | `I0`, `I17` | 2, 3, 5, 7 | 2.792189 (2.819673) | 2026-11-20 |
| [17x19x20_3703_rational](large/17x19x20_3703_rational) | ⟨17×19×20⟩ | 3703 | 3800 | 97 | ⟨21×21×21:4723⟩ | `I0`, `J0`, `I0`, `I0`, `K20`, `I0`, `K0` | 2, 7, 29 | 2.809713 (2.818555) | 2026-11-21 |
| [17x20x20_3796_rational](large/17x20x20_3796_rational) | ⟨17×20×20⟩ | 3796 | 3972 | 176 | ⟨21×21×21:4723⟩ | `I0`, `J0`, `I0`, `I0`, `K20`, `I0` | 2, 7, 29 | 2.801814 (2.817222) | 2026-11-21 |
| [17x20x21_3997_rational](large/17x20x21_3997_rational) | ⟨17×20×21⟩ | 3997 | 4174 | 177 | ⟨21×21×21:4723⟩ | `I0`, `J0`, `I0`, `I0`, `I17` | 2, 7, 29 | 2.803853 (2.818502) | 2026-11-21 |
| [17x21x21_4258_rational](large/17x21x21_4258_rational) | ⟨17×21×21⟩ | 4258 | 4310 | 52 | ⟨21×21×21:4723⟩ | `I0`, `I19`, `I18`, `I0` | 2, 7, 29 | 2.809789 (2.813871) | 2026-11-21 |
| [18x18x19_3225_rational](large/18x18x19_3225_rational) | ⟨18×18×19⟩ | 3225 | 3355 | 130 | ⟨19×19×19:3633⟩ | `I0`, `J0` | 2, 3, 5, 7 | 2.777714 (2.791302) | 2026-11-20 |
| [18x19x19_3424_rational](large/18x19x19_3424_rational) | ⟨18×19×19⟩ | 3424 | 3633 | 209 | ⟨19×19×19:3633⟩ | `I0` | 2, 3, 5, 7 | 2.781068 (2.801315) | 2026-11-20 |
| [18x19x20_3784_rational](large/18x19x20_3784_rational) | ⟨18×19×20⟩ | 3784 | 3878 | 94 | ⟨21×21×21:4723⟩ | `I0`, `J0`, `I0`, `I0`, `K20`, `K0` | 2, 7, 29 | 2.798878 (2.807214) | 2026-11-21 |
| [18x19x21_4012_rational](large/18x19x21_4012_rational) | ⟨18×19×21⟩ | 4012 | 4090 | 78 | ⟨21×21×21:4723⟩ | `I0`, `J0`, `I0`, `J19`, `I0` | 2, 3, 7, 29 | 2.803266 (2.809772) | 2026-11-21 |
| [18x20x20_3865_rational](large/18x20x20_3865_rational) | ⟨18×20×20⟩ | 3865 | 3994 | 129 | ⟨21×21×21:4723⟩ | `I0`, `J0`, `I0`, `I0`, `K20` | 2, 7, 29 | 2.789868 (2.800957) | 2026-11-21 |
| [18x20x21_4073_rational](large/18x20x21_4073_rational) | ⟨18×20×21⟩ | 4073 | 4257 | 184 | ⟨21×21×21:4723⟩ | `I0`, `J0`, `I0`, `I0` | 2, 7, 29 | 2.792235 (2.807077) | 2026-11-21 |
| [18x21x21_4327_rational](large/18x21x21_4327_rational) | ⟨18×21×21⟩ | 4327 | 4440 | 113 | ⟨21×21×21:4723⟩ | `I0`, `I19`, `I18` | 2, 7, 29 | 2.797274 (2.805887) | 2026-11-20 |
| [19x19x20_3868_rational](large/19x19x20_3868_rational) | ⟨19×19×20⟩ | 3868 | 3994 | 126 | ⟨21×21×21:4723⟩ | `I0`, `J0`, `I0`, `K20`, `K0` | 2, 7, 29 | 2.789259 (2.800083) | 2026-11-21 |
| [19x19x21_4092_rational](large/19x19x21_4092_rational) | ⟨19×19×21⟩ | 4092 | 4184 | 92 | ⟨21×21×21:4723⟩ | `I0`, `J0`, `I0`, `J19` | 2, 7, 29 | 2.792931 (2.800397) | 2026-11-21 |
| [19x20x20_3935_rational](large/19x20x20_3935_rational) | ⟨19×20×20⟩ | 3935 | 3994 | 59 | ⟨21×21×21:4723⟩ | `I0`, `J0`, `I0`, `K20` | 2, 7, 29 | 2.779014 (2.784010) | 2026-11-21 |
| [19x20x21_4154_rational](large/19x20x21_4154_rational) | ⟨19×20×21⟩ | 4154 | 4374 | 220 | ⟨21×21×21:4723⟩ | `I0`, `J0`, `I0` | 2, 7, 29 | 2.782007 (2.799238) | 2026-11-20 |
| [19x21x21_4394_rational](large/19x21x21_4394_rational) | ⟨19×21×21⟩ | 4394 | 4723 | 329 | ⟨21×21×21:4723⟩ | `I0`, `I19` | 2, 7, 29 | 2.785635 (2.809614) | 2026-11-20 |
| [19x21x22_4914_rational](large/19x21x22_4914_rational) | ⟨19×21×22⟩ | 4914 | 4983 | 69 | ⟨22×22×22:5141⟩ | `J0`, `J0`, `J19`, `K0` | 2, 3, 11 | 2.808317 (2.812924) | 2026-11-21 |
| [19x22x22_4994_rational](large/19x22x22_4994_rational) | ⟨19×22×22⟩ | 4994 | 5141 | 147 | ⟨22×22×22:5141⟩ | `J0`, `J0`, `J19` | 2, 3, 11 | 2.799311 (2.808847) | 2026-11-20 |
| [19x22x23_5173_rational](large/19x22x23_5173_rational) | ⟨19×22×23⟩ | 5173 | 5440 | 267 | ⟨23×23×23:6009⟩ | `I0`, `J0`, `I21`, `I0`, `I0` | 2, 3, 11 | 2.797262 (2.813725) | 2026-11-21 |
| [19x23x23_5480_rational](large/19x23x23_5480_rational) | ⟨19×23×23⟩ | 5480 | 5656 | 176 | ⟨23×23×23:6009⟩ | `I0`, `I21`, `I20`, `I19` | 2, 3, 11 | 2.802538 (2.812828) | 2026-11-21 |
| [20x20x21_4230_rational](large/20x20x21_4230_rational) | ⟨20×20×21⟩ | 4230 | 4394 | 164 | ⟨21×21×21:4723⟩ | `I0`, `J0` | 2, 7, 29 | 2.772234 (2.784863) | 2026-11-20 |
| [20x21x21_4471_rational](large/20x21x21_4471_rational) | ⟨20×21×21⟩ | 4471 | 4723 | 252 | ⟨21×21×21:4723⟩ | `I0` | 2, 7, 29 | 2.775643 (2.793750) | 2026-11-20 |
| [20x21x22_4987_rational](large/20x21x22_4987_rational) | ⟨20×21×22⟩ | 4987 | 5035 | 48 | ⟨22×22×22:5141⟩ | `J0`, `K21`, `K0` | 2, 3, 11 | 2.797387 (2.800534) | 2026-11-20 |
| [20x21x23_5178_rational](large/20x21x23_5178_rational) | ⟨20×21×23⟩ | 5178 | 5330 | 152 | ⟨23×23×23:6009⟩ | `I0`, `J0`, `I21`, `J0`, `I0` | 2, 3, 11 | 2.796123 (2.805582) | 2026-11-21 |
| [20x22x22_5005_rational](large/20x22x22_5005_rational) | ⟨20×22×22⟩ | 5005 | 5141 | 136 | ⟨23×23×23:6009⟩ | `I0`, `J0`, `I21`, `I0`, `K22` | 2, 3, 11 | 2.784385 (2.793149) | 2026-11-21 |
| [20x22x23_5255_rational](large/20x22x23_5255_rational) | ⟨20×22×23⟩ | 5255 | 5581 | 326 | ⟨23×23×23:6009⟩ | `I0`, `J0`, `I21`, `I0` | 2, 3, 11 | 2.786820 (2.806399) | 2026-11-21 |
| [20x23x23_5554_rational](large/20x23x23_5554_rational) | ⟨20×23×23⟩ | 5554 | 5903 | 349 | ⟨23×23×23:6009⟩ | `I0`, `I21`, `I20` | 2, 3, 11 | 2.791367 (2.811097) | 2026-11-21 |
| [20x25x25_6828_rational](large/20x25x25_6828_rational) | ⟨20×25×25⟩ | 6828 | 6924 | 96 | ⟨25×25×25:7507⟩ | `I0`, `I0`, `I0`, `I21`, `I0` | 2, 3, 5, 37 | 2.807697 (2.812137) | 2026-11-21 |
| [21x21x22_5009_rational](large/21x21x22_5009_rational) | ⟨21×21×22⟩ | 5009 | 5141 | 132 | ⟨23×23×23:6009⟩ | `I0`, `J0`, `I21`, `K0`, `J0` | 2, 3, 11 | 2.783958 (2.792458) | 2026-11-21 |
| [21x21x23_5277_rational](large/21x21x23_5277_rational) | ⟨21×21×23⟩ | 5277 | 5395 | 118 | ⟨23×23×23:6009⟩ | `I0`, `J0`, `I21`, `J0` | 2, 3, 11 | 2.787493 (2.794685) | 2026-11-21 |
| [21x22x22_5082_rational](large/21x22x22_5082_rational) | ⟨21×22×22⟩ | 5082 | 5141 | 59 | ⟨23×23×23:6009⟩ | `I0`, `J0`, `I21`, `K0` | 2, 3, 11 | 2.774626 (2.778379) | 2026-11-21 |
| [21x22x23_5344_rational](large/21x22x23_5344_rational) | ⟨21×22×23⟩ | 5344 | 5603 | 259 | ⟨23×23×23:6009⟩ | `I0`, `J0`, `I21` | 2, 3, 11 | 2.777589 (2.792903) | 2026-11-21 |
| [21x23x23_5628_rational](large/21x23x23_5628_rational) | ⟨21×23×23⟩ | 5628 | 6009 | 381 | ⟨23×23×23:6009⟩ | `I0`, `I21` | 2, 3, 11 | 2.781010 (2.802105) | 2026-11-20 |
| [21x23x24_6224_rational](large/21x23x24_6224_rational) | ⟨21×23×24⟩ | 6224 | 6339 | 115 | ⟨24×24×24:6488⟩ | `J0`, `J0`, `J0`, `K0` | 2, 3, 5, 37 | 2.800631 (2.806500) | 2026-11-21 |
| [21x24x24_6331_rational](large/21x24x24_6331_rational) | ⟨21×24×24⟩ | 6331 | 6440 | 109 | ⟨24×24×24:6488⟩ | `J0`, `J0`, `J0` | 2, 3, 5, 37 | 2.793392 (2.798839) | 2026-11-21 |
| [21x24x25_6564_rational](large/21x24x25_6564_rational) | ⟨21×24×25⟩ | 6564 | 6924 | 360 | ⟨25×25×25:7507⟩ | `I0`, `J0`, `I23`, `I0`, `I0` | 2, 3, 5, 37 | 2.792798 (2.809763) | 2026-11-21 |
| [21x25x25_6911_rational](large/21x25x25_6911_rational) | ⟨21×25×25⟩ | 6911 | 7223 | 312 | ⟨25×25×25:7507⟩ | `I0`, `I0`, `I0`, `I21` | 2, 3, 5, 37 | 2.797073 (2.811043) | 2026-11-21 |
| [21x26x26_7757_rational](large/21x26x26_7757_rational) | ⟨21×26×26⟩ | 7757 | 7781 | 24 | ⟨26×26×26:8051⟩ | `J0`, `J0`, `J0`, `J0`, `J0` | 2, 5, 11 | 2.810360 (2.811329) | 2026-11-21 |
| [22x22x23_5423_rational](large/22x22x23_5423_rational) | ⟨22×22×23⟩ | 5423 | 5625 | 202 | ⟨23×23×23:6009⟩ | `I0`, `J0` | 2, 3, 11 | 2.768446 (2.780221) | 2026-11-20 |
| [22x23x23_5710_rational](large/22x23x23_5710_rational) | ⟨22×23×23⟩ | 5710 | 6009 | 299 | ⟨23×23×23:6009⟩ | `I0` | 2, 3, 11 | 2.771826 (2.788181) | 2026-11-20 |
| [22x23x24_6321_rational](large/22x23x24_6321_rational) | ⟨22×23×24⟩ | 6321 | 6396 | 75 | ⟨24×24×24:6488⟩ | `J0`, `J0`, `K0` | 2, 3, 5, 37 | 2.791711 (2.795474) | 2026-11-21 |
| [22x23x25_6557_rational](large/22x23x25_6557_rational) | ⟨22×23×25⟩ | 6557 | 6752 | 195 | ⟨25×25×25:7507⟩ | `I0`, `J0`, `I23`, `J0`, `I0` | 2, 3, 5, 37 | 2.791288 (2.800596) | 2026-11-21 |
| [22x24x24_6352_rational](large/22x24x24_6352_rational) | ⟨22×24×24⟩ | 6352 | 6488 | 136 | ⟨25×25×25:7507⟩ | `I0`, `J0`, `I23`, `K0`, `I0` | 2, 3, 5, 37 | 2.780688 (2.787415) | 2026-11-21 |
| [22x24x25_6650_rational](large/22x24x25_6650_rational) | ⟨22×24×25⟩ | 6650 | 7016 | 366 | ⟨25×25×25:7507⟩ | `I0`, `J0`, `I23`, `I0` | 2, 3, 5, 37 | 2.783220 (2.800161) | 2026-11-21 |
| [22x25x25_6997_rational](large/22x25x25_6997_rational) | ⟨22×25×25⟩ | 6997 | 7441 | 444 | ⟨25×25×25:7507⟩ | `I0`, `I0`, `I0` | 2, 3, 5, 37 | 2.787311 (2.806681) | 2026-11-21 |
| [22x26x26_7819_rational](large/22x26x26_7819_rational) | ⟨22×26×26⟩ | 7819 | 7906 | 87 | ⟨26×26×26:8051⟩ | `J0`, `J0`, `J0`, `J0` | 2, 5, 11 | 2.799238 (2.802693) | 2026-11-21 |
| [23x23x24_6344_rational](large/23x23x24_6344_rational) | ⟨23×23×24⟩ | 6344 | 6488 | 144 | ⟨25×25×25:7507⟩ | `I0`, `J0`, `I23`, `K0`, `J0` | 2, 3, 5, 37 | 2.779731 (2.786857) | 2026-11-21 |
| [23x23x25_6662_rational](large/23x23x25_6662_rational) | ⟨23×23×25⟩ | 6662 | 6814 | 152 | ⟨25×25×25:7507⟩ | `I0`, `J0`, `I23`, `J0` | 2, 3, 5, 37 | 2.783235 (2.790367) | 2026-11-21 |
| [23x24x24_6424_rational](large/23x24x24_6424_rational) | ⟨23×24×24⟩ | 6424 | 6488 | 64 | ⟨25×25×25:7507⟩ | `I0`, `J0`, `I23`, `K0` | 2, 3, 5, 37 | 2.771228 (2.774361) | 2026-11-21 |
| [23x24x25_6734_rational](large/23x24x25_6734_rational) | ⟨23×24×25⟩ | 6734 | 7040 | 306 | ⟨25×25×25:7507⟩ | `I0`, `J0`, `I23` | 2, 3, 5, 37 | 2.774192 (2.788178) | 2026-11-21 |
| [23x25x25_7078_rational](large/23x25x25_7078_rational) | ⟨23×25×25⟩ | 7078 | 7507 | 429 | ⟨25×25×25:7507⟩ | `I0`, `I0` | 2, 3, 5, 37 | 2.777975 (2.796416) | 2026-11-20 |
| [23x25x26_7757_rational](large/23x25x26_7757_rational) | ⟨23×25×26⟩ | 7757 | 7915 | 158 | ⟨26×26×26:8051⟩ | `J0`, `J0`, `J0`, `K0` | 2, 5, 11, 41 | 2.795230 (2.801523) | 2026-11-21 |
| [23x26x26_7876_rational](large/23x26x26_7876_rational) | ⟨23×26×26⟩ | 7876 | 8051 | 175 | ⟨26×26×26:8051⟩ | `J0`, `J0`, `J0` | 2, 5, 11, 41 | 2.788603 (2.795434) | 2026-11-21 |
| [24x24x25_6820_rational](large/24x24x25_6820_rational) | ⟨24×24×25⟩ | 6820 | 7064 | 244 | ⟨25×25×25:7507⟩ | `I0`, `J0` | 2, 3, 5, 37 | 2.765837 (2.776851) | 2026-11-20 |
| [24x25x25_7157_rational](large/24x25x25_7157_rational) | ⟨24×25×25⟩ | 7157 | 7507 | 350 | ⟨25×25×25:7507⟩ | `I0` | 2, 3, 5, 37 | 2.769143 (2.784039) | 2026-11-20 |
| [24x25x26_7870_rational](large/24x25x26_7870_rational) | ⟨24×25×26⟩ | 7870 | 7977 | 107 | ⟨26×26×26:8051⟩ | `J0`, `J0`, `K0` | 2, 5, 11, 41 | 2.787402 (2.791598) | 2026-11-21 |
| [24x26x26_7943_rational](large/24x26x26_7943_rational) | ⟨24×26×26⟩ | 7943 | 8051 | 108 | ⟨26×26×26:8051⟩ | `J0`, `J0` | 2, 5, 11, 41 | 2.778982 (2.783161) | 2026-11-20 |
| [24x27x27_8655_rational](large/24x27x27_8655_rational) | ⟨24×27×27⟩ | 8655 | 9000 | 345 | ⟨27×27×27:9233⟩ | `I0`, `I0`, `I0` | 2, 5, 11, 41 | 2.783873 (2.795875) | 2026-11-21 |
| [25x25x26_7943_rational](large/25x25x26_7943_rational) | ⟨25×25×26⟩ | 7943 | 8051 | 108 | ⟨26×26×26:8051⟩ | `J0`, `K0` | 2, 5, 11, 41 | 2.778523 (2.782702) | 2026-11-20 |
| [25x26x26_7997_rational](large/25x26x26_7997_rational) | ⟨25×26×26⟩ | 7997 | 8051 | 54 | ⟨26×26×26:8051⟩ | `J0` | 2, 5, 11, 41 | 2.769417 (2.771491) | 2026-11-20 |
| [25x26x27_8345_rational](large/25x26x27_8345_rational) | ⟨25×26×27⟩ | 8345 | 8701 | 356 | ⟨27×27×27:9233⟩ | `I0`, `J0`, `I0` | 2, 5, 11, 41 | 2.771798 (2.784622) | 2026-11-21 |
| [25x27x27_8743_rational](large/25x27x27_8743_rational) | ⟨25×27×27⟩ | 8743 | 9233 | 490 | ⟨27×27×27:9233⟩ | `I0`, `I0` | 2, 5, 11, 41 | 2.775382 (2.792057) | 2026-11-21 |
| [25x28x28_9658_rational](large/25x28x28_9658_rational) | ⟨25×28×28⟩ | 9658 | 9846 | 188 | ⟨28×28×28:9846⟩ | `J0`, `J0`, `J0` | 2, 3, 5, 11 | 2.785170 (2.791022) | 2026-11-21 |
| [26x26x27_8437_rational](large/26x26x27_8437_rational) | ⟨26×26×27⟩ | 8437 | 8727 | 290 | ⟨27×27×27:9233⟩ | `I0`, `J0` | 2, 5, 11, 41 | 2.764071 (2.774404) | 2026-11-21 |
| [26x27x27_8828_rational](large/26x27x27_8828_rational) | ⟨26×27×27⟩ | 8828 | 9233 | 405 | ⟨27×27×27:9233⟩ | `I0` | 2, 5, 11, 41 | 2.767278 (2.780940) | 2026-11-21 |
| [26x27x28_9652_rational](large/26x27x28_9652_rational) | ⟨26×27×28⟩ | 9652 | 9794 | 142 | ⟨28×28×28:9846⟩ | `J0`, `J0`, `K0` | 2, 3, 5, 11, 23 | 2.784177 (2.788609) | 2026-11-21 |
| [26x28x28_9730_rational](large/26x28x28_9730_rational) | ⟨26×28×28⟩ | 9730 | 9846 | 116 | ⟨28×28×28:9846⟩ | `J0`, `J0` | 2, 3, 5, 11, 23 | 2.776406 (2.779990) | 2026-11-21 |
| [26x29x29_10555_rational](large/26x29x29_10555_rational) | ⟨26×29×29⟩ | 10555 | 11203 | 648 | ⟨29×29×29:11203⟩ | `I0`, `I0`, `I0` | 2, 3, 5, 11 | 2.781340 (2.799228) | 2026-11-21 |
| [27x27x28_9730_rational](large/27x27x28_9730_rational) | ⟨27×27×28⟩ | 9730 | 9846 | 116 | ⟨28×28×28:9846⟩ | `J0`, `K0` | 2, 3, 5, 11 | 2.776022 (2.779605) | 2026-11-21 |
| [27x28x28_9788_rational](large/27x28x28_9788_rational) | ⟨27×28×28⟩ | 9788 | 9846 | 58 | ⟨28×28×28:9846⟩ | `J0` | 2, 3, 5, 11 | 2.767676 (2.769456) | 2026-11-21 |
| [27x28x29_10191_rational](large/27x28x29_10191_rational) | ⟨27×28×29⟩ | 10191 | 10602 | 411 | ⟨29×29×29:11203⟩ | `I0`, `J0`, `I0` | 2, 3, 5, 11 | 2.770070 (2.781937) | 2026-11-21 |
| [27x29x29_10648_rational](large/27x29x29_10648_rational) | ⟨27×29×29⟩ | 10648 | 11203 | 555 | ⟨29×29×29:11203⟩ | `I0`, `I0` | 2, 3, 5, 11 | 2.773499 (2.788695) | 2026-11-21 |
| [27x30x30_11699_rational](large/27x30x30_11699_rational) | ⟨27×30×30⟩ | 11699 | 11889 | 190 | ⟨30×30×30:11889⟩ | `J0`, `J0`, `J0` | 2, 3, 7, 13 | 2.782841 (2.787627) | 2026-11-21 |
| [28x28x29_10290_rational](large/28x28x29_10290_rational) | ⟨28×28×29⟩ | 10290 | 10630 | 340 | ⟨29×29×29:11203⟩ | `I0`, `J0` | 2, 3, 5, 11 | 2.762919 (2.772640) | 2026-11-21 |
| [28x29x29_10739_rational](large/28x29x29_10739_rational) | ⟨28×29×29⟩ | 10739 | 11203 | 464 | ⟨29×29×29:11203⟩ | `I0` | 2, 3, 5, 11 | 2.766015 (2.778621) | 2026-11-21 |
| [28x29x30_11682_rational](large/28x29x30_11682_rational) (no file) | ⟨28×29×30⟩ | 11682 | 11863 | 181 | ⟨30×30×30:11889⟩ | `J0`, `J0`, `K0` | 2, 3, 7, 13 | 2.781730 (2.786297) | 2026-11-21 |
| [28x30x30_11765_rational](large/28x30x30_11765_rational) (no file) | ⟨28×30×30⟩ | 11765 | 11889 | 124 | ⟨30×30×30:11889⟩ | `J0`, `J0` | 2, 3, 7, 13 | 2.774520 (2.777624) | 2026-11-21 |
| [28x31x31_12710_rational](large/28x31x31_12710_rational) | ⟨28×31×31⟩ | 12710 | 13433 | 723 | ⟨31×31×31:13433⟩ | `I0`, `I0`, `I0` | 2, 3, 7, 13 | 2.779405 (2.795677) | 2026-11-21 |
| [29x29x30_11765_rational](large/29x29x30_11765_rational) (no file) | ⟨29×29×30⟩ | 11765 | 11889 | 124 | ⟨30×30×30:11889⟩ | `J0`, `K0` | 2, 3, 7, 13 | 2.774195 (2.777298) | 2026-11-21 |
| [29x30x30_11827_rational](large/29x30x30_11827_rational) (no file) | ⟨29×30×30⟩ | 11827 | 11889 | 62 | ⟨30×30×30:11889⟩ | `J0` | 2, 3, 7, 13 | 2.766497 (2.768040) | 2026-11-21 |
| [29x30x31_12289_rational](large/29x30x31_12289_rational) | ⟨29×30×31⟩ | 12289 | 12759 | 470 | ⟨31×31×31:13433⟩ | `I0`, `J0`, `I0` | 2, 3, 7, 13 | 2.768874 (2.779910) | 2026-11-21 |
| [29x31x31_12809_rational](large/29x31x31_12809_rational) | ⟨29×31×31⟩ | 12809 | 13433 | 624 | ⟨31×31×31:13433⟩ | `I0`, `I0` | 2, 3, 7, 13 | 2.772151 (2.786092) | 2026-11-21 |
| [29x32x32_13994_rational](large/29x32x32_13994_rational) (no file) | ⟨29×32×32⟩ | 13994 | 14196 | 202 | ⟨32×32×32:14196⟩ | `J0`, `J0`, `J0` | 2, 7, 13, 53 | 2.780833 (2.785008) | 2026-11-21 |
| [30x30x31_12395_rational](large/30x30x31_12395_rational) | ⟨30×30×31⟩ | 12395 | 12789 | 394 | ⟨31×31×31:13433⟩ | `I0`, `J0` | 2, 3, 7, 13 | 2.762221 (2.771392) | 2026-11-21 |
| [30x31x31_12906_rational](large/30x31x31_12906_rational) | ⟨30×31×31⟩ | 12906 | 13433 | 527 | ⟨31×31×31:13433⟩ | `I0` | 2, 3, 7, 13 | 2.765203 (2.776895) | 2026-11-21 |
| [30x31x32_13974_rational](large/30x31x32_13974_rational) (no file) | ⟨30×31×32⟩ | 13974 | 14196 | 222 | ⟨32×32×32:14196⟩ | `J0`, `J0`, `K0` | 2, 7, 13, 53 | 2.779835 (2.784426) | 2026-11-21 |
| [30x32x32_14064_rational](large/30x32x32_14064_rational) (no file) | ⟨30×32×32⟩ | 14064 | 14196 | 132 | ⟨32×32×32:14196⟩ | `J0`, `J0` | 2, 7, 13, 53 | 2.773158 (2.775870) | 2026-11-21 |
| [31x31x32_14064_rational](large/31x31x32_14064_rational) (no file) | ⟨31×31×32⟩ | 14064 | 14196 | 132 | ⟨32×32×32:14196⟩ | `J0`, `K0` | 2, 7, 13, 53 | 2.772878 (2.775590) | 2026-11-21 |
| [31x32x32_14130_rational](large/31x32x32_14130_rational) (no file) | ⟨31×32×32⟩ | 14130 | 14196 | 66 | ⟨32×32×32:14196⟩ | `J0` | 2, 7, 13, 53 | 2.765740 (2.767089) | 2026-11-21 |
<!-- large:end -->

<!-- products:begin -->
#### Larger formats: shared-factor products, and deletions from them

910 formats with a dimension above 16, each below Sedoglavic's table of 2026-11-15 and none implied by that table
together with the other schemes of this repository (by taking a sub-format, splitting a dimension or a Kronecker
product).  The full table is [products/INDEX.md](products/INDEX.md); the largest gains are below.

- **Products (683 formats, below the table by 1 to 243 products, 18,974 in total).**  Each is a Kronecker product of
  a base scheme with a second format, the block sizes, in which every group of base products that share a factor
  is replaced by one scheme of a larger format (a worked example is in
  [the method section](#the-schemes-of-products-shared-factor-products)).  This is the serendipitous product as Perminov defines and uses it in
  [arXiv:2606.02480](https://arxiv.org/abs/2606.02480) (Section 2.5), where he attributes it to Smith and its
  general form to Sedoglavic; the base schemes come from Perminov's catalog.  The method and the base schemes are
  theirs.  Ours is a screen in which every rank is the rank of a scheme we hold or can build, a builder that
  carries out each plan, and the exact check of the result.
- **Deletions (227 formats, below the table by 1 to 97, 4,265 in total).**  One index deleted from a product scheme
  (two for three of them), followed by LITA's 2-reduction, as for the schemes of `large/`.  Part of each gain is
  inherited from the product scheme; "Own" in the table is what the deletion adds beyond what the table and our
  other schemes imply (1 to 67).
- **Coefficients.**  Products: 390 integer, 157 over ℤ[1/2], 136 with other denominators.  Deletions: 85 integer,
  68 over ℤ[1/2], 74 with other denominators.  "Invertible" lists the primes that occur in denominators.
- **Form.**  Compressed JSON in [products/](products), one folder per scheme with its build plan or deletion path
  and its certificate: see [Files](#files).  Each was certified by evaluating every Brent equation in exact integer
  arithmetic ([checks/check_products.log](checks/check_products.log)).
- **Exponents.**  None is near a record: the lowest is 2.7886 (⟨9×17×18⟩:1576), against 2.7743 for ⟨3×3×6:40⟩.

Largest relative gains among the products.  "Base scheme" is the catalog scheme the product starts from.  "Block
sizes" ⟨p×q×s⟩ is the second format: every entry of A becomes a p×q block, every entry of B a q×s block and every
entry of C a p×s block.  Position by position the two multiply to the format: ⟨11×4×3⟩ with blocks ⟨1×3×6⟩ is
⟨11×12×18⟩.  Every scheme is stored with its dimensions in that order; the catalog file may hold the base scheme
with its dimensions in another order (here ⟨3×4×11⟩), and the plan says how it is turned.

| Format | Rank | Table | Gain | Base scheme | Block sizes | Invertible | Exponent (table's) |
|---|---|---|---|---|---|---|---|
| [⟨11×12×18⟩](products/11x12x18_1400_dyadic) | 1400 | 1476 | 76 (5.1%) | ⟨11×4×3:104⟩ | ⟨1×3×6⟩ | 2 | 2.7959 (2.8163) |
| [⟨13×20×21⟩](products/13x20x21_3165_integer) | 3165 | 3304 | 139 (4.2%) | ⟨13×4×3:123⟩ | ⟨1×5×7⟩ | none | 2.8099 (2.8249) |
| [⟨9×9×27⟩](products/9x9x27_1287_dyadic) | 1287 | 1343 | 56 (4.2%) | ⟨3×3×9:63⟩ | ⟨3×3×3⟩ | 2 | 2.7932 (2.8098) |
| [⟨15×20×32⟩](products/15x20x32_5220_rational) | 5220 | 5440 | 220 (4.0%) | ⟨3×4×8:75⟩ | ⟨5×5×4⟩ | 3 | 2.8007 (2.8142) |
| [⟨13×15×32⟩](products/13x15x32_3594_integer) | 3594 | 3744 | 150 (4.0%) | ⟨13×3×4:123⟩ | ⟨1×5×8⟩ | none | 2.8106 (2.8246) |
| [⟨11×20×21⟩](products/11x20x21_2719_rational) | 2719 | 2828 | 109 (3.9%) | ⟨11×4×3:104⟩ | ⟨1×5×7⟩ | 3, 5 | 2.8115 (2.8255) |
| [⟨12×13×18⟩](products/12x13x18_1662_dyadic) | 1662 | 1725 | 63 (3.7%) | ⟨4×13×3:123⟩ | ⟨3×1×6⟩ | 2 | 2.8019 (2.8159) |
| [⟨13×15×28⟩](products/13x15x28_3165_integer) | 3165 | 3281 | 116 (3.5%) | ⟨13×3×4:123⟩ | ⟨1×5×7⟩ | none | 2.8099 (2.8224) |
| [⟨9×11×24⟩](products/9x11x24_1400_dyadic) | 1400 | 1450 | 50 (3.4%) | ⟨3×11×4:104⟩ | ⟨3×1×6⟩ | 2 | 2.7959 (2.8094) |
| [⟨6×18×27⟩](products/6x18x27_1738_dyadic) | 1738 | 1800 | 62 (3.4%) | ⟨2×6×9:86⟩ | ⟨3×3×3⟩ | 2 | 2.8054 (2.8186) |
| [⟨9×13×32⟩](products/9x13x32_2229_dyadic) | 2229 | 2303 | 74 (3.2%) | ⟨3×13×4:123⟩ | ⟨3×1×8⟩ | 2 | 2.8109 (2.8228) |
| [⟨13×18×28⟩](products/13x18x28_3747_dyadic) | 3747 | 3870 | 123 (3.2%) | ⟨13×3×4:123⟩ | ⟨1×6×7⟩ | 2 | 2.8092 (2.8203) |
| [⟨6×18×28⟩](products/6x18x28_1788_dyadic) | 1788 | 1844 | 56 (3.0%) | ⟨2×6×14:133⟩ | ⟨3×3×2⟩ | 2 | 2.8033 (2.8148) |
| [⟨13×20×27⟩](products/13x20x27_4035_integer) | 4035 | 4160 | 125 (3.0%) | ⟨13×4×3:123⟩ | ⟨1×5×9⟩ | none | 2.8124 (2.8228) |
| [⟨11×15×28⟩](products/11x15x28_2719_rational) | 2719 | 2803 | 84 (3.0%) | ⟨11×3×4:104⟩ | ⟨1×5×7⟩ | 3, 5 | 2.8115 (2.8223) |

Largest own gains among the deletions, relative to the table's rank.  "From" is the product scheme and the
deleted index, named as for `large/`: in ⟨m×k×n⟩, `I17` removes row 17 of A (and of C), `J17` the inner index 17 (a
column of A with the matching row of B) and `K17` column 17 of B, counted from 0.  So ⟨12×13×18:1662⟩, `K17` is
that scheme without the last column of B, a scheme for ⟨12×13×17⟩.  The source is written in the orientation in
which the deletion leaves the dimensions sorted, so it can be a product scheme of the list turned: ⟨12×11×18:1400⟩
is the ⟨11×12×18⟩ scheme of the table above.

| Format | Rank | Table | Gain | Own | From | Invertible | Exponent (table's) |
|---|---|---|---|---|---|---|---|
| [⟨6×17×28⟩](products/6x17x28_1741_dyadic) | 1741 | 1796 | 55 | 47 | ⟨6×18×28:1788⟩, `J17` | 2 | 2.8134 (2.8251) |
| [⟨12×13×17⟩](products/12x13x17_1617_dyadic) | 1617 | 1655 | 38 | 38 | ⟨12×13×18:1662⟩, `K17` | 2 | 2.8117 (2.8206) |
| [⟨11×11×18⟩](products/11x11x18_1352_dyadic) | 1352 | 1383 | 31 | 31 | ⟨12×11×18:1400⟩, `I11` | 2 | 2.8139 (2.8227) |
| [⟨9×14×24⟩](products/9x14x24_1809_dyadic) | 1809 | 1856 | 47 | 41 | ⟨9×15×24:1873⟩, `J14` | 2 | 2.8077 (2.8173) |
| [⟨11×12×17⟩](products/11x12x17_1369_dyadic) | 1369 | 1410 | 41 | 31 | ⟨11×12×18:1400⟩, `K17` | 2 | 2.8079 (2.8193) |
| [⟨8×17×30⟩](products/8x17x30_2454_dyadic) | 2454 | 2509 | 55 | 55 | ⟨8×18×30:2526⟩, `J17` | 2 | 2.8166 (2.8246) |
| [⟨13×19×21⟩](products/13x19x21_3098_integer) | 3098 | 3171 | 73 | 67 | ⟨13×20×21:3165⟩, `J19` | none | 2.8192 (2.8274) |
| [⟨9×11×23⟩](products/9x11x23_1371_dyadic) | 1371 | 1419 | 48 | 29 | ⟨9×11×24:1400⟩, `K0` | 2 | 2.8031 (2.8165) |
| [⟨9×17×17⟩](products/9x17x17_1542_dyadic) | 1542 | 1574 | 32 | 32 | ⟨9×18×18:1600⟩, `K0`, `J6` | 2 | 2.8005 (2.8084) |
| [⟨6×17×27⟩](products/6x17x27_1693_rational) | 1693 | 1728 | 35 | 35 | ⟨6×18×27:1738⟩, `J17` | 2, 11 | 2.8157 (2.8235) |

Five of the deletions start from the plain Kronecker product of ⟨3×3×6:40⟩ with ⟨3×6×3:40⟩, a scheme for
⟨9×18×18⟩ of rank 40 · 40 = 1600, which ties the table: ⟨9×17×18⟩ 1576 (table 1591), ⟨8×18×18⟩ 1560 (table 1575), ⟨9×17×17⟩ 1542 (table 1574), ⟨9×16×18⟩ 1537 (table 1560), ⟨7×18×18⟩ 1424 (table 1425).
<!-- products:end -->

#### Superseded by the schemes above

| Scheme | Format | Rank | Superseded by | Valid over | Found |
|---|---|---|---|---|---|
| [13x14x14_1564_rational](schemes/13x14x14_1564_rational) | ⟨13×14×14⟩ | 1564 | 13x14x14_1538 | 2 and 5 invertible | 2026-11-11 |
| [13x15x15_1763_rational](schemes/13x15x15_1763_rational) | ⟨13×15×15⟩ | 1763 | 13x15x15_1762 | 2, 5 and 17 invertible | 2026-11-10 |
| [15x16x16_2202_rational](schemes/15x16x16_2202_rational) | ⟨15×16×16⟩ | 2202 | 15x16x16_2200 | 2, 3 and 5 invertible | 2026-11-11 |

These three were records when found (against 1579 / 1582, 1776 / 1776 and 2220 / 2223).  They stay here because
PLinOpt's `data/` and, for the first two, Perminov's catalog hold them under these names.

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
programs come from one quick heuristic pass (see [the rectangular schemes](#the-rectangular-schemes-projection-and-2-reduction)),
so their counts are upper bounds that more optimization would lower.

**Exponent.** 3 ln r / ln(mkn), as in the complexity column of fmm.univ-lille.fr: composing the scheme with its two
cyclic rotations gives a scheme for ⟨mkn×mkn×mkn:r³⟩, and recursion on it multiplies N×N matrices in O(N^ω) for this
ω.  For a square format ⟨n×n×n:r⟩ it is log_n r.  Used recursively, a square scheme whose three maps take q
operations multiplies N×N matrices in (1 + q/(r − n²)) N^ω + O(N²) operations, so q fixes the leading constant: 4.93
for the 55, 4.64 for KS-51 (plus O(N² log N) for its basis changes), 7.28 for the 201 and 6.35 for the 166.

**Rings and fields.** Where a scheme works depends on its coefficients:

- integer coefficients: every ring;
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

`large/` is different.  Each folder there holds:

- `SHAPE_mRANK_Q.json.gz`: the scheme in the JSON layout of Perminov's catalog (`n`, `m`, `u`, `v`, `w`; u on A and
  v on B row-major, w on C transposed, index k·n₀ + i), with sparse rows `[{"index": i, "value": v}, …]` and values
  that are integers or `"a/b"` strings.  `n` is the format as the scheme stands, a permutation of the sorted format
  in the folder name.  A `note` field names the LITA file and the deletions.
- `path_1.json`, `path_2.json`, …: one file per deletion, from LITA's square down to the scheme, with the deleted
  index, the direction of every reduction step and the products dropped.  `tools/kronexact replay` redoes a path
  exactly.
- `certificate.log`: the output of the last replay and of the exact check.

[large/index.json](large/index.json) is the table of the index in machine-readable form.  PLinOpt `.sms` files of
these schemes would be tens of megabytes each, so they are not included.

`products/` has one folder per scheme, named like those of `large/`:

- `SHAPE_mRANK.json.gz`: the scheme, in the same layout as in `large/`, with its dimensions in the order of the
  folder name.
- For a product, `plan.json`: the base scheme, the block sizes (the second format) and, for every group of base
  products that share a factor, the products in it and the scheme that replaces it, each given down to files.  A file node names a file of Perminov's
  catalog at commit f70078c by its path there (`"catalog"`) or a file of this repository by its path from the
  plan's folder (`"here"`); a `trivial` node is the schoolbook scheme of a format.  The plan forms the product with
  the base scheme as its file has it and ends with the rotations and transpositions that sort the dimensions.
  `tools/kronexact build plan.json OUT --catalog DIR` carries it out.
- For a deletion, `source_plan.json` (the plan of the product scheme it starts from) and `path_1.json` (the
  deletions and their reductions, as in `large/`).  `tools/kronexact replay source_plan.json path_1.json OUT
  --catalog DIR` builds the product scheme and redoes the path.
- `certificate.log`: the output of that build or replay, which ends with the exact check.

`products/base/` holds the one base scheme that is in neither place.
[products/index.json](products/index.json) and [products/INDEX.md](products/INDEX.md) are the full table.

## Checking

```sh
python3 tools/check_scheme.py schemes/4x4x4_48_201/4x4x4_48_201
python3 tools/check_scheme.py --mod 2 schemes/4x4x4_47_mod_2_166/4x4x4_47_mod_2_166
tools/check_all.sh                              # every scheme, Python standard library only
PLINOPT=/path/to/plinopt tools/check_all.sh     # also PLinOpt's SLPchecker, MMchecker and ALTchecker
python3 tools/ring_slp.py --check schemes/4x11x14_426_integer/4x11x14_426_integer   # programs against their matrices
tools/check_large.sh                            # large/: random matrices modulo four primes, standard library only
cargo build --release --manifest-path tools/kronexact/Cargo.toml
EXACT=1 tools/check_large.sh                    # large/: also every Brent equation, in exact integers
tools/check_large.sh products                   # products/: random matrices modulo four primes
EXACT=1 tools/check_large.sh products           # products/: also every Brent equation, in exact integers
```

`check_scheme.py` checks the 3×3 and 4×4 schemes against the Brent equations, exactly: every coefficient of the
trilinear form (729 or 4,096 of them) must be right, modulo 2 for the F₂ schemes.  The rectangular schemes are too big
for that in plain Python, so it runs them on random integer matrices with entries up to 10⁹ in exact rational
arithmetic, where a wrong scheme passes a trial with probability below 10⁻⁶.  Every run also changes one coefficient
and requires the check to fail.  For every scheme whose programs are plain sums, `check_all.sh` also runs
`ring_slp.py --check`: it evaluates each program symbolically, requires it to equal its matrix exactly, and requires
every division to be by a prime that occurs in a denominator of that matrix.
[checks/check_all.log](checks/check_all.log) is the output of the last full run with PLinOpt.

The rectangular schemes were verified exactly against the Brent equations when they were found.  Two independent
routes were used: an exact modular contraction with a bound on the entries, and an integer contraction after clearing
denominators.  The logs of those runs, with the pivots of each reduction, are in
[checks/discovery](checks/discovery): every one ends with the output verifying exactly and the corrupted copy
rejected.

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

### The schemes of large/: checking and reproducing

`tools/kronexact` (Rust; `rayon` and `serde_json` only) is the certificate for the schemes of `large/`.

- **`check SCHEME.json.gz`** clears the common denominator of each of the three factors and evaluates every Brent
  equation in 128-bit integers, after proving from the largest entries that no partial sum can overflow.  For
  ⟨31×32×32⟩ that is about 10⁹ equations.  It then changes one coefficient and requires the check to fail.  The
  check does not depend on how a scheme was found.
- **`replay SOURCE PATH OUT`** redoes one deletion and its reductions over ℚ: the dependencies among the Kronecker
  products are computed modulo three primes just below 2⁶¹, lifted to rationals from two of them and compared with
  the third, and the fold is done in exact rational arithmetic.  The result is then checked as above.
- **`tools/check_json.py`** is a second opinion from other code: it multiplies random matrices modulo four primes.

We gated `kronexact` before using it: on the ⟨15×16×16:2200⟩ scheme it reproduces, entry for entry, the scheme that
the earlier Python pipeline had verified by two other routes, and it rejects a scheme with a product removed.

To reproduce a scheme from LITA's files: download `schemes/kxkxk_rR.npz` from
[khoruzhii/lita](https://github.com/khoruzhii/lita) (commit 472a79f), convert it with
`python3 tools/lita_npz2json.py IN.npz kxkxk_mR_Q.json`, then run `kronexact replay` with `path_1.json`,
`path_2.json`, … in turn, each on the output of the one before.  The paths were found by a search modulo one
20-bit prime over which index to delete and which products to drop; that search program is not in this repository.

One deletion takes far more off an odd square's rank than off an even one's: 153 at k = 17 rising to 527 at k = 31,
against 38 at k = 18 rising to 66 at k = 32.  Further deletions keep paying: every format we reached from
k = 18 or k = 20 is reached at a lower rank from the odd square above it, so no scheme in the list starts from those
two.  The largest relative gain is 7.7% (⟨17×19×19⟩, 3354 against 3633).  The squares themselves do not reduce (we
tried k = 17, 18 and 19).

### The schemes of products/: shared-factor products

**The product.**  Take a base scheme for ⟨a×b×c⟩ with r products and a second format ⟨p×q×s⟩, the block sizes: every
entry of A becomes a p×q block, every entry of B a q×s block, every entry of C a p×s block, and the result is a scheme
for ⟨ap×bq×cs⟩.  The plain Kronecker product replaces each base product by a scheme of ⟨p×q×s⟩ and has rank
r · R(p,q,s).  If g of the base products have the same factor on B up to a scalar, then after blocking that factor
is one q×s block matrix Y and the g other factors are p×q blocks X₁, …, X_g, and the g block products X₁·Y, …, X_g·Y
are one product: the X's stacked into a gp×q matrix, times Y.  So the group can be replaced by one scheme of
⟨gp×q×s⟩, at R(gp,q,s) instead of g · R(p,q,s).  A shared factor on A gives ⟨p×q×gs⟩ in the same way (the other
factors side by side), and one on C gives ⟨p×gq×s⟩.  That is the serendipitous product of Section 2.5 of Perminov's
paper, which attributes it to Smith and its general form to Sedoglavic (a scheme written as a sum of blocks, each
block replaced by a scheme of the block's format times the second format).  The paper applies it across the catalog;
the groups of base products sharing a factor are its "buds", a term it credits to Sedoglavic.

Example 1: ⟨11×4×3:104⟩ with block sizes ⟨1×3×6⟩ gives ⟨11×12×18⟩.  The plain Kronecker product turns each of the 104
base products into a row vector times a 3×6 matrix, 18 multiplications: 104 · 18 = 1872.  The 104 base products fall
into 30 groups that share their factor on B:

| Groups | Products in each | Replaced by | Cost |
|---|---|---|---|
| 15 | 4 | ⟨4×3×6:54⟩ | 810 |
| 14 | 3 | ⟨3×3×6:40⟩ | 560 |
| 1 | 2 | ⟨2×3×6:30⟩ | 30 |

That is 1400 for ⟨11×12×18⟩, where the table has 1476.

Example 2: the catalog's ⟨3×3×9:63⟩ has 27 pairs of products that share a factor.  With block sizes ⟨3×3×3⟩,
each pair is replaced by a scheme of ⟨3×3×6⟩ of rank 40 and each of the other 9 products by ⟨3×3×3:23⟩:
27 · 40 + 9 · 23 = 1287 for ⟨9×9×27⟩, where the table has 1343 and the plain product has 63 · 23 = 1449.

**What we did.**  Perminov's paper reports ranks; many of them need a scheme for every format that appears in a
replacement, and those in turn may be sub-formats, direct sums or products of other schemes.

1. *A screen that only uses what can be built.*  Every rank in a plan is the rank of a scheme we hold as a file, or
   of one that sub-format, direct sum or Kronecker product gives from such files.  The bar a plan has to beat is
   Sedoglavic's table closed under the same three operations.
2. *A builder.*  `kronexact build` executes a plan down to its files and writes the scheme.
3. *An exact check* of every result, as for `large/`, and a second check by other code (random matrices modulo four
   primes).

864 schemes were built this way; 683 are listed.  126 of the listed ones use the rank-40 scheme of ⟨3×3×6⟩ as a
replacing block, which the catalog's JSON files do not have (they have rank 42).  It is Smirnov's scheme in the
accurate form of Dumas, Pernet and Sedoglavic, PLinOpt's `data/3x3x6_40_DPS-accurate`, here as
[products/base/3x3x6_m40_DPS-accurate.json](products/base/3x3x6_m40_DPS-accurate.json).

**Blocks with two dimensions.**  Products that share factors in two slots form a grid, an image of ⟨1×q×r⟩ (Section
2.6.4 of Perminov's paper).  38 of the listed schemes use one.  6 use a variant in which the factors of four
products lie in a plane in each of two slots without being proportional; their sum is still an image of ⟨1×2×2⟩.
Where we built the same format with and without them, either gained 1 to 12 products.

**Deletions.**  The product schemes do not 2-reduce as they stand (we tried the 40 of lowest rank).  After deleting
one index they do.  We searched every product scheme of rank up to 4000, deleting the first and the last index of
each slot: 628 results below the table, all certified, of which the best per format that is not implied by other
schemes is listed (227, with the five from ⟨9×18×18:1600⟩ below).  Product schemes above rank 4000 were not searched.

**Exponent.**  We also tried ⟨3×3×6:40⟩ for a better exponent.  No rank-40 scheme we hold has two products sharing a
factor, the ⟨9×18×18:1600⟩ products do not 2-reduce, and every deletion moves the exponent away from 2.7743.  The
five deletions listed above are what that attempt left.

**To reproduce.**  The plans name the catalog's files by their paths in
[Perminov's catalog](https://github.com/dronperminov/FastMatrixMultiplication) at commit f70078c, so a checkout of
that commit is needed; the other files they name are in this repository.  Every scheme file is, byte for byte, the
compressed output of one command run in its folder, and `certificate.log` is what that command printed:

```sh
cargo build --release --manifest-path tools/kronexact/Cargo.toml
KX=$PWD/tools/kronexact/target/release/kronexact
cd products/9x9x27_1287_dyadic                  # a product
$KX build plan.json out.json --catalog /path/to/FastMatrixMultiplication
gunzip -c 9x9x27_m1287.json.gz | cmp - out.json
cd ../9x17x17_1542_dyadic                # a deletion: builds the product scheme, then replays the path
$KX replay source_plan.json path_1.json out.json --catalog /path/to/FastMatrixMultiplication
gunzip -c 9x17x17_m1542.json.gz | cmp - out.json
```

Both commands end with the exact check of what they wrote.

### The rectangular schemes: projection and 2-reduction

Twelve of the twenty-two start from the square schemes of Khoruzhii, Serafin, Gelß and Pokutta, *Local Improvements
to Trilinear Aggregation* ([khoruzhii/lita](https://github.com/khoruzhii/lita)), as included in Perminov's catalog.
Nine start from other schemes of that catalog, and one is a direct 2-reduction of a ternary catalog scheme by
MerlijnW70 ([MerlijnW70/fmm-schemes](https://github.com/MerlijnW70/fmm-schemes)).  The steps:

1. **Project.** Delete one index: a row of A, a column of B, or an inner index (a column of A together with the
   matching row of B).  This gives a scheme for the next smaller format; products with a factor that becomes zero
   disappear.
2. **Reduce.** In the projected scheme the Kronecker products of two of the three factors of the products become
   linearly dependent; the scheme is *2-reducible* in LITA's sense.  Each dependency lets one product's third factor
   be folded into the others, which removes that product.
3. **Choose what to remove.** Which products the dependencies remove is a search.  An exchange search over pivot sets
   picks them so that many more products vanish than there are dependencies: 14 dependencies remove 30 products for
   13×14×14 at rank 1564, and 17 remove 113 for 14×15×15.

The pivot search runs modulo the prime 1048573.  The reduction is then replayed exactly over ℚ, with the
dependencies lifted from several primes.  The result is verified against the Brent equations by two independent
routes, and a copy with one coefficient changed must be rejected.  Finally an independent program runs it on random
integer matrices.  The method is LITA's, and Perminov's catalog already holds projected LITA schemes; our part is the
choice of which products to remove.

**The sweep of 2026-11-20.**  The pivot search was rewritten in Rust, about ten times faster per core, and run over
every scheme file of the catalog at commit f70078c that it could read: 2,625 files, each reduced directly and after
every single-index deletion in each of the three positions, 72,826 runs.  Every result below both the table and the
catalog was replayed exactly and verified as above.  The verified schemes were then used as sources themselves,
round after round, until a round gave nothing new.  What it showed:

- Which index is deleted matters, and so does its position.  From LITA's ⟨16×16×16:2236⟩ the best row deletion gives
  2202, and inner index 9 gives 2200.
- Reducing a catalog scheme without deleting anything gave no record.
- Schemes found this way are good sources.  LITA's ⟨15×15×15:1977⟩ alone leads to seven of the schemes here, each
  from an earlier one by one more deletion: 14×15×15, 14×14×15, 13×15×15, 13×14×15, 13×14×14, 13×13×15 and 13×13×14.
  The family ends there in our runs: one more deletion reaches ⟨13×13×13⟩ at 1417 at best, where LITA's own square
  has 1379, and the formats with a 12 come out 44 or more above the tables.
- More search per reduction (5 times the idle budget and 16 times the restarts) found nothing that the standard
  setting does not find.

How each scheme was derived.  "Products left" is the count after the deletion and before the reduction; a + b in
the next two columns means successive reductions along different factors.  Indices count from 0 and refer to the
source in the ordering in which it was found, which for the four reordered schemes is not the ordering stored here;
the path files in [checks/discovery](checks/discovery) give every step as it was run.

| Scheme | Source | Deleted | Products left | Dependencies | Products removed | Rank | Found |
|---|---|---|---|---|---|---|---|
| 4x11x13_397 | catalog ⟨4×12×13:423⟩, ternary | inner index 0 | 418 | 17 | 21 | 397 | 2026-11-20 |
| 4x11x14_426 | catalog ⟨4×12×14:455⟩, ternary | inner index 3 | 443 | 16 | 17 | 426 | 2026-11-20 |
| 4x11x16_477 | catalog ⟨4×12×16:513⟩, ternary | inner index 11 | 501 | 18 | 24 | 477 | 2026-11-20 |
| 4x13x14_498 | catalog ⟨4×13×15:520⟩ | column 2 of B | 520 | 17 | 22 | 498 | 2026-11-20 |
| 8x11x13_749 | catalog ⟨8×12×13:785⟩ | inner index 5 | 775 | 22 | 26 | 749 | 2026-11-20 |
| 9x11x11_705 | catalog ⟨9×11×12:738⟩ | column 7 of B | 728 | 20 | 23 | 705 | 2026-11-20 |
| 9x11x15_953 | catalog ⟨9×12×15:996⟩ | inner index 0 | 985 | 15 + 5 + 1 | 25 + 6 + 1 | 953 | 2026-11-20 |
| 9x11x16_987 | catalog ⟨9×12×16:1035⟩ | inner index 2 | 1012 | 11 + 6 | 19 + 6 | 987 | 2026-11-20 |
| 11x13x16_1444 | catalog ⟨12×13×16:1509⟩ | row 1 of A | 1479 | 19 + 11 + 2 | 22 + 11 + 2 | 1444 | 2026-11-20 |
| 12x13x13_1275 | LITA ⟨13×13×13:1379⟩ | row 0 of A | 1360 | 15 | 85 | 1275 | 2026-11-10, verified 2026-11-11 |
| 13x13x14_1477 | the 13x13x15_1577 scheme | inner index 0 | 1560 | 14 | 83 | 1477 | 2026-11-20 |
| 13x13x15_1577 | the 13x15x15_1762 scheme | column 10 of B, reduced to 1645, then column 2 of B | 1747, then 1610 | 19, then 14 + 6 | 102, then 27 + 6 | 1577 | 2026-11-20 |
| 13x14x14_1538 | the 13x14x15_1645 scheme | inner index 0 | 1628 | 15 | 90 | 1538 | 2026-11-20 |
| 13x14x14_1564 | LITA ⟨14×14×14:1594⟩ | row 7 of A | 1594 | 14 | 30 | 1564 | 2026-11-10, verified 2026-11-11 |
| 13x14x15_1645 | the 13x15x15_1762 scheme | column 0 of B | 1742 | 14 | 97 | 1645 | 2026-11-20 |
| 13x14x16_1805 | MerlijnW70's ⟨13×14×16:1806⟩, ternary | none | 1806 | 1 | 1 | 1805 | 2026-11-10 |
| 13x15x15_1762 | LITA ⟨15×15×15:1977⟩ | row 14 of A, reduced to 1842, then row 6 of A | 1961, then 1791 | 23, then 16 | 119, then 29 | 1762 | 2026-11-20 |
| 13x15x15_1763 | the 14x15x15_1842 scheme | row 7 of A | 1797 | 21 | 34 | 1763 | 2026-11-10 |
| 14x14x15_1715 | the 14x15x15_1842 scheme | inner index 0 | 1827 | 17 | 112 | 1715 | 2026-11-10 |
| 14x15x15_1842 | LITA ⟨15×15×15:1977⟩ | row 0 of A | 1955 | 17 | 113 | 1842 | 2026-11-10 |
| 15x16x16_2200 | LITA ⟨16×16×16:2236⟩ | inner index 9 | 2236 | 16 + 1 | 34 + 2 | 2200 | 2026-11-20 |
| 15x16x16_2202 | LITA ⟨16×16×16:2237⟩ | row 9 of A | 2237 | 16 | 35 | 2202 | 2026-11-10, verified 2026-11-11 |

The catalog sources are in its `schemes/results`; the exact paths, the pivots and the verification logs of every
scheme are in [checks/discovery](checks/discovery).

Coefficients.  After each product is rescaled so that its A and B factors are primitive integer vectors, all
denominators sit on the C side, and an entry of C needs at most one division by a constant; the fourth column counts
the entries of C that need one.

| Scheme | Denominator primes | Largest denominator | Divisions after rescaling | Valid over F_p for |
|---|---|---|---|---|
| 4x11x14_426 | none (coefficients −2 to 2) | 1 | 0 | every p |
| 13x14x16_1805 | none (coefficients −1, 0, 1) | 1 | 0 | every p |
| 4x11x13_397 | 2 | 2 | 29 of the 52 entries of C | p ≠ 2 |
| 4x11x16_477 | 2 | 2 | 20 of 64 | p ≠ 2 |
| 4x13x14_498 | 2 | 8 | 56, one per entry of C | p ≠ 2 |
| 8x11x13_749 | 2 | 16 | 104, one per entry | p ≠ 2 |
| 9x11x11_705 | 2 | 8 | 99, one per entry | p ≠ 2 |
| 9x11x16_987 | 2 | 8 | 144, one per entry | p ≠ 2 |
| 11x13x16_1444 | 2 | 8 | 104 of 176 | p ≠ 2 |
| 9x11x15_953 | 2, 3 | 432 | 135, one per entry | p ≠ 2, 3 |
| 12x13x13_1275 | 2, 3, 13 | 117 | 156, one per entry | p ≠ 2, 3, 13 |
| 13x13x14_1477 | 2, 5, 17 | 1088 | 182, one per entry | p ≠ 2, 5, 17 |
| 13x13x15_1577 | 2, 5, 17 | 1088 | 195, one per entry | p ≠ 2, 5, 17 |
| 13x14x14_1538 | 2, 5, 17 | 1088 | 182, one per entry | p ≠ 2, 5, 17 |
| 13x14x14_1564 | 2, 5 | 200 | 182, one per entry | p ≠ 2, 5 |
| 13x14x15_1645 | 2, 5, 17 | 1088 | 195, one per entry | p ≠ 2, 5, 17 |
| 13x15x15_1762 | 2, 5, 17 | 1088 | 195, one per entry | p ≠ 2, 5, 17 |
| 13x15x15_1763 | 2, 5, 17 | 1088 | 195, one per entry | p ≠ 2, 5, 17 |
| 14x14x15_1715 | 2, 5, 17 | 1088 | 210, one per entry | p ≠ 2, 5, 17 |
| 14x15x15_1842 | 2, 5, 17 | 1088 | 210, one per entry | p ≠ 2, 5, 17 |
| 15x16x16_2200 | 2, 3, 5, 7 | 1440 | 240, one per entry | p ≠ 2, 3, 5, 7 |
| 15x16x16_2202 | 2, 3, 5 | 720 | 240, one per entry | p ≠ 2, 3, 5 |

Programs.  The programs of the fifteen schemes of 2026-11-20 are made by [tools/ring_slp.py](tools/ring_slp.py), a
greedy elimination of common pairs that only ever forms p·x + q·y with p and q coprime integers.  Every constant
of such a program is an integer or has a denominator that occurs in the scheme: the integer scheme's programs
contain no division, and those of the ℤ[1/2] schemes divide by powers of 2 only.  The C side is made from the
program of its transpose.  Counts are A side + B side + C side; the divisions and integer multiplications together
are the scalings.

| Scheme | Additions | Divisions (of which by a power of 2) | Integer multiplications | Total |
|---|---|---|---|---|
| 4x11x13_397 | 482 + 1,244 + 876 = 2,602 | 25 (25) | 47 | 2,674 |
| 4x11x14_426 | 472 + 1,194 + 989 = 2,655 | 0 | 10 | 2,665 |
| 4x11x16_477 | 489 + 1,134 + 1,079 = 2,702 | 3 (3) | 3 | 2,708 |
| 4x13x14_498 | 1,356 + 5,041 + 1,234 = 7,631 | 2,002 (2,002) | 2,259 | 11,892 |
| 8x11x13_749 | 1,863 + 2,935 + 2,704 = 7,502 | 759 (759) | 847 | 9,108 |
| 9x11x11_705 | 1,339 + 3,533 + 2,475 = 7,347 | 1,172 (1,172) | 1,502 | 10,021 |
| 9x11x15_953 | 1,726 + 4,010 + 4,318 = 10,054 | 1,713 (1,472) | 1,690 | 13,457 |
| 9x11x16_987 | 1,755 + 3,462 + 4,768 = 9,985 | 2,091 (2,091) | 1,716 | 13,792 |
| 11x13x16_1444 | 2,775 + 3,157 + 4,056 = 9,988 | 345 (345) | 112 | 10,445 |
| 13x13x14_1477 | 4,192 + 4,420 + 5,863 = 14,475 | 3,024 (1,188) | 2,207 | 19,706 |
| 13x13x15_1577 | 4,271 + 4,890 + 5,557 = 14,718 | 2,861 (1,123) | 2,183 | 19,762 |
| 13x14x14_1538 | 4,467 + 4,604 + 6,007 = 15,078 | 3,071 (1,204) | 2,376 | 20,525 |
| 13x14x15_1645 | 4,666 + 4,556 + 6,456 = 15,678 | 3,260 (1,377) | 2,334 | 21,272 |
| 13x15x15_1762 | 5,044 + 5,100 + 5,892 = 16,036 | 3,119 (1,306) | 2,305 | 21,460 |
| 15x16x16_2200 | 14,074 + 13,892 + 15,364 = 43,330 | 9,661 (4,319) | 12,812 | 65,803 |

The programs of the seven schemes of 2026-11-10 and 11 come from one randomized loop of PLinOpt's `optimizer -O 1`.
They are correct over ℚ, but apart from the ternary one they also divide by constants the scheme itself does not
need, so over another ring use the matrices, or make new programs with `python3 tools/ring_slp.py PREFIX`.

| Scheme | Additions | Divisions (of which by a power of 2) | Integer multiplications | Total |
|---|---|---|---|---|
| 12x13x13_1275 | 12,287 | 2,160 (1,735) | 984 | 15,431 |
| 13x14x14_1564 | 32,015 | 11,422 (1,223) | 1,523 | 44,960 |
| 13x14x16_1805 | 10,903 | 0 | 0 | 10,903 |
| 13x15x15_1763 | 16,335 | 2,495 (694) | 1,171 | 20,001 |
| 14x14x15_1715 | 16,825 | 2,680 (792) | 1,210 | 20,715 |
| 14x15x15_1842 | 17,598 | 2,741 (845) | 1,192 | 21,531 |
| 15x16x16_2202 | 43,925 | 13,636 (2,368) | 2,063 | 59,624 |

Both sets of counts are upper bounds that more optimization would lower.

Five of the earlier schemes (13x14x14_1564, 13x15x15_1763, 14x14x15_1715, 14x15x15_1842 and 15x16x16_2202) are in
PLinOpt's `data/` through [jgdumas/plinopt#40](https://github.com/jgdumas/plinopt/pull/40), merged 2026-11-13, and
the first four are in Perminov's catalog since its commit f70078c.  The schemes of 2026-11-20 are, for now, only
here.

## Credits

- Khoruzhii, Serafin, Gelß and Pokutta ([LITA](https://github.com/khoruzhii/lita)): the square schemes behind twelve
  of the rectangular ones and every scheme of `large/`, and 2-reducibility, the reduction all of them use.
- Andrew Perminov's [catalog](https://github.com/dronperminov/FastMatrixMultiplication), where we found LITA's and
  MerlijnW70's schemes and the nine schemes that nine of the records of 2026-11-20 start from; the base schemes
  of all but five of the schemes of `products/`; his paper [arXiv:2606.02480](https://arxiv.org/abs/2606.02480), whose
  shared-factor product those schemes are built with; and his 58-addition scheme cn122, the start of the 55.
- Alexandre Sedoglavic's table at [fmm.univ-lille.fr](https://fmm.univ-lille.fr/) and
  [sedoglavic/fmm_digest](https://github.com/sedoglavic/fmm_digest), and the general form of the serendipitous
  product, which Perminov's paper attributes to him.
- Warren D. Smith, to whose report *Fast matrix multiplication formulae* (technical report, 2002) Perminov's
  paper attributes the serendipitous product.  We have not found a copy of the report and rely on that paper's
  account of it.
- Smirnov's ⟨3×3×6:40⟩, in Dumas, Pernet and Sedoglavic's accurate form (PLinOpt's `data/3x3x6_40_DPS-accurate`),
  the replacing block in 126 of the listed product schemes.
- Heule, Kauers and Seidl's database of rank-23 3×3 schemes; Yinqi Sun's chain covering; Karstadt and Schwartz's
  alternative-basis model; Nielsen's 52; Stapleton and Perminov's 51.
- Dumas, Pernet and Sedoglavic for the rational rank-48 class, made from AlphaEvolve's complex scheme, and Li, Wang
  and Hu for the curve it lies on; Ruchit Patel for the value-set search.
- Kauers and Moosbauer for flip graphs, their ⟨4×4×5:60⟩ scheme and their rank-47 class.
- Jean-Guillaume Dumas and the PLinOpt authors for the format and the checking and optimization tools.
- The searches, checks and write-ups were done with Claude Code (Anthropic).

## License

[Apache License 2.0](LICENSE).
