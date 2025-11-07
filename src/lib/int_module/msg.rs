mod msg {
    pub fn trim(msg: &str) -> &str {
        msg.trim()
    }

    pub fn capitalize(msg: &str) -> std::borrow::Cow<'_, str> {
        if let Some(letter) = msg.get(0..1) {
            format!("{}{}", letter.to_uppercase(), &msg[1..msg.len()]).into()
        } else {
            msg.into()
        }
    }

    pub fn exciting(msg: &str) -> String {
        format!("{}!", msg)
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_all_caps() {
        let result = all_caps("test");
        let expected = String::from("TEST");
        assert_eq!(result, expected, "string should be all uppercase");
    }
}

fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

pub fn sample() {
    {
        use msg::{trim, capitalize, exciting};

        let hello = {
            let msg = "hello ";
            let msg = trim(msg);
            capitalize(msg)
        };
        let world = {
            let msg = "world";
            exciting(msg)
        };
        let msg = format!("{}, {}", hello, world);
        
        assert_eq!(&msg, "Hello, world!");
        println!("{}", msg); 
    }
}