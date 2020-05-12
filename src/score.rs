use yew::{html, Html};

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

pub fn c_maj_7() -> Html {
    Builder::new()
        .space(0.5)
        .clef(F_CLEF)
        .space(6.)
        .chord()
        .space(6.)
        .barline()
        .into_svg()
}

pub fn c_7() -> Html {
    Builder::new()
        .space(0.5)
        .clef(G_CLEF)
        .space(6.)
        .accidentals()
        .space(1.5)
        .chord()
        .space(6.)
        .barline()
        .into_svg()
}

pub fn triad_example(bottom: i16) -> Html {
    Builder::new()
        .space(0.5)
        .clef(G_CLEF)
        .space(6.)
        .triad(&StaffPosition(bottom))
        .space(6.)
        .barline()
        .into_svg()
}

struct Builder {
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

    fn space(mut self, size: f32) -> Self {
        self.cursor += size * STAFF_SPACE;
        self
    }

    fn clef(mut self, clef: char) -> Self {
        let y = match clef {
            G_CLEF => STAFF_SPACE * 3.,
            F_CLEF => STAFF_SPACE,
            _ => 0.,
        };
        let clef = html! { <text x=self.cursor y=y>{ clef }</text> };
        self.nodes.push(clef);
        self
    }

    fn accidentals(mut self) -> Self {
        let accidentals = html! { <text x=self.cursor y="8">{ ACCIDENTAL_FLAT }</text> };
        self.nodes.push(accidentals);
        self
    }

    fn chord(mut self) -> Self {
        let d = format!("M{},20 h8.752", self.cursor - 1.);
        let chord = html! {
            <g>
                <text x=self.cursor y="8">{ NOTEHEAD_WHOLE }</text>
                <text x=self.cursor y="12">{ NOTEHEAD_WHOLE }</text>
                <text x=self.cursor y="16">{ NOTEHEAD_WHOLE }</text>
                <text x=self.cursor y="20">{ NOTEHEAD_WHOLE }</text>
                <path d=d stroke-width=LEGER_LINE_THICKNESS stroke="black" />
            </g>
        };
        self.nodes.push(chord);
        self
    }

    fn barline(mut self) -> Self {
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

    fn triad(mut self, bottom: &StaffPosition) -> Self {
        let staff_positions: Vec<StaffPosition> =
            vec![0, 2, 4].iter().map(|i: &i16| bottom + *i).collect();
        let bottom_pos = &staff_positions[0];
        let top_pos = &staff_positions[2];
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

    fn into_svg(self) -> Html {
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

/// Staff pos: semitones relative to bottom staff line, increasing toward higher pitches
fn note(cursor: f32, staff_pos: &StaffPosition) -> Html {
    let y = staff_pos.to_y();
    html! {
        <text x=cursor y=y>{ NOTEHEAD_WHOLE }</text>
    }
}

struct StaffPosition(i16);

impl StaffPosition {
    pub fn to_y(&self) -> f32 {
        (4. - f32::from(self.0) / 2.) * STAFF_SPACE
    }
}

impl ::std::ops::Add<i16> for &StaffPosition {
    type Output = StaffPosition;

    fn add(self, rhs: i16) -> StaffPosition {
        StaffPosition(self.0 + rhs)
    }
}
