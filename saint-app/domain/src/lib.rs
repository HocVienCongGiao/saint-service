pub fn test_func() {
    println!("hello");
}

pub mod boundaries;
mod entity;
pub mod interactors;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
