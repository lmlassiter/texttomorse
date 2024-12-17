use rodio::{OutputStream, Sink, Source};
use std::{collections::HashMap, thread, time::Duration, io};


// Morse code mapping
fn get_morse_map() -> HashMap<char, &'static str> {
    let mut morse_map = HashMap::new();
    morse_map.insert('A', ".-");    morse_map.insert('B', "-...");
    morse_map.insert('C', "-.-.");  morse_map.insert('D', "-..");
    morse_map.insert('E', ".");     morse_map.insert('F', "..-.");
    morse_map.insert('G', "--.");   morse_map.insert('H', "....");
    morse_map.insert('I', "..");    morse_map.insert('J', ".---");
    morse_map.insert('K', "-.-");   morse_map.insert('L', ".-..");
    morse_map.insert('M', "--");    morse_map.insert('N', "-.");
    morse_map.insert('O', "---");   morse_map.insert('P', ".--.");
    morse_map.insert('Q', "--.-");  morse_map.insert('R', ".-.");
    morse_map.insert('S', "...");   morse_map.insert('T', "-");
    morse_map.insert('U', "..-");   morse_map.insert('V', "...-");
    morse_map.insert('W', ".--");   morse_map.insert('X', "-..-");
    morse_map.insert('Y', "-.--");  morse_map.insert('Z', "--..");
    morse_map.insert('1', ".----"); morse_map.insert('2', "..---");
    morse_map.insert('3', "...--"); morse_map.insert('4', "....-");
    morse_map.insert('5', "....."); morse_map.insert('6', "-....");
    morse_map.insert('7', "--..."); morse_map.insert('8', "---..");
    morse_map.insert('9', "----."); morse_map.insert('0', "-----");
    morse_map.insert(' ', " ");     // Space between words
    morse_map
}

// Translate text to Morse
fn text_to_morse(input: &str, morse_map: &HashMap<char, &str>) -> String {
    input
        .to_uppercase()
        .chars()
        .filter_map(|c| morse_map.get(&c))
        .map(|&code| code.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

// Play sound for Morse symbols
fn play_morse_sound(morse: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for symbol in morse.chars() {
        match symbol {
            '.' => {
                println!(".");
                sink.append(rodio::source::SineWave::new(800.0).take_duration(Duration::from_millis(200)));
            },
            '-' => {
                println!("-");
                sink.append(rodio::source::SineWave::new(800.0).take_duration(Duration::from_millis(600)));
            },
            ' ' => {
                println!(" ");
                thread::sleep(Duration::from_millis(700)); // Space between letters
            },
            _ => {},
        }
        sink.sleep_until_end();
    }
}


fn main() {
    let morse_map = get_morse_map();

    println!("Enter a message to convert to Morse Code:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let morse_code = text_to_morse(&input.trim(), &morse_map);
    println!("\nMorse Code:\n{}", morse_code);

    println!("\nPlaying Morse Code Sound...");
    play_morse_sound(&morse_code);
}
