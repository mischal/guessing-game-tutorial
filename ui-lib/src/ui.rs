#[derive(Debug)]
pub struct InputOption {
    key: char,
    description: String
}

impl InputOption {
    pub fn new(key: char, description: String) -> InputOption {
        InputOption {
            key,
            description
        }
    }
}

pub mod out {
    use crate::ui::InputOption;

    pub fn line(str: &str) {
        println!("{}", str);
    }

    pub fn title(str: &str) {
        repeat('=', str.len() + 4);
        println!("= {} =", str);
        repeat('=',str.len() + 4);
    }

    pub fn show_input_options(input_options: &Vec<InputOption>) {

        for i in input_options {
            println!("{} - {}", i.key, i.description);
        }

    }

    fn repeat(char: char, len: usize) {
        println!("{}", std::iter::repeat(char).take(len).collect::<String>());
    }

}

pub mod input {
    use std::io;

    pub fn read_line(str: &str) -> String {

        println!("{}", str);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("err");
        String::from( input.trim())


    }





}