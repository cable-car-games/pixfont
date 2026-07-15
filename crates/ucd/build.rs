// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: 2026 Rareș Nistor

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

fn main() {
    println!("cargo:rerun-if-changed=ucd");

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let mut out_file = File::create(out_dir.join("ucd.rs")).expect("failed to create file");

    write_blocks(&mut out_file).unwrap();
    write_codepoints(&mut out_file).unwrap();
}

fn write_blocks(w: &mut impl Write) -> io::Result<()> {
    let mut file = File::open("ucd/Blocks.txt")?;
    let mut r = BufReader::new(&mut file);

    writeln!(w, "impl Block {{")?;
    writeln!(w, "  pub const ALL: &[Block] = &[")?;

    loop {
        let mut line = String::new();
        if r.read_line(&mut line)? == 0 {
            break;
        }

        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        let (range, name) = line
            .split_once(";")
            .expect("expected only single separator");
        let name = name.trim();

        let (start, end) = range
            .split_once("..")
            .expect("expected range to have single '..' separator");
        let (start, end) = (
            u32::from_str_radix(start, 16).unwrap(),
            u32::from_str_radix(end, 16).unwrap(),
        );

        writeln!(
            w,
            "    Block {{ range: RangeInclusive {{ start: 0x{start:04X}, last: 0x{end:04X} }}, name: \"{name}\" }},"
        )?;
    }

    writeln!(w, "  ];")?;
    writeln!(w, "}}")?;

    Ok(())
}

fn write_codepoints(w: &mut impl Write) -> io::Result<()> {
    let mut file = File::open("ucd/UnicodeData.txt")?;
    let mut r = BufReader::new(&mut file);

    writeln!(w, "impl Codepoint {{")?;
    writeln!(w, "  pub const ALL: &[Codepoint] = &[")?;

    loop {
        let mut line = String::new();
        if r.read_line(&mut line)? == 0 {
            break;
        }

        let mut fields = line.split(';');
        let fields = {
            let mut f = [""; 15];
            f.fill_with(|| fields.next().unwrap());
            f
        };

        let codepoint = u32::from_str_radix(fields[0], 16).unwrap();
        let name = fields[1];

        let mut aliases = Vec::new();
        if !fields[10].is_empty() {
            aliases.push(fields[10]);
        }

        writeln!(
            w,
            "    Codepoint {{ codepoint: 0x{codepoint:04X}, name: \"{name}\", aliases: &[{}] }},",
            aliases
                .into_iter()
                .fold(String::new(), |acc, alias| format!("{acc} \"{alias}\","))
        )?;
    }

    writeln!(w, "  ];")?;
    writeln!(w, "}}")?;

    Ok(())
}
