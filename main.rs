/*
Author: Kelly Raines
Date: 4-29-2022
Description: Encodes/decodes an input file from the user
into Vigenère or to English depending
on the command used to then output the translation into
an output file called translation.txt.
Usage: cargo run <input file> <e/d> <key>
*/

// imports
use std::{env, process, char, fs, io::Write};
mod lib;
use lib::Config;
extern crate unidecode;
use unidecode::unidecode;
 
// main function
pub fn main() {

    // read inputs from the command line
    let inputs = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        // terminates the current process on error
        process::exit(1);
    });
    // clones the config so it can be used to get the direction and key from the inputs
    let input_clone = inputs.clone();
    // contents of the file is the message to translate
    let message = lib::run(inputs).unwrap_or_else(|err| {
        eprintln!("Error reading the file: {}", err);
        process::exit(1)
    });
    // create an output file called translation.txt
    let mut output_file = fs::File::create("translation.txt").expect("File failed to be created");
    
    // displays on the screen what was read from the file
    println!("Read: {}", message);

    let key = input_clone.key;
    // uses the key inputted to get the necessary size for encoding/decoding
    let extended_key = generate_key(key, &message);
 
    let translation;
    // checks whether the user decided to encode or decode their message and uses the respective function
    if input_clone.direction.eq("e") {
        
        translation = to_vigenere(&message, &extended_key).0;
        // stores the punctuation/ capitalization of a message
        let message_punctuation = to_vigenere(&message, &extended_key).1;
        output_file.write(translation.as_bytes()).expect("Failed to write to file");
        // separates the encryption from the information about the punctuation in the message
        output_file.write("\n--------------------\n".as_bytes()).expect("Failed to write to file");
        writeln!(output_file, "{:?}", message_punctuation).expect("Failed to write to file");
    }
    else {
        // decodes the message and writes it to the output file
        translation = to_english(&message, &extended_key);
        output_file.write(translation.as_bytes()).expect("Failed to write to file");
    }
}

// repeats the key until it reaches the size of what is being translated
fn generate_key(mut key: String, message: &String) -> String {
    let mut iterator = 0;
    // sets the length according to the number of letters in the message
    let message = remove_punctuation(&message);
 
    while key.len() != message.len() {
        // sets the iterator back to 0 when it is the length of the message
        if iterator == message.len() {
            iterator = 0;
        }
        key.push(key.chars().nth(iterator).unwrap());
        iterator += 1;
    }
    key
}
 
// removes punctuation and special characters
fn remove_punctuation(message: &String) -> String {
    // transliterates unicode strings to pure ASCII
    let valid_message = unidecode(message);
    let no_punc_message = valid_message.replace(&['(', ')', '[', ']', ',', '\"', '.', ';', ':', '\'', ' ', '\n', '\r'][..], "");
    no_punc_message
}
 
// converts letters a-z, A-Z to 0-25
fn alpha_to_int(letter: char) -> i32 {
    match letter {
        'a'|'A' => 0,
        'b'|'B' => 1,
        'c'|'C' => 2,
        'd'|'D' => 3,
        'e'|'E' => 4,
        'f'|'F' => 5,
        'g'|'G' => 6,
        'h'|'H' => 7,
        'i'|'I' => 8,
        'j'|'J' => 9,
        'k'|'K' => 10,
        'l'|'L' => 11,
        'm'|'M' => 12,
        'n'|'N' => 13,
        'o'|'O' => 14,
        'p'|'P' => 15,
        'q'|'Q' => 16,
        'r'|'R' => 17,
        's'|'S' => 18,
        't'|'T' => 19,
        'u'|'U' => 20,
        'v'|'V'=> 21,
        'w'|'W' => 22,
        'x'|'X' => 23,
        'y'|'Y' => 24,
        'z'|'Z' => 25,
        _ => -1, // handles any unrecognized characters
    }
}

// converts 0-25 to letters A-Z
fn int_to_alpha(num: i32) -> char {
    match num {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        _ =>  ' ' // handles any unrecognized numbers
    }
}

// encodes the punctuation within the message
fn encode_punctuation(punc: char) -> i32 {
    match punc {
        ' ' => 2,
        '!' => 3,
        '\"' => 4,
        '#' => 5,
        '$' => 6,
        '%' => 7,
        '&' => 8,
        '\'' => 9,
        '(' => 10,
        ')' => 11,
        '*' => 12,
        '+' => 13,
        ',' => 14,
        '-' => 15,
        '.' => 16,
        '/' => 17,
        ':' => 18,
        ';' => 19,
        '<' => 20,
        '=' => 21,
        '>' => 22,
        '?' => 23,
        '@' => 24,
        '[' => 25,
        '\\' => 26,
        ']' => 27,
        '^' => 28,
        '_' => 29,
        '`' => 30,
        '{' => 31,
        '|' => 32,
        '}' => 33,
        '~' => 34,
        '0' => 35,
        '1' => 36,
        '2' => 37,
        '3' => 38,
        '4' => 39,
        '5' => 40,
        '6' => 41,
        '7' => 42,
        '8' => 43,
        '9' => 44,
        '\n' => 45,
        '\r' => 46,
        _ => 47 // handles any unrecognized characters
    }
}

// decodes the punctuation within the message
fn decode_punctuation(num: i32) -> char {
    match num {
        2 => ' ',
        3 => '!',
        4 => '\"',
        5 => '#',
        6 => '$',
        7 => '%',
        8 => '&',
        9 => '\'',
        10 => '(',
        11 => ')',
        12 => '*',
        13 => '+',
        14 => ',',
        15 => '-',
        16 => '.',
        17 => '/',
        18 => ':',
        19 => ';',
        20 => '<',
        21 => '=',
        22 => '>',
        23 => '?',
        24 => '@',
        25 => '[',
        26 => '\\',
        27 => ']',
        28 => '^',
        29 => '_',
        30 => '`',
        31 => '{',
        32 => '|',
        33 => '}',
        34 => '~',
        35 => '0',
        36 => '1',
        37 => '2',
        38 => '3',
        39 => '4',
        40 => '5',
        41 => '6',
        42 => '7',
        43 => '8',
        44 => '9',
        45 => '\n',
        46 => '\r',
        47 => 'ᔕ',
        _ => 'ᔕ' // handles any unrecognized numbers
    }
}

// converts the string of punctuation from the file back into integers
fn parse_punc_symbols(symbols: &str) -> Result<Vec<i32>, bool> {
    let mut symbols_no_punc: String = symbols.split(",").collect();
    symbols_no_punc = symbols_no_punc.replace(&['[', ']', '\n', '\r'], "");
    // parses the symbols and converts is to an integer and collects it into a vector
    let symbols_as_numbers: Vec<i32> = symbols_no_punc.split_whitespace().map(|s| s.parse().expect("Error parsing punctuation")).collect();
    Ok(symbols_as_numbers)
}

// function to encode the message
fn to_vigenere(message: &String, key: &String) -> (String, Vec<i32>) {
    
    let mut translation = String::new();
    let mut translation_punc= vec![];
    let mut index_of_letters = 0;
    
    for i in unidecode(message).chars() {

        // if the letter is not recognized, then it is punctuation
        if alpha_to_int(i) == -1 {
            translation_punc.push(encode_punctuation(i));
        }
        else {
            // stores whether the letter was uppercase or lowercase
            if i.is_ascii_uppercase() {
                translation_punc.push(0);
            }
            else {
                translation_punc.push(1);
            }

            // gets the remainder of the added values of each character from the message and key together
            let translated_value = (alpha_to_int(i) + alpha_to_int(key.chars().nth(index_of_letters).unwrap())) % 26;
            // convert the value back as an encoded letter
            let translated_char = int_to_alpha(translated_value);
            translation.push(translated_char);
            // increments the index for the key only when it is a letter
            index_of_letters += 1;
        }
    }
    // return a tuple of the translated message and stores the encoded punctuation inside the message
    (translation, translation_punc)
}

// function to decode the message
fn to_english(message: &String, key: &String) -> String {

    let mut translation = String::new();
    let mut index = 0;
    let mut end_of_words = 0;

    // gets rid of any weird carriage return
    let valid_message = message.replace("\r", "");
    // splits the message in the file between the letters and the punctuation
    let message_as_vector: Vec<&str> = valid_message.split("\n--------------------\n").collect();
    let encoded_message = message_as_vector[0];
    let encoded_punc = parse_punc_symbols(message_as_vector[1]).unwrap();

    for (i,j) in encoded_message.chars().zip(key.chars()) {
        
        // gets the remainder of the subtracted values of each character from the message and key
        let translated_value = (alpha_to_int(i) - alpha_to_int(j) + 26) % 26;
        // converts the value back as a decoded letter
        let mut translated_char = int_to_alpha(translated_value);

        // inserts punctuation between letters until it reaches information about the capitalization of a certain letter
        while encoded_punc[index] != 1 && encoded_punc[index] != 0 {
            translation.push(decode_punctuation(encoded_punc[index]));
            index += 1;
        }

        // changes the letter to lowercase if the original message was a lowercase letter
        if encoded_punc[index] == 1 {
            translated_char = translated_char.to_ascii_lowercase();
        }
        index += 1;
        translation.push(translated_char);
        end_of_words += 1;

        // checks at the end of the letters in the message if there is extra punctuation afterwards
        if end_of_words == encoded_message.len() {
            // inserts punctuation until it reaches the end of the punctuation information stored
            'outer: while index < encoded_punc.len() {
                while encoded_punc[index] != 1 && encoded_punc[index] != 0 {
                    translation.push(decode_punctuation(encoded_punc[index]));
                    index += 1;
    
                    if index == encoded_punc.len() {
                        // breaks out of both loops if the end is reached
                        break 'outer;
                    }
                }
            }
        }
    }
    translation
}