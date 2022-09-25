use rand::{thread_rng, Rng};
use yew::{function_component, html, use_state, Callback, Html};

mod styles;

struct Mine {
  value: i32,
  is_open: bool,
}

fn gen_cells() -> Vec<i32> {
  let mut raw_cells = (1..=90).collect::<Vec<_>>();
  let mut mine_cells = (1..=10).collect::<Vec<_>>();
  raw_cells.fill(0);
  mine_cells.fill(9);
  raw_cells.append(&mut mine_cells);
  let mut cells = raw_cells.clone();
  let len = raw_cells.len();
  let enumerate_cells = raw_cells.iter().enumerate();

  for (index, _) in enumerate_cells {
    let mut rng = thread_rng();
    let num: usize = rng.gen_range(index..len);
    cells.swap(index, num);
  }

  cells
}

fn gen_mine_cells() -> Vec<Mine> {
  let cells = gen_cells();
  let mut result_cells = cells.clone();
  let around: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
  ];

  for (index, cell) in cells.iter().enumerate() {
    if *cell < 9 {
      continue;
    }

    for (x, y) in around {
      let current = (index as i32) + y * 10 + x;
      if current < 0 || current > 100 {
        continue;
      }
      let current = current as usize;

      if cells[current] < 9 {
        result_cells[current] += 1;
      }
    }
  }

  let mut result = vec![];
  for item in result_cells.iter() {
    result.push(Mine {
      value: *item,
      is_open: false,
    });
  }

  result
}

#[function_component(MineSweeper)]
pub fn mine_sweeper() -> Html {
  let cells = use_state(gen_mine_cells);
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
      {cells.iter().map(|item| {
        let handle_click = {
          if !item.is_open {
            let cells = cells.clone();
            // Callback::from(move |_| cells.set(*cells));
          }
        };

        html!(
          <div key={item.value.to_string()}>
            {
              if item.is_open {
                item.value.to_string()
              } else {
                String::from("")
              }
            }
          </div>
        )
      }).collect::<Html>()}
    </div>
  }
}
