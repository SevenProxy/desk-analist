use yew::prelude::*;
use crate::components::navbar::{NavabarRoot, NavBarLogo, NavBarItems};
use crate::components::header::{HeaderRoot, HeaderTitle};

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
      <main class="orbitron-font w-full h-full">
        <div class="flex items-top w-full h-full relative">
          <NavabarRoot>
            <NavBarLogo />
            <NavBarItems />
          </NavabarRoot>
          <div class="w-full z-20 bg-[--foreground]">
            <HeaderRoot>
              <HeaderTitle text={"Dashboard"} />
            </HeaderRoot>
            { for ctx.props().children.iter() }
          </div>
        </div>
      </main>
    }
  }

}