use crate::error::ParseError;

use super::super::utils::Reader;
use riff::Chunk;
use std::io::{Read, Seek};

// TODO: ControllerPalette should contain an index. probably like so...
// enum ControllerPalette {
//      General(GeneralEnum),
//      Midi(u8)
// }
//
/// 8.2.1 Source Enumerator Controller Palettes
///
/// The SoundFont format supports two distinct controller palettes.
pub enum ControllerPalette {
    /// General Controller palette of controllers is selected.
    ///
    /// The `index` field value corresponds to one of the following controller sources.  
    /// - 0  No Controller
    /// - 2  Note-On Velocity
    /// - 3  Note-On Key Number
    /// - 10 Poly Pressure
    /// - 13 Channel Pressure
    /// - 14 Pitch Wheel
    /// - 16 Pitch Wheel Sensitivity
    /// - 127 Link
    General = 0,
    /// MIDI Controller Palette is selected. The `index` field value corresponds to one of the 128 MIDI Continuous Controller messages as defined in the MIDI specification.
    Midi = 1,
}

/// 8.2.2 Source Directions
pub enum SourceDirection {
    /// The direction of the controller should be from the minimum value to the maximum value. So, for example, if the controller source is Key Number, then Key Number value of 0 corresponds to the minimum possible controller output, and Key Number value of 127 corresponds to the maximum possible controller input.
    Positive = 0,
    /// The direction of the controller should be from the maximum value to the minimum value. So, for example, if the controller source is Key Number, then a Key Number value of 0 corresponds to the maximum possible controller output, and the Key Number value of 127 corresponds to the minimum possible controller input.
    Negative = 1,
}

// 8.2.3 Source Polarities
//
/// The SoundFont 2.01 format supports two polarities for any controller. The polarity if specified by bit 9 of the source enumeration field.
pub enum SourcePolarity {
    /// The controller should be mapped with a minimum value of 0 and a maximum value of 1. This is also called Unipolar. Thus it behaves similar to the Modulation Wheel controller of the MIDI specification.
    Unipolar = 0,
    /// The controller sound be mapped with a minimum value of -1 and a maximum value of 1. This is also called Bipolar. Thus it behaves similar to the Pitch Wheel controller of the MIDI specification.
    Bipolar = 1,
}

/// 8.2.4 Source Types
/// Specifies Continuity of the controller
///
/// The SoundFont 2.01 format may be used to support various types of controllers. This field completes the definition of the controller. A controller type specifies how the minimum value approaches the maximum value.
pub enum SourceTypes {
    /// The SoundFont modulator controller moves linearly from the minimum to the maximum value in the direction and with the polarity specified by the ‘D’ and ‘P’ bits.
    Linear = 0,
    /// The SoundFont modulator controller moves in a concave fashion from the minimum to the maximum value in the direction and with the polarity specified by the ‘D’ and ‘P’ bits. The concave characteristic follows variations of the mathematical equation:
    ///
    /// `output = log(sqrt(value^2)/(max value)^2)`
    Concave = 1,
    /// The SoundFont modulator controller moves in a convex fashion from the minimum to the maximum value in the direction and with the polarity specified by the ‘D’ and ‘P’ bits. The convex curve is the same curve as the concave curve, except the start and end points are reversed.
    Convex = 2,
    /// The SoundFont modulator controller output is at a minimum value while the controller input moves from the minimum to half of the maximum, after which the controller output is at a maximum. This occurs in the direction and with the polarity specified by the ‘D’ and ‘P’ bits.
    Switch = 3,
}

#[allow(dead_code)]
/// 8.2  Modulator Source Enumerators  
/// Flags telling the polarity of a modulator.
pub struct ModulatorSource {
    index: u8,
    controller_palette: ControllerPalette,
    direction: SourceDirection,
    polarity: SourcePolarity,
    /// Specifies Continuity of the controller
    src_type: SourceTypes,
}

#[allow(dead_code)]
/// 8.3  Modulator Transform Enumerators
pub enum ModulatorTransform {
    /// The output value of the multiplier is to be fed directly to the summing node of the given destination.
    Linear = 0,
    /// The output value of the multiplier is to be the absolute value of the input value, as defined by the relationship:
    ///
    /// `output = square root ((input value)^2)`
    ///
    /// or alternatively:
    ///
    /// `output = output * sgn(output)`
    Absolute = 2,
}

#[derive(Debug, Clone)]
pub struct Modulator {
    pub src: u16,  // TODO: ModulatorSource
    pub dest: u16, // TODO: GeneratorType
    pub amount: i16,
    pub amt_src: u16,   // TODO: ModulatorSource
    pub transform: u16, // TODO: ModulatorTransform
}

impl Modulator {
    pub fn read(reader: &mut Reader) -> Result<Self, ParseError> {
        let src: u16 = reader.read_u16()?;
        let dest: u16 = reader.read_u16()?;
        let amount: i16 = reader.read_i16()?;
        let amt_src: u16 = reader.read_u16()?;
        let transform: u16 = reader.read_u16()?;

        Ok(Self {
            src,
            dest,
            amount,
            amt_src,
            transform,
        })
    }

    pub fn read_all<F: Read + Seek>(pmod: &Chunk, file: &mut F) -> Result<Vec<Self>, ParseError> {
        assert!(pmod.id().as_str() == "pmod" || pmod.id().as_str() == "imod");

        let size = pmod.len();
        if size % 10 != 0 || size == 0 {
            Err(ParseError::InvalidModulatorChunkSize(size))
        } else {
            let amount = size / 10;

            let data = pmod.read_contents(file).unwrap();
            let mut reader = Reader::new(data);

            (0..amount).map(|_| Self::read(&mut reader)).collect()
        }
    }
}

/// 8.4  Default Modulators
mod default_modulators {
    // TODO: default_modulators

    // use super::*;
    // use crate::data::generator::GeneratorType;

    // /// 8.4.1  MIDI Note-On Velocity to Initial Attenuation
    // static DEFAULT_VEL2ATT_MOD: Modulator = Modulator {
    //     dest: GeneratorType::InitialAttenuation,
    //     amount: 960,

    //     src1: 2,
    //     flags1: MOD_GC | MOD_CONCAVE | MOD_UNIPOLAR | MOD_NEGATIVE,

    //     src2: 0,
    //     flags2: 0,
    // };

    // /// 8.4.2  MIDI Note-On Velocity to Filter Cutoff
    // static DEFAULT_VEL2FILTER_MOD: Modulator = Modulator {
    //     dest: GeneratorType::InitialFilterFc,
    //     amount: -2400,

    //     src1: 2,
    //     flags1: MOD_GC | MOD_LINEAR | MOD_UNIPOLAR | MOD_NEGATIVE,

    //     src2: 2,
    //     flags2: MOD_GC | MOD_SWITCH | MOD_UNIPOLAR | MOD_POSITIVE,
    // };

    // /// 8.4.3  MIDI Channel Pressure to Vibrato LFO Pitch Depth
    // static DEFAULT_AT2VIBLFO_MOD: Modulator = Modulator {
    //     dest: GeneratorType::VibLfoToPitch,
    //     amount: 50,

    //     src1: 13,
    //     flags1: MOD_GC | MOD_LINEAR | MOD_UNIPOLAR | MOD_POSITIVE,

    //     src2: 0,
    //     flags2: 0,
    // };

    // /// 8.4.4  MIDI Continuous Controller 1 to Vibrato LFO Pitch Depth
    // static DEFAULT_MOD2VIBLFO_MOD: Modulator = Modulator {
    //     dest: GeneratorType::VibLfoToPitch,
    //     amount: 50,

    //     src1: 1,
    //     flags1: MOD_CC | MOD_LINEAR | MOD_UNIPOLAR | MOD_POSITIVE,

    //     src2: 0,
    //     flags2: 0,
    // };

    // /// 8.4.5  MIDI Continuous Controller 7 to Initial Attenuation
    // static DEFAULT_ATT_MOD: Modulator = Modulator {
    //     dest: GeneratorType::InitialAttenuation,
    //     amount: 960,

    //     src1: 7,
    //     flags1: MOD_CC | MOD_CONCAVE | MOD_UNIPOLAR | MOD_NEGATIVE,

    //     src2: 0,
    //     flags2: 0,
    // };

    // /// 8.4.6  MIDI Continuous Controller 10 to Pan Position
    // static DEFAULT_PAN_MOD: Modulator = Modulator {
    //     amount: 500,
    //     dest: GeneratorType::Pan,

    //     src1: 10,
    //     flags1: MOD_CC | MOD_LINEAR | MOD_BIPOLAR | MOD_POSITIVE,

    //     src2: 0,
    //     flags2: 0,
    // };

    // /// 8.4.7  MIDI Continuous Controller 11 to Initial Attenuation
    // static DEFAULT_EXPR_MOD: Modulator = Modulator {
    //     amount: 960,
    //     dest: GeneratorType::InitialAttenuation,

    //     src1: 11,
    //     flags1: MOD_CC | MOD_CONCAVE | MOD_UNIPOLAR | MOD_NEGATIVE,

    //     src2: 0,
    //     flags2: 0,
    // };

    // /// 8.4.8  MIDI Continuous Controller 91 to Reverb Effects Send
    // static DEFAULT_REVERB_MOD: Modulator = Modulator {
    //     amount: 200,
    //     dest: GeneratorType::ReverbEffectsSend,

    //     src1: 91,
    //     flags1: MOD_CC | MOD_LINEAR | MOD_UNIPOLAR | MOD_POSITIVE,

    //     src2: 0,
    //     flags2: 0,
    // };

    // /// 8.4.9  MIDI Continuous Controller 93 to Chorus Effects Send
    // static DEFAULT_CHORUS_MOD: Modulator = Modulator {
    //     amount: 200,
    //     dest: GeneratorType::ChorusSend,

    //     src1: 93,
    //     flags1: MOD_CC | MOD_LINEAR | MOD_UNIPOLAR | MOD_POSITIVE,

    //     src2: 0,
    //     flags2: 0,
    // };

    // /// 8.4.10  MIDI Pitch Wheel to Initial Pitch Controlled by MIDI Pitch Wheel Sensitivity
    // static DEFAULT_PITCH_BEND_MOD: Modulator = Modulator {
    //     amount: 12700,
    //     dest: GeneratorType::Pitch,

    //     src1: 14, // PITCHWHEEL
    //     flags1: MOD_GC | MOD_LINEAR | MOD_BIPOLAR | MOD_POSITIVE,

    //     src2: 16, // PITCHWHEELSENS
    //     flags2: MOD_GC | MOD_LINEAR | MOD_UNIPOLAR | MOD_POSITIVE,
    // };
}
