#[derive(Debug, PartialEq, Clone)]
pub struct ValChecker {
    pub values: Vec<Vec<bool>>,
}

impl ValChecker {
    pub fn new(top: u8) -> Self {
        Self {
            values: vec![vec![false; top.into()]; top.into()],
        }
    }

    pub fn set(&mut self, loc: u8, value: u8) {
        self.values[usize::from(loc) - 1][usize::from(value) - 1] = true;
    }

    pub fn unset(&mut self, loc: u8, value: u8) {
        self.values[usize::from(loc) - 1][usize::from(value) - 1] = false;
    }

    pub fn is_solved(&self) -> bool {
        self.values.iter().flatten().all(|x| *x == true)
    }

    pub fn available_values(&self, loc: u8) -> Vec<Option<u8>> {
        self.values[usize::from(loc) - 1]
            .iter()
            .enumerate()
            .map(|(l, v)| if !*v { Some((l + 1) as u8) } else { None })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOP: u8 = 4;

    fn gen_values(top: u8) -> Vec<(u8, u8)> {
        let top_range = 1..=top;
        top_range
            .clone()
            .flat_map(|x| top_range.clone().map(move |y| (x, y)))
            .collect()
    }

    #[test]
    fn test_new() {
        let temp = ValChecker::new(TOP);
        assert_eq!(temp.values, vec![vec![false; TOP as usize]; TOP as usize]);
    }

    #[test]
    fn test_set() {
        let mut temp = ValChecker::new(TOP);
        let values = gen_values(TOP);

        for (ele, val) in values {
            temp.set(ele, val);
        }

        assert_eq!(temp.values, vec![vec![true; TOP as usize]; TOP as usize]);
    }

    #[test]
    fn test_unset() {
        let mut temp = ValChecker::new(TOP);
        let values = gen_values(TOP);

        for (ele, val) in values.clone() {
            temp.set(ele, val);
        }

        for (ele, val) in values {
            temp.unset(ele, val);
        }

        assert_eq!(temp.values, vec![vec![false; TOP as usize]; TOP as usize]);
    }

    #[test]
    fn test_is_solved() {
        let mut temp = ValChecker::new(TOP);
        let values = gen_values(TOP);

        for (ele, val) in values {
            assert_ne!(temp.is_solved(), true);
            temp.set(ele, val);
        }

        assert_eq!(temp.is_solved(), true);
    }
}
