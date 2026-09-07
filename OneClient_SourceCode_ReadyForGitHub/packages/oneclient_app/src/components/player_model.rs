use std::cell::RefCell;

use bytes::Bytes;
use freya::engine::prelude::{
    Data, Paint, SamplingOptions, Shader, SkData, SkImage, SkRect, TileMode,
};
use freya::prelude::*;
use skia_safe::RuntimeEffect;
use skia_safe::runtime_effect::ChildPtr;

use freya::prelude::*;
use reqwest;

#[derive(PartialEq, Clone)]
pub struct PlayerModel {
    uuid: String,
    width: Size,
    height: Size,
}

impl PlayerModel {
    pub fn new(uuid: impl Into<String>) -> Self {
        Self {
            uuid: uuid.into(),
            width: Size::fill(),
            height: Size::fill(),
        }
    }

    pub fn width(mut self, width: Size) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Size) -> Self {
        self.height = height;
        self
    }
}

impl Component for PlayerModel {
    fn render(&self) -> impl IntoElement {
        let uuid = self.uuid.clone();
        
        let mut img_bytes = use_state(|| None::<Bytes>);
        
        let uuid_copy = uuid.clone();
        let bytes_setter = img_bytes.clone();
        
        use_side_effect(move || {
            let u = uuid_copy.clone();
            let mut setter = bytes_setter.clone();
            
            spawn(async move {
                let url = format!("https://visage.surgeplay.com/full/512/{}", u);
                match reqwest::get(&url).await {
                    Ok(resp) => {
                        if let Ok(bytes) = resp.bytes().await {
                            setter.set(Some(bytes));
                        } else {
                            setter.set(Some(Bytes::new()));
                        }
                    },
                    Err(_) => setter.set(Some(Bytes::new())),
                }
            });
        });

        rect()
            .width(self.width.clone())
            .height(self.height.clone())
            .center()
            .child(match &*img_bytes.read() {
                Some(bytes) if !bytes.is_empty() => {
                    let data = unsafe { freya::engine::prelude::SkData::new_bytes(bytes) };
                    if let Some(sk_img) = freya::engine::prelude::SkImage::from_encoded(data) {
                        let handle = freya::elements::image::ImageHandle::new(sk_img, bytes.clone());
                        freya::elements::image::image(handle)
                            .width(Size::fill())
                            .height(Size::fill())
                            .aspect_ratio(AspectRatio::Min)
                            .into_element()
                    } else {
                        label().text("Loading 3D...").into_element()
                    }
                }
                Some(_) => label().text("Error Loading 3D Model").into_element(),
                None => label().text("Loading 3D Model...").into_element(),
            })
    }

}

