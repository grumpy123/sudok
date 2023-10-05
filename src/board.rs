use crate::field::Field;

#[derive(PartialEq)]
#[derive(Debug)]
pub(crate) struct Board {
    fields: [[Field; 9]; 9],
}

impl Board {
    fn new() -> Board {
        Board {
            fields: [
                Field::new9(),
                Field::new9(),
                Field::new9(),
                Field::new9(),
                Field::new9(),
                Field::new9(),
                Field::new9(),
                Field::new9(),
                Field::new9(),
            ]
        }
    }

    pub(crate) fn parse(printout: &str) -> Result<Board, String> {
        let parts: Vec<&str> = printout.split(char::is_whitespace).filter(|c| !c.is_empty()).collect();
        if parts.len() != 9 * 9 {
            return Err(format!("Wrong number of fields in input, expected 81 got {len}.", len = parts.len()));
        }
        let _vals: Vec<Option<i8>> = parts.iter().map(|x| x.parse().ok()).collect();

        let mut b = Board::new();
        for (i, val) in _vals.iter().enumerate() {
            if val.is_some() {
                let x = i % 9;
                let y = i / 9;
                b.field(x, y).solve(val.unwrap())?;
            }
        }

        return Ok(b);
    }

    pub(crate) fn field(&mut self, x: usize, y: usize) -> &mut Field {
        &mut self.fields[x][y]
    }

    pub(crate) fn field_seq(&mut self, i: usize) -> &mut Field {
        &mut self.fields[i % 9][i / 9]
    }

    fn _all_fields(&self) -> impl Iterator<Item=(usize, usize, &Field)> {
        let mut res : Vec<(usize, usize, &Field)> = vec![];
        for (x, row) in self.fields.iter().enumerate() {
            for (y, f) in row.iter().enumerate() {
                res.push((x, y, f));
            }
        }
        return res.into_iter();
    }

    fn num_solved(&self) -> usize {
        self._all_fields().filter(|(_, _, x)| x.is_solved()).count()
    }

    pub(crate) fn is_solved(&self) -> bool {
        self.num_solved() == 81
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;

    #[test]
    fn test_board_basics() {
        let mut b = Board::new();
        for x in 1..8 {
            assert!(b.field(0, 0).eliminate(x).unwrap());
            assert!(!b.field(0, 0).is_solved());
        }
        assert!(b.field(0, 0).eliminate(9).unwrap());
        assert!(b.field(0, 0).is_solved());

        assert_eq!(1, b.num_solved());
        assert!(!b.is_solved());
    }

    fn test_board() -> Board {
        let b_res = Board::parse("\
        3 6 _ _ 1 7 _ _ 5
        _ 5 _ _ 6 _ 3 1 2
        1 8 2 _ 3 _ 7 9 6
        _ _ 6 _ _ _ _ 5 1
        5 _ _ 6 _ 9 _ _ 8
        9 2 _ _ _ _ 6 _ _
        2 9 4 _ 8 _ 5 6 7
        7 3 8 _ 4 _ _ 2 _
        6 _ _ 7 9 _ _ 8 3
    ");
        assert!(b_res.is_ok());
        return b_res.unwrap();
    }

    #[test]
    fn test_parse_board() {
        assert_eq!(Err("Wrong number of fields in input, expected 81 got 3.".to_string()), Board::parse("ala ma kota"));


        let mut b = test_board();

        assert_eq!(Some(3), b.field(0, 0).value);
        assert_eq!(None, b.field(2, 0).value);
        assert_eq!(Some(5), b.field(1, 1).value);
        assert_eq!(Some(3), b.field(8, 8).value);

        assert_eq!(44, b.num_solved());
    }
}