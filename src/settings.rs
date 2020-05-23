use yew::prelude::*;

pub struct Settings {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub allow_dbl_accidentals: bool,
    pub on_toggle: Callback<bool>,
}

pub enum Msg {
    Toggled,
}

impl Component for Settings {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggled => self.props.on_toggle.emit(!self.props.allow_dbl_accidentals)
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let on_toggle = self.link.callback(|_| Msg::Toggled);
        html! {
            <>
            <h2>{ "Settings" }</h2>
            <p>{ "The settings apply with the next random chord." }</p>
            <div class="form-check">
                <input
                    id="dbl-toggle"
                    class="form-check-input"
                    type="checkbox"
                    checked=self.props.allow_dbl_accidentals
                    onchange=on_toggle
                />
                <label for="dbl-toggle" class="form-check-label">
                    { "Allow double sharps and flats" }
                </label>
            </div>
            </>

        }
    }
}
