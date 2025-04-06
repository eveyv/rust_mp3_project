Can install and play music as is, though it is not even remotely ideal at the moment. I plan on building this out the way I would prefer a music player worked and I intend to integrate a compact AI
that will serve to find new music, radio stations, and upcoming concerts for music of interest, based on listening patterns, music composition and prompts.

To play:
git clone
go to the fn main in /src/main.rs
change the music directory to the one you want to use. note, right now, you need to have a directory of loose MP3 files, not organized by artist/album, etc. No logic to handles nested directories exists yet, but it will.
