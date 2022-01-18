pub struct Fast;

/// Utility mixins for xml.Fast objects. Designed to be imported with 'using'.
pub struct Xmls {}

impl Xmls {
    // static
    pub fn string_attr(&self, reader: Fast, attr: String, default: Option<String>) -> Option<String> {
        todo!("should implement xml logic");
        // if reader.has.resolve(attr) {
        //     return Some(reader.att.resolve(attr));
        // }

        // default
    }

    // static
    pub fn float_attr(&self, reader: Fast, attr: String, default: Option<f32>) -> Option<f32> {
        todo!("should implement xml logic");
        // if reader.has.resolve(attr) {
        //     let s: String = reader.att.resolve(attr);
        //     let value = s.parse::<f32>().unwrap();
        //     if let Ok(value) = s.parse::<f32>() {
        //         return Some(value);
        //     }
        // }

        // default
    }

    // static
    pub fn int_attr(&self, reader: Fast, attr: String, default: Option<i32>) -> Option<i32> {
        todo!("should implement xml logic");
        // if reader.has.resolve(attr) {
        //     let s: String = reader.att.resolve(attr);
        //     if let Ok(value) = s.parse::<i32>() {
        //         return Some(value);
        //     }
        // }

        // default
    }
}
