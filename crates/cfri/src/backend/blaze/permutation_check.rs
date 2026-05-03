use crate::backend::{
    binary_extension_fields::B128, poly::multilinear::MultilinearPolynomial, Error,
};
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub(crate) struct BlazePermutationCheck<'a> {
    pub(crate) x: &'a MultilinearPolynomial<B128>,
    pub(crate) y: &'a MultilinearPolynomial<B128>,
    pub(crate) permutation: &'a [usize],
    pub(crate) alpha: B128,
    pub(crate) beta: B128,
}

#[derive(Clone, Debug)]
pub(crate) struct BlazePermutationWitness {
    pub(crate) f_tree: ProductTree,
    pub(crate) g_tree: ProductTree,
}

#[derive(Clone, Debug)]
pub(crate) struct ProductTree {
    levels: Vec<Vec<B128>>,
}

#[derive(Clone, Debug)]
pub(crate) struct ProductTreeLevelPolys {
    pub(crate) left: MultilinearPolynomial<B128>,
    pub(crate) right: MultilinearPolynomial<B128>,
    pub(crate) parent: MultilinearPolynomial<B128>,
}

#[derive(Clone, Debug)]
pub(crate) struct ProductTreeTerminalQuery {
    pub(crate) selector: B128,
    pub(crate) child_chunk: usize,
    pub(crate) parent_chunk: usize,
    pub(crate) left_point: Vec<B128>,
    pub(crate) right_point: Vec<B128>,
    pub(crate) parent_point: Vec<B128>,
}

impl ProductTree {
    pub(crate) fn new(leaves: Vec<B128>) -> Result<Self, Error> {
        if !leaves.len().is_power_of_two() || leaves.is_empty() {
            return Err(Error::InvalidPcsOpen(
                "Blaze permutation product tree expects non-empty power-of-two leaves".to_string(),
            ));
        }

        let mut levels = vec![leaves];
        while levels.last().unwrap().len() > 1 {
            let parent = levels
                .last()
                .unwrap()
                .par_chunks_exact(2)
                .map(|chunk| chunk[0] * chunk[1])
                .collect::<Vec<_>>();
            levels.push(parent);
        }
        Ok(Self { levels })
    }

    pub(crate) fn root(&self) -> B128 {
        self.levels.last().unwrap()[0]
    }

    pub(crate) fn leaves(&self) -> &[B128] {
        &self.levels[0]
    }

    pub(crate) fn num_relation_levels(&self) -> usize {
        self.levels.len().saturating_sub(1)
    }

    pub(crate) fn relation_polys(&self, level: usize) -> Result<ProductTreeLevelPolys, Error> {
        if level + 1 >= self.levels.len() {
            return Err(Error::InvalidPcsOpen(
                "Blaze permutation product tree relation level is out of range".to_string(),
            ));
        }
        let children = &self.levels[level];
        let parent = &self.levels[level + 1];
        let left = children
            .chunks_exact(2)
            .map(|chunk| chunk[0])
            .collect::<Vec<_>>();
        let right = children
            .chunks_exact(2)
            .map(|chunk| chunk[1])
            .collect::<Vec<_>>();
        debug_assert_eq!(left.len(), parent.len());
        Ok(ProductTreeLevelPolys {
            left: MultilinearPolynomial::new(left),
            right: MultilinearPolynomial::new(right),
            parent: MultilinearPolynomial::new(parent.clone()),
        })
    }

    pub(crate) fn check_relations_direct(&self) -> bool {
        (0..self.num_relation_levels()).all(|level| {
            self.levels[level]
                .par_chunks_exact(2)
                .zip(self.levels[level + 1].par_iter())
                .all(|(children, parent)| children[0] * children[1] == *parent)
        })
    }

    pub(crate) fn packed_witness_evals(&self) -> Vec<B128> {
        let leaves = self.leaves();
        let mut evals = Vec::with_capacity(leaves.len() << 1);
        evals.extend_from_slice(leaves);
        for level in self.levels.iter().skip(1) {
            evals.extend_from_slice(level);
        }
        evals.push(B128::from(0));
        debug_assert_eq!(evals.len(), leaves.len() << 1);
        evals
    }

    pub(crate) fn relation_evals(&self) -> Vec<B128> {
        let leaves = self.leaves();
        let mut evals = Vec::with_capacity(leaves.len());
        for level in 0..self.num_relation_levels() {
            evals.extend(
                self.levels[level]
                    .par_chunks_exact(2)
                    .zip(self.levels[level + 1].par_iter())
                    .map(|(children, parent)| *parent - children[0] * children[1])
                    .collect::<Vec<_>>(),
            );
        }
        evals.push(B128::from(0));
        debug_assert_eq!(evals.len(), leaves.len());
        evals
    }

    pub(crate) fn packed_witness_poly(&self) -> MultilinearPolynomial<B128> {
        MultilinearPolynomial::new(self.packed_witness_evals())
    }

    pub(crate) fn relation_poly(&self) -> MultilinearPolynomial<B128> {
        MultilinearPolynomial::new(self.relation_evals())
    }
}

impl ProductTreeLevelPolys {
    pub(crate) fn terminal_expression(&self, point: &[B128]) -> B128 {
        self.parent.evaluate(point) - self.left.evaluate(point) * self.right.evaluate(point)
    }
}

pub(crate) fn terminal_queries(point: &[B128]) -> Vec<ProductTreeTerminalQuery> {
    let num_vars = point.len();
    (0..num_vars)
        .map(|level| {
            let relation_vars = num_vars - level - 1;
            let relation_high_bits = bits_le((1usize << (level + 1)).saturating_sub(2), level + 1);
            let child_high_bits = bits_le((1usize << level).saturating_sub(2), level);

            let mut left_point = Vec::with_capacity(num_vars);
            left_point.push(B128::from(0));
            left_point.extend_from_slice(&point[..relation_vars]);
            left_point.extend_from_slice(&child_high_bits);

            let mut right_point = Vec::with_capacity(num_vars);
            right_point.push(B128::from(1));
            right_point.extend_from_slice(&point[..relation_vars]);
            right_point.extend_from_slice(&child_high_bits);

            let mut parent_point = Vec::with_capacity(num_vars);
            parent_point.extend_from_slice(&point[..relation_vars]);
            parent_point.extend_from_slice(&relation_high_bits);

            ProductTreeTerminalQuery {
                selector: selector(&point[relation_vars..], &relation_high_bits),
                child_chunk: usize::from(level != 0),
                parent_chunk: 1,
                left_point,
                right_point,
                parent_point,
            }
        })
        .collect()
}

pub(crate) fn prove_permutation_check(
    check: &BlazePermutationCheck<'_>,
) -> Result<BlazePermutationWitness, Error> {
    validate_shape(check)?;
    let f_leaves = check
        .x
        .evals
        .iter()
        .zip(check.permutation.iter())
        .map(|(x, pi)| check.alpha - (*x + check.beta * B128::from(*pi as u64)))
        .collect::<Vec<_>>();
    let g_leaves = check
        .y
        .evals
        .iter()
        .enumerate()
        .map(|(i, y)| check.alpha - (*y + check.beta * B128::from(i as u64)))
        .collect::<Vec<_>>();

    Ok(BlazePermutationWitness {
        f_tree: ProductTree::new(f_leaves)?,
        g_tree: ProductTree::new(g_leaves)?,
    })
}

pub(crate) fn verify_permutation_check_direct(
    check: &BlazePermutationCheck<'_>,
    witness: &BlazePermutationWitness,
    boundary_point: &[B128],
) -> Result<(), Error> {
    validate_shape(check)?;
    if boundary_point.len() != check.x.num_vars() {
        return Err(Error::InvalidPcsOpen(
            "Blaze permutation boundary point has wrong dimension".to_string(),
        ));
    }

    if witness.f_tree.root() != witness.g_tree.root() {
        return Err(Error::InvalidPcsOpen(
            "Blaze permutation product roots differ".to_string(),
        ));
    }
    if !witness.f_tree.check_relations_direct() || !witness.g_tree.check_relations_direct() {
        return Err(Error::InvalidPcsOpen(
            "Blaze permutation product tree relation failed".to_string(),
        ));
    }

    let f_leaf_poly = MultilinearPolynomial::new(witness.f_tree.leaves().to_vec());
    let g_leaf_poly = MultilinearPolynomial::new(witness.g_tree.leaves().to_vec());
    let permutation_poly = MultilinearPolynomial::new(
        check
            .permutation
            .iter()
            .map(|pi| B128::from(*pi as u64))
            .collect::<Vec<_>>(),
    );
    let identity_poly = MultilinearPolynomial::new(
        (0..check.permutation.len())
            .map(|i| B128::from(i as u64))
            .collect::<Vec<_>>(),
    );

    let expected_f = check.alpha
        - (check.x.evaluate(boundary_point)
            + check.beta * permutation_poly.evaluate(boundary_point));
    let expected_g = check.alpha
        - (check.y.evaluate(boundary_point) + check.beta * identity_poly.evaluate(boundary_point));
    if f_leaf_poly.evaluate(boundary_point) != expected_f
        || g_leaf_poly.evaluate(boundary_point) != expected_g
    {
        return Err(Error::InvalidPcsOpen(
            "Blaze permutation boundary opening check failed".to_string(),
        ));
    }

    Ok(())
}

fn validate_shape(check: &BlazePermutationCheck<'_>) -> Result<(), Error> {
    if check.x.evals.len() != check.y.evals.len()
        || check.x.evals.len() != check.permutation.len()
        || check.x.num_vars() != check.y.num_vars()
        || !check.permutation.len().is_power_of_two()
    {
        return Err(Error::InvalidPcsOpen(
            "Blaze permutation check inputs have incompatible shapes".to_string(),
        ));
    }

    let mut seen = vec![false; check.permutation.len()];
    for &idx in check.permutation {
        if idx >= seen.len() || seen[idx] {
            return Err(Error::InvalidPcsOpen(
                "Blaze permutation table is not a permutation".to_string(),
            ));
        }
        seen[idx] = true;
    }
    Ok(())
}

fn bits_le(value: usize, width: usize) -> Vec<B128> {
    (0..width)
        .map(|bit| B128::from(((value >> bit) & 1) as u64))
        .collect()
}

fn selector(point: &[B128], bits: &[B128]) -> B128 {
    debug_assert_eq!(point.len(), bits.len());
    point
        .iter()
        .zip(bits.iter())
        .fold(B128::from(1), |acc, (coord, bit)| {
            if *bit == B128::from(1) {
                acc * *coord
            } else {
                acc * (B128::from(1) - *coord)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(v: u64) -> B128 {
        B128::from(v)
    }

    #[test]
    fn permutation_check_accepts_honest_instance() {
        let permutation = vec![2, 0, 3, 1];
        let x = MultilinearPolynomial::new(vec![b(5), b(7), b(11), b(13)]);
        let mut y_evals = vec![b(0); permutation.len()];
        for (src, &dst) in permutation.iter().enumerate() {
            y_evals[dst] = x.evals[src];
        }
        let y = MultilinearPolynomial::new(y_evals);
        let check = BlazePermutationCheck {
            x: &x,
            y: &y,
            permutation: &permutation,
            alpha: b(101),
            beta: b(17),
        };
        let witness = prove_permutation_check(&check).unwrap();

        verify_permutation_check_direct(&check, &witness, &[b(3), b(9)]).unwrap();
    }

    #[test]
    fn permutation_check_rejects_tampered_y() {
        let permutation = vec![2, 0, 3, 1];
        let x = MultilinearPolynomial::new(vec![b(5), b(7), b(11), b(13)]);
        let mut honest_y_evals = vec![b(0); permutation.len()];
        for (src, &dst) in permutation.iter().enumerate() {
            honest_y_evals[dst] = x.evals[src];
        }
        let honest_y = MultilinearPolynomial::new(honest_y_evals);
        let check = BlazePermutationCheck {
            x: &x,
            y: &honest_y,
            permutation: &permutation,
            alpha: b(101),
            beta: b(17),
        };
        let witness = prove_permutation_check(&check).unwrap();

        let bad_y = MultilinearPolynomial::new(vec![b(12), b(5), b(13), b(7)]);
        let bad_check = BlazePermutationCheck { y: &bad_y, ..check };
        assert!(verify_permutation_check_direct(&bad_check, &witness, &[b(3), b(9)]).is_err());
    }

    #[test]
    fn product_tree_relation_expression_matches_boolean_tree_relations() {
        let tree =
            ProductTree::new(vec![b(3), b(5), b(7), b(11), b(13), b(17), b(19), b(23)]).unwrap();
        for level in 0..tree.num_relation_levels() {
            let relation = tree.relation_polys(level).unwrap();
            for index in 0..(1 << relation.parent.num_vars()) {
                let point = (0..relation.parent.num_vars())
                    .map(|bit| {
                        if ((index >> bit) & 1) == 1 {
                            b(1)
                        } else {
                            b(0)
                        }
                    })
                    .collect::<Vec<_>>();
                assert_eq!(relation.terminal_expression(&point), b(0));
            }
        }
    }

    #[test]
    fn packed_witness_layout_keeps_legacy_blaze_shape() {
        let leaves = vec![b(3), b(5), b(7), b(11)];
        let parent_0 = leaves[0] * leaves[1];
        let parent_1 = leaves[2] * leaves[3];
        let root = parent_0 * parent_1;
        let tree = ProductTree::new(leaves.clone()).unwrap();

        assert_eq!(
            tree.packed_witness_evals(),
            vec![
                leaves[0],
                leaves[1],
                leaves[2],
                leaves[3],
                parent_0,
                parent_1,
                root,
                b(0),
            ]
        );
        assert_eq!(tree.relation_evals(), vec![b(0), b(0), b(0), b(0)]);
        assert_eq!(tree.packed_witness_poly().num_vars(), 3);
        assert_eq!(tree.relation_poly().num_vars(), 2);
    }

    #[test]
    fn terminal_queries_reconstruct_product_expression_not_zero_table() {
        let leaves = vec![b(3), b(5), b(7), b(11), b(13), b(17), b(19), b(23)];
        let tree = ProductTree::new(leaves).unwrap();
        let relation = tree.relation_poly();
        let witness_chunks = tree.packed_witness_poly().split(1);
        let point = vec![b(2), b(3), b(4)];

        let reconstructed = terminal_queries(&point)
            .iter()
            .map(|query| {
                let left = witness_chunks[query.child_chunk].evaluate(&query.left_point);
                let right = witness_chunks[query.child_chunk].evaluate(&query.right_point);
                let parent = witness_chunks[query.parent_chunk].evaluate(&query.parent_point);
                query.selector * (parent - left * right)
            })
            .sum::<B128>();

        let direct_expression = terminal_queries(&point)
            .iter()
            .enumerate()
            .map(|(level, query)| {
                let relation = tree.relation_polys(level).unwrap();
                let relation_point_len = relation.parent.num_vars();
                query.selector * relation.terminal_expression(&point[..relation_point_len])
            })
            .sum::<B128>();

        assert_eq!(direct_expression, reconstructed);
        assert_eq!(relation.evaluate(&point), b(0));
        assert!(reconstructed != b(0));
    }
}
