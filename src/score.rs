use log::info;
use tonality::Accidental;
use yew::{html, Html};

use crate::chord;

// With font-size 16
const STAFF_SPACE: f32 = 4.;
const STAFF_LINE_THICKNESS: f32 = 0.13 * STAFF_SPACE;
const STEM_THICKNESS: f32 = 0.12 * STAFF_SPACE;
const LEGER_LINE_THICKNESS: f32 = 0.16 * STAFF_SPACE;
const THIN_BARLINE_THICKNESS: f32 = 0.16 * STAFF_SPACE;
const BARLINE_SEPARATION: f32 = 0.4 * STAFF_SPACE;

const G_CLEF: char = '\u{e050}';
const F_CLEF: char = '\u{e062}';
const NOTEHEAD_WHOLE: char = '\u{e0a2}';
const ACCIDENTAL_FLAT: char = '\u{e260}';
const ACCIDENTAL_NATURAL: char = '\u{e261}';
const ACCIDENTAL_SHARP: char = '\u{e262}';
const ACCIDENTAL_DOUBLE_SHARP: char = '\u{e263}';
const ACCIDENTAL_DOUBLE_FLAT: char = '\u{e264}';

fn accidental_glyph(acc: &Accidental) -> char {
    match acc {
        Accidental::DblFlat => ACCIDENTAL_DOUBLE_FLAT,
        Accidental::Flat => ACCIDENTAL_FLAT,
        Accidental::Natural => ACCIDENTAL_NATURAL,
        Accidental::Sharp => ACCIDENTAL_SHARP,
        Accidental::DblSharp => ACCIDENTAL_DOUBLE_SHARP,
    }
}

pub fn c_maj_7() -> Html {
    let staff_positions = vec![
        StaffPosition(-2),
        StaffPosition(0),
        StaffPosition(2),
        StaffPosition(4),
    ];
    Builder::new()
        .space(0.5)
        .clef(&chord::Clef::F)
        .space(6.)
        .chord(&staff_positions)
        .space(6.)
        .barline()
        .into_svg()
}

pub fn c_7() -> Html {
    let staff_positions = vec![
        StaffPosition(-2),
        StaffPosition(0),
        StaffPosition(2),
        StaffPosition(4),
    ];
    Builder::new()
        .space(0.5)
        .clef(&chord::Clef::G)
        .space(6.)
        .accidentals(&vec![(Accidental::Flat, StaffPosition::new(4))])
        .space(1.5)
        .chord(&staff_positions)
        .space(6.)
        .barline()
        .into_svg()
}

pub fn triad_example(bottom: i32) -> Html {
    Builder::new()
        .space(0.5)
        .clef(&chord::Clef::G)
        .space(6.)
        .triad(&StaffPosition(bottom))
        .space(6.)
        .barline()
        .into_svg()
}

pub struct Builder {
    cursor: f32,
    nodes: Vec<Html>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            cursor: 0.,
            nodes: Vec::new(),
        }
    }

    pub fn space(mut self, size: f32) -> Self {
        self.cursor += size * STAFF_SPACE;
        self
    }

    pub fn clef(mut self, clef: &chord::Clef) -> Self {
        use chord::Clef;
        let y = match clef {
            Clef::G => STAFF_SPACE * 3.,
            Clef::F => STAFF_SPACE,
            Clef::C => STAFF_SPACE * 2.,
        };
        let clef = html! { <text x=self.cursor y=y>{ clef.to_glyph() }</text> };
        self.nodes.push(clef);
        self
    }

    pub fn accidentals(mut self, accs: &[(Accidental, StaffPosition)]) -> Self {
        info!("Adding accidentals {:?}", accs);
        let mut sorted_accs: Vec<_> = accs.to_owned();
        sorted_accs.sort_unstable_by_key(|(_, pos)| -pos.0);
        let indents = align_accidentals(&sorted_accs);
        let accidentals = sorted_accs
            .iter()
            .zip(indents)
            .map(|((acc, pos), indent)| {
                let glyph = accidental_glyph(acc);
                let y = pos.to_y();
                html! { <text x=self.cursor + indent y=y>{ glyph }</text> }
            })
            .collect::<Html>();
        self.nodes.push(accidentals);
        self
    }

    pub fn triad(self, bottom: &StaffPosition) -> Self {
        let staff_positions: Vec<StaffPosition> =
            vec![0, 2, 4].iter().map(|i: &i32| bottom + *i).collect();
        self.chord(&staff_positions)
    }

    pub fn barline(mut self) -> Self {
        let d = format!(
            "M{},0 v16 M{},0 v16",
            self.cursor,
            self.cursor + BARLINE_SEPARATION
        );
        let barline = html! { <path d=d stroke-width=THIN_BARLINE_THICKNESS stroke="black" /> };
        self.nodes.push(barline);
        self.cursor += BARLINE_SEPARATION + 0.5 * THIN_BARLINE_THICKNESS;
        self
    }

    pub fn chord(mut self, staff_positions: &[StaffPosition]) -> Self {
        let bottom_pos: &StaffPosition = staff_positions.iter().min().unwrap();
        let top_pos: &StaffPosition = staff_positions.iter().max().unwrap();
        let lowest_leger = bottom_pos.0 - bottom_pos.0 % 2;

        // Even valued positions <= -2 and >= 10 are candidates for leger lines
        let leger_positions = (lowest_leger..=-2)
            .step_by(2)
            .chain((10..=top_pos.0).step_by(2));
        let legers = leger_positions.map(|l| {
            let d = format!("M{},{} h8.752", self.cursor - 1., StaffPosition(l).to_y());
            html! { <path d=d stroke-width=LEGER_LINE_THICKNESS stroke="black" /> }
        });

        // Create a note at each position
        let notes = staff_positions.iter().map(|i| note(self.cursor, i));
        let triad = html! {
            <g>
                { for legers }
                { for notes }
            </g>
        };
        self.nodes.push(triad);
        self
    }

    pub fn into_svg(self) -> Html {
        let view_box = format!("-8 -16 {} 48", self.cursor + 16.);
        let staff = staff(self.cursor);
        html! {
            <svg class="score bravura" viewBox=view_box>
                { staff }
                { for self.nodes }
            </svg>
        }
    }
}

fn staff(width: f32) -> Html {
    let d = format!(
        "M0,0 h{w} M0,4 h{w} M0,8 h{w} M0,12 h{w} M0,16 h{w}",
        w = width
    );
    html! { <path d=d stroke-width=STAFF_LINE_THICKNESS stroke="black" /> }
}

fn note(cursor: f32, staff_pos: &StaffPosition) -> Html {
    let y = staff_pos.to_y();
    html! {
        <text x=cursor y=y>{ NOTEHEAD_WHOLE }</text>
    }
}

fn align_accidentals(accs: &[(Accidental, StaffPosition)]) -> Vec<f32> {
    if accs.is_empty() {
        return Vec::new();
    }
    let mut indents = Vec::new();
    let mut top: &StaffPosition = &accs[0].1;
    let mut indent = 0.;
    for (_, pos) in accs.iter() {
        // Accidentals at least a seventh apart can share column
        if top.0 - pos.0 >= 6 {
            top = &pos;
            indent = 0.;
        }        
        indents.push(indent);
        indent -= STAFF_SPACE;
    }
    indents
}

/// A position in a staff relative to the bottom staff line
///
/// increasing toward higher pitches
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StaffPosition(i32);

impl StaffPosition {
    pub fn new(pos: i32) -> Self {
        Self(pos)
    }

    pub fn to_y(&self) -> f32 {
        (4. - self.0 as f32 / 2.) * STAFF_SPACE
    }
}

impl ::std::ops::Add<i32> for &StaffPosition {
    type Output = StaffPosition;

    fn add(self, rhs: i32) -> StaffPosition {
        StaffPosition(self.0 + rhs)
    }
}
