use yew::prelude::*;
use crate::ui::LogoType;

pub struct NavBarLogo;

impl Component for NavBarLogo {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <menu class="w-full px-2 py-4 border-b-[1px] border-[--border-color] ">
        <LogoType />
      </menu>
    }
  }

}