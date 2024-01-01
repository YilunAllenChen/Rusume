use yew::prelude::*;

pub fn with_side_tip(ele: Html, tip: String) -> Html {
    html! {
        <div class="flex mx-2 justify-between">
            <div class="w-1/3 text-sm font-semibold text-right self-center pr-4">
                {tip}
            </div>
            {ele}
        </div>
    }
}
