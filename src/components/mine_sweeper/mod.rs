use rand::{thread_rng, Rng};
use uuid::Uuid;
use yew::{
  classes, function_component, html, use_effect_with_deps, use_state, Callback, Html, MouseEvent,
};

#[derive(Clone, Debug, PartialEq)]
enum MineValue {
  Some(i32),
  Mine(String),
}

#[derive(Clone, Debug, PartialEq)]
struct Mine {
  value: MineValue,
  is_open: bool,
  flag: bool,
  id: String,
}

enum GameState {
  Gamimg,
  Lose,
  Win,
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
const TOTAL_MINES: i32 = 10;

fn init_values() -> Vec<i32> {
  let total = ROWS * COLS;
  let mut raw_values = (1..=(total - TOTAL_MINES)).collect::<Vec<_>>();
  let mut mine_values = (1..=TOTAL_MINES).collect::<Vec<_>>();
  raw_values.fill(0);
  mine_values.fill(9);
  raw_values.append(&mut mine_values);
  let mut cells = raw_values.clone();
  let len = raw_values.len();
  let enumerate_cells = raw_values.iter().enumerate();

  for (index, _) in enumerate_cells {
    let mut rng = thread_rng();
    let num: usize = rng.gen_range(index..len);
    cells.swap(index, num);
  }

  cells
}

fn gen_mine_cells() -> Vec<Mine> {
  let values = init_values();
  let mut result_values = values.clone();

  for (index, cell) in values.iter().enumerate() {
    let index = index as i32;

    if *cell < 9 {
      continue;
    }

    let border_arr = pickup_border(&index);

    for (i, (x, y)) in AROUND.iter().enumerate() {
      if border_arr.contains(&i) {
        continue;
      }
      let current = (index + (y * COLS) + x) as usize;

      if values[current] < 9 {
        result_values[current] += 1;
      }
    }
  }

  value_to_cells(&result_values)
}

fn value_to_cells(cells: &Vec<i32>) -> Vec<Mine> {
  let mut result = vec![];
  for cell in cells {
    let id = Uuid::new_v4().to_string();
    let value = if *cell < 9 {
      MineValue::Some(*cell)
    } else {
      MineValue::Mine(String::from("mine"))
    };

    result.push(Mine {
      id,
      value,
      flag: false,
      is_open: false,
    });
  }

  result
}

fn pickup_border(index: &i32) -> Vec<usize> {
  let mut dispatch_arr = vec![];
  let remainder = *index % COLS;
  let total = ROWS * COLS;
  if *index < COLS {
    dispatch_arr.push(0);
    dispatch_arr.push(1);
    dispatch_arr.push(2);
  }

  if total - *index - 1 < COLS {
    dispatch_arr.push(4);
    dispatch_arr.push(5);
    dispatch_arr.push(6);
  }

  if remainder == COLS - 1 {
    dispatch_arr.push(2);
    dispatch_arr.push(3);
    dispatch_arr.push(4);
  }

  if remainder == 0 {
    dispatch_arr.push(6);
    dispatch_arr.push(7);
    dispatch_arr.push(0);
  }
  dispatch_arr
}

fn open_related_cells<'a>(cells: &'a mut Vec<Mine>, index: &'a usize) {
  let index = *index;
  if cells[index].is_open {
    return;
  }

  cells[index].is_open = true;
  if let MineValue::Some(0) = cells[index].value {
    let index = index as i32;
    let border_arr = pickup_border(&index);

    for (i, (x, y)) in AROUND.iter().enumerate() {
      if border_arr.contains(&i) {
        continue;
      }

      let current = (index + (y * COLS) + x) as usize;

      if cells[current].is_open {
        continue;
      }

      open_related_cells(cells, &current);
    }
  }
}

fn open_all_cells(cells: &mut Vec<Mine>) {
  for cell in cells {
    cell.is_open = true;
  }
}

#[function_component(MineSweeper)]
pub fn mine_sweeper() -> Html {
  let cells = use_state(gen_mine_cells);
  let state = use_state(|| GameState::Gamimg);

  let handle_contextmenu = {
    Callback::from(|e: MouseEvent| {
      e.prevent_default();
    })
  };

  {
    let cells = cells.clone();
    let state = state.clone();
    use_effect_with_deps(
      move |cells| {
        let mut st = GameState::Win;
        let mut closed_count = 0;
        for cell in cells.iter() {
          if let MineValue::Mine(value) = &cell.value {
            if value.eq("mine_red") {
              st = GameState::Lose;
              break;
            }
          }
          if closed_count > TOTAL_MINES {
            st = GameState::Gamimg;
            break;
          }

          if !cell.is_open {
            closed_count += 1;
          }
        }

        state.set(st);

        || ()
      },
      cells,
    );
  }

  let render_cell = |(index, item): (usize, &Mine)| {
    let handle_opened_click = || {
      // TODO: 处理已经打开的按钮点击事件
    };
    // 点击事件处理
    let handle_click = {
      let cells = cells.clone();

      Callback::from(move |_| {
        let mut new_cells = (*cells).clone();
        let cell = &mut new_cells[index];
        if cell.is_open {
          handle_opened_click();
          return;
        }

        if let MineValue::Mine(_) = cell.value {
          cell.value = MineValue::Mine("mine_red".to_string());
          open_all_cells(&mut new_cells);
        } else {
          open_related_cells(&mut new_cells, &index);
        }

        cells.set(new_cells);
      })
    };

    let handle_right_click = {
      let cells = cells.clone();
      Callback::from(move |_| {
        let mut new_cells = (*cells).clone();
        let cell = &mut new_cells[index];
        if cell.is_open {
          handle_opened_click();
          return;
        }
        cell.flag = !cell.flag;
        cells.set(new_cells);
      })
    };

    let get_class_name = || {
      let mut class_names = vec![];
      class_names.push("cell".to_string());

      if item.is_open {
        class_names.push("opened".to_string());
        match &item.value {
          MineValue::Some(i) => {
            class_names.push(format!("type{}", i));
          }
          MineValue::Mine(name) => {
            class_names.push(name.to_string());
          }
        }
      } else if item.flag {
        class_names.push("flag".to_string());
      }

      classes!(class_names)
    };

    html! {
      <div
        class={get_class_name()}
        key={item.id.clone()}
        onclick={handle_click}
        oncontextmenu={handle_right_click}
      />
    }
  };

  html! {
    <>
      <div>
        {
          match *state {
            GameState::Gamimg => "游戏中",
            GameState::Lose => "你输了",
            GameState::Win => "你赢了",
          }
        }
      </div>
      <header></header>
      <section class="flex">
        // <div class="hor h-240px w-4px" />
        <div
          class="w-240px h-240px grid grid-cols-10 grid-rows-10"
          oncontextmenu={handle_contextmenu}
        >
          {cells.iter().enumerate().map(render_cell).collect::<Html>()}
        </div>
      </section>
    </>
  }
}
