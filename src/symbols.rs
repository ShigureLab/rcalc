use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable<T, U> {
    variables: HashMap<String, T>,
    functions: HashMap<String, U>,
}

#[derive(Debug, PartialEq)]
pub enum SymbolError {
    ReDefinition,
    UnDefinition,
}

impl<T, U> SymbolTable<T, U>
where
    T: Copy,
    U: Copy,
{
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn define_variable(&mut self, name: &String, value: T) -> Result<(), SymbolError> {
        if self.variables.contains_key(name) {
            return Err(SymbolError::ReDefinition);
        }
        self.variables.insert(name.clone(), value);
        Ok(())
    }

    pub fn get_variable(&mut self, name: &String) -> Result<T, SymbolError> {
        if !self.variables.contains_key(name) {
            return Err(SymbolError::UnDefinition);
        }
        Ok(*self.variables.get(name).unwrap())
    }

    pub fn define_function(&mut self, name: &String, value: U) -> Result<(), SymbolError> {
        if self.functions.contains_key(name) {
            return Err(SymbolError::ReDefinition);
        }
        self.functions.insert(name.clone(), value);
        Ok(())
    }

    pub fn get_function(&mut self, name: &String) -> Result<U, SymbolError> {
        if !self.functions.contains_key(name) {
            return Err(SymbolError::UnDefinition);
        }
        Ok(*self.functions.get(name).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::assert_close;
    pub type Func<T> = fn(Vec<T>) -> T;

    #[test]
    fn success() {
        let name = "a".into();
        let a = 1.0;
        let mut symbols = SymbolTable::<f64, Func<f64>>::new();
        assert_eq!(symbols.define_variable(&name, a), Ok(()));
        let a_result_from_symbols = symbols.get_variable(&name);
        assert!(a_result_from_symbols.is_ok());
        if let Ok(a_from_symbols) = a_result_from_symbols {
            assert_close(a_from_symbols, a);
        }
    }

    #[test]
    fn redefined() {
        let name = "a".into();
        let a = 1.0;
        let mut symbols = SymbolTable::<f64, Func<f64>>::new();
        assert_eq!(symbols.define_variable(&name, a), Ok(()));
        assert_eq!(
            symbols.define_variable(&name, a),
            Err(SymbolError::ReDefinition)
        );
    }

    #[test]
    fn undefined() {
        let name = "a".into();
        let mut symbols = SymbolTable::<f64, Func<f64>>::new();
        assert_eq!(symbols.get_variable(&name), Err(SymbolError::UnDefinition));
    }
}
