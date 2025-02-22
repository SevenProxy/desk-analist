use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsHeaderRoot {
  #[prop_or_default]
  pub children: Children,
}

pub struct HeaderRoot;

impl Component for HeaderRoot {
  type Message = ();
  type Properties = PropsHeaderRoot;

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <div class="w-full px-8 bg-[--foreground]">
        <header class="flex items-center justify-center border border-[--border-color] rounded-full w-full my-4 py-2 shadow bg-[--border-color]">
          { for ctx.props().children.iter() }
        </header>
      </div>
    }
  }

}