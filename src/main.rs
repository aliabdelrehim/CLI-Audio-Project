use std::{fs::File};
use std::io::BufReader;
use rodio::{Decoder};


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

    for file_path in files {
        let file = File::open(file_path);
        if let Ok(file) = file {
            let reader = BufReader::new(file);
            let decoded_file = Decoder::try_from(reader);
            if let Ok(decoded_file) = decoded_file {

                // Sink handles the audio playback instead of stream_handle.play
                _sink.append(decoded_file);
            }
        }
    }

    let mut song_counter = 0;

    // loop over playback songs with play/pause for x and y seconds
    while !_sink.empty() {

        song_counter += 1; // Increment the counter
        println!("Playing song number: {}", song_counter);

        print!("Playing for 5 seconds...\n");
        _sink.play();
        std::thread::sleep(std::time::Duration::from_secs(5));

        print!("Paused for 5 second...\n");
        _sink.pause();
        std::thread::sleep(std::time::Duration::from_secs(5));

        print!("Resuming playback for 3 seconds...\n");
        _sink.play();
        std::thread::sleep(std::time::Duration::from_secs(3)); 

        _sink.skip_one();

        if _sink.len() == 1 {
            break;
        }      
        
    }

    //code exectution is successful if it reaches ok function (success exit)
    Ok(()) 
}