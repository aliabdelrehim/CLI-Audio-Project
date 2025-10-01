use std::{fs::File};
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
        "Blue_One_Love.mp3",
        "coldplay_a-sky-full-of-stars-coldplay.mp3", 
        "Show_Me_The_Meaning_Of_Being_Lonely.mp3"
    ];

    // Prompt the user to start playback
    if !prompt_user("Please enter 'start' to begin playback:") {
        return Ok(()); // Exit the program if the user doesn't enter "start"
    }

    let button = "start"; //default start button

    // Iterate over each file in the playlist
    for (index, file_path) in files.iter().enumerate() {
        // Open the audio file
        let file = File::open(file_path);
        if let Ok(file) = file {
            //read and decode the audio file and store in decoded_file variable
            let reader = BufReader::new(file);
            let decoded_file = Decoder::try_from(reader);
            // get the index of the songs
            let song_index = index;
            if let Ok(decoded_file) = decoded_file {

                // Sink handles the audio playback instead of stream_handle.play
                _sink.append(decoded_file);
                
                if button == "start" {
                    // loop over playback songs with play/pause for x and y seconds
                    while !_sink.empty() {
                        println!("Now playing song at index {}: {}", song_index, file_path);

                        let mut input_text = String::new();
                        println!("press p to pause, r to resume, k to go to the next song or 'quit' to exit");

                        // access the keyboard input from the keyboard library
                        io::stdin()
                            .read_line(&mut input_text)
                            .expect("failed to read from stdin");

                        let button = input_text.trim();

                        // pause the song
                        if button == "p" {
                            _sink.pause();
                            println!("song paused");
                            std::thread::sleep(std::time::Duration::from_secs(1));
                            
                        }

                        // resume the song
                        if button == "r" {
                            _sink.play();
                            println!("song resumed");
                            std::thread::sleep(std::time::Duration::from_secs(1));
                        }

                        // play the next song
                        if button == "k" {
                            _sink.skip_one();
                            println!("next song playing");
                            std::thread::sleep(std::time::Duration::from_secs(1));
                        }

                        // quit the program
                        if button == "quit" {
                            println!("exiting program");
                            return Ok(());
                        }
                        
                    }

                    }
            }
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