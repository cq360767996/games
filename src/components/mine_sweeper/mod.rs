use rand::{thread_rng, Rng};
use uuid::Uuid;
use yew::{function_component, html, use_state, Callback, Html};

mod styles;

#[derive(Clone)]
struct Mine {
  value: i32,
  is_open: bool,
  id: String,
}

#[function_component(MineSweeper)]
pub fn mine_sweeper() -> Html {
  let cells = use_state(gen_mine_cells);
  let ended = use_state(|| false);

  fn init_cells() -> Vec<i32> {
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
    let cells = init_cells();
    let mut result_cells = cells.clone();
    let around = [
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
        if current < 0 || current >= 100 {
          continue;
        }
        let current = current as usize;

        if cells[current] < 9 {
          result_cells[current] += 1;
        }
      }
    }

    let mut result = vec![];
    for item in &result_cells {
      let id = Uuid::new_v4().to_string();
      result.push(Mine {
        id,
        value: *item,
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

  fn open_related_cells<'a>(cells: &'a mut Vec<Mine>, index: &'a usize) {
    cells[*index].is_open = true;
    if cells[*index].value != 0 {
      return;
    }

    let direction = [(0, -1), (1, 0), (-1, 0), (0, 1)];
    for (x, y) in direction {
      let current = (*index as i32) + x + y * 10;

      if current < 0 || current >= 100 {
        continue;
      }

      let current = current as usize;
      if cells[current].is_open {
        continue;
      }

      open_related_cells(cells, &current);
    }
  }

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
        html! {<div key={item.id.clone()} onclick={handle_click}>{render_text(&item)}</div>}
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
      <div class={styles::container()}>{render_cell()}</div>
    </>
  }
}
