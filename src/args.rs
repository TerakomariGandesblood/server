use std::path::PathBuf;

use clap::Parser;
use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Style};
use clap_verbosity_flag::Verbosity;
use supports_color::Stream;

#[derive(Parser)]
#[command(version, about, long_about = None, styles = get_styles())]
pub struct Args {
    /// 网站文件夹路径
    #[arg(value_parser = dir_exists)]
    pub path: PathBuf,

    /// 安装并运行服务
    #[cfg(not(windows))]
    #[arg(long, default_value_t = false)]
    pub install: bool,

    /// 停止并卸载服务
    #[cfg(not(windows))]
    #[arg(long, default_value_t = false)]
    pub uninstall: bool,

    #[command(flatten)]
    pub verbose: Verbosity,
}

fn dir_exists(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    if let Ok(exists) = path.try_exists() {
        if !exists {
            return Err(String::from("the folder does not exist"));
        }

        if !path.is_dir() {
            return Err(String::from("this path is not a folder"));
        }

        Ok(path)
    } else {
        Err(String::from("failed to check if the file exists"))
    }
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
