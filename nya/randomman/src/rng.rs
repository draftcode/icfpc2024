use crate::assembler::ToExpr;

use anyhow::{bail, Result};
use common::expr::{BinOp, Expr};
use rand::Rng as _;
use std::rc::Rc;

use crate::util::Direction;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Rng {
    Default,
    Better,
    DefaultRev,
    SmallModRev,
    Reference,
}

impl Rng {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "default" => Some(Self::Default),
            "better" => Some(Self::Better),
            "default-rev" => Some(Self::DefaultRev),
            "small-mod-rev" => Some(Self::SmallModRev),
            "reference" => Some(Self::Reference),
            _ => None,
        }
    }

    pub fn skip_first_seed(&self) -> bool {
        matches!(self, Self::DefaultRev | Self::SmallModRev)
    }

    pub fn next(&self, state: u64) -> (Direction, u64) {
        match self {
            Self::Default => (
                (state >> 62).into(),
                ((state as u128).wrapping_mul(48271) % 18446744073709551557) as u64,
            ),
            // https://arxiv.org/abs/2001.05304v3
            Self::Better => (
                (state >> 62).into(),
                state.wrapping_mul(0xd1342543de82ef95).wrapping_add(1),
            ),
            Self::DefaultRev => (
                (state >> 62).into(),
                // pow(48271, -1, 18446744073709551557) = 17779510845628573806
                ((state as u128).wrapping_mul(17779510845628573806) % 18446744073709551557) as u64,
            ),
            Self::SmallModRev => (
                // 830579: smallest prime less than 94 ** 3 = 830584
                // pow(48271, -1, 830579) = 81542
                (state / 207645).into(),
                ((state as u128).wrapping_mul(81542) % 830579) as u64,
            ),
            Self::Reference => ((state >> 62).into(), rand::thread_rng().gen::<u64>()),
        }
    }

    fn expr(&self) -> Expr {
        // RNG expression takes `s` as an argument.
        match self {
            Self::Default | Self::DefaultRev => icfp! {
                (% (* s 48271) 18446744073709551557)
            },
            Self::Better => icfp! {
                (% (+ (* s 0xd1342543de82ef95) 1) 18446744073709551616)
            },
            Self::SmallModRev => icfp! {
                (% (* s 48271) 830579)
            },
            Self::Reference => panic!("reference RNG is not compilable"),
        }
    }

    pub fn compile_expr(
        &self,
        problem_id: usize,
        seed: u64,
        stride: usize,
        moves: usize,
    ) -> Result<Expr> {
        let rng_expr = self.expr();

        let header = format!("solve lambdaman{problem_id} ");
        let seed = seed as u128;

        let steps = (moves / stride) as u128;

        let mut seeds = vec![seed as u64];
        for _ in 1..=steps {
            let (_, new_seed) = self.next(*seeds.last().unwrap());
            seeds.push(new_seed);
        }
        let last_seed = seeds.pop().unwrap();
        if seeds.contains(&last_seed) {
            bail!("seed cycle detected");
        }
        let last_seed = last_seed as u128;

        let div = if self == &Self::SmallModRev {
            207645
        } else {
            4611686018427387904
        };

        let step_expr = match stride {
            1 => icfp! { (take 1 (drop (/ s (#div)) "LUDR")) },
            2 => icfp! { (take 2 (drop (* (/ s (#div)) 2) "LLUUDDRR")) },
            _ => bail!("unsupported stride: {stride}"),
        };

        let expr = match self {
            // ***HELP ME***: Optimize this code.
            Self::Default | Self::Better => icfp! {
                (concat (#header) (fix (fn f s ->
                    (if (== s (#last_seed)) {
                        ""
                    } else {
                        (concat (#step_expr) (f (#rng_expr)))
                    })
                ) (#seed)))
            },
            Self::DefaultRev | Self::SmallModRev => {
                icfp! {
                    (fix (fn f s ->
                        (if (== s (#seed)) {
                            (#header)
                        } else {
                            (concat (f (#rng_expr)) (#step_expr))
                        })
                    ) (#last_seed))
                }
            }
            Self::Reference => panic!("reference RNG is not compilable"),
        };
        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_prev() {
        let (_, state) = Rng::DefaultRev.next(1);
        let (_, state) = Rng::Default.next(state);
        assert_eq!(state, 1);
    }
}
