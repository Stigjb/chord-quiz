use num_traits::FromPrimitive;
use rand::Rng;
use tonality::Tpc;
use yew::prelude::*;

use crate::chord::{Chord, Kind};
use crate::settings::Settings;
use crate::tpc_octave::TpcOctave;

pub struct App {
    link: ComponentLink<Self>,
    chord: Chord,
    revealed: bool,
    num_chords: usize,
    num_correct: usize,
    use_dbl_accidentals: bool,
}

pub enum Msg {
    Reveal,
    NextCorrect,
    NextWrong,
    ToggleDblAccidentals,
}

fn root_range(chord: &Kind, use_dbl_accidentals: bool) -> (isize, isize) {
    if use_dbl_accidentals {
        (
            chord.flattest_root() as isize,
            chord.sharpest_root() as isize,
        )
    } else {
        (
            chord.flattest_root_no_dbl_flat() as isize,
            chord.sharpest_root_no_dbl_sharp() as isize,
        )
    }
}

fn random_chord(use_dbl_accidentals: bool) -> Chord {
    let mut rng = rand::thread_rng();

    let chord = match rng.gen_range(0, 9) {
        0 => Kind::Maj,
        1 => Kind::Min,
        2 => Kind::Dim,
        3 => Kind::Aug,
        4 => Kind::Dom7,
        5 => Kind::Dim7,
        6 => Kind::Maj7,
        7 => Kind::Min7,
        _ => Kind::Min7b5,
    };

    let (tpc_low, tpc_high) = root_range(&chord, use_dbl_accidentals);

    let tpc_num = rng.gen_range(tpc_low, tpc_high);
    let tpc: Tpc = FromPrimitive::from_isize(tpc_num).unwrap();
    let octave = rng.gen_range(2, 5);
    let root = TpcOctave(tpc, octave);
    Chord::new(root.clone(), chord.clone())
        .expect(&format!("Out of range with {:?}, {:?}", root, chord))
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let chord = random_chord(false);
        let revealed = false;
        Self {
            link,
            chord,
            revealed,
            num_chords: 0,
            num_correct: 0,
            use_dbl_accidentals: false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Reveal => self.revealed = true,
            Msg::NextCorrect => {
                self.num_correct += 1;
                self.num_chords += 1;
                self.chord = random_chord(self.use_dbl_accidentals);
                self.revealed = false
            }
            Msg::NextWrong => {
                self.num_chords += 1;
                self.chord = random_chord(self.use_dbl_accidentals);
                self.revealed = false
            }
            Msg::ToggleDblAccidentals => self.use_dbl_accidentals = !self.use_dbl_accidentals,
        }
        true
    }

    fn view(&self) -> Html {
        let answer = if self.revealed {
            self.chord.to_string()
        } else {
            String::new()
        };
        let button = if self.revealed {
            let on_correct = self.link.callback(|_| Msg::NextCorrect);
            let on_wrong = self.link.callback(|_| Msg::NextWrong);
            html! {
                <div class="btn-group" role="group">
                    <button class="btn btn-success" onclick=on_correct>{ "Got it right" }</button>
                    <button class="btn btn-warning" onclick=on_wrong>{ "Got it wrong" }</button>
                </div>
            }
        } else {
            let on_reveal = self.link.callback(|_| Msg::Reveal);
            html! {
                <button class="btn btn-primary" onclick=on_reveal>{ "Reveal answer" }</button>
            }
        };
        let on_toggle = self.link.callback(|_| Msg::ToggleDblAccidentals);
        let score = format!("{} out of {} correct", self.num_correct, self.num_chords);
        html! {
            <>
            <main class="container">
                <h1>{ "Identify the chord" }</h1>
                <p>{ score }</p>
                <div class="score-wrapper">{ self.chord.to_svg() }</div>
                <div class="answer">{ answer }</div>
                { button }
                <Settings allow_dbl_accidentals=self.use_dbl_accidentals on_toggle=on_toggle />
            </main>
            <footer class="footer">
                <div class="container text-muted">
                    { "The graphics are generated in your browser as an SVG using Steinberg's Bravura musical font. " }
                    { "The app is programmed in Rust using the Yew framework. " }
                    { "Repository: "}
                    <a href="https://github.com/stigjb/chord-quiz">{ "stigjb/chord-quiz" }</a>
                </div>
            </footer>
            </>
        }
    }
}
