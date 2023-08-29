use super::loc::Loc;
use super::checker::ValChecker;

#[derive(Debug, PartialEq, Clone)]
pub struct Verifier {
    size: u8,
    rows: ValChecker,
    cols: ValChecker,
    sqrs: ValChecker,
}

impl Verifier {
    pub fn new(size: u8) -> Self {
        let top = size.pow(2);
        Self {
            size,
            rows: ValChecker::new(top),
            cols: ValChecker::new(top),
            sqrs: ValChecker::new(top),
        }
    }

    pub fn set(&mut self, loc: &Loc, val: u8) {
        self.rows.set(loc.row, val);
        self.cols.set(loc.col, val);
        self.sqrs.set(loc.sqr(self.size), val);
    }

    pub fn unset(&mut self, loc: &Loc, val: u8) {
        self.rows.unset(loc.row, val);
        self.cols.unset(loc.col, val);
        self.sqrs.unset(loc.sqr(self.size), val);
    }

    pub fn is_solved(&self) -> bool {
        self.rows.is_solved() && self.cols.is_solved() && self.sqrs.is_solved()
    }

    pub fn available_values(&self, loc: &Loc) -> Vec<u8> {
        let rows = self.rows.available_values(loc.row);
        let cols = self.cols.available_values(loc.col);
        let sqrs = self.sqrs.available_values(loc.sqr(self.size));

        rows.iter()
            .zip(cols.iter())
            .zip(sqrs.iter())
            .filter_map(|((r, c), s)| {
                if r.is_some() && c.is_some() && s.is_some() {
                    *r
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: u8 = 2;
    const TOP: u8 = SIZE.pow(2);

    #[test]
    fn test_new() {
        assert_eq!(
            Verifier::new(SIZE),
            Verifier{
                size: SIZE,
                rows: ValChecker::new(TOP),
                cols: ValChecker::new(TOP),
                sqrs: ValChecker::new(TOP),
            }
        )
    }

    #[test]
    fn test_simple_set() {
        let mut temp = Verifier::new(SIZE);

        temp.set(&Loc::new(1,1), 1);

        assert!(temp.rows.values[0][0]);
        assert!(temp.cols.values[0][0]);
        assert!(temp.sqrs.values[0][0]);
    }

    #[test]
    fn test_simple_unset() {
        let mut temp = Verifier::new(SIZE);

        temp.set(&Loc::new(1,1), 1);
        temp.unset(&Loc::new(1,1), 1);

        assert!(!temp.rows.values[0][0]);
        assert!(!temp.cols.values[0][0]);
        assert!(!temp.sqrs.values[0][0]);
    }
    #[test]
    fn test_is_solved_rows() {
        let temp = Verifier{
            size: SIZE,
            rows: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
            cols: ValChecker{values: vec![vec![false; TOP as usize]; TOP as usize]},
            sqrs: ValChecker{values: vec![vec![false; TOP as usize]; TOP as usize]},
        };

        assert!(!temp.is_solved())
    }

    #[test]
    fn test_is_solved_cols() {
        let temp = Verifier{
            size: SIZE,
            rows: ValChecker{values: vec![vec![false; TOP as usize]; TOP as usize]},
            cols: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
            sqrs: ValChecker{values: vec![vec![false; TOP as usize]; TOP as usize]},
        };

        assert!(!temp.is_solved())
    }

    #[test]
    fn test_is_solved_sqrs() {
        let temp = Verifier{
            size: SIZE,
            rows: ValChecker{values: vec![vec![false; TOP as usize]; TOP as usize]},
            cols: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
            sqrs: ValChecker{values: vec![vec![false; TOP as usize]; TOP as usize]},
        };

        assert!(!temp.is_solved())
    }

    #[test]
    fn test_is_solved_rows_cols() {
        let temp = Verifier{
            size: SIZE,
            rows: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
            cols: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
            sqrs: ValChecker{values: vec![vec![false; TOP as usize]; TOP as usize]},
        };

        assert!(!temp.is_solved())
    }

    #[test]
    fn test_is_solved_rows_sqrs() {
        let temp = Verifier{
            size: SIZE,
            rows: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
            cols: ValChecker{values: vec![vec![false; TOP as usize]; TOP as usize]},
            sqrs: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
        };

        assert!(!temp.is_solved())
    }

    #[test]
    fn test_is_solved_cols_sqrs() {
        let temp = Verifier{
            size: SIZE,
            rows: ValChecker{values: vec![vec![false; TOP as usize]; TOP as usize]},
            cols: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
            sqrs: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
        };

        assert!(!temp.is_solved())
    }

    #[test]
    fn test_is_solved_all() {
        let temp = Verifier{
            size: SIZE,
            rows: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
            cols: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
            sqrs: ValChecker{values: vec![vec![true; TOP as usize]; TOP as usize]},
        };

        assert!(temp.is_solved())
    }
}