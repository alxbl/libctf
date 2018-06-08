mod model;

fn open() {
    let u = model::User { id: 42, handle: String::from("alxbl"), access: 999, assigned: None};
    println!("hello world.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        open();
        assert_eq!(2 + 2, 4);
    }
}
