use std::collections::HashSet;

#[derive(PartialEq)]
#[derive(Debug)]
pub(crate) struct Field {
    // Set when the value is knows
    pub(crate) value: Option<i8>,
    // Starts as 1..9 and is whittled down over time
    pub(crate) options: HashSet<i8>,
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

    pub(crate) fn new9() -> [Field; 9] {
        [Field::new(), Field::new(), Field::new(), Field::new(), Field::new(), Field::new(), Field::new(), Field::new(), Field::new()]
    }

    pub(crate) fn is_solved(&self) -> bool {
        match self.value {
            Some(_) => true,
            None => false,
        }
    }

    pub(crate) fn set(&mut self, v: i8) -> Result<(), String> {
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

    pub(crate) fn eliminate(&mut self, v: i8) -> Result<bool, String> {
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

#[cfg(test)]
mod tests {
    use crate::field::Field;

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
}
