# CLI Audio Player

This is a simple command-line interface (CLI) audio player built in Rust. It allows users to play, pause, resume, skip, and quit playback of a predefined playlist of audio files.

## Features
- Play audio files from a predefined playlist.
- Pause and resume playback.
- Skip to the next song in the playlist.
- Quit the program at any time.

## Prerequisites
To run this project, you need the following:
- **Rust**: Ensure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **Audio Files**: The program uses `.mp3` files. Ensure the files in the playlist exist in the same directory as the executable or adjust the file paths in the code.
- **Dependency**: Add Rodio and Cpal to the cargo.toml file using the following command
  ```bash
  cargo add cpal rodio

## How to Run
1. Clone this repository to your local machine:
   ```bash
   git clone https://github.com/aliabdelrehim/CLIProject.git

2. Navigate to the project directory:
   ```bash
   cd C:\your-folder-path\CLIProject

3. Build the project:
   ```bash
   cargo build --release

4. Run the project
   ```bash
   cargo run

How to Use
1. When you run the program, it will prompt you to enter start to begin playback:

   ```bash
   Please enter 'start' to begin playback:

2. Type start and press Enter.

During playback, you can control the audio using the following commands:

p: Pause the current song.
r: Resume the paused song.
k: Skip to the next song in the playlist.
quit: Exit the program.

3. Example interaction:

   ```bash
   Please enter 'start' to begin playback:
   start
   press p to pause, r to resume, k to go to the next song or 'quit' to exit
   p
   song paused
   press p to pause, r to resume, k to go to the next song or 'quit' to exit
   r
   song resumed
   press p to pause, r to resume, k to go to the next song or 'quit' to exit
   k
   Next song playing
   press p to pause, r to resume, k to go to the next song or 'quit' to exit
   quit
   exiting program

Notes
. The program uses the rodio crate for audio playback. Ensure your system supports audio output.
. The playlist is hardcoded in the files array in main.rs. You can modify it to include your own audio files.

Dependencies
This project uses the following Rust crates:

- rodio: For audio playback.
- std: For standard input/output and file handling.

Author
Ali Abdelrahim - https://github.com/aliabdelrehim
