use yew::prelude::*;

pub struct NavBarItems;

impl Component for NavBarItems {
  type Message = ();
  type Properties = ();

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, _: &Context<Self>) -> Html {
    html! {
      <div class="h-[140px] py-2 my-4 mx-2 bg-[--foreground] rounded-xl text-white">
        {"a"}
      </div>
    }
  }

}