use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsHeaderTitle {
  pub text: String,
}

pub struct HeaderTitle;

impl Component for HeaderTitle {
  type Message = ();
  type Properties = PropsHeaderTitle;

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <h1 class="text-3xl text-white font-bold select-none eventmouse-none">
        { format!("{}", ctx.props().text ) }
      </h1>
    }
  }

}