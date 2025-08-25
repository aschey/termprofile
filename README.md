# termprofile

A library to detect and handle terminal color/styling support.

Terminal environments can have several different levels of color support:

- **true color/RGB/24 bit** - any valid RGB color, many modern terminals support
  this
- **ANSI 256** - Indexed colors in the
  [256 color list](https://www.ditig.com/256-colors-cheat-sheet), most older
  terminals support this
- **ANSI 16** - Only the first 16 colors in the ANSI color list, commonly seen
  in non-graphical environments like login shells
- **No Color** - Text modifiers like bold and italics may be supported, but no
  color should be used. This is usually set by override variables.
- **No TTY** - The output is not a TTY and no escape sequences should be used.

## Usage

### Color Support Detection

```rust
use std::io::stdout;
use termprofile::TermProfile;

let profile = TermProfile::detect(&stdout());
println!("Detected profile: {profile:?}");
```

#### Overriding Variables

Variables can be overridden before detecting the color profile.

```rust
use std::io::stdout;
use termprofile::{TermProfile, TermVar, TermVars};

let mut vars = TermVars::from_env();
vars.overrides.force_color = TermVar::new("1");
let profile = TermProfile::detect_with_vars(&stdout(), vars);
println!("Profile with override: {profile:?}");
```

### Color Conversion

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

If the `ratatui` feature is enabled, all of the above conversions work with
Ratatui types.

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

Unfortunately, there is no standard way to accurately detect color support in
terminals. There is a way to
[query specifically for true color support](https://github.com/termstandard/colors?tab=readme-ov-file#querying-the-terminal),
but few terminals support this. Instead, we must rely on a number of environment
variables that have organically emerged as a pseudo-standard over time.

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
  to do the same thing. We treat both the same way. When this is set to a truthy
  value, the color support level will be at least ANSI 16, with other variables
  used to decide if further support is available.

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

- [`NO_COLOR`](https://bixense.com/clicolors) - When set to a truthy value, this
  forces colors to be disabled.

- [`TTY_FORCE`](https://github.com/charmbracelet/colorprofile/blob/561b8ac1cff6f8c286c7dd86e95cab3875c7ac01/env.go#L130) -
  this can be set to a truthy value to treat the terminal like a TTY even if the
  call to
  [`is_terminal`](https://doc.rust-lang.org/std/io/trait.IsTerminal.html)
  returns false. May be useful in some nonstandard platforms or in some
  scenarios like reading output from a subprocess.

### Windows

If the `windows-version` feature is enabled, additional checks will be performed
to detect support for ANSI colors based on the active version of Windows. You
may want to enable this if support for older versions of Windows 10 (2016 and
prior) is important to you.

### CI Platforms

Since CI platforms will render build output in an environment that's not a true
TTY, color support detection likely won't work automatically. We try to account
for some variables supplied by common CI providers, but we can't account for all
of them.

If your CI provider isn't supported, you can use `CLICOLOR_FORCE` to force color
output.

### Other Special Cases

#### Terminal multiplexers

Terminal multiplexers like GNU Screen and tmux affect the color support of the
terminal. We attempt to detect these cases properly, but it's difficult to do so
perfectly since they obscure some information from the host terminal.

#### SSH

Environment variables may not be passed into your SSH session depending on your
configuration, which can cause color support to be detected incorrectly.

## Acknowledgements

This library takes inspiration from many other implementations:

- [charmbracelet/colorprofile](https://github.com/charmbracelet/colorprofile)
- [charmbracelet/ansi](https://github.com/charmbracelet/x/tree/main/ansi)
- [muesli/termenv](https://github.com/muesli/termenv)
- [wezterm/termwiz](https://github.com/wezterm/wezterm/tree/main/termwiz)
- [isaacs/color-support](https://github.com/isaacs/color-support)
- [chalk/supports-color](https://github.com/chalk/supports-color)
