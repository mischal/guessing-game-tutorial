#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod ui {

    pub mod out {

        pub fn line(str: &str) {
            println!("{}", str);
        }

        pub fn title(str: &str) {
            repeat('=', str.len() + 4);
            println!("= {} =", str);
            repeat('=',str.len() + 4);
        }

        fn repeat(char: char, len: usize) {
            println!("{}", std::iter::repeat(char).take(len).collect::<String>());
        }

    }


}

pub fn testlib()  {
    println!("testing lib call");
}