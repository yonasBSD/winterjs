use ion::{
    class::{NativeObject, Reflector},
    ClassDefinition, Context, Exception, Heap, Object, Promise, Result, TracedHeap,
};
use mozjs_sys::jsapi::JSObject;
use runtime::{
    globals::fetch::Request as FetchRequest, globals::fetch::Response as FetchResponse,
    promise::future_to_promise,
};

use crate::{ion_err, ion_mk_err};

#[js_class]
pub struct Env {
    reflector: Reflector,
    assets: Heap<*mut JSObject>,
}

impl Env {
    pub fn new_obj(cx: &Context) -> *mut JSObject {
        let assets = EnvAssets::new_object(
            cx,
            Box::new(EnvAssets {
                reflector: Default::default(),
            }),
        );
        Env::new_object(
            cx,
            Box::new(Env {
                reflector: Default::default(),
                assets: Heap::new(assets),
            }),
        )
    }
}

#[js_class]
impl Env {
    #[ion(constructor)]
    pub fn constructor() -> Result<Env> {
        ion_err!("Cannot construct this type", Type)
    }

    #[allow(non_snake_case)]
    #[ion(name = "ASSETS", get)]
    pub fn get_assets(&self) -> *mut JSObject {
        self.assets.get()
    }
}

#[js_class]
pub struct EnvAssets {
    reflector: Reflector,
}

#[js_class]
impl EnvAssets {
    #[ion(constructor)]
    pub fn constructor() -> Result<EnvAssets> {
        ion_err!("Cannot construct this type", Type)
    }

    pub fn fetch(&self, cx: &Context, request: &FetchRequest) -> Option<Promise> {
        let request_heap = TracedHeap::new(request.reflector().get());

        unsafe {
            future_to_promise::<_, _, _, Exception>(cx, move |cx| async move {
                let request =
                    FetchRequest::get_mut_private(&cx, &request_heap.root(&cx).into()).unwrap();

                let mut http_req = http::Request::builder()
                    .uri(request.get_url())
                    .method(request.method());

                for header in request.headers(&cx) {
                    http_req = http_req.header(header.0.clone(), header.1.clone())
                }

                let request_body = request.take_body()?;
                let (cx, body_bytes) = cx.await_native_cx(|cx| request_body.into_bytes(cx)).await;
                let body_bytes = body_bytes?;
                let body = match body_bytes {
                    Some(bytes) => hyper::Body::from(bytes),
                    None => hyper::Body::empty(),
                };

                let http_req = http_req.body(body)?;

                let (parts, body) = http_req.into_parts();
                let request = super::super::Request { parts, body };

                let url = url::Url::parse(request.parts.uri.to_string().as_str())?;
                let (cx, response) = cx
                    .await_native(super::CloudflareRequestHandler::serve_static_file(request))
                    .await;
                let response = response.map_err(|e| {
                    ion_mk_err!(format!("Failed to fetch static asset due to {e}"), Normal)
                })?;
                let response = FetchResponse::from_hyper_response(&cx, response, url)?;
                Ok(FetchResponse::new_object(&cx, Box::new(response)))
            })
        }
    }
}

pub fn define(cx: &Context, global: &Object) -> bool {
    Env::init_class(cx, global).0 && EnvAssets::init_class(cx, global).0
}
