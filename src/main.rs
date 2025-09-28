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

    let files = [
        "Blue_One_Love.mp3",
        "coldplay_a-sky-full-of-stars-coldplay.mp3", 
        "Show_Me_The_Meaning_Of_Being_Lonely.mp3"
    ];

    let mut input_text = String::new();
    println!("Please enter 'start' to play the playlist:");

    // access the keyboard input from the library
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let button = input_text.trim();


    for file_path in files {
        let file = File::open(file_path);
        if let Ok(file) = file {
            let reader = BufReader::new(file);
            let decoded_file = Decoder::try_from(reader);
            if let Ok(decoded_file) = decoded_file {

                // Sink handles the audio playback instead of stream_handle.play
                _sink.append(decoded_file);
                
                if button == "start" {
                    // loop over playback songs with play/pause for x and y seconds
                    while !_sink.empty() {

                        let mut input_text = String::new();
                        println!("press p to pause, r to resume, k to go to the next song");

                        // access the keyboard input from the library
                        io::stdin()
                            .read_line(&mut input_text)
                            .expect("failed to read from stdin");

                        let button = input_text.trim();

                        if button == "p" {
                            _sink.pause();
                            println!("song paused");
                            std::thread::sleep(std::time::Duration::from_secs(1));
                            
                        }

                        if button == "r" {
                            _sink.play();
                            println!("song resumed");
                            std::thread::sleep(std::time::Duration::from_secs(1));
                        }

                        if button == "k" {
                            _sink.skip_one();
                            println!("next song playing");
                            std::thread::sleep(std::time::Duration::from_secs(1));
                        }
                        
                    }

                    }
                
            }
        }
    }

    

    //code exectution is successful if it reaches ok function (success exit)
    Ok(()) 
}

