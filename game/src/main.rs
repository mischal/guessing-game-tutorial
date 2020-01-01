use std::io;
use rand::Rng;
use std::cmp::Ordering;
use ui_lib::ui;
use ui_lib::ui::InputOption;


fn main() {


    let mut guess_game = GuessGame::new_game(1, 20);
    guess_game.start();
}



// global const
const DEFAULT_MAX_VAL: u32 = 20;

enum UserInput {
    Quit,
    Help,
    Config,
    Guess(u32),
    Info
}

struct GuessGame {
    from: u32,
    to: u32,
    secret_number: u32,
    runs: u32,
    input_options: Vec<InputOption>
}

impl GuessGame {




    fn print_intro() {
        ui::out::title("Guess the number!");
    }

    fn print_help(&self) {
        ui::out::title("Guess the number! Help");
        ui::out::show_input_options(&self.input_options);
    }

    fn new_game(from: u32, to: u32) -> GuessGame {
        let mut guess_game = GuessGame {
            from,
            to,
            secret_number: 0,
            runs: 0,
            input_options: vec![
                ui::InputOption::new('h', String::from("print the help")),
                ui::InputOption::new('q', String::from("quit the game")),
                ui::InputOption::new('i', String::from("show current game info")),
                ui::InputOption::new('c', String::from("config the game"))
            ]
        };

        guess_game.init_game();
        return guess_game;
    }

    fn init_game(&mut self) {
        self.generate_secret_number();
        self.runs = 0;
        println!("starting new game...");
    }

    fn generate_secret_number(&mut self)  {
        println!("The new secret number is between 1 and {}", self.to);
        self.secret_number = rand::thread_rng().gen_range(1, self.to + 1);
    }

    fn ask_max_number(&self) -> u32 {
        let mut max_val = String::new();

        println!("max number to guess (higher = more difficult): ");
        io::stdin().read_line(&mut max_val)
            .expect("Failed to read line");

        match max_val.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a number. using default value {}", DEFAULT_MAX_VAL);
                DEFAULT_MAX_VAL
            }
        }
    }

    fn start(&mut self) {
        GuessGame::print_intro();
        self.print_help();
        println!("starting game. please guess between {} and {}", self.from, self.to);
        self.start_loop();
    }

    fn ask_for_input(&self)  -> UserInput {

        let guess = ui::input::read_line("Please input your guess!");

        if guess == "q" {
            UserInput::Quit
        } else if guess == "h" {
            UserInput::Help
        } else if guess == "c" {
            UserInput::Config
        } else if guess == "i" {
            UserInput::Info
        } else {
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("nr eingeben");
                    0
                }
            };
            UserInput::Guess(guess)
        }
    }

    fn check_guess(&mut self, guess: &u32) -> bool {

        let mut win = false;
        self.runs += 1;

        match guess.cmp( &self.secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { win = true; }
        };
        win
    }

    fn start_loop(&mut self) {

        loop {

            match self.ask_for_input() {
                UserInput::Quit => {
                    println!("quit...");
                    break;
                }

                UserInput::Config => {
                    self.to = self.ask_max_number();
                    self.init_game();
                    continue;
                }

                UserInput::Help => {
                    self.print_help();
                    continue;
                }

                UserInput::Info => {
                    println!("number is between {} and {}", self.from, self.to);
                    println!("current trials: {}", self.runs);
                    continue;
                }

                UserInput::Guess(guess) => {
                    println!("You guessed: {}", guess);
                    if self.check_guess(&guess) {
                        println!("You win! total runs: {}", self.runs);
                        break;
                    }
                }
            }
        }
    }

}





