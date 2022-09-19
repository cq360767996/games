use yew::{function_component, html, use_state, Callback};

#[function_component(MineSweeper)]
pub fn mine_sweeper() -> Html {
  let counter = use_state(|| 0);

  let plus = {
    let counter = counter.clone();
    Callback::from(move |_| counter.set(*counter + 1))
  };

  let minus = {
    let counter = counter.clone();
    Callback::from(move |_| counter.set(*counter - 1))
  };

  html! {
      <div>
        <button onclick={plus}>{ "plus" }</button>
        <button onclick={minus}>{ "minus" }</button>
          <p>
              <b>{ "Current value: " }</b>
              { *counter }
          </p>
      </div>
  }
}
