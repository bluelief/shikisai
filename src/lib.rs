//!Decorate text easily.
//!

pub trait Color {
    fn black(&self) -> String;
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
    fn white(&self) -> String;
    fn bold(&self) -> String;
}

impl Color for str {
    fn black(&self) -> String {
        return format!("\x1b[30m{}\x1b[0m", self);
    }
    fn red(&self) -> String {
        return format!("\x1b[31m{}\x1b[0m", self);
    }
    fn green(&self) -> String {
        return format!("\x1b[32m{}\x1b[0m", self);
    }
    fn yellow(&self) -> String {
        return format!("\x1b[33m{}\x1b[0m", self);
    }
    fn blue(&self) -> String {
        return format!("\x1b[34m{}\x1b[0m", self);
    }
    fn magenta(&self) -> String {
        return format!("\x1b[35m{}\x1b[0m", self);
    }
    fn cyan(&self) -> String {
        return format!("\x1b[36m{}\x1b[0m", self);
    }
    fn white(&self) -> String {
        return format!("\x1b[37m{}\x1b[0m", self);
    }
    fn bold(&self) -> String {
        return format!("\x1b[1m{}\x1b[0m", self);
    }
}

impl Color for String {
    fn black(&self) -> String {
        return format!("\x1b[30m{}\x1b[0m", self);
    }
    fn red(&self) -> String {
        return format!("\x1b[31m{}\x1b[0m", self);
    }
    fn green(&self) -> String {
        return format!("\x1b[32m{}\x1b[0m", self);
    }
    fn yellow(&self) -> String {
        return format!("\x1b[33m{}\x1b[0m", self);
    }
    fn blue(&self) -> String {
        return format!("\x1b[34m{}\x1b[0m", self);
    }
    fn magenta(&self) -> String {
        return format!("\x1b[35m{}\x1b[0m", self);
    }
    fn cyan(&self) -> String {
        return format!("\x1b[36m{}\x1b[0m", self);
    }
    fn white(&self) -> String {
        return format!("\x1b[37m{}\x1b[0m", self);
    }
    fn bold(&self) -> String {
        return format!("\x1b[1m{}\x1b[0m", self);
    }
}
