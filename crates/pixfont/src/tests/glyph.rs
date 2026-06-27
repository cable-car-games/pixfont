// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use crate::*;

#[test]
fn pixel_thing() {
    let pixels = pixvec(
        r"
  xx
    x
  xxx
 x  x
  xx x
",
        Point::new(0, 0),
    );
}

fn pixvec(str: &str, origin: Point) -> Pixels {
    let mut rows: Vec<Vec<bool>> = Vec::new();

    for line in str.split('\n') {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(char != ' ');
        }

        rows.push(row);
    }

    let width = rows.iter().map(Vec::len).max().unwrap();
    let size = Size::new(width.try_into().unwrap(), rows.len().try_into().unwrap());

    let pixels = rows
        .iter()
        .flat_map(|row| {
            let mut vec = row.clone();
            vec.append(&mut vec![false; width - row.len()]);
            vec
        })
        .collect();

    Pixels::with_pixels(pixels, size, origin)
}
