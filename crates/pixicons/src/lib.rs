// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::borrow::Cow;

use image::{Pixel, RgbaImage};

pub use pixicons_macros::layer;
pub use pixicons_macros::layers;

pub mod icon;

mod layers {
    type Pixbuf = [u8; 1024];

    #[derive(Debug, Clone, Copy)]
    struct LayerNode {
        layer: Layer,
        pixbuf: &'static Pixbuf,
        children: &'static [LayerNode],
    }

    include!(concat!(env!("OUT_DIR"), "/layers.rs"));

    impl Layer {
        // TODO: this is slow and horrible, we can probably cache, bsearch, but I'm tired now
        pub fn pixbuf(self, context: &[Layer]) -> &'static Pixbuf {
            let mut candidates = vec![LAYER_DATA.child(self).unwrap()];

            // TODO: refine this bit, I want the effects to stack if relevant
            //       ie. (file.pentacom.new, 'new' layer uses font/new)
            candidates.extend(
                context
                    .iter()
                    .filter_map(|layer| LAYER_DATA.child(*layer))
                    .filter_map(|layer| layer.child(self)),
            );

            candidates.into_iter().next_back().unwrap().pixbuf
        }
    }

    impl LayerNode {
        fn child(&self, layer: Layer) -> Option<&'static LayerNode> {
            self.children.iter().find(|node| node.layer == layer)
        }
    }
}

pub use layers::Layer;

pub fn icon<'a, Message>(layers: impl Into<Layers<'a>>) -> icon::Icon<'a> {
    icon::Icon::new(layers)
}

impl Layer {
    pub fn image(self, context: &[Layer]) -> RgbaImage {
        let pixbuf = self.pixbuf(context);
        RgbaImage::from_raw(16, 16, Vec::from(pixbuf)).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layers<'a>(Cow<'a, [Layer]>);

impl<'a> Layers<'a> {
    pub fn image(&self) -> RgbaImage {
        let mut image = RgbaImage::new(16, 16);
        let Layers(layers) = self;

        for (index, layer) in layers.iter().enumerate() {
            image::imageops::overlay(&mut image, &layer.image(&layers[0..index]), 0, 0);
        }

        image
    }
}

impl<'a> From<&'a [Layer]> for Layers<'a> {
    fn from(slice: &'a [Layer]) -> Self {
        Self(Cow::Borrowed(slice))
    }
}

impl<'a, const N: usize> From<&'a [Layer; N]> for Layers<'a> {
    fn from(slice: &'a [Layer; N]) -> Self {
        Self(Cow::Borrowed(slice.as_slice()))
    }
}

impl FromIterator<Layer> for Layers<'_> {
    fn from_iter<T: IntoIterator<Item = Layer>>(iter: T) -> Self {
        Self(Cow::Owned(iter.into_iter().collect()))
    }
}

impl From<Layers<'_>> for iced::widget::image::Handle {
    fn from(layers: Layers) -> Self {
        let image = layers.image();
        Self::from_rgba(
            image.width(),
            image.height(),
            image
                .pixels()
                .flat_map(|pixel| pixel.channels())
                .copied()
                .collect::<Vec<_>>(),
        )
    }
}
