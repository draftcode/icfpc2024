use crate::assembler::ToExpr;

use anyhow::{bail, Result};
use common::expr::{BinOp, Expr};
use std::rc::Rc;

use crate::util::Direction;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Rng {
    Default,
    Better,
}

impl Rng {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "default" => Some(Self::Default),
            "better" => Some(Self::Better),
            _ => None,
        }
    }

    pub fn next(&self, mut state: u64) -> (Direction, u64) {
        let dir = match state >> 62 {
            0 => Direction::Left,
            1 => Direction::Up,
            2 => Direction::Down,
            3 => Direction::Right,
            _ => unreachable!(),
        };
        match self {
            Self::Default => {
                state = ((state as u128).wrapping_mul(48271) % 18446744073709551557) as u64;
            }
            Self::Better => {
                // https://arxiv.org/abs/2001.05304v3
                state = state.wrapping_mul(0xd1342543de82ef95).wrapping_add(1);
            }
        }
        (dir, state)
    }

    fn expr(&self) -> Expr {
        // RNG expression takes `s` as an argument.
        match self {
            Self::Default => icfp! {
                (% (* s 48271) 18446744073709551557)
            },
            Self::Better => icfp! {
                (% (+ (* s 0xd1342543de82ef95) 1) 18446744073709551616)
            },
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

        let step_expr = match stride {
            1 => icfp! { (take 1 (drop (/ s 4611686018427387904) "LUDR")) },
            2 => icfp! { (take 2 (drop (* (/ s 4611686018427387904) 2) "LLUUDDRR")) },
            _ => bail!("unsupported stride: {stride}"),
        };

        // ***HELP ME***: Optimize this code.
        let expr = icfp! {
            (concat (#header) (fix (fn f s ->
                (if (== s (#last_seed)) {
                    ""
                } else {
                    (concat (#step_expr) (f (#rng_expr)))
                })
            ) (#seed)))
        };
        Ok(expr)
    }
}
