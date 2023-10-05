use crate::board::Board;

trait Solver {
    fn apply(&self, b: &mut Board) -> Result<bool, String>;
}

struct EliminateObviousSolver {}

impl Solver for EliminateObviousSolver {
    /// Return `true` if the board state has been updated.
    fn apply(&self, b: &mut Board) -> Result<bool, String> {
        let mut made_progress = false;
        for i in 0..81 {
            let f = b.field_seq(i);
            if !f.is_solved() {
                continue
            }

            let v = f.value.unwrap();
            let x = i % 9;
            let y = i / 9;
            for j in 0..9 {
                if j != y {
                    if b.field(x, j).eliminate(v)? {
                        made_progress = true;
                    }
                }
                if j != x {
                    if b.field(j, y).eliminate(v)? {
                        made_progress = true;
                    }
                }
            }
        }

        Ok(made_progress)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::solver::EliminateObviousSolver;
    use crate::solver::Solver;

    fn empty_board() -> Board {
        let b_res = Board::parse("\
        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _ _
    ");
        assert!(b_res.is_ok());
        return b_res.unwrap();
    }

    #[test]
    fn test_obvious_solver() {
        let mut b = empty_board();
        let s = EliminateObviousSolver{};

        b.field_seq(0).solve(1).unwrap();

        // todo: verify with assert
        s.apply(&mut b).unwrap();

        assert!(!b.field(0, 5).options.contains(&1));
        assert!(!b.field(8, 0).options.contains(&1));
        assert!(b.field(1, 5).options.contains(&1));
    }
}