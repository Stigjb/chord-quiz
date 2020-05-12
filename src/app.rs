use log::info;
use yew::prelude::*;

use crate::score;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        info!("rendered!");
        html! {
            <div class="container">
                <h1>{ "Identify the chord" }</h1>
                <h2>{ "Triads"}</h2>
                <div class="d-flex flex-wrap">
                    { for (-5..10).map(score::triad_example) }
                </div>
                <h2>{ "Tetrachords"}</h2>
                <div class="d-flex flex-wrap">
                    { score::c_maj_7() }
                    { score::c_7() }
                </div>
            </div>
        }
    }
}
