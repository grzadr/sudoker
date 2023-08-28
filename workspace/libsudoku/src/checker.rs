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

    #[test]
    fn test_valchecker_new() {
        let temp = ValChecker::new(3);
        assert_eq!(temp.values, vec![[false; 3]; 3])
    }

}