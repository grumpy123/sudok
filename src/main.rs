use std::collections::HashSet;

use format;

#[derive(PartialEq)]
#[derive(Debug)]
struct Field {
    // Set when the value is knows
    value: Option<i8>,
    // Starts as 1..9 and is whittled down over time
    options: HashSet<i8>,
}

impl Field {
    fn new() -> Field {
        let mut opts = HashSet::new();
        opts.extend(1..10);
        Field {
            value: None,
            options: opts,
        }
    }

    fn new9() -> [Field; 9] {
        [Field::new(), Field::new(), Field::new(), Field::new(), Field::new(), Field::new(), Field::new(), Field::new(), Field::new()]
    }

    fn is_solved(&self) -> bool {
        match self.value {
            Some(_) => true,
            None => false,
        }
    }

    fn set(&mut self, v: i8) -> Result<(), String> {
        match self.value {
            Some(x) if x == v => Ok(()),
            Some(x) => Err(format!("{v} can't be the solution for this field, it already has a solution {x}.")),
            None => {
                if !self.options.contains(&v) {
                    return Err(format!("{v} can't be the solution for this field, it's not a valid option."));
                }
                self.value = Some(v);
                self.options = HashSet::new();
                self.options.insert(v);
                return Ok(());
            }
        }
    }

    fn eliminate(&mut self, v: i8) -> Result<bool, String> {
        let res = self.options.remove(&v);
        if !res {
            return Ok(false);
        }

        match self.options.len() {
            0 => Err("The field is left with no possible value".to_string()),
            1 => {
                let &x = self.options.iter().next().unwrap();
                self.value = Some(x);
                Ok(true)
            }
            _ => Ok(false)
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Board {
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

    fn parse(printout :&str) -> Result<Board, String> {
        let parts: Vec<&str> = printout.split(char::is_whitespace).filter(|c| !c.is_empty()).collect();
        if parts.len() != 9*9 {
            return Err(format!("Wrong number of fields in input, expected 81 got {len}.", len=parts.len()))
        }
        let _vals: Vec<Option<i8>> = parts.iter().map(|x| x.parse().ok()).collect();

        let mut b = Board::new();
        for (i, val) in _vals.iter().enumerate() {
            if val.is_some() {
                let x = i % 9;
                let y = i / 9;
                b.field(x, y).set(val.unwrap())?;
            }
        }

        return Ok(b);
    }

    fn field(&mut self, x: usize, y: usize) -> &mut Field {
        &mut self.fields[x][y]
    }
}

fn main() {
    // let mut b = Board::new();
}

#[test]
fn test_is_solved() {
    let f = Field::new();
    assert_eq!(f.value, None);
    assert_eq!(f.options.len(), 9);
    assert!(f.options.contains(&1));
    assert!(f.options.contains(&9));
}

#[test]
fn test_eliminate() {
    let mut f = Field::new();
    for x in 1..4 {
        f.eliminate(x).expect("no error expected");
    }
    assert_eq!(f.options.len(), 6);
    assert!(!f.options.contains(&3));
    assert!(!f.is_solved());
    for x in 5..10 {
        f.eliminate(x).expect("no error expected");
    }
    assert_eq!(f.options.len(), 1);
    assert!(!f.options.contains(&9));
    assert!(f.options.contains(&4));
    assert!(f.is_solved());
}

#[test]
fn test_set_field() {
    let mut f = Field::new();
    assert_eq!(Ok(()), f.set(8));
    assert!(f.is_solved());
    assert_eq!(Some(8), f.value);
    assert_eq!(1, f.options.len());

    assert_eq!(
        Err("1 can't be the solution for this field, it already has a solution 8.".to_string()),
        f.set(1)
    );

    f = Field::new();
    f.eliminate(3).unwrap();
    assert_eq!(
        Err("3 can't be the solution for this field, it's not a valid option.".to_string()),
        f.set(3)
    );
}

#[test]
fn test_board_basics() {
    let mut b = Board::new();
    for x in 1..8 {
        assert!(!b.field(0, 0).eliminate(x).unwrap());
        assert!(!b.field(0, 0).is_solved());
    }
    assert!(b.field(0, 0).eliminate(9).unwrap());
    assert!(b.field(0, 0).is_solved());
}

#[test]
fn test_parse_board() {
    assert_eq!(Err("Wrong number of fields in input, expected 81 got 3.".to_string()), Board::parse("ala ma kota"));
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
    let mut b = b_res.unwrap();

    assert_eq!(Some(3), b.field(0, 0).value);
    assert_eq!(None, b.field(2, 0).value);
    assert_eq!(Some(3), b.field(0, 0).value);
    assert_eq!(Some(3), b.field(0, 0).value);
}