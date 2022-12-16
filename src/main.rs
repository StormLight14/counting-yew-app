use yew::prelude::*;
enum Msg {
    Team1AddOne,
    Team1RemoveOne,
    Team2AddOne,
    Team2RemoveOne,
}

struct CounterComponent {
    team1_count: i64,
    team2_count: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { team1_count: 0 , team2_count: 0}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Team1AddOne => {
                self.team1_count += 1;
                return true;
            }
            Msg::Team1RemoveOne => {
                self.team1_count -= 1;
                return true;
            }
            Msg::Team2AddOne => {
                self.team2_count += 1;
                return true;
            }
            Msg::Team2RemoveOne => {
                self.team2_count -= 1;
                return true;
            }
        }

    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
        <>
          <div class="container">
            <p>{"Team 1 Score: "} {self.team1_count}</p>
            <button onclick={link.callback(|_| Msg::Team1AddOne)}>{"+1"}</button>
            <button onclick={link.callback(|_| Msg::Team1RemoveOne)}>{"-1"}</button>
            <p>{"Team 2 Score: "} {self.team2_count}</p>
            <button onclick={link.callback(|_| Msg::Team2AddOne)}>{"+1"}</button>
            <button onclick={link.callback(|_| Msg::Team2RemoveOne)}>{"-1"}</button>
          </div>
          <footer>
          <p>{"StormLight14 on GitHub"}</p>
          </footer>
          </>
          
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
