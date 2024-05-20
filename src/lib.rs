use base64::Engine;
use csv::ReaderBuilder;
use libtimeleft::{get_time_left, set_csv, CSVTime, DrawFn, DrawType};
use std::borrow::BorrowMut;
use std::panic;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;
use web_sys::{HtmlParagraphElement, UrlSearchParams};

struct TimeElements {
    class: HtmlParagraphElement,
    timeleft: HtmlParagraphElement,
    current_time: HtmlParagraphElement,
}
static mut TIME_ELEMENTS: Option<TimeElements> = None;

fn default_draw(
    draw_type: DrawType,
    redraw_all: bool,
    class: &String,
    time_left: &String,
    current_time: &String,
) {
    if unsafe { TIME_ELEMENTS.is_none() } {
        return;
    }
    let local = unsafe { TIME_ELEMENTS.borrow_mut().as_mut().unwrap() };
    match draw_type {
        DrawType::In => {
            if redraw_all {
                local.class.set_inner_text(&format!("Class: {}", class));
            }
            local
                .timeleft
                .set_inner_text(&format!("TimeLeft: {}", time_left));
            local
                .current_time
                .set_inner_text(&format!("Current Time: {}", current_time));
        }
        DrawType::Before => {
            if redraw_all {
                local
                    .class
                    .set_inner_text(&format!("Next Class: {}", class));
            }
            local
                .timeleft
                .set_inner_text(&format!("Time Till Start: {}", time_left));
            local
                .current_time
                .set_inner_text(&format!("Current Time: {}", current_time));
        }
        DrawType::Out => {
            local.class.set_inner_text(&format!("Outta school B)"));
        }
    }
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: String);
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    type Error;
    #[wasm_bindgen(constructor)]
    fn new() -> Error;
    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;
}
pub fn hook_impl(info: &panic::PanicInfo) {
    let mut msg = info.to_string();
    msg.push_str("\n\nStack:\n\n");
    let e = Error::new();
    let stack = e.stack();
    msg.push_str(&stack);
    msg.push_str("\n\n");
    error(msg);
}
#[wasm_bindgen]
pub fn greet() {
    console_log!("Hello world");
}
#[wasm_bindgen]
pub fn handler() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(hook_impl));
        run();
    });
    get_time_left(&(default_draw as DrawFn));
}
fn get_by_id<T>(id: &str) -> T
where
    T: std::convert::From<wasm_bindgen::JsValue>,
{
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let val: JsValue = document.get_element_by_id(id).expect(id).into();
    val.into()
}
const DEFAULT_B64:&'static str=
            "ZGF5cyxuYW1lLGJlZ2luLGVuZA0KTW9uIFR1ZSBXZWQgVGh1LCgxc3QpIEJpb2xvZ3ksNzozMCw4OjI1DQpNb24gVHVlIFdlZCBUaHUsKDJuZCkgQ2hvaXIsODozMCw5OjI1DQpNb24gVHVlIFdlZCBUaHUsKDNyZCkgSGlzdG9yeSw5OjMwLDEwOjI1DQpNb24gVHVlIFdlZCBUaHUsKDR0aCkgUHVibGljIFNwZWFraW5nLDEwOjMwLDExOjIzDQpNb24gVHVlIFdlZCBUaHUsTHVuY2gsMTE6MzAsMTI6MDANCk1vbiBUdWUgV2VkIFRodSxSVEksMTI6MDAsMTI6MjUNCk1vbiBUdWUgV2VkIFRodSwoNXRoKSBTVEFUUywxMjozMCwxMzoyNQ0KTW9uIFR1ZSBXZWQgVGh1LCg2dGgpIEVuZ2xpc2ggSUlJLDEzOjMwLDE0OjI1DQpNb24gVHVlIFdlZCBUaHUsKDd0aCkgTGVhZGVyc2hpcCwxNDozMCwxNToyNQ0KDQo="
;

#[wasm_bindgen]
pub fn run() {
    //console_error_panic_hook::set_once();
    let window = web_sys::window().expect("no global `window` exists");

    let params;
    let mut result;
    let b64: &[u8] = if let Ok(s) = window.location().search() {
        params = UrlSearchParams::new_with_str(&s).unwrap();
        result = params.get("times");
        result.get_or_insert(DEFAULT_B64.to_string()).as_bytes()
    } else {
        DEFAULT_B64.as_bytes()
    };
    let d = base64::prelude::BASE64_URL_SAFE.decode(b64).unwrap();
    let mut f = ReaderBuilder::new().from_reader(d.as_slice());

    // let csvtimes_local = Vec::new();
    //make csvtimes with all strings
    let mut csvs = Vec::new();
    for i in f.deserialize::<CSVTime>() {
        csvs.push(i.unwrap().into());
    }
    let len = csvs.len();
    set_csv(csvs);
    if len == 0 {
        panic!("len is zero",);
    }

    //set timeelements
    let time_elements: &mut Option<TimeElements> = unsafe { TIME_ELEMENTS.borrow_mut() };
    *time_elements = Some(TimeElements {
        class: get_by_id("class"),
        timeleft: get_by_id("timeleft"),
        current_time: get_by_id("currenttime"),
    });
    libtimeleft::get_time_left(&(default_draw as DrawFn));
}
