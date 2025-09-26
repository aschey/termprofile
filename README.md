# termprofile

A library to detect and handle terminal color/styling support.

Terminal environments can have several levels of color support:

- **true color** - sometimes referred to as RGB or 24bit, can set any valid RGB
  color
- **ANSI 256** - Indexed colors in the
  [256 color list](https://www.ditig.com/256-colors-cheat-sheet)
- **ANSI 16** - Only the first 16 colors in the ANSI color list, commonly seen
  in non-graphical environments like login shells
- **No Color** - Text modifiers like bold and italics can be used, but no colors
  should be emitted. This is usually set by override variables.
- **No TTY** - The output is not a TTY and no escape sequences should be used.

## Feature Flags

All features are disabled by default.

- `terminfo` - Enables checking against the terminfo database for color support.
  See [terminfo](#terminfo).

- `query-detect` - Enables querying for truecolor support via
  [DECRQSS](https://vt100.net/docs/vt510-rm/DECRQSS.html). See
  [querying the terminal](#querying-the-terminal).

- `windows-version` - Enables additional checks for color support based on the
  current version of Windows. See [windows](#windows).

- `convert` - Enables converting incompatible colors based on the color support
  level.

- `color-cache` - Adds an optional LRU cache for color conversion operations.
  This can be useful when rendering under high frame rates.

- `ratatui` - Enables direct conversion to Ratatui style and color objects.

- `ratatui-underline-color` - Enables Ratatui's `underline-color` feature and
  includes underline colors in Ratatui style conversions.

## Usage

### Color Support Detection

```rust
use std::io::stdout;
use termprofile::{TermProfile, DetectorSettings};

let profile = TermProfile::detect(&stdout(), DetectorSettings::default());
println!("Detected profile: {profile:?}");
```

#### Overriding Variables

Variables can be overridden before detecting the color profile.

```rust
use std::io::stdout;
use termprofile::{TermProfile, TermVars, DetectorSettings};

let mut vars = TermVars::from_env(&stdout(), DetectorSettings::default());
vars.overrides.force_color = "1".into();
let profile = TermProfile::detect_with_vars(vars);
println!("Profile with override: {profile:?}");
```

#### Custom Sources

Environment variables can be sourced from an in-memory map instead of reading
directly from the environment.

```rust
use std::collections::HashMap;
use std::io::stdout;
use termprofile::{TermProfile, TermVars, DetectorSettings};

let source = HashMap::from_iter([("TERM", "xterm-256color"), ("COLORTERM", "1")]);

let vars = TermVars::from_source(&source, &stdout(), DetectorSettings::default());
let profile = TermProfile::detect_with_vars(vars);
println!("Profile: {profile:?}");
```

### Color Conversion

Colors can be automatically adapted to the nearest compatible type based on the
given profile.

```rust
use termprofile::TermProfile;
use std::io::stdout;
use anstyle::{Color, RgbColor, Ansi256Color};

let profile = TermProfile::Ansi256;
let rgb_color: Color = RgbColor(209, 234, 213).into();
let adapted_color = profile.adapt_color(rgb_color);

assert_eq!(adapted_color, Some(Ansi256Color(253).into()));
```

Automatic color conversion doesn't always produce optimal results. Use
`ProfileColor` to have more control over the conversion.

```rust
use termprofile::{TermProfile, ProfileColor};
use anstyle::{Color, RgbColor, Ansi256Color, AnsiColor};

let profile = TermProfile::Ansi256;

let color = ProfileColor::new(Color::Rgb(RgbColor(209, 234, 213)), profile)
  .ansi_256(240)
  .ansi_16(AnsiColor::White);

assert_eq!(color.adapt(), Some(Ansi256Color(240).into()));
```

### Style Conversion

Styles can be converted as well. Text modifiers will be removed if the profile
is set to `NoTTY`.

```rust
use termprofile::TermProfile;
use std::io::stdout;
use anstyle::{Color, RgbColor, AnsiColor, Style};

let profile = TermProfile::Ansi16;

let fg = RgbColor(106, 132, 92).into();
let bg = RgbColor(4, 35, 212).into();
let style = Style::new().fg_color(Some(fg)).bg_color(Some(bg));

let adapted_style = profile.adapt_style(style);

assert_eq!(
    adapted_style,
    Style::new()
        .fg_color(Some(AnsiColor::Cyan.into()))
        .bg_color(Some(AnsiColor::BrightBlue.into()))
);
```

### Ratatui Conversions

`anstyle` is used for color conversions due to its compatibility with other
terminal color crates, but it does not have support for a `Color::Reset`
variant, which can be important for TUI apps. If the `ratatui` feature is
enabled, all of the above conversions work with Ratatui types, allowing for full
compatibility without requiring `anstyle` as an intermediate layer.

```rust
use termprofile::TermProfile;
use std::io::stdout;
use ratatui::style::Color;

let profile = TermProfile::Ansi256;
let rgb_color = Color::Rgb(209, 234, 213);
let adapted_color = profile.adapt_color(rgb_color);

assert_eq!(adapted_color, Some(Color::Indexed(253)));
```

### Caching

Color conversion can be somewhat expensive if you're rendering at a high frame
rate. The `color-cache` feature enables an opt-in LRU cache that can be used to
cache a certain amount of calculations.

```rust
use termprofile::{set_color_cache_enabled, set_color_cache_size};

set_color_cache_enabled(true);
set_color_cache_size(256.try_into().expect("non-zero size"));
```

## Color Detection Details

Sadly, there is no standard way to accurately detect color support in terminals.
Instead, we must rely on a number of environment variables and other methods
that have organically emerged as a pseudo-standard over time.

### Querying the Terminal

The most reliable way to detect true color support is to
[query for it](https://github.com/termstandard/colors?tab=readme-ov-file#querying-the-terminal).
This is preferable over environment variables because it works over SSH and is
not susceptible to ambiguity caused by terminal multiplexers. Unfortunately,
this method isn't supported in many terminals yet.

### Terminal Variables

- [`COLORTERM`](https://lists.jedsoft.org/lists/slang-users/2016/0000014.html) -
  the terminal supports true color if this is set to `24bit` or `truecolor`
- `TERM` - the most common variable supplied by a terminal, this denotes the
  name of the terminal program. We maintain a list of terminals that are known
  to have truecolor support as well as some fuzzy matching logic for common
  suffixes (e.g. `-256color` for ANSI 256 support).
- `TERM_PROGRAM` - less common than `TERM`, but can report more granular
  information for a few terminals.
- `TERM_PROGRAM_VERSION` - used sparingly, but some terminals only gain true
  color support after a certain version.

### Overrides

Several variables can be set manually by users in order to override the color
detection behavior.

- [`CLICOLOR_FORCE`](https://bixense.com/clicolors) /
  [`FORCE_COLOR`](https://force-color.org) - two competing standards that seem
  to do the same thing. When either of these is set to a truthy value, the color
  support level will be at least ANSI 16, with other variables used to decide if
  further support is available.

  In addition to true/false values,
  [chalk](https://github.com/chalk/chalk?tab=readme-ov-file#chalklevel) supports
  using numerical values to set a specific color level. Unfortunately, this
  creates [ambiguity](https://github.com/chalk/chalk/issues/624) with
  `FORCE_COLOR=1` which could be interpreted to mean either "color level 1" or
  "true". Instead, we support semantic values to force a specific color value.

  - `no_color` - disables all colors
  - `ansi` or `ansi16` - forces ANSI 16 color
  - `ansi256` - forces ANSI 256 colors
  - `truecolor` or `24bit` - true color

  example: `CLICOLOR_FORCE="ansi256"`

  This can be useful for testing how your program works with a specific color
  support level.

- [`CLICOLOR`](https://bixense.com/clicolors) - Will enable colors if `TERM` is
  unset and the output is a terminal.

- [`NO_COLOR`](https://bixense.com/clicolors) - When set to a truthy value, this
  forces colors to be disabled.

- [`TTY_FORCE`](https://github.com/charmbracelet/colorprofile/blob/561b8ac1cff6f8c286c7dd86e95cab3875c7ac01/env.go#L130) -
  this can be set to a truthy value to treat the terminal like a TTY even if the
  call to
  [`is_terminal`](https://doc.rust-lang.org/std/io/trait.IsTerminal.html)
  returns false. May be useful when running a subprocess or in some nonstandard
  platforms such as webassembly.

### Terminfo

If the `terminfo` feature is enabled, the
[terminfo](https://en.wikipedia.org/wiki/Terminfo) database is queried for
available properties:

- `colors` - returns the number of available colors. Due to limitations with
  terminfo, true color terminals generally only report 256 colors with this
  property. `TERM` values ending in -direct (`kitty-direct` or
  `alacritty-direct`, for example) are the exception and may report color values
  \> 256 here.
- `RGB` and `Tc` - nonstandard extensions to terminfo, this is a boolean that
  may be set in some newer terminals to indicate truecolor support.

### Windows

If the `windows-version` feature is enabled, additional checks will be performed
to detect support for ANSI colors based on the active version of Windows. You
may want to enable this if support for older versions of Windows 10 (prior to
build
[#14931](https://devblogs.microsoft.com/commandline/24-bit-color-in-the-windows-console/),
released in 2016) is important to you.

### CI Platforms

Since CI platforms will render build output in an environment that's not a true
TTY, color support detection likely won't work automatically. We try to account
for some variables supplied by common CI providers, but we can't account for all
of them.

If your CI provider isn't supported, you can use `FORCE_COLOR` or
`CLICOLOR_FORCE` to force color output.

### Other Special Cases

#### Terminal multiplexers

Terminal multiplexers like GNU Screen and tmux affect the color support of the
terminal. We attempt to detect these cases properly, but it's difficult to do so
perfectly since they obscure some information from the host terminal.

Newer versions of Screen support truecolor, but there doesn't seem to be a way
to see if it's enabled, so we cannot accurately detect this case.

#### SSH

Environment variables may not be passed into your SSH session depending on your
configuration, which can cause color support to be detected incorrectly. For
best results, enable the `query-detect` feature and use a terminal that supports
[the DECRQSS query](https://github.com/termstandard/colors?tab=readme-ov-file#querying-the-terminal).

## Acknowledgements

This library takes inspiration from many other implementations:

- [charmbracelet/colorprofile](https://github.com/charmbracelet/colorprofile)
- [charmbracelet/ansi](https://github.com/charmbracelet/x/tree/main/ansi)
- [muesli/termenv](https://github.com/muesli/termenv)
- [wezterm/termwiz](https://github.com/wezterm/wezterm/tree/main/termwiz)
- [isaacs/color-support](https://github.com/isaacs/color-support)
- [chalk/supports-color](https://github.com/chalk/supports-color)
