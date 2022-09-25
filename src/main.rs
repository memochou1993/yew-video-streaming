use js_sys::Boolean;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, HtmlVideoElement, MediaStream, MediaStreamConstraints};
use yew::prelude::*;

#[function_component(Producer)]
fn producer() -> Html {
    wasm_bindgen_futures::spawn_local(async move {
        let navigator = window().unwrap().navigator();
        let media_devices = navigator.media_devices().unwrap();
        let video_element = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("webcam")
            .unwrap()
            .unchecked_into::<HtmlVideoElement>();
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&Boolean::from(true));
        let device_query = media_devices
            .get_user_media_with_constraints(&constraints)
            .unwrap();
        let device = JsFuture::from(device_query)
            .await
            .unwrap()
            .unchecked_into::<MediaStream>();
        video_element.set_src_object(Some(&device));
    });

    html!(
        <div class="producer">
            <h3>
                {"Producer"}
                <video autoplay=true id="webcam"></video>
            </h3>
        </div>
    )
}

#[function_component(Consumer)]
fn consumer() -> Html {
    html!(
        <div class="consumer">
            <h3>
                {"Consumer"}
            </h3>
        </div>
    )
}

#[function_component(App)]
fn app() -> Html {
    html!(
        <div class={"grid"}>
            <Producer />
            <Consumer />
        </div>
    )
}

fn main() {
    yew::start_app::<App>();
}
