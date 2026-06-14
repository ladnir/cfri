#!/usr/bin/env python3
"""Decisive closure test: is there a FINITE alphabet of column value-classes closed under the fold?

The first moment only needs, ultimately, whether each top codeword coordinate is zero.  A column at
replica 2R is v=(a,b) with a,b in F^R; the fold with challenge t in F^* produces two replica-R
columns: a + t*b (left block) and a + (t+1)*b (right block).  So the "behavior" of v is the function
  t |-> ( class_R(a + t*b), class_R(a + (t+1)*b) )
where class_R is the (recursively defined) equivalence class of a replica-R column.  Two columns are
equivalent (bisimilar) iff they have the same behavior function -- because then they are
indistinguishable in every coordinate of every ancestor codeword, for every challenge.

Base: class_1 on F^1 = (x == 0)  -> 2 classes {zero, nonzero}.
Refine: class_{2R}(v) = behavior function above, using class_R labels.

If |class_{2^j}| stays BOUNDED as j grows, the per-column class alphabet is finite and closed under
the fold => an EXACT transfer-operator recurrence on the (polynomial-size) class-histogram is
tractable => the tight first moment, hence near-MDS, is provable.  If it grows per level => genuine
state-explosion wall.

Run: python rfc_column_class_closure.py --q 3 --max-dim 8
"""

from __future__ import annotations

import argparse
import itertools


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--q", type=int, required=True)
    ap.add_argument("--max-dim", type=int, default=8, help="largest replica dim 2^j to refine to")
    ap.add_argument("--t-domain", choices=["nonzero", "all"], default="nonzero")
    args = ap.parse_args()
    q = args.q
    t_dom = list(range(1, q)) if args.t_domain == "nonzero" else list(range(q))

    # level 1: F^1
    dim = 1
    # class label of each vector at the current level, keyed by the tuple
    labels: dict[tuple, int] = {}
    for x in range(q):
        labels[(x,)] = 0 if x == 0 else 1
    nclasses = len(set(labels.values()))
    print(f"# column-class closure under det-1 fold: q={q} t_domain={args.t_domain}")
    print("replica_dim,num_classes,num_vectors")
    print(f"{dim},{nclasses},{q**dim}")

    while dim * 2 <= args.max_dim:
        newdim = dim * 2
        # signature of v=(a,b), a,b in F^dim: function t -> (label(a+t b), label(a+(t+1)b))
        sig_to_class: dict[tuple, int] = {}
        new_labels: dict[tuple, int] = {}
        for v in itertools.product(range(q), repeat=newdim):
            a = v[:dim]
            b = v[dim:]
            sig = []
            for t in t_dom:
                left = tuple((a[i] + t * b[i]) % q for i in range(dim))
                right = tuple((a[i] + (t + 1) * b[i]) % q for i in range(dim))
                sig.append((labels[left], labels[right]))
            sig = tuple(sig)
            cls = sig_to_class.get(sig)
            if cls is None:
                cls = len(sig_to_class)
                sig_to_class[sig] = cls
            new_labels[v] = cls
        labels = new_labels
        dim = newdim
        nclasses = len(sig_to_class)
        print(f"{dim},{nclasses},{q**dim}")

    print(f"# final: {nclasses} classes at replica_dim={dim}")


if __name__ == "__main__":
    main()
