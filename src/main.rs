use std::fs;
use clap::Parser;
use serde::Deserialize;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // the pattern to look for
    food: String,
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
    let food_emojis = &foods.emojis;

    // Return explicit food match from input
    for food in food_emojis {
        if food.name == input {
            return Some(food.emoji.to_string());
        }
    }
    // Return loose food match from input
    for food in food_emojis {
        if food.name.contains(input) {
            return Some(food.emoji.to_string());
        }
    }
    None
}

fn main() {
    let args = Cli::parse();

    let food_selected = &get_food_emoji(&args.food);
    let qty_selected = args.qty.to_string().parse::<i32>().unwrap();
    
    match food_selected {
        Some(val) => {
            for _ in 0..qty_selected {
                print!("{val}");
            }
            println!("");
        },
        None => println!("Unknown food :(")
    }
}
