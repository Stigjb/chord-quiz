use std::fmt;

use tonality::{Accidental, Interval, Step, Tpc};
use yew::Html;

use crate::score;
use crate::score::StaffPosition;

#[derive(Clone, Debug, PartialEq)]
pub struct Chord {
    root: Tpc,
    kind: Kind,
    tpcs: Vec<Tpc>,
}

impl Chord {
    pub fn new(root: Tpc, kind: Kind) -> Option<Self> {
        kind.with_root(&root).map(|tpcs| Self { root, kind, tpcs })
    }

    pub fn to_svg(&self) -> Html {
        let clef = Clef::G;
        score::Builder::new()
            .space(0.5)
            .clef(&clef)
            .space(6.)
            .accidentals(&self.accidentals())
            .space(1.5)
            .chord(&self.staff_positions(&clef))
            .space(6.)
            .barline()
            .into_svg()
    }

    pub fn staff_positions(&self, clef: &Clef) -> Vec<StaffPosition> {
        let root_position = clef.position(self.tpcs[0].step());
        self.tpcs
            .iter()
            .map(|tpc| {
                let mut pos = clef.position(tpc.step());
                while pos < root_position {
                    pos = &pos + 7;
                }
                pos
            })
            .collect()
    }

    fn accidentals(&self) -> Vec<(Accidental, StaffPosition)> {
        self.tpcs
            .iter()
            .zip(self.staff_positions(&Clef::G))
            .filter_map(|(t, p)| match t.altered_step(None) {
                (_, Some(acc)) => Some((acc, p)),
                (_, None) => None,
            })
            .collect()
    }
}

impl Default for Chord {
    fn default() -> Self {
        Self::new(Tpc::C, Kind::Triad(Triad::Maj)).unwrap()
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
        let (step, alter) = self.root.altered_step(None);
        let alter = alter
            .map(|acc| match acc {
                Accidental::DblFlat => "♭♭",
                Accidental::Flat => "♭",
                Accidental::Natural => "",
                Accidental::Sharp => "♯",
                Accidental::DblSharp => "♯♯",
            })
            .unwrap_or("");
        let kind = match self.kind {
            Kind::Triad(Triad::Aug) => "+",
            Kind::Triad(Triad::Maj) => "",
            Kind::Triad(Triad::Min) => "m",
            Kind::Triad(Triad::Dim) => "m♭5",
            Kind::Tetrad(Tetrad::Maj7) => "maj7",
            Kind::Tetrad(Tetrad::Min7) => "m7",
            Kind::Tetrad(Tetrad::Min7b5) => "m7♭5",
            Kind::Tetrad(Tetrad::Dom7) => "7",
            Kind::Tetrad(Tetrad::Dim7) => "dim7",
        };
        write!(f, "{:?}{}{}", step, alter, kind)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Triad(Triad),
    Tetrad(Tetrad),
}

impl Kind {
    pub fn intervals(&self) -> Vec<Interval> {
        match self {
            Self::Triad(t) => t.intervals(),
            Self::Tetrad(t) => t.intervals(),
        }
    }
    pub fn with_root(&self, root: &Tpc) -> Option<Vec<Tpc>> {
        self.intervals()
            .iter()
            .map(|&interval| root.clone() + interval)
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
    pub fn intervals(&self) -> Vec<Interval> {
        use Interval::*;
        match self {
            Self::Maj => vec![Unison, Maj3, P5],
            Self::Min => vec![Unison, Min3, P5],
            Self::Dim => vec![Unison, Min3, Dim5],
            Self::Aug => vec![Unison, Maj3, Aug5],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tetrad {
    Dom7,
    Maj7,
    Min7,
    Min7b5,
    Dim7,
}

impl Tetrad {
    pub fn intervals(&self) -> Vec<Interval> {
        use Interval::*;
        match self {
            Tetrad::Dom7 => vec![Unison, Maj3, P5, Min7],
            Tetrad::Maj7 => vec![Unison, Maj3, P5, Maj7],
            Tetrad::Min7 => vec![Unison, Min3, P5, Min7],
            Tetrad::Min7b5 => vec![Unison, Min3, Dim5, Min7],
            Tetrad::Dim7 => vec![Unison, Min3, Dim5, Dim7],
        }
    }
}
