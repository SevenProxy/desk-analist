use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsNavbarRoot {
  #[prop_or_default]
  pub children: Children,
}

pub struct NavabarRoot;

impl Component for NavabarRoot {
  type Message = ();
  type Properties = PropsNavbarRoot;

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <nav class="max-w-[180px] min-w-[180px] min-h-screen relative">
        <div class="dark:bg-[--background] w-full h-full fixed z-10 py-4 top-0 left-0">
          { for ctx.props().children.iter() }
        </div>
      </nav>
    }
  }

}