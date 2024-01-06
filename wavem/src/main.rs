use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use symphonia::core::audio::{AudioBufferRef, AudioBuffer, Signal};
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;


fn process_s16(buf: &AudioBuffer<i16>) -> Vec<i16> {
    let mut samples = Vec::new();
    for &sample in buf.chan(0) {
        // Do something with `sample`.
        samples.push(sample);
    }
    samples
}

// fn ignore_end_of_stream_error(result: Result) -> Result {
//     match result {
//         Err(Error::IoError(err))
//             if err.kind() == std::io::ErrorKind::UnexpectedEof
//                 && err.to_string() == "end of stream" =>
//         {
//             // Do not treat "end of stream" as a fatal error. It's the currently only way a
//             // format reader can indicate the media is complete.
//             Ok(())
//         }
//         _ => result
//     }
// }

fn main() {
    // The path to the audio file.
    let path = Path::new("/Users/patrickvonplaten/audios/sample.wav");

    // Open the media source.
    let file = std::fs::File::open(&path).expect("failed to open media");

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
    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts)
                                                    .expect("unsupported codec");

    // The decode loop.
    let result: Result<T, symphonia::symphonia_core::errors::Error> = loop {
        // Get the next packet from the media format.
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(err) => break Err(err),
        };

        match decoder.decode(&packet) {
            Ok(_decoded) => continue,
            Err(Error::DecodeError(err)) => panic!("{:?}", err),
            Err(err) => break Err(err),
        }
    };

    // ignore_end_of_stream_error(result)?;
}
