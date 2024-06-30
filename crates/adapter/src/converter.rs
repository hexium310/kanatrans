use anyhow::Result;

pub trait Converter {
    fn convert(&self, pronunciation: &[&str]) -> Result<String>;
}

// Uses mock! instead of automock because Clippy warns needles_lifetimes without tests
#[cfg(test)]
mockall::mock! {
    pub Converter {}
    impl Converter for Converter {
        fn convert<'a>(&self, pronunciation: &[&'a str]) -> Result<String>;
    }
}
