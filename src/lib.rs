#[macro_use] extern crate logos;

pub mod token;
pub mod ast;
pub mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
