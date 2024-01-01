use yew::prelude::*;

pub fn with_side_tip(ele: Html, tip: String) -> Html {
    html! {
        <div class="flex m-2">
            <div class="w-1/4 text-lg font-semibold text-right self-center pr-4">
                {tip}
            </div>
            {ele}
        </div>
    }
}
