use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable<T> {
    map: HashMap<String, T>,
}

#[derive(Debug, PartialEq)]
pub enum SymbolError {
    ReDefinition,
    UnDefinition,
}

impl<T> SymbolTable<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        SymbolTable {
            map: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: T) -> Result<(), SymbolError> {
        if self.map.contains_key(name) {
            return Err(SymbolError::ReDefinition);
        }
        self.map.insert(name.into(), value);
        Ok(())
    }

    pub fn get(&mut self, name: &str) -> Result<T, SymbolError> {
        if !self.map.contains_key(name) {
            return Err(SymbolError::UnDefinition);
        }
        Ok(*self.map.get(name).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::assert_close;

    #[test]
    fn success() {
        let name = "a";
        let a = 1.0;
        let mut symbols = SymbolTable::<f64>::new();
        assert_eq!(symbols.define(name, a), Ok(()));
        let a_result_from_symbols = symbols.get(name);
        assert!(a_result_from_symbols.is_ok());
        if let Ok(a_from_symbols) = a_result_from_symbols {
            assert_close(a_from_symbols, a);
        }
    }

    #[test]
    fn redefined() {
        let name = "a";
        let a = 1.0;
        let mut symbols = SymbolTable::<f64>::new();
        assert_eq!(symbols.define(name, a), Ok(()));
        assert_eq!(symbols.define(name, a), Err(SymbolError::ReDefinition));
    }

    #[test]
    fn undefined() {
        let name = "a";
        let mut symbols = SymbolTable::<f64>::new();
        assert_eq!(symbols.get(name), Err(SymbolError::UnDefinition));
    }
}
