use js_sys::{Array, Boolean, JsString, Reflect};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    console, window, HtmlVideoElement, MediaStream, MediaStreamConstraints, MediaStreamTrack,
    MediaStreamTrackProcessor, MediaStreamTrackProcessorInit, ReadableStreamDefaultReader,
    VideoEncoder, VideoEncoderConfig, VideoEncoderInit, VideoFrame, VideoTrack,
};
use yew::prelude::*;

static VIDEO_CODEC: &str = "vp09.00.10.08";
static VIDEO_HEIGHT: u32 = 1280;
static VIDEO_WIDTH: u32 = 720;

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
        let video_track = Box::new(
            device
                .get_video_tracks()
                .find(&mut |_: JsValue, _: u32, _: Array| true)
                .unchecked_into::<VideoTrack>(),
        );
        let error_handler = Closure::wrap(Box::new(move |e: JsValue| {
            console::log_1(&JsString::from("error"));
            console::log_1(&e);
        }) as Box<dyn FnMut(JsValue)>);
        let output_handler =
            Closure::wrap(Box::new(move |chunk| console::log_1(&chunk)) as Box<dyn FnMut(JsValue)>);
        let video_encoder_init = VideoEncoderInit::new(
            error_handler.as_ref().unchecked_ref(),
            output_handler.as_ref().unchecked_ref(),
        );
        let video_encoder = VideoEncoder::new(&video_encoder_init).unwrap();
        let video_encoder_config = VideoEncoderConfig::new(&VIDEO_CODEC, VIDEO_HEIGHT, VIDEO_WIDTH);
        video_encoder.configure(&video_encoder_config);
        let processor = MediaStreamTrackProcessor::new(&MediaStreamTrackProcessorInit::new(
            &video_track.unchecked_into::<MediaStreamTrack>(),
        ))
        .unwrap();
        let reader = processor
            .readable()
            .get_reader()
            .unchecked_into::<ReadableStreamDefaultReader>();
        loop {
            let result = JsFuture::from(reader.read()).await.map_err(|e| {
                console::log_1(&e);
            });
            match result {
                Ok(js_frame) => {
                    let video_frame = Reflect::get(&js_frame, &JsString::from("value"))
                        .unwrap()
                        .unchecked_into::<VideoFrame>();
                    video_encoder.encode(&video_frame);
                    video_frame.close();
                }
                Err(_e) => {
                    console::log_1(&JsString::from("error"));
                }
            }
        }
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
