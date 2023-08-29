pub type L = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Loc{
    pub row: u8,
    pub col: u8
}

impl Loc {
    pub fn new(row: L, col: L) -> Self {
        Self{row, col}
    }

    pub fn sqr(&self, size: u8) -> u8 {
        ((self.row - 1).div_euclid(size) * size) + (self.col - 1).div_euclid(size) + 1
    }
}

impl From<(u8, u8)> for Loc {
    fn from(other: (u8, u8)) -> Self {
        Self::new(other.0, other.1)
    }
}

impl From<Loc> for (u8, u8) {
    fn from(other: Loc) -> (u8, u8) {
        (other.row, other.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loc_new() {
        let temp = Loc::new(1, 1);
        assert!(temp.row == 1 && temp.col == 1);
    }

    #[test]
    fn test_loc_from_tuple() {
        let temp = Loc::from((1, 1));
        assert!(temp.row == 1 && temp.col == 1);
    }

    #[test]
    fn test_loc_into_tuple() {
        assert_eq!(<(u8, u8)>::from(Loc::new(1, 1)), (1,1));
    }

    #[test]
    fn test_loc_sqr() {
        assert_eq!(Loc::new(1, 1).sqr(1), 1);

        assert_eq!(Loc::new(1, 1).sqr(2), 1);
        assert_eq!(Loc::new(1, 2).sqr(2), 1);
        assert_eq!(Loc::new(1, 3).sqr(2), 2);
        assert_eq!(Loc::new(1, 4).sqr(2), 2);
        assert_eq!(Loc::new(2, 1).sqr(2), 1);
        assert_eq!(Loc::new(2, 2).sqr(2), 1);
        assert_eq!(Loc::new(2, 3).sqr(2), 2);
        assert_eq!(Loc::new(2, 4).sqr(2), 2);
        assert_eq!(Loc::new(3, 1).sqr(2), 3);
        assert_eq!(Loc::new(3, 2).sqr(2), 3);
        assert_eq!(Loc::new(3, 3).sqr(2), 4);
        assert_eq!(Loc::new(3, 4).sqr(2), 4);
        assert_eq!(Loc::new(4, 1).sqr(2), 3);
        assert_eq!(Loc::new(4, 2).sqr(2), 3);
        assert_eq!(Loc::new(4, 3).sqr(2), 4);
        assert_eq!(Loc::new(4, 4).sqr(2), 4);

        assert_eq!(Loc::new(1, 1).sqr(3), 1);
        assert_eq!(Loc::new(1, 2).sqr(3), 1);
        assert_eq!(Loc::new(1, 3).sqr(3), 1);
        assert_eq!(Loc::new(1, 4).sqr(3), 2);
        assert_eq!(Loc::new(1, 5).sqr(3), 2);
        assert_eq!(Loc::new(1, 6).sqr(3), 2);
        assert_eq!(Loc::new(1, 7).sqr(3), 3);
        assert_eq!(Loc::new(1, 8).sqr(3), 3);
        assert_eq!(Loc::new(1, 9).sqr(3), 3);
        assert_eq!(Loc::new(2, 1).sqr(3), 1);
        assert_eq!(Loc::new(2, 2).sqr(3), 1);
        assert_eq!(Loc::new(2, 3).sqr(3), 1);
        assert_eq!(Loc::new(2, 4).sqr(3), 2);
        assert_eq!(Loc::new(2, 5).sqr(3), 2);
        assert_eq!(Loc::new(2, 6).sqr(3), 2);
        assert_eq!(Loc::new(2, 7).sqr(3), 3);
        assert_eq!(Loc::new(2, 8).sqr(3), 3);
        assert_eq!(Loc::new(2, 9).sqr(3), 3);
        assert_eq!(Loc::new(3, 1).sqr(3), 1);
        assert_eq!(Loc::new(3, 2).sqr(3), 1);
        assert_eq!(Loc::new(3, 3).sqr(3), 1);
        assert_eq!(Loc::new(3, 4).sqr(3), 2);
        assert_eq!(Loc::new(3, 5).sqr(3), 2);
        assert_eq!(Loc::new(3, 6).sqr(3), 2);
        assert_eq!(Loc::new(3, 7).sqr(3), 3);
        assert_eq!(Loc::new(3, 8).sqr(3), 3);
        assert_eq!(Loc::new(3, 9).sqr(3), 3);
        assert_eq!(Loc::new(4, 1).sqr(3), 4);
        assert_eq!(Loc::new(4, 2).sqr(3), 4);
        assert_eq!(Loc::new(4, 3).sqr(3), 4);
        assert_eq!(Loc::new(4, 4).sqr(3), 5);
        assert_eq!(Loc::new(4, 5).sqr(3), 5);
        assert_eq!(Loc::new(4, 6).sqr(3), 5);
        assert_eq!(Loc::new(4, 7).sqr(3), 6);
        assert_eq!(Loc::new(4, 8).sqr(3), 6);
        assert_eq!(Loc::new(4, 9).sqr(3), 6);
        assert_eq!(Loc::new(5, 1).sqr(3), 4);
        assert_eq!(Loc::new(5, 2).sqr(3), 4);
        assert_eq!(Loc::new(5, 3).sqr(3), 4);
        assert_eq!(Loc::new(5, 4).sqr(3), 5);
        assert_eq!(Loc::new(5, 5).sqr(3), 5);
        assert_eq!(Loc::new(5, 6).sqr(3), 5);
        assert_eq!(Loc::new(5, 7).sqr(3), 6);
        assert_eq!(Loc::new(5, 8).sqr(3), 6);
        assert_eq!(Loc::new(5, 9).sqr(3), 6);
        assert_eq!(Loc::new(6, 1).sqr(3), 4);
        assert_eq!(Loc::new(6, 2).sqr(3), 4);
        assert_eq!(Loc::new(6, 3).sqr(3), 4);
        assert_eq!(Loc::new(6, 4).sqr(3), 5);
        assert_eq!(Loc::new(6, 5).sqr(3), 5);
        assert_eq!(Loc::new(6, 6).sqr(3), 5);
        assert_eq!(Loc::new(6, 7).sqr(3), 6);
        assert_eq!(Loc::new(6, 8).sqr(3), 6);
        assert_eq!(Loc::new(6, 9).sqr(3), 6);
        assert_eq!(Loc::new(7, 1).sqr(3), 7);
        assert_eq!(Loc::new(7, 2).sqr(3), 7);
        assert_eq!(Loc::new(7, 3).sqr(3), 7);
        assert_eq!(Loc::new(7, 4).sqr(3), 8);
        assert_eq!(Loc::new(7, 5).sqr(3), 8);
        assert_eq!(Loc::new(7, 6).sqr(3), 8);
        assert_eq!(Loc::new(7, 7).sqr(3), 9);
        assert_eq!(Loc::new(7, 8).sqr(3), 9);
        assert_eq!(Loc::new(7, 9).sqr(3), 9);
        assert_eq!(Loc::new(8, 1).sqr(3), 7);
        assert_eq!(Loc::new(8, 2).sqr(3), 7);
        assert_eq!(Loc::new(8, 3).sqr(3), 7);
        assert_eq!(Loc::new(8, 4).sqr(3), 8);
        assert_eq!(Loc::new(8, 5).sqr(3), 8);
        assert_eq!(Loc::new(8, 6).sqr(3), 8);
        assert_eq!(Loc::new(8, 7).sqr(3), 9);
        assert_eq!(Loc::new(8, 8).sqr(3), 9);
        assert_eq!(Loc::new(8, 9).sqr(3), 9);
        assert_eq!(Loc::new(9, 1).sqr(3), 7);
        assert_eq!(Loc::new(9, 2).sqr(3), 7);
        assert_eq!(Loc::new(9, 3).sqr(3), 7);
        assert_eq!(Loc::new(9, 4).sqr(3), 8);
        assert_eq!(Loc::new(9, 5).sqr(3), 8);
        assert_eq!(Loc::new(9, 6).sqr(3), 8);
        assert_eq!(Loc::new(9, 7).sqr(3), 9);
        assert_eq!(Loc::new(9, 8).sqr(3), 9);
        assert_eq!(Loc::new(9, 9).sqr(3), 9);
    }
}