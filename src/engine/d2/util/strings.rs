use std::fmt;

/// Utility mixins for Strings. Designed to be imported with 'using'.
pub trait StringExtensions {
    /// Gets the extension of a file name, or None if there is no extension
    fn file_extension(&self) -> Option<String>;

    /// Returns a file name without its extension
    fn remove_file_extension(&self) -> String;

    /// Gets the extension of a full path or URL, with special handling for '/' and '?' characters.
    /// Returns None if there is no extension
    fn url_extension(&self) -> Option<String>;

    /// Joins two strings with a path separator
    fn join_path(&self, relative: String) -> String;

    fn hash_code<S: Into<String>>(&self, str: S) -> u32;

    /// Substitute all "{n}" tokens with the corresponding values.
    ///
    // ```
    // "{1} sat on a {0}".substitute(["wall", "Humpty Dumpty"]);
    // // returns "Humpty Dumpty sat on a wall"
    // ```
    //
    fn substitute(&self, values: Vec<String>) -> String;
}

impl StringExtensions for String {
    fn file_extension(&self) -> Option<String> {
        if let Some(dot) = self.rfind(".") {
            Some(self[dot + 1..].to_string())
        } else {
            None
        }
    }

    fn remove_file_extension(&self) -> String {
        if let Some(dot) = self.rfind(".") {
            self[..dot].to_string()
        } else {
            self.clone()
        }
    }

    fn url_extension(&self) -> Option<String> {
        let mut out = self.clone();
        let question = self.rfind("?");
        if let Some(question) = self.rfind("?") {
            out = out[..question].to_string();
        }

        if let Some(slash) = self.rfind("/") {
            out = out[slash + 1..].to_string();
        }

        out.file_extension()
    }

    fn join_path(&self, relative: String) -> String {
        const SEP: char = '/'; // std::path::MAIN_SEPARATOR
        let mut out = self.clone();
        if let Some(last) = self.chars().last() {
            if last != SEP {
                out.push(SEP)
            }
        }

        format!("{}{}", out, relative)
    }

    fn hash_code<S: Into<String>>(&self, str: S) -> u32 {
        let mut code = 0;
        let str: String = str.into();
        if !str.is_empty() {
            let chars = str.chars(); //.nth(i).unwrap()
            for ch in str.chars() {
                code = 31 * code + ch.to_digit(10).unwrap();
            }
        }

        code
    }

    fn substitute(&self, values: Vec<String>) -> String {
        // FIXME -> If your {0} replacement has a {1} in it, then that'll get replaced next
        // iteration
        let mut out = String::from("");
        for ii in 0..values.len() {
            out = self.replace(format!("{{{}}}", ii).as_str(), values[ii].as_str());
        }

        out
    }
}

// TODO: should remade it
impl StringExtensions for str {
    fn file_extension(&self) -> Option<String> {
        if let Some(dot) = self.rfind(".") {
            Some(self[dot + 1..].to_string())
        } else {
            None
        }
    }

    fn remove_file_extension(&self) -> String {
        if let Some(dot) = self.rfind(".") {
            self[..dot].to_string()
        } else {
            self.to_owned()
        }
    }

    // FIXME: what did it mean. i need tests for this function
    fn url_extension(&self) -> Option<String> {
        let mut out = self.to_string();

        if let Some(question) = out.rfind("?") {
            out = out[..question].to_string();
        }

        if let Some(slash) = out.rfind("/") {
            out = out[slash + 1..].to_string()
        }

        out.file_extension()
    }

    fn join_path(&self, relative: String) -> String {
        const SEP: char = '/'; // std::path::MAIN_SEPARATOR
        let mut out = self.to_string();
        if let Some(last) = self.chars().last() {
            if last != SEP {
                out.push(SEP)
            }
        }

        format!("{}{}", out, relative)
    }

    fn hash_code<S: Into<String>>(&self, str: S) -> u32 {
        let mut code = 0;
        let str: String = str.into();
        if !str.is_empty() {
            let chars = str.chars(); //.nth(i).unwrap()
            for ch in str.chars() {
                code = 31 * code + ch.to_digit(10).unwrap();
            }
        }

        code
    }

    fn substitute(&self, values: Vec<String>) -> String {
        // FIXME -> If your {0} replacement has a {1} in it, then that'll get replaced next
        // iteration
        let mut out = String::from("");
        for ii in 0..values.len() {
            out = out.replace(format!("{{{}}}", ii).as_str(), values[ii].as_str());
        }

        out
    }
}
