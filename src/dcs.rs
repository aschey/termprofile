use std::io;
use std::time::Duration;

use termina::Terminal;
use termina::escape::csi::{Csi, Device, Sgr};
use termina::escape::dcs::{Dcs, DcsRequest, DcsResponse};
use termina::style::{ColorSpec, RgbColor};

use crate::detect::Event;
use crate::{
    DUMB, DetectorSettings, IsTerminal, QueryTerminal, Rgb, SCREEN, TMUX, TTY_FORCE, TermVar,
    VariableSource, prefix_or_equal,
};

impl DetectorSettings<DefaultTerminal> {
    pub fn with_dcs() -> io::Result<Self> {
        Ok(Self {
            enable_dcs: true,
            enable_terminfo: true,
            enable_tmux_info: true,
            query_terminal: DefaultTerminal::new()?,
        })
    }
}

pub struct DefaultTerminal {
    terminal: termina::PlatformTerminal,
    timeout: Duration,
}

impl DefaultTerminal {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            terminal: termina::PlatformTerminal::new()?,
            timeout: std::time::Duration::from_millis(100),
        })
    }

    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

impl io::Write for DefaultTerminal {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.terminal.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.terminal.flush()
    }
}

impl QueryTerminal for DefaultTerminal {
    fn setup(&mut self) -> io::Result<()> {
        self.terminal.enter_raw_mode()
    }

    fn cleanup(&mut self) -> io::Result<()> {
        self.terminal.enter_cooked_mode()
    }

    fn read_event(&mut self) -> io::Result<Event> {
        if !self
            .terminal
            .poll(termina::Event::is_escape, self.timeout.into())?
        {
            return Ok(Event::TimedOut);
        }
        let event = self.terminal.read(termina::Event::is_escape)?;
        Ok(match event {
            termina::Event::Dcs(Dcs::Response {
                value: DcsResponse::GraphicRendition(sgrs),
                ..
            }) => sgrs
                .iter()
                .find_map(|s| {
                    if let Sgr::Background(ColorSpec::TrueColor(rgb)) = s {
                        Event::BackgroundColor(Rgb {
                            red: rgb.red,
                            green: rgb.green,
                            blue: rgb.blue,
                        })
                        .into()
                    } else {
                        None
                    }
                })
                .unwrap_or(Event::Other),
            termina::Event::Csi(Csi::Device(Device::DeviceAttributes(()))) => {
                Event::DeviceAttributes
            }
            _ => Event::Other,
        })
    }
}

pub(crate) fn dcs_detect<S, Q, T>(
    source: &S,
    out: &T,
    query_terminal: &mut Q,
    term: &str,
) -> io::Result<bool>
where
    S: VariableSource,
    Q: QueryTerminal,
    T: IsTerminal,
{
    const TEST_COLOR: Rgb = Rgb {
        red: 150,
        green: 150,
        blue: 150,
    };
    let tty_force = TermVar::from_source(source, TTY_FORCE);
    // Screen and tmux don't support this sequence
    if (!out.is_terminal() && !tty_force.is_truthy())
        || term == DUMB
        || prefix_or_equal(term, TMUX)
        || !TermVar::from_source(source, &TMUX.to_ascii_uppercase()).is_empty()
        || prefix_or_equal(term, SCREEN)
    {
        return Ok(false);
    }

    query_terminal.setup()?;
    write!(
        query_terminal,
        "{}{}{}{}",
        Csi::Sgr(Sgr::Background(ColorSpec::TrueColor(
            RgbColor {
                red: TEST_COLOR.red,
                green: TEST_COLOR.green,
                blue: TEST_COLOR.blue
            }
            .into()
        ))),
        Dcs::Request(DcsRequest::GraphicRendition),
        Csi::Sgr(Sgr::Reset),
        Csi::Device(Device::RequestPrimaryDeviceAttributes),
    )?;
    query_terminal.flush()?;

    let mut true_color = false;
    loop {
        let event = query_terminal.read_event()?;

        match event {
            Event::TimedOut => {
                return Ok(false);
            }
            Event::BackgroundColor(rgb) => {
                true_color = rgb == TEST_COLOR;
            }
            Event::DeviceAttributes => {
                break;
            }
            Event::Other => {}
        }
    }
    query_terminal.cleanup()?;
    Ok(true_color)
}
