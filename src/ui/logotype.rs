use yew::prelude::*;

pub struct LogoType;

impl Component for LogoType {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <span class=" orbitron-font flex items-center justify-center">
        <p class="font-bold text-4xl text-[--color]">{"D."}</p>
        <p class="text-white font-bold text-3xl">{"A>"}</p>
      </span>
    }
  }

}