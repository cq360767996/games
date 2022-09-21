use yew::{function_component, html, Html};

mod styles;

#[function_component(MineSweeper)]
pub fn mine_sweeper() -> Html {
  let items = (1..=100).collect::<Vec<i32>>();

  // let counter = use_state(|| 0);

  // let plus = {
  //   let counter = counter.clone();
  //   Callback::from(move |_| counter.set(*counter + 1))
  // };

  // let minus = {
  //   let counter = counter.clone();
  //   Callback::from(move |_| counter.set(*counter - 1))
  // };

  html! {
    <div class={styles::container()}>
      {items.iter().map(|order|html!(<div key={order.to_string()}></div>)).collect::<Html>()}
    </div>
  }
}
