pub trait IntoBaseExt {
    fn into_base(&self) -> String;
}

impl IntoBaseExt for str {
    fn into_base(&self) -> String {
        let mut s = self;
        if s.starts_with("$") {
            s = &s[1..];
        }
        s = &s[1..(s.len() - 1)];
        s.to_string()
    }
}
