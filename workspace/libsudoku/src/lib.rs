use std::collections::HashMap;
use std::ops::Deref;
use serde_json::Value;
pub mod loc;
mod checker;
mod verifier;
use loc::Loc;
use verifier::Verifier;


impl From<(u8, &BoardMap)> for Verifier {
    fn from(other: (u8, &BoardMap)) -> Self {
        let (size, map) = other;
        let mut s = Self::new(size);
        for (loc, value) in map {
            if let Some(v) = value {
                s.set(loc, *v);
            }
        }
        s
    }
}

type BoardMap = HashMap<Loc, Option<u8>>;

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    size: u8,
    top: u8,
    values: BoardMap,
    verification: Verifier,
}

impl Board {
    fn calc_top(n: u8) -> u8 {
        n.pow(2)
    }

    fn gen_all_locs(top: u8) -> Vec<Loc> {
        let rn = 1..=top;
        rn.clone()
            .flat_map(|x| rn.clone().map(move |y| Loc::new(x, y)))
            .collect()
    }

    fn init_values(top: u8) -> BoardMap {
        let mut values = HashMap::new();
        values.reserve(top.pow(2).into());
        for l in Board::gen_all_locs(top) {
            values.insert(l, None);
        }
        values
    }

    pub fn new(size: u8) -> Self {
        let top = Board::calc_top(size);
        let values = Board::init_values(top);
        Self {
            size,
            top,
            values,
            verification: Verifier::new(size),
        }
    }

    pub fn is_solved(&self) -> bool {
        self.verification.is_solved()
    }

    pub fn get_str(&self, loc: &Loc) -> String {
        if let Some(v) = self.values.get(loc).unwrap() {
            v.to_string()
        } else {
            "■".to_string()
        }
    }

    fn draw_upper_bar(size: u8) -> String {
        let size = usize::from(size);
        let sqr = vec![vec!["═══"; size].join("═"); size].join("╦");
        format!("╔{}╗\n", sqr)
    }

    fn draw_lower_bar(size: u8) -> String {
        let size = usize::from(size);
        let sqr = vec![vec!["═══"; size].join("═"); size].join("╩");
        format!("╚{}╝\n", sqr)
    }

    fn draw_middle_bar(size: u8) -> String {
        let size = usize::from(size);
        let sqr = vec![vec!["═══"; size].join("═"); size].join("╬");
        format!("╠{}╣\n", sqr)
    }

    fn draw_middle_soft(size: u8) -> String {
        let size = usize::from(size);
        let sqr = vec![vec!["───"; size].join("┼"); size].join("║");
        format!("║{}║\n", sqr)
    }

    pub fn draw(&self) -> String {
        let mut output = String::new();

        output.push_str(&Board::draw_upper_bar(self.size));
        for row in 1..=self.top {
            let mut temp: Vec<String> = Vec::new();
            temp.reserve(usize::from(self.top) * 2);
            for col in 1..=self.top {
                temp.push(format!(" {} ", self.get_str(&Loc::new(row, col))));
                if col % self.size != 0 {
                    temp.push("│".to_string());
                } else {
                    temp.push("║".to_string());
                }
            }
            output.push_str(&format!("║{}║\n", temp[..temp.len() - 1].join("")));
            if row < self.top {
                if row % self.size == 0 {
                    output.push_str(&Board::draw_middle_bar(self.size));
                } else {
                    output.push_str(&Board::draw_middle_soft(self.size));
                }
            }
        }
        output.push_str(&Board::draw_lower_bar(self.size));
        output
    }

    pub fn to_vec(&self) -> Vec<Vec<Option<u8>>> {
        let mut result = vec![vec![None; self.top.into()]; self.top.into()];
        for (loc, value) in self.iter() {
            if let Some(v) = value {
                let (row, col) = (loc.row, loc.col);
                result[usize::from(row - 1)][usize::from(col - 1)] = Some(*v);
            }
        }
        result
    }

    pub fn available_values(&self, loc: &Loc) -> Vec<u8> {
        self.verification.available_values(loc)
    }

    pub fn set(&mut self, loc: &Loc, value: Option<u8>) {
        let current = self.values.get_mut(loc).unwrap();

        if let Some(old) = current {
            self.verification.unset(loc, *old);
            *current = None;
        }

        if let Some(new) = value {
            *current = Some(new);
            self.verification.set(loc, new);
        }
    }

    pub fn try_set(&mut self, loc: &Loc, value: Option<u8>) -> Result<(), String> {
        if value.is_some() && !self.available_values(loc).contains(&value.unwrap()) {
            return Err(format!(
                "'{}' cannot be inserted in {:?}",
                value.unwrap(),
                loc
            ));
        }

        self.set(loc, value);
        Ok(())
    }

    fn try_to_solve(&mut self, locs: &[Loc]) -> bool {
        if locs.is_empty() {
            return self.is_solved();
        }

        let loc = &locs[0];
        let rest = &locs[1..];

        for value in self.available_values(loc) {
            self.set(loc, Some(value));
            if self.try_to_solve(rest) {
                return true;
            }
            self.set(loc, None);
        }

        false
    }

    pub fn solve(&self) -> Option<Self> {
        let mut result = self.clone();

        let mut missing_fields: Vec<Loc> = result
            .iter()
            .filter_map(|(loc, value)| if value.is_none() { Some(*loc) } else { None })
            .collect();

        missing_fields.sort();

        if missing_fields.is_empty() && !result.is_solved() {
            return None;
        }

        let mut possible_values = Vec::new();

        for loc in missing_fields {
            let values = self.available_values(&loc);
            if values.is_empty() {
                return None;
            }
            possible_values.push((loc, values));
        }

        possible_values
            .sort_by(|(_, lvec), (_, rvec)| lvec.len().partial_cmp(&rvec.len()).unwrap());

        let locs: Vec<Loc> = possible_values.into_iter().map(|(loc, _)| loc).collect();
        
        if result.try_to_solve(&locs[..]) {
            Some(result)
        } else {
            None
        }
    }
}

impl From<(u8, BoardMap)> for Board {
    fn from(other: (u8, BoardMap)) -> Self {
        let (size, values) = other;
        let top = Board::calc_top(size);
        let expected_len = usize::from(top.pow(2));
        assert_eq!(
            values.len(),
            expected_len,
            "BoardMap length is {} instead of {}",
            values.len(),
            expected_len
        );
        let mut keys: Vec<Loc> = values.keys().copied().collect();
        keys.sort();

        assert!(keys.into_iter().eq(Board::gen_all_locs(top)));

        let verification = Verifier::from((size, &values));

        Self {
            size,
            top,
            values,
            verification,
        }
    }
}

impl Deref for Board {
    type Target = BoardMap;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

use serde::ser::{Serialize, SerializeSeq, Serializer};

impl Serialize for Board {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.top as usize))?;
        for element in self.to_vec() {
            seq.serialize_element(&element)?;
        }
        seq.end()
    }
}

impl From<&Board> for Vec<Vec<Option<u8>>> {
    fn from(other: &Board) -> Vec<Vec<Option<u8>>> {
        let mut result = vec![vec![None; other.top.into()]; other.top.into()];
        for (loc, value) in other.iter() {
            if let Some(v) = value {
                let (row, col) = (loc.row, loc.col);
                result[usize::from(row - 1)][usize::from(col - 1)] = Some(*v);
            }
        }
        result
    }
}

impl From<Value> for Board {
    fn from(other: Value) -> Self {
        let rows = match other.as_array() {
            Some(a) => a,
            _ => panic!("Could convert to array - {}", other),
        };
        let size = f64::from(rows.len() as u8).sqrt() as u8;
        if rows.len() == 9 {
            assert_eq!(size, 3);
        } else if rows.len() == 4 {
            assert_eq!(size, 2)
        } else if rows.len() == 1 {
            assert_eq!(size, 1);
        }
        let mut values: HashMap<Loc, Option<u8>> = HashMap::new();

        for (row, cols) in rows.iter().enumerate() {
            let row = (row + 1) as u8;
            for (col, v) in cols.as_array().unwrap().iter().enumerate() {
                let col = (col + 1) as u8;
                values.insert(
                    Loc::new(row, col),
                    if let serde_json::Value::Number(v) = v {
                        Some(v.as_u64().unwrap() as u8)
                    } else {
                        None
                    },
                );
            }
        }
        Board::from((size, values))
    }
}

impl From<&Board> for serde_json::Value {
    fn from(other: &Board) -> serde_json::Value {
        serde_json::json!(other)
    }
}

impl From<&str> for Board {
    fn from(other: &str) -> Self {
        Self::from(serde_json::from_str::<Value>(other).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_gen_all_locs() {
        assert_eq!(Board::gen_all_locs(1), vec![Loc::new(1, 1)]);
        assert_eq!(
            Board::gen_all_locs(4),
            vec![
                (1, 1).into(),
                Loc::new(1, 2),
                Loc::new(1, 3),
                Loc::new(1, 4),
                Loc::new(2, 1),
                Loc::new(2, 2),
                Loc::new(2, 3),
                Loc::new(2, 4),
                Loc::new(3, 1),
                Loc::new(3, 2),
                Loc::new(3, 3),
                Loc::new(3, 4),
                Loc::new(4, 1),
                Loc::new(4, 2),
                Loc::new(4, 3),
                Loc::new(4, 4),
            ]
        );
    }

    #[test]
    fn test_board_new() {
        assert_eq!(
            Board::new(1),
            Board {
                size: 1,
                top: 1,
                values: HashMap::from([(Loc::new(1, 1), None)]),
                verification: Verifier::new(1)
            }
        );
        assert_eq!(
            Board::new(2),
            Board {
                size: 2,
                top: 4,
                values: HashMap::from([
                    (Loc::new(1, 1), None),
                    (Loc::new(1, 2), None),
                    (Loc::new(1, 3), None),
                    (Loc::new(1, 4), None),
                    (Loc::new(2, 1), None),
                    (Loc::new(2, 2), None),
                    (Loc::new(2, 3), None),
                    (Loc::new(2, 4), None),
                    (Loc::new(3, 1), None),
                    (Loc::new(3, 2), None),
                    (Loc::new(3, 3), None),
                    (Loc::new(3, 4), None),
                    (Loc::new(4, 1), None),
                    (Loc::new(4, 2), None),
                    (Loc::new(4, 3), None),
                    (Loc::new(4, 4), None),
                ]),
                verification: Verifier::new(2)
            }
        );
    }

    #[test]
    fn test_board_from() {
        assert_eq!(
            Board::new(1),
            Board::from((1, HashMap::from([(Loc::new(1, 1), None)])))
        );
        assert_eq!(
            Board::new(2),
            Board::from((
                2,
                HashMap::from([
                    (Loc::new(1, 1), None),
                    (Loc::new(1, 2), None),
                    (Loc::new(1, 3), None),
                    (Loc::new(1, 4), None),
                    (Loc::new(2, 1), None),
                    (Loc::new(2, 2), None),
                    (Loc::new(2, 3), None),
                    (Loc::new(2, 4), None),
                    (Loc::new(3, 1), None),
                    (Loc::new(3, 2), None),
                    (Loc::new(3, 3), None),
                    (Loc::new(3, 4), None),
                    (Loc::new(4, 1), None),
                    (Loc::new(4, 2), None),
                    (Loc::new(4, 3), None),
                    (Loc::new(4, 4), None),
                ])
            ))
        )
    }

    #[test]
    fn test_board_is_solved() {
        let boards = [
            Board::from("[[1]]"),
            Board::from("[[1,2,3,4],[3,4,1,2],[2,1,4,3],[4,3,2,1]]"),
            Board::from("[[4,3,5,2,6,9,7,8,1], [6,8,2,5,7,1,4,9,3], [1,9,7,8,3,4,5,6,2], [8,2,6,1,9,5,3,4,7], [3,7,4,6,8,2,9,1,5], [9,5,1,7,4,3,6,2,8], [5,1,9,3,2,6,8,7,4], [2,4,8,9,5,7,1,3,6], [7,6,3,4,1,8,2,5,9]]"),
        ];

        for b in boards {
            assert!(
                b.is_solved(),
                "{:?} is solved, but `is_solved()` returned `false`",
                b
            );
        }
    }

    #[test]
    fn test_board_is_solved_false() {
        let boards = [
            Board::from((1, HashMap::from([(Loc::new(1, 1), None)]))),
            Board::from((
                2,
                HashMap::from([
                    (Loc::new(1, 1), Some(1)),
                    (Loc::new(1, 2), Some(2)),
                    (Loc::new(1, 3), Some(3)),
                    (Loc::new(1, 4), Some(4)),
                    (Loc::new(2, 1), Some(3)),
                    (Loc::new(2, 2), Some(4)),
                    (Loc::new(2, 3), Some(1)),
                    (Loc::new(2, 4), Some(2)),
                    (Loc::new(3, 1), Some(2)),
                    (Loc::new(3, 2), Some(1)),
                    (Loc::new(3, 3), Some(4)),
                    (Loc::new(3, 4), Some(3)),
                    (Loc::new(4, 1), Some(4)),
                    (Loc::new(4, 2), Some(3)),
                    (Loc::new(4, 3), Some(1)),
                    (Loc::new(4, 4), Some(2)),
                ]),
            )),
        ];

        for b in boards {
            assert!(
                !b.is_solved(),
                "{:?} is not solved, but `is_solved()` returned `true`",
                b
            );
        }
    }

    #[test]
    fn test_board_from_json() {
        assert_eq!(
            Board::from((
                2,
                HashMap::from([
                    (Loc::new(1, 1), Some(1)),
                    (Loc::new(1, 2), Some(2)),
                    (Loc::new(1, 3), Some(3)),
                    (Loc::new(1, 4), Some(4)),
                    (Loc::new(2, 1), Some(3)),
                    (Loc::new(2, 2), Some(4)),
                    (Loc::new(2, 3), Some(1)),
                    (Loc::new(2, 4), Some(2)),
                    (Loc::new(3, 1), Some(2)),
                    (Loc::new(3, 2), Some(1)),
                    (Loc::new(3, 3), Some(4)),
                    (Loc::new(3, 4), Some(3)),
                    (Loc::new(4, 1), Some(4)),
                    (Loc::new(4, 2), Some(3)),
                    (Loc::new(4, 3), Some(2)),
                    (Loc::new(4, 4), Some(1)),
                ]),
            )),
            Board::from("[[1,2,3,4],[3,4,1,2],[2,1,4,3],[4,3,2,1]]")
        )
    }

    #[test]
    fn test_board_available_values() {
        let tests = [
            (Board::from("[[null]]"), Loc::new(1, 1), vec![1]),
            (Board::from("[[1]]"), Loc::new(1, 1), vec![]),
            (
                Board::from("[[1,2,3,4],[3,4,1,2],[2,1,4,3],[4,3,1,2]]"),
                Loc::new(1, 1),
                vec![],
            ),
            (
                Board::from("[[null,2,3,4],[3,4,1,2],[2,1,4,3],[4,3,1,2]]"),
                Loc::new(1, 1),
                vec![1],
            ),
            (
                Board::from("[[null,null,3,4],[3,4,1,null],[null,1,4,3],[4,3,1,null]]"),
                Loc::new(1, 1),
                vec![1, 2],
            ),
            (
                Board::from("[[null,2,3,4],[3,4,1,null],[null,1,4,3],[4,3,1,null]]"),
                Loc::new(1, 1),
                vec![1],
            ),
            (
                Board::from("[[null,2,3,4],[3,null,1,2],[2,1,null,3],[4,3,2,null]]"),
                Loc::new(4, 4),
                vec![1],
            ),
            (
                Board::from("[[null,null,null,2,6,null,7,null,1], [6,8,null,null,7,null,null,9,null], [1,9,null,null,null,4,5,null,null], [8,2,null,1,null,null,null,4,null], [null,null,4,6,null,2,9,null,null], [null,5,null,null,null,3,null,2,8], [null,null,9,3,null,null,null,7,4], [null,4,null,null,5,null,null,3,6], [7,null,3,null,1,8,null,null,null]]"),
                Loc::new(6,1),
                vec![9]
            )
        ];

        for (board, loc, expected) in tests {
            assert_eq!(board.available_values(&loc), expected)
        }
    }

    #[test]
    fn test_board_set_none() {
        let tests = [
            (
                Board::from("[[null]]"),
                Loc::new(1, 1),
                Some(1),
                Board::from("[[1]]"),
            ),
            (
                Board::from("[[null]]"),
                Loc::new(1, 1),
                None,
                Board::from("[[null]]"),
            ),
            (
                Board::from("[[1]]"),
                Loc::new(1, 1),
                Some(1),
                Board::from("[[1]]"),
            ),
            (
                Board::from("[[1]]"),
                Loc::new(1, 1),
                None,
                Board::from("[[null]]"),
            ),
        ];

        for (mut initial, loc, value, expected) in tests {
            initial.set(&loc, value);
            assert_eq!(initial, expected)
        }
    }

    #[test]
    fn test_board_clone() {
        let initial = Board::from("[[1]]");
        let mut cloned = initial.clone();

        assert_eq!(initial, cloned);

        cloned.set(&Loc::new(1, 1), None);
        assert_ne!(initial, cloned);
    }

    #[test]
    fn test_board_solve() {
        let tests = [
            (Board::from("[[null]]"), Board::from("[[1]]")),
            (
                Board::from("[[null,2,3,4],[3,null,1,2],[2,1,null,3],[4,3,2,null]]"),
                Board::from("[[1,2,3,4],[3,4,1,2],[2,1,4,3],[4,3,2,1]]"),
            ),
            (
                Board::from("[[null,null,null,2,6,null,7,null,1], [6,8,null,null,7,null,null,9,null], [1,9,null,null,null,4,5,null,null], [8,2,null,1,null,null,null,4,null], [null,null,4,6,null,2,9,null,null], [null,5,null,null,null,3,null,2,8], [null,null,9,3,null,null,null,7,4], [null,4,null,null,5,null,null,3,6], [7,null,3,null,1,8,null,null,null]]"),
                Board::from("[[4,3,5,2,6,9,7,8,1], [6,8,2,5,7,1,4,9,3], [1,9,7,8,3,4,5,6,2], [8,2,6,1,9,5,3,4,7], [3,7,4,6,8,2,9,1,5], [9,5,1,7,4,3,6,2,8], [5,1,9,3,2,6,8,7,4], [2,4,8,9,5,7,1,3,6], [7,6,3,4,1,8,2,5,9]]"),
            )
        ];
        for (initial, expected) in tests {
            if let Some(solved) = initial.solve() {
                assert_eq!(
                    solved,
                    expected,
                    "Board was not solved properly:\n{}\n{}",
                    initial.draw(),
                    expected.draw()
                );
            } else {
                panic!("Board seems unsolvable:\n{}", initial.draw());
            }
        }
    }
}
