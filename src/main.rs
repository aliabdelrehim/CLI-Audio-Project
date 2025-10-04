use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder};
use std::io;

/*main function returns a result type that is either Ok
or an error type std::io::Error
std::io: standard input/output library
*/
fn main() -> std::io::Result<()> {

    // Get an output stream handle to the default physical sound device.
    // Note that the playback stops when the stream_handle is dropped.//!
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
    let _sink = rodio::Sink::connect_new(&stream_handle.mixer());

    // array of playlist songs
    let files = [
        "censor-beep-10-seconds.mp3",
        "Blue_One_Love.mp3",
        "coldplay_a-sky-full-of-stars-coldplay.mp3", 
        "Show_Me_The_Meaning_Of_Being_Lonely.mp3"
    ];

    // Prompt the user to start playback
    if !prompt_user("Please enter 'start' to begin playback:") {
        return Ok(()); // Exit the program if the user doesn't enter "start"
    }

    let mut current_index = 0;

    while current_index < files.len(){
        
        let file_path = files[current_index];
        
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let decoded_file = Decoder::new(reader);

        if let Ok(decoded_file) = decoded_file {
            // Sink handles the audio playback instead of stream_handle.play
            _sink.append(decoded_file);
            println!("Now playing song {}: {}", current_index + 1, file_path);
        }

        // loop over playback songs with play/pause for x and y seconds
        'song_loop: while !_sink.empty() {     

            let mut input_text = String::new();
            println!("press p to pause, r to resume, k/j to go to the next/previous song or 'quit' to exit");

            // access the keyboard input from the keyboard library
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            let button = input_text.trim();

            match button {
                "p" => {
                    _sink.pause();
                    println!("song paused");
                }

                "r" => {
                    _sink.play();
                    println!("song resumed");
                    if _sink.empty() {
                        println!("song finished playing");
                        current_index += 1;
                        break 'song_loop;
                    }
                }
                
                "k" => {
                    if current_index >= files.len() - 1 {
                        println!("Already at the last song");
                        continue;
                    }
                    println!("next song playing");
                    _sink.skip_one();
                    current_index += 1;
                    break 'song_loop;
                }
                        
                "j" => {
                    if current_index > 0 {
                         println!("previous song playing");
                        _sink.skip_one();
                        current_index -= 1;
                        break 'song_loop;
                    } else {
                        println!("Restarting the playlist");
                        _sink.skip_one();
                        break 'song_loop;
                    }
                }

                "quit" => {
                    println!("Exiting program.");
                    return Ok(());
                }

                // Add this wildcard pattern to handle all other inputs
                _ => {
                    println!("Invalid command");
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
            
        }
        
    }

    //code exectution is successful if it reaches ok function (success exit)
    Ok(()) 
}

// Function to prompt the user for input and check if they entered "start".
fn prompt_user(message: &str) -> bool {
    let mut input_text = String::new();
    println!("{}", message);

    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read from stdin");

    let button = input_text.trim();
    if button == "start" {
        true
    } else if button == "quit" {
        println!("Exiting program.");
        false
    } else {
        println!("Invalid input. Exiting program.");
        false
    }
}