use std::{io::BufReader, thread, time::Duration};

fn main() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("assets/music.ogg").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    thread::sleep(Duration::from_secs(3));
    sink.set_seek(Duration::from_secs(50));
    sink.sleep_until_end();
}
