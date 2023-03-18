pub mod scanner;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn import_test() {
        let mut s = HashSet::new();
        s.insert("i32".to_string());
        s.insert("String".to_string());
        s.insert("Option<HashSet<f64>>".to_string());
        let imports = scanner::convert_to_imports(s);
        println!("{}", imports);
    }
}