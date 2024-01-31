use std::any::type_name;
use std::env;
use std::fs;
use std::i16;
use std::path::Path;
use std::process;

use std::borrow::Cow;
use symphonia::core::audio::{AudioBuffer, AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::sample::Sample;

use std::fs::File;
use std::io::{self, Write};

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn write_int16_vector_to_file(vec: &Vec<i16>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for &value in vec {
        writeln!(file, "{}", value)?;
    }

    Ok(())
}

trait FromSample: Sized {
    fn from_i16(sample: i16) -> Self;
    fn from_i32(sample: i32) -> Self;
    fn from_f32(sample: f32) -> Self;
}


impl FromSample for i16 {
    fn from_i16(sample: i16) -> Self {
        sample
    }

    fn from_i32(sample: i32) -> Self {
        sample as i16
    }

    fn from_f32(sample: f32) -> Self {
        sample as i16
    }
}

impl FromSample for i32 {
    fn from_i16(sample: i16) -> Self {
        sample as i32
    }

    fn from_i32(sample: i32) -> Self {
        sample
    }

    fn from_f32(sample: f32) -> Self {
        sample as i32
    }
}

impl FromSample for f32 {
    fn from_i16(sample: i16) -> Self {
        sample as f32
    }

    fn from_i32(sample: i32) -> Self {
        sample as f32
    }

    fn from_f32(sample: f32) -> Self {
        sample
    }
}


fn read_samples<T: Clone + FromSample>(path: &Path) -> Result<Vec<T>, Error> {
    let file = std::fs::File::open(path).expect("failed to open media");

    // Create the media source stream.
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // Create a probe hint using the file's extension. [Optional]
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = FormatOptions {
        enable_gapless: true,
        ..Default::default()
    };

    // Probe the media source.
    let mut probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;

    // Get the instantiated format reader.
    let mut format = probed.format;

    // Find the first audio track with a known (decodeable) codec.
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks");

    let sample_len: usize = if let Some(n_frames) = track.codec_params.n_frames {
        n_frames as usize
    } else {
        0
    };

    let mut samples_buffer: Vec<T> = Vec::with_capacity(sample_len);

    // Use the default options for the decoder.
    let dec_opts: DecoderOptions = Default::default();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");

    loop {
        // Get the next packet from the media format.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(_) => break,
        };

        match decoder.decode(&packet) {
            Ok(decoded) => {
                match decoded {
                    AudioBufferRef::S16(buf) => {
                        // wave
                        let samples = buf.chan(0);
                        samples_buffer.extend(samples.iter().map(|&x| T::from_i16(x)));

                    }
                    AudioBufferRef::S32(buf) => {
                        // flac
                        let samples = buf.chan(0);
                        samples_buffer.extend(samples.iter().map(|&x| T::from_i32(x)));
                    }
                    AudioBufferRef::F32(buf) => {
                        // mp3
                        let samples = buf.chan(0);
                        samples_buffer.extend(samples.iter().map(|&x| T::from_f32(x)));
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            }
            Err(Error::DecodeError(err)) => panic!("{:?}", err),
            Err(_) => break,
        };
    };
    Ok(samples_buffer)
}

fn main() {
    // let path: &str = "/Users/patrickvonplaten/audios/sample.wav";
    let args: Vec<String> = env::args().collect();

    // Check if an argument is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    // The second argument is the file path
    let dir = Path::new(&args[1]);

    for entry in fs::read_dir(&dir).unwrap() {
        let filename = entry.unwrap().file_name();
        let filename = filename.to_str().unwrap(); // Get DirEntry from the iterator

        let file_path = Path::new(filename);
        let abs_file_path = dir.join(file_path);

        let vec: Vec<f32> = read_samples(&abs_file_path).unwrap();

        println!("Done Rust. Length {:?}", vec.len());
    }
}
