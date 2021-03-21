use byte_slice_cast::AsByteSlice;
use std::{fs::File, io::Write};

fn main() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    synth_sf2();
}

fn synth_sf2() {
    let mut pcm = File::create("Out.sf2.pcm").unwrap();

    let settings = oxisynth::SynthDescriptor::default();

    let mut synth = oxisynth::Synth::new(settings).unwrap();

    let mut file = std::fs::File::open("./testdata/sin.sf2").unwrap();
    let sin = synth.sfload(&mut file, true).unwrap();

    let mut file = std::fs::File::open("./testdata/Boomwhacker.sf2").unwrap();
    let boom = synth.sfload(&mut file, true).unwrap();

    synth.program_select(0, sin, 0, 0).unwrap();
    synth.program_select(1, boom, 0, 0).unwrap();

    let mut samples = [0f32; 44100 / 8];

    for _ in 0..5 {
        for n in 50..100 {
            synth.note_on(0, n, 127).unwrap();
            synth.note_on(1, n, 127).unwrap();

            synth.write(samples.as_mut());
            pcm.write(samples.as_byte_slice()).unwrap();

            synth.note_off(0, n);
            synth.note_off(1, n);
        }
        for n in 0..50 {
            synth.note_on(0, 100 - n, 127).unwrap();
            synth.note_on(1, 100 - n, 127).unwrap();

            synth.write(samples.as_mut());
            pcm.write(samples.as_byte_slice()).unwrap();

            synth.note_off(0, 100 - n);
            synth.note_off(1, 100 - n);
        }
    }

    drop(synth);
}