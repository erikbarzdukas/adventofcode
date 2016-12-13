/**
 * Aw yea, advent of code
 */
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;
#[derive(Hash, Eq, PartialEq, Debug)]

//Current position on grid
struct Position {
    x: i32,
    y: i32
}

/**
 * Ingest directions from the input text file.
 */
fn ingest_directions() -> Vec<(char, i32)> {

    let mut directions: Vec<(char, i32)> = Vec::new();
    
    //Ingest directions
    let path = Path::new("/root/adventofcode/src/directions_file.txt");

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    let mut directions_string = String::new();
    match file.read_to_string(&mut directions_string) {
        Err(why) => panic!("Couldn't read {}: {}", path.display(), why.description()),
        Ok(_) => print!("Read directions from {}", path.display()),
    };

    directions_string = directions_string.replace(" ","");
    let mut rotation: char = 'L';
    let mut stepcount: String = String::new();
    
    for character in directions_string.chars() {
        match character {
            'L' => rotation = 'L',
            'R' => rotation = 'R',
            ',' | '\n' => {
                match stepcount.parse::<i32>() {
                    Err(why) => println!("Error: {}", why.description()),
                    Ok(i)    => {
                        directions.push((rotation, i));
                        stepcount = String::new();
                    }
                }
            }
            '0' ... '9' => {
                stepcount.push(character);
            }
            _ => println!("Error parsing commands"),
        }
    }

    return directions;

}

fn main() {

    let mut position = Position {x: 0, y: 0};
    let mut orientation = 0;
    let mut position_history = HashSet::new();

    println!("Beginning Easter Bunny HQ location");
    println!("Reading directions....");
    let directions = ingest_directions();
    println!("Parsed directions:");

    //For each direction
    //A series of tuples (L, 4)
    for value in directions.iter() {
      
        //Orient
        match value.0 {
            'L' => { 
                /*Decrement orientation*/
                if orientation == 0 {
                    orientation = 3;
                } else {
                    orientation -= 1;
                }
            },
            'R' => { 
                /*Increment orientation*/
                if orientation == 3 {
                    orientation = 0;
                } else {
                    orientation += 1;
                }
            },
             _  => println!("Incorrect orientation direction"),
        }
        //Inc/Dev Position
        match orientation {
            0 => { position.y += value.1},
            1 => { position.x += value.1},
            2 => { position.y -= value.1},
            3 => { position.x -= value.1},
            _ => println!("Couldn't update the position"),
        }

        if position_history.contains(Position {x: position.x, y: position.y}) {
            println!("The first duplicate position is: {},{}", position.x, position.y);
        } else {
            position_history.insert(position);
        }
    }

    //Take the sum of the absolute values of the position
    println!("The distance to the easter bunny hq is: {}", position.x.abs() + position.y.abs());
}
