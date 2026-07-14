// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use pixicons::Layer;
use pixicons::Layers;
use pixicons::layer;
use pixicons::layers;

#[test]
fn test_single_layer() {
    assert_eq!(layer!(file), Layer::File);
}

#[test]
fn test_multiple_layers() {
    assert_eq!(
        layers!(file.font.new),
        Layers::from(&[Layer::File, Layer::Font, Layer::New])
    )
}
