use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::replacer::Replacer;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub replacements: Vec<Replacement>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Replacement {
    Regex {
        #[serde(with = "serde_regex")]
        pattern: Regex,
        replacement: String,
        limit: usize,
    },
    #[serde(skip_serializing, skip_deserializing)]
    Replacer {
        #[serde(with = "serde_regex")]
        pattern: Regex,
        replacer: Box<dyn Replacer>,
    },
}

impl Config {
    pub fn register_callback(&mut self, pattern: Regex, cb: Box<dyn Replacer>) {
        self.replacements.push(Replacement::Replacer {
            pattern,
            replacer: cb,
        });
    }
}