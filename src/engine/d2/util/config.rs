use std::{collections::HashMap, fmt};

type ConfigSection = HashMap<String, String>;

/// An INI-like config file parser.
///  *
/// ```ini
/// ; This is a comment
/// foo = some value
/// [my section]
/// password = "  quotes are optional, and useful if you want to preserve surrounding spaces  "
/// ```
pub struct Config {
    pub main_section: ConfigSection,
    pub sections: HashMap<String, ConfigSection>,
}

// to Serialize back to an INI file. use fmt::Display
impl Config {
    pub fn new() -> Self {
        Self {
            main_section: ConfigSection::new(),
            sections: HashMap::new(),
        }
    }

    /// Parse the contents of an INI file.
    // static
    pub fn parse(text: String) -> Self {
        // let config = Config::new();

        // let commentPattern = ~/^\s*;/;
        // let sectionPattern = ~/^\s*\[\s*([^\]]*)\s*\]/;
        // let pair_pattern = ~/^\s*([\w\.\-_]+)\s*=\s*(.*)/;

        // let currentSection = config.mainSection;
        // for line in ~/\r\n|\r|\n/g.split(text) {
        //     if (commentPattern.match(line)) {
        //         // Ignore this line

        //     } else if (sectionPattern.match(line)) {
        //         let name = sectionPattern.matched(1);
        //         if (config.sections.contains_key(&name)) {
        //             // Handle duplicate sections, should this be allowed?
        //             currentSection = config.sections.get(name);
        //         } else {
        //             currentSection = ConfigSection::new();
        //             config.sections.set(name, currentSection);
        //         }

        //     } else if (pair_pattern.match(line)) {
        //         let key = pair_pattern.matched(1);
        //         let value = pair_pattern.matched(2);
        //         let quote = value.chars.nth(0).unwrap();
        //         if ((quote == '"'.to_digit(10) || quote == '\''.to_digit(10)) &&
        //                 value.chars.last() == quote) {
        //             // Trim off quotes
        //             value = value[1..value.len()-1];
        //         }
        //         currentSection.set(key, value
        //             // Unescape certain characters
        //             .replace("\\n", "\n")
        //             .replace("\\r", "\r")
        //             .replace("\\t", "\t")
        //             .replace("\\'", "\'")
        //             .replace("\\\"", "\"")
        //             .replace("\\\\", "\\")
        //         );
        //     }
        // }

        // return config;
        unimplemented!()
    }

    /// Shorthand for sections.get(name).
    #[inline]
    pub fn section(&self, name: String) -> Option<&ConfigSection> {
        self.sections.get(&name)
    }

    /// Searches for a value with a full path. A path is a section and key name separated by a dot. A
    /// path without a dot is assumed to be in the main section.
    ///  *
    /// Eg: get("foo.bar") is the same as section("foo").get("bar");
    pub fn get(&self, path: &String) -> Option<&String> {
        match path.find(".") {
            Some(idx) => {
                if let Some(section) = self.sections.get(&path[..idx]) {
                    // section.get(&path[idx + 1..].to_string())
                    todo!("should deal with it");
                } else {
                    None
                }
            }
            None => self.main_section.get(path),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for (key, value) in self.main_section.iter() {
            out.push_str(format!("{} = \"{}\"\n", key, value).as_str());
        }

        for name in self.sections.keys() {
            out.push_str(format!("[{}]\n", name).as_str());

            if let Some(section) = self.sections.get(name) {
                for (key, value) in section.iter() {
                    out.push_str(format!("{} =\"{}\"\n", key, value).as_str());
                }
            }
        }

        write!(f, "{}", out)
    }
}
