use yew::{function_component, html, Html};

mod styles;

#[function_component(MineSweeper)]
pub fn mine_sweeper() -> Html {
  let mut safe_cells = (1..=90).collect::<Vec<_>>();
  let mut mine_cells = (1..=10).collect::<Vec<_>>();
  safe_cells.fill(0);
  mine_cells.fill(1);
  safe_cells.append(&mut mine_cells);

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
      {safe_cells.iter().map(|order|html!(<div key={order.to_string()}>{order}</div>)).collect::<Html>()}
    </div>
  }
}
