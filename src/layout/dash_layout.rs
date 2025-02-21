use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PropsDashLayout {
  #[prop_or_default]
  pub children: Children,
}

pub struct DashLayout;

impl Component for DashLayout  {
  type Message = ();
  type Properties = PropsDashLayout;

  fn create(_: &Context<Self>) -> Self {
    Self
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    html! {
      <main>
        <div>
          { for ctx.props().children.iter() }
        </div>
      </main>
    }
  }

}