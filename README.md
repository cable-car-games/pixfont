# PixFont

PixFont aims to be a simple but capable way to make bitmap fonts you can use and share anywhere.

This is early work in progress. It may not work or build if you look at it wrong.

To do before Preview 1:

- [ ] Save project to disk
- [ ] Load project from disk
- [ ] Export
  - [ ] BitFontMaker2
- [ ] Directory preview

Goals:

- [x] Nice easy to use editor
  - [x] Guidelines
  - [x] Flexible metrics
  - [ ] Local or web
- [x] Import from established pixel font formats
  - [x] Pentacom BitFontMaker2
  - [ ] Windows / OS2 bitmap font (.FON)
  - [ ] Mac OS classic?
  - [ ] BDF, PCF
- [ ] Export to a variety of formats
  - [ ] Trimmed native format (.pxfontb / .pxf)
  - [ ] Pentacom BitFontMaker2
  - [ ] BDF, PCF, Windows/OS2
  - [ ] FontForge project
  - [ ] UFO project
  - [ ] BMFont (PNG atlas)
  - [ ] TrueType

Known issues:

- [ ] Tab navigation (not implicit in iced yet)

## Build from source

This project is nowhere near ready for use, but you can build the latest and greatest by installing Rust, and firing a few commands.

```bash
git clone --recurse-submodules https://github.com/cable-car-games/pixfont.git
cargo build
cargo run -p pixfont-studio
```

Binary releases will be made available once some of the features bake a bit.

## Licence

The PixFont library is permissively licenced under the MIT or Apache 2.0 licences.

PixFont Studio is copyleft licenced under the AGPL 3.0 or later licence.

More info in [LICENSE.md](LICENSE.md).
