use crate::config::get_config;
use colored::Colorize;

pub trait MaybeColorize {
    fn green(&self) -> String;
    fn red(&self) -> String;
    fn purple(&self) -> String;
}

impl MaybeColorize for &str {
    fn green(&self) -> String {
        let config = get_config();

        if config.tty {
            Colorize::green(*self).to_string()
        } else {
            self.to_string()
        }
    }

    fn red(&self) -> String {
        let config = get_config();

        if config.tty {
            Colorize::red(*self).to_string()
        } else {
            self.to_string()
        }
    }

    fn purple(&self) -> String {
        let config = get_config();

        if config.tty {
            Colorize::purple(*self).to_string()
        } else {
            self.to_string()
        }
    }
}

impl MaybeColorize for String {
    fn green(&self) -> String {
        MaybeColorize::green(&self.as_str())
    }

    fn red(&self) -> String {
        MaybeColorize::red(&self.as_str())
    }

    fn purple(&self) -> String {
        MaybeColorize::purple(&self.as_str())
    }
}

#[cfg(test)]
mod test {}