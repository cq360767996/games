use yew::{function_component, html, use_effect_with_deps, use_state, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct NumberProps {
  pub time: i32,
  pub is_countdown: bool,
}

#[function_component(Number)]
pub fn number(props: &NumberProps) -> Html {
  let class_names = use_state(|| vec![]);

  {
    let class_names = class_names.clone();

    use_effect_with_deps(
      move |time| {
        let mut names = vec![];
        let mut t = time.clone();
        for _ in (1..=3).collect::<Vec<_>>().iter() {
          let remainder = t % 10;
          t /= 10;
          names.insert(0, format!("num d{}", remainder));
        }
        class_names.set(names);
        || ()
      },
      props.time,
    );
  }

  html! {
    <div class="nums flex justify-around items-center">
      {class_names.iter().map(|item| html!{<div class={item} />}).collect::<Html>()}
    </div>
  }
}
