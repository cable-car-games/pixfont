# PixFont Studio 0.2.0

> [!WARNING]
> This is the draft for the next release.

This is incredibly early software. The total man hours is about two weeks of
work so far.

New in this release:

- (todo)

What broke:

- (todo)

What still isn't working:

- Other import/export formats
- Copy and paste
- Fill tool
- Keyboard shortcuts

## Bug hunting tips

The binaries for previews include all sorts of goodies and debugging information
built in to help with tracking down bugs.

Report issues on [GitHub][gh-issues] or [the itch.io topic][itch-issues].

[gh-issues]: https://github.com/cable-car-games/pixfont/issues/new
[itch-issues]: https://itch.io/t/6610492/pre-release-bug-tracking

If you can, start PixFont Studio from a terminal with `RUST_BACKTRACE=full` to
show the full stack trace.

```sh
# macOS and Linux
RUST_BACKTRACE=full ./pixfont-studio
```

```powershell
# Windows (powershell)
$env:RUST_BACKTRACE = "full"
./pixfont-studio.exe
```

Nicer error handling and reporting coming in a future release.

## For Windows and macOS users

These releases aren't notarised yet.

Windows will pop up the SmartScreen prompt when you run the app, which you will
need to accept.

macOS will block the app from running without first removing the quarantine
attribute.

```sh
xattr -d com.apple.quarantine path/to/pixfont-studio
```

I will be getting an Apple developer certificate to notarise releases going
forward later this year.
