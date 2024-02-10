use std::env;
use std::fs;
use std::path::Path;
use std::process;

use num_traits::ToPrimitive;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;


trait FromSample: Sized {
    fn from_sample<T: ToPrimitive>(sample: T) -> Self;
}


impl FromSample for i16 {
    fn from_sample<T: ToPrimitive>(sample: T) -> Self {
        sample.to_i16().unwrap()
    }
}

impl FromSample for i32 {
    fn from_sample<T: ToPrimitive>(sample: T) -> Self {
        sample.to_i32().unwrap()
    }
}

impl FromSample for f32 {
    fn from_sample<T: ToPrimitive>(sample: T) -> Self {
        sample.to_f32().unwrap()
    }
}

fn convert<T, S>(buf: std::borrow::Cow<symphonia::core::audio::AudioBuffer<T>>, samples_buffer: &mut Vec<S>)
where
    T: symphonia::core::sample::Sample + ToPrimitive,
    S: Clone + FromSample,
{
    samples_buffer.extend(buf.chan(0).iter().map(|&x| S::from_sample(x)));
}


fn read_samples<T>(path: &Path) -> Result<Vec<T>, Error>
where
    T: Clone + FromSample,
{
    let file = std::fs::File::open(path).expect("failed to open media");

    // Create the media source stream.
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // Create a probe hint using the file's extension. [Optional]
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source.
    let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;

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
                    AudioBufferRef::U8(buf) =>  convert(buf, &mut samples_buffer), 
                    AudioBufferRef::U16(buf) => convert(buf, &mut samples_buffer),
                    // AudioBufferRef::U24(buf) => convert(buf, &mut samples_buffer),
                    AudioBufferRef::U32(buf) => convert(buf, &mut samples_buffer),
                    AudioBufferRef::S8(buf) =>  convert(buf, &mut samples_buffer),
                    AudioBufferRef::S16(buf) => convert(buf, &mut samples_buffer),
                    // AudioBufferRef::S24(buf) => convert(buf, &mut samples_buffer),
                    AudioBufferRef::S32(buf) => convert(buf, &mut samples_buffer),
                    AudioBufferRef::F32(buf) => convert(buf, &mut samples_buffer),
                    AudioBufferRef::F64(buf) => convert(buf, &mut samples_buffer),
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

        let vec = read_samples::<i32>(&abs_file_path).unwrap();

        println!("Done Rust. Length {:?}", vec.len());
    }
}
