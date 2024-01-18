use std::path::Path;

use symphonia::core::audio::{AudioBuffer, Signal};
use symphonia::core::sample::Sample;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;


use std::fs::File;
use std::io::{self, Write};


fn write_int16_vector_to_file(vec: &Vec<i16>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    for &value in vec {
        writeln!(file, "{}", value)?;
    }

    Ok(())
}


fn read_samples(file_path: &str, samples_buffer: &mut Vec<i16>) {
    let path = Path::new(file_path);

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
    let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts).expect("unsupported format");

    // Get the instantiated format reader.
    let mut format = probed.format;

    // Find the first audio track with a known (decodeable) codec.
    let track = format.tracks()
                    .iter()
                    .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
                    .expect("no supported audio tracks");

    // Use the default options for the decoder.
    let dec_opts: DecoderOptions = Default::default();

    // Create a decoder for the track.
    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts).expect("unsupported codec");

    loop {
        // Get the next packet from the media format.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(_) => break,
        };

        match decoder.decode(&packet) {
            Ok(_decoded) => {
                let mut audio_buffer: AudioBuffer<i16> = _decoded.make_equivalent();
                _decoded.convert(&mut audio_buffer);

                samples_buffer.extend(audio_buffer.chan(0));
                continue
            },
            Err(Error::DecodeError(err)) => panic!("{:?}", err),
            Err(_) => break,
        };
    };
}


fn main() {
    let path: &str = "/Users/patrickvonplaten/audios/sample.wav";
    let mut vec: Vec<i16> = Vec::new();

    read_samples(path, &mut vec);

    println!("Done");
}
