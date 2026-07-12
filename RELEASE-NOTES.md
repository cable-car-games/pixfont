# PixFont Studio 0.1.0

The first preview release for PixFont Studio.

This is incredibly early software. I only spent like a week's worth of hours on it. Expect crashes.

What is working:

- Create a blank font
- Add glyphs from predefined sets
- Save and load your project
- Import and export [Pentacom BitFontMaker2](https://www.pentacom.jp/pentacom/bitfontmaker2/) fonts
- Edit the glyphs

What isn't working:

- Other export formats
- Copy and paste
- Fill tool
- Keyboard shortcuts

## Bug hunting tips

These are debug builds with all sorts of additional debugging information built in to help with tracking down bugs.

Report issues on [GitHub](https://github.com/cable-car-games/pixfont/issues) or [itch.io](https://itch.io/category/6300851/new-topic).

Start PixFont Studio from a terminal and set `RUST_BACKTRACE=full` to show the full stack trace.

```sh
# macOS and Linux
RUST_BACKTRACE=full ./pixfont-studio
```

```powershell
# windows (powershell)
$env:RUST_BACKTRACE = "full"
./pixfont-studio.exe
```

Nicer error handling and reporting coming in a future release.

## For Windows and macOS users

These releases aren't signed yet.

Windows will pop-up with the SmartScreen prompt, which you will need to accept.

macOS users will need to clear the quarantine attribute:

```sh
xattr -d com.apple.quarantine path/to/pixfont-studio
```

I'm working on getting an Apple developer certificate to notarise releases going forward, probably later this year.
