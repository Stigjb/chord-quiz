use std::fmt;

use log::info;
use tonality;
use tonality::{Accidental, Alteration, Key, Step, Tpc};
use yew::Html;

use crate::score;
use crate::score::{triad_example, StaffPosition};

#[derive(Clone, Debug, PartialEq)]
pub struct Chord {
    key: tonality::Key,
    kind: Kind,
}

impl Chord {
    pub fn new(key: tonality::Key, kind: Kind) -> Self {
        Self { key, kind }
    }

    pub fn to_svg(&self) -> Html {
        let clef = Clef::G;
        score::Builder::new()
            .space(0.5)
            .clef(&clef)
            .space(6.)
            .accidentals(&self.accidentals())
            .space(1.5)
            .chord(&self.staff_positions(clef))
            .space(6.)
            .barline()
            .into_svg()
    }

    fn tpcs(&self) -> Vec<Tpc> {
        self.kind.with_key(self.key.clone())
    }

    pub fn staff_positions(&self, clef: Clef) -> Vec<StaffPosition> {
        self.tpcs()
            .into_iter()
            .map(|tpc| clef.position(tpc.step()))
            .collect()
    }

    fn accidentals(&self) -> Vec<(Accidental, StaffPosition)> {
        self.tpcs()
            .iter()
            .zip(self.staff_positions(Clef::G))
            .filter_map(|(t, p)| match t.altered_step(None) {
                (_, Some(acc)) => {
                    info!("Accidental for {:?}: {:?}", t, acc);
                    Some((acc, p))
                }
                (_, None) => {
                    info!("No accidental for {:?}", t);
                    None
                }
            })
            .collect()
    }
}

impl Default for Chord {
    fn default() -> Self {
        Self {
            key: Key::default(),
            kind: Kind::Triad(Triad::Maj),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Clef {
    G, // Bottom staff line E4
    C, // Bottom staff line F3
    F, // Bottom staff line G2
}

impl Clef {
    fn bottom_staff_position(&self) -> Step {
        match self {
            Self::G => Step::E,
            Self::C => Step::F,
            Self::F => Step::G,
        }
    }

    pub fn to_glyph(&self) -> char {
        match self {
            Self::G => '\u{e050}',
            Self::C => '@',
            Self::F => '\u{e062}',
        }
    }

    pub fn position(&self, step: Step) -> StaffPosition {
        let delta = step as i32 - self.bottom_staff_position() as i32;
        StaffPosition::new(delta)
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self.kind {
            Kind::Triad(Triad::Aug) => "augmented",
            Kind::Triad(Triad::Maj) => "major",
            Kind::Triad(Triad::Min) => "minor",
            Kind::Triad(Triad::Dim) => "diminished",
        };
        write!(f, "{:?} {}", self.key, kind)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Triad(Triad),
}

impl Kind {
    pub fn intervals(&self) -> Vec<(isize, isize)> {
        match self {
            Self::Triad(t) => t.intervals(),
        }
    }
    pub fn with_key(&self, key: Key) -> Vec<Tpc> {
        self.intervals()
            .iter()
            .filter_map(|&(scale_deg, alter)| key.scale_degree(scale_deg).alter(alter))
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Triad {
    Maj,
    Min,
    Dim,
    Aug,
}

impl Triad {
    pub fn intervals(&self) -> Vec<(isize, Alteration)> {
        match self {
            Self::Maj => vec![(0, 0), (2, 0), (4, 0)],
            Self::Min => vec![(0, 0), (2, -1), (4, 0)],
            Self::Dim => vec![(0, 0), (2, -1), (4, -1)],
            Self::Aug => vec![(0, 0), (2, 0), (4, 1)],
        }
    }
}
