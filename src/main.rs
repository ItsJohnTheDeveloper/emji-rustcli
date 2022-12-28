use std::fs;
use clap::Parser;
use serde::Deserialize;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // the pattern to look for
    fruit: String,
    qty: i32,
}

#[derive(Deserialize, Debug)]
struct Emojis {
    emojis: Vec<Emoji>
}

#[derive(Deserialize, Debug)]
struct Emoji {
    name: String,
    emoji: String,
}

fn get_food_emoji(input: &str) -> Option<String> {
    let json_file = fs::read_to_string("src/data/foods.json").expect("Unable to read file");
    let foods: Emojis = serde_json::from_str(&json_file).expect("Unable to parse JSON");
    let fruit_emojis = &foods.emojis;

    // Return explicit fruit match from input
    for fruit in fruit_emojis {
        if fruit.name == input {
            return Some(fruit.emoji.to_string());
        }
    }
    // Return loose fruit match from input
    for fruit in fruit_emojis {
        if fruit.name.contains(input) {
            return Some(fruit.emoji.to_string());
        }
    }
    None
}

fn main() {
    let args = Cli::parse();

    let fruit_selected = &get_food_emoji(&args.fruit);
    let qty_selected = args.qty.to_string().parse::<i32>().unwrap();
    
    match fruit_selected {
        Some(val) => {
            for _ in 0..qty_selected {
                print!("{val}");
            }
            println!("");
        },
        None => println!("Unknown fruit :(")
    }
}
