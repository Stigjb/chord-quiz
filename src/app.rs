use log::info;
use num_traits::FromPrimitive;
use rand::Rng;
use tonality::Key;
use yew::prelude::*;

use crate::{
    chord::{Chord, Kind},
    score,
};

pub struct App {
    link: ComponentLink<Self>,
    chord: Chord,
    revealed: bool,
}

pub enum Msg {
    Reveal,
    Next,
}

fn random_chord() -> Chord {
    let (key_lo, key_hi) = (Key::Gb as isize, Key::Fs as isize);
    let key_num = rand::thread_rng().gen_range(key_lo, key_hi);
    let key: Key = FromPrimitive::from_isize(key_num).unwrap();
    Chord::new(key, Kind::Triad(crate::chord::Triad::Maj))
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let chord = random_chord();
        let revealed = false;
        Self {
            link,
            chord,
            revealed,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Reveal => self.revealed = true,
            Msg::Next => {
                self.chord = random_chord();
                self.revealed = false
            }
        }
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");
        let answer = if self.revealed {
            self.chord.to_string()
        } else {
            String::new()
        };
        let button = if self.revealed {
            let on_next = self.link.callback(|_| Msg::Next);
            html! {
                <>
                <button class="btn btn-primary" onclick=on_next>{ "Next chord" }</button>
                </>
            }
        } else {
            let on_reveal = self.link.callback(|_| Msg::Reveal);
            html! {
                <button class="btn btn-primary" onclick=on_reveal>{ "Reveal answer" }</button>
            }
        };
        html! {
            <>
            <main class="container">
                <h1>{ "Identify the chord" }</h1>
                <div class="d-flex flex-wrap">{ self.chord.to_svg() }</div>
                <div id="answer">{ answer }</div>
                { button }
            </main>
            <footer class="footer">
                <div class="container text-muted">
                    { "Author: Stig Johan Berggren <" }
                    <a href="mailto:stigjb@gmail.com">{ "stigjb@gmail.com" }</a>
                    { ">" }
                </div>
            </footer>
            </>
        }
    }
}
