use rand::Rng;
use text_io::read;

fn main() {
    // Setup Variables

    // Setup the game
    loop {
        game();

        // Quit the game or play another round
        println!("Play another game? Y/N");
        let input: String = read!();
        if input == "N" {
            break;
        }
    }
}

fn game() {
    // Init text
    println!("Guessing game, guess a number between 1 and 100:");

    // Setup local vars
    let mut rng = rand::thread_rng();
    let goal = rng.gen_range(0..100);
    let mut guesses: i32 = 0;

    // Read user input
    loop {
        let input: i32 = read!();
        guesses += 1;

        // Validate user input
        if input > goal {
            println!("Lower");
        } else if input < goal {
            println!("Higher");
        } else if input == goal {
            println!("You have won in {} guesses", guesses);
            break;
        }
    }
}
