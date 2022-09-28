use rand::{thread_rng, Rng};
use uuid::Uuid;
use yew::{function_component, html, use_state, Callback, Html, MouseEvent};

#[derive(Clone)]
struct Mine {
  value: i32,
  is_open: bool,
  id: String,
}

const AROUND: [(i32, i32); 8] = [
  (-1, -1),
  (0, -1),
  (1, -1),
  (1, 0),
  (1, 1),
  (0, 1),
  (-1, 1),
  (-1, 0),
];

const ROWS: i32 = 10;
const COLS: i32 = 10;

fn init_cells() -> Vec<i32> {
  let total = ROWS * COLS;
  let mut raw_cells = (1..=(total - ROWS)).collect::<Vec<_>>();
  let mut mine_cells = (1..=ROWS).collect::<Vec<_>>();
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
  let cells = init_cells();
  let mut result_cells = cells.clone();
  let total = ROWS * COLS;

  for (index, cell) in cells.iter().enumerate() {
    if *cell < 9 {
      continue;
    }

    for (x, y) in AROUND {
      let current = (index as i32) + y * COLS + x;
      if current < 0 || current >= total {
        continue;
      }
      let current = current as usize;

      if cells[current] < 9 {
        result_cells[current] += 1;
      }
    }
  }

  let mut result = vec![];
  for value in result_cells {
    let id = Uuid::new_v4().to_string();
    result.push(Mine {
      id,
      value,
      is_open: false,
    });
  }

  result
}

fn render_text(item: &Mine) -> String {
  if item.is_open {
    item.value.to_string()
  } else {
    String::from("")
  }
}

fn pickup_border(index: &i32) -> Vec<usize> {
  let mut dispatch_arr = vec![];
  let remainder = *index % COLS;
  if remainder == 9 {
    dispatch_arr.push(2);
    dispatch_arr.push(3);
    dispatch_arr.push(4);
  }

  if remainder == 0 {
    dispatch_arr.push(6);
    dispatch_arr.push(7);
    dispatch_arr.push(1);
  }
  dispatch_arr
}

fn open_related_cells<'a>(cells: &'a mut Vec<Mine>, index: &'a usize) {
  let total = ROWS * COLS;
  if cells[*index].is_open {
    return;
  }

  cells[*index].is_open = true;
  if cells[*index].value != 0 {
    return;
  }

  let border_arr = pickup_border(&(*index as i32));

  for (i, (x, y)) in AROUND.iter().enumerate() {
    if border_arr.contains(&i) {
      continue;
    }

    let current = (*index as i32) + x + y * COLS;

    if current < 0 || current >= total {
      continue;
    }

    let current = current as usize;
    if cells[current].is_open {
      continue;
    }

    open_related_cells(cells, &current);
  }
}

#[function_component(MineSweeper)]
pub fn mine_sweeper() -> Html {
  let cells = use_state(gen_mine_cells);
  let ended = use_state(|| false);

  let handle_contextmenu = {
    Callback::from(|e: MouseEvent| {
      e.prevent_default();
    })
  };

  let render_cell = || -> Html {
    cells
      .iter()
      .enumerate()
      .map(|(index, item)| {
        let handle_click = {
          let cells = cells.clone();
          let ended = (&ended).clone();

          Callback::from(move |_| {
            let mut new_cells = (*cells).clone();
            let cell = &mut new_cells[index];
            if cell.is_open {
              return;
            }

            if cell.value < 9 {
              open_related_cells(&mut new_cells, &index);
            } else {
              ended.set(true);
              return;
            }

            cells.set(new_cells);
          })
        };

        let handle_right_click = {
          Callback::from(move |e: MouseEvent| {
            log::info!("right: {}", e.buttons());
          })
        };

        html! {
          <div
            class="cell opened"
            key={item.id.clone()}
            onclick={handle_click}
            oncontextmenu={handle_right_click}
            >
              {render_text(&item)}
          </div>
        }
      })
      .collect::<Html>()
  };

  html! {
    <>
      <div>
        {if *ended {
          String::from("游戏结束")
        } else {
          String::from("游戏中")
        }}
      </div>
      <div
        class="w-500px h-500px grid grid-cols-10 grid-rows-10"
        oncontextmenu={handle_contextmenu}
      >
        {render_cell()}
      </div>
    </>
  }
}
