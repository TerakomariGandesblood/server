use std::net::Ipv4Addr;

use clap::Parser;
use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Style};
use clap_verbosity_flag::Verbosity;
use supports_color::Stream;

#[derive(Parser)]
#[command(version, about, long_about = None, styles = get_styles())]
pub struct Args {
    /// 监听地址
    #[arg(short, long, default_value_t = Ipv4Addr::new(127,0,0,1))]
    pub host: Ipv4Addr,

    /// 监听端口
    #[arg(short, long, default_value_t = 8001)]
    pub port: u16,

    #[command(flatten)]
    pub verbose: Verbosity,
}

const HEADER: Style = AnsiColor::Green.on_default().bold();
const USAGE: Style = AnsiColor::Green.on_default().bold();
const LITERAL: Style = AnsiColor::Cyan.on_default().bold();
const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
const ERROR: Style = AnsiColor::Red.on_default().bold();
const VALID: Style = AnsiColor::Cyan.on_default().bold();
const INVALID: Style = AnsiColor::Yellow.on_default().bold();
const HELP_STYLES: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID);

fn get_styles() -> Styles {
    if supports_color::on(Stream::Stdout).is_some() {
        HELP_STYLES
    } else {
        Styles::plain()
    }
}
