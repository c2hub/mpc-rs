//! Contains consts for ANSI color escape sequences
#![allow(dead_code)]

/// Red ANSI color
pub const RED      :&'static str = "\x1b[31m";
/// Green ANSI color
pub const GREEN    :&'static str = "\x1b[32m";
/// Yellow ANSI color
pub const YELLOW   :&'static str = "\x1b[33m";
/// Blue ANSI color
pub const BLUE     :&'static str = "\x1b[34m";
/// Magenta ANSI color
pub const MAGENTA  :&'static str = "\x1b[35m";
/// Cyan ANSI color
pub const CYAN     :&'static str = "\x1b[36m";
/// Reset to default ANSI color
pub const RESET    :&'static str = "\x1b[0m";
