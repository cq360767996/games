use web_sys::HtmlInputElement;
use yew::prelude::*;

enum Msg {
	AddOne,
	MinusOne,
}

struct Model {
	value: i64,
	node_ref: NodeRef,
}

impl Component for Model {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self {
			value: 0,
			node_ref: NodeRef::default(),
		}
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::AddOne => {
				self.value += 1;
				// the value has changed so we need to
				// re-render for it to appear on the page
				true
			}
			Msg::MinusOne => {
				self.value -= 1;
				true
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		// This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
		let link = ctx.link();
		let plus = link.callback(|_| Msg::AddOne);
		let minus = link.callback(|_| Msg::MinusOne);
		html! {
				<div>
					<input ref={self.node_ref.clone()} value={1} />
					<button onclick={plus}>{ "+1" }</button>
					<button onclick={minus}>{ "-1" }</button>
					<p>{ self.value }</p>
				</div>
		}
	}

	fn rendered(&mut self, _: &Context<Self>, first_render: bool) {
		if first_render {
			if let Some(input) = self.node_ref.cast::<HtmlInputElement>() {
				input.focus();
			}
		}
	}
}

fn main() {
	yew::start_app::<Model>();
}
