// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use image::ImageReader;
use image::Pixel;
use std::collections::{BTreeMap, BTreeSet, LinkedList};
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{fs, io};

const ICON_SIZE: usize = 16;
const PIXELS_SIZE: usize = (ICON_SIZE * 4) * ICON_SIZE;

use crate::Level::Error;

macro_rules! msg {
    ($level:expr, $($arg:tt)*) => {
        $crate::msg($level, &format!($($arg)*))
    };
}

macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::fatal(&format!($($arg)*))
    };
}

fn main() {
    println!("cargo:rerun-if-changed=layers");

    let layers_dir = Path::new("layers");

    let layers = get_layer_tree(layers_dir);
    let layer_names = get_all_layer_names(layers_dir, &layers);

    let layers = decode_layers(layers_dir, &layers);

    write_layer_rs(&layers, layer_names).unwrap();
}

//
//

#[derive(Debug, Clone, Default)]
struct LayerTree(BTreeMap<LayerName, LayerTree>);

#[derive(Debug, Clone)]
struct Layer {
    pixels: [u8; PIXELS_SIZE],
    variants: BTreeMap<LayerName, Layer>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct LayerName {
    snake_case: String,
    pascal_case: String,
}

impl Display for LayerName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.snake_case)
    }
}

impl LayerName {
    fn make(snake_case: String, pascal_case: String) -> Self {
        if Self::pascal_case(&snake_case) != pascal_case {
            println!("pascal_case(\"{snake_case}\") != \"{pascal_case}\"");
            fatal!("'{snake_case}' is an invalid layer name")
        }

        if Self::snake_case(&pascal_case) != snake_case {
            println!("pascal_case(\"{snake_case}\") != \"{pascal_case}\"");
            fatal!("'{snake_case}' is an invalid layer name")
        }

        Self {
            snake_case,
            pascal_case,
        }
    }

    fn from_snake_case(s: &str) -> Self {
        Self::make(s.to_string(), Self::pascal_case(s))
    }

    fn from_pascal_case(s: &str) -> Self {
        Self::make(Self::snake_case(s), s.to_string())
    }

    fn snake_case(s: &str) -> String {
        let mut segments: Vec<&str> = vec![];
        let mut last_segment_start = 0;
        for (index, char) in s.char_indices() {
            if char.is_ascii_uppercase() {
                segments.push(&s[last_segment_start..index]);
                last_segment_start = index;
            }
        }
        segments.push(&s[last_segment_start..]);

        segments
            .into_iter()
            .fold(String::new(), |acc, segment| {
                acc + "_" + &segment.to_ascii_lowercase()
            })
            .trim_matches('_')
            .to_string()
    }

    fn pascal_case(s: &str) -> String {
        s.split('_')
            .map(|segment| {
                let mut s = String::new();
                let mut chars = segment.chars();

                if let Some(first) = chars.next() {
                    s.push(first.to_ascii_uppercase());
                }

                s.extend(chars);
                s
            })
            .fold(String::new(), |acc, segment| acc + &segment)
    }
}

//
//

fn get_layer_tree(path: &Path) -> BTreeMap<LayerName, LayerTree> {
    onion(path).unwrap_or_else(|error| fatal!("failed to find layers: {:?}", error))
}

fn onion(path: &Path) -> io::Result<BTreeMap<LayerName, LayerTree>> {
    let mut map: BTreeMap<LayerName, LayerTree> = BTreeMap::new();

    path.read_dir()?.try_for_each(|child| {
        let child = child?;
        let path = child.path();
        let name = path.file_prefix().unwrap().to_string_lossy();
        let layer_name = LayerName::from_snake_case(&name);

        let is_dir = {
            let metadata = fs::metadata(&path)?;
            if metadata.is_file() {
                false
            } else if metadata.is_dir() {
                true
            } else {
                fatal!("{:?} is not a file or a dir", path.display())
            }
        };

        let entry = map.entry(layer_name);

        if is_dir {
            let variants = onion(&path)?;
            entry
                .and_modify(|LayerTree(variants)| *variants = variants.clone())
                .or_insert(LayerTree(variants));
        } else {
            entry.or_default();
        }

        io::Result::Ok(())
    })?;

    Ok(map)
}

fn get_all_layer_names<'a>(
    path: &Path,
    layers: &'a BTreeMap<LayerName, LayerTree>,
) -> BTreeSet<&'a LayerName> {
    let all_layers = {
        let mut all_layers = BTreeSet::new();
        let mut queue = LinkedList::from_iter(layers.iter());

        loop {
            let Some((name, LayerTree(variants))) = queue.pop_front() else {
                break all_layers;
            };

            all_layers.insert(name);
            for variant in variants {
                queue.push_back(variant);
            }
        }
    };

    if !all_layers.iter().fold(true, |acc, layer| {
        let exists = path.join(Path::new(&format!("{}.png", layer))).exists();
        if !exists {
            msg!(Error, "can't find base layer for '{}'", layer);
        }

        acc && exists
    }) {
        exit(1)
    }

    all_layers
}

//
//

fn decode_layers(
    path: &Path,
    layer: &BTreeMap<LayerName, LayerTree>,
) -> BTreeMap<LayerName, Layer> {
    let mut map = BTreeMap::new();

    for (name, LayerTree(variants)) in layer {
        let img_path = path.join(PathBuf::from(name.snake_case.clone() + ".png"));
        let var_path = path.join(Path::new(name.snake_case.as_str()));

        let image = ImageReader::open(&img_path)
            .unwrap_or_else(|error| fatal!("failed to open {}: {error:?}", img_path.display()));
        let image = image
            .decode()
            .unwrap_or_else(|error| fatal!("failed to decode {}: {error:?}", img_path.display()))
            .to_rgba8();

        if image.width() as usize != ICON_SIZE || image.height() as usize != ICON_SIZE {
            fatal!("")
        }

        let mut image = image.pixels().flat_map(|pixel| pixel.channels());

        let mut pixels = [0u8; PIXELS_SIZE];
        pixels.fill_with(|| match image.next() {
            Some(pixel) => *pixel,
            None => unimplemented!(),
        });

        map.insert(
            name.clone(),
            Layer {
                pixels,
                variants: decode_layers(&var_path, variants),
            },
        );
    }

    map
}

//
//

fn write_layer_rs<'a>(
    layers: &'a BTreeMap<LayerName, Layer>,
    layer_names: impl IntoIterator<Item = &'a LayerName>,
) -> io::Result<()> {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let mut w = File::create(out_dir.join("layers.rs")).unwrap();

    write_layer_enum(&mut w, layer_names.into_iter()).unwrap();
    writeln!(w)?;
    write!(w, "const LAYER_DATA: LayerNode = ")?;
    write_layer_tree(
        &mut w,
        &LayerName::from_pascal_case("Blank"),
        &Layer {
            pixels: [0u8; PIXELS_SIZE],
            variants: layers.clone(),
        },
    )?;
    writeln!(w, ";")?;

    Ok(())
}

fn write_layer_enum<'a>(
    w: &mut impl Write,
    mut layer: impl Iterator<Item = &'a LayerName>,
) -> io::Result<()> {
    writeln!(w, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]")?;
    writeln!(w, "pub enum Layer {{")?;
    writeln!(w, "  Blank,")?;
    layer.try_for_each(|name| writeln!(w, "  {},", name.pascal_case))?;
    writeln!(w, "}}")?;

    Ok(())
}

fn write_layer_tree(w: &mut impl Write, name: &LayerName, layer: &Layer) -> io::Result<()> {
    writeln!(w, "LayerNode {{")?;
    writeln!(w, "  layer: Layer::{},", name.pascal_case)?;
    write!(w, "  pixbuf: &[")?;

    for byte in layer.pixels {
        write!(w, "0x{byte:02X}, ")?;
    }

    writeln!(w, "],")?;
    write!(w, "  children: &[")?;

    for (name, layer) in &layer.variants {
        write_layer_tree(w, name, layer)?;
        write!(w, ", ")?;
    }

    writeln!(w, "],")?;
    write!(w, "}}")?;
    Ok(())
}

//
//

#[allow(unused)]
enum Level {
    Error,
    Warning,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Level::Error => "error",
            Level::Warning => "warning",
        })
    }
}

fn msg(level: Level, message: &str) {
    println!("cargo::{level}={message}")
}

fn fatal(message: &str) -> ! {
    msg(Error, message);
    exit(1)
}
