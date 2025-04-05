use std::thread;
use std::time::Duration;
use std::io::{self, Write};

fn box_breathing() {
    clear_screen();
    print!("How many minutes will you be breathing?: ");
    let duration = loop {
        print!("How many minutes would you like to breath?: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .unwrap();
        match choice.trim().parse() {
            Ok(minutes @ 1..=60) => break minutes * 60, // @ binds the value to duration
            _ => println!("Invalid input, needs to be between 1 and 60."),
        }
    };

    countdown(3, "get ready to breath in...");

    let mut secs = duration;
    while secs > 0 {
        countdown(4, "Breath in...");
        secs -= 4;

        countdown(4, "Hold...");
        secs -= 4;

        countdown(4, "Breath out...");
        secs -= 4;

        // breaks so that you don't hold your breath in too long
        if secs == 0 { break; }

        countdown(4, "Hold...");
        secs -= 4;
    }

    println!("You finished your breathing exercise!");
    await_input("Press enter to continue...");
}

fn countdown(secs: u8, message: &str) {
    println!("{}", message);
    for i in (1..=secs).rev() {
        println!("{}", i);
        thread::sleep(Duration::from_secs(1));
    }
}

fn await_input(message: &str) {
    let mut input = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    loop {
        clear_screen();
        println!(
            "~~~Namaste, welcome to the CLI breathing tool~~~\n\
             1. Box breathing\n\
             2. Exit"
        );
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => box_breathing(),
            "2" => break,
            _ => {
                println!("Apologies, we don't offer that");
                await_input("Press enter to return to menu...");
            },
        }
    }

    println!("We hope to see you again");
}
