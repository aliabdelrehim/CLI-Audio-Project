use std::{fs::File, io::BufRead};
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
                stream_handle.mixer().add(decoded_file);
            }
        }
    }
    
    // The sound plays in a separate audio thread (test for 5 seconds),
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
    
    //code exectution is successful if it reaches ok function (success exit)
    Ok(()) 
}