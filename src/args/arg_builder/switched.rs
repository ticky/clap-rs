use Arg;

#[derive(Debug)]
pub struct Switched<'b> {
    pub short: Option<char>,
    pub long: Option<&'b str>,
    pub aliases: Option<Vec<(&'b str, bool)>>, // (name, visible)
    pub disp_ord: usize,
    pub unified_ord: usize,
}

impl<'e> Default for Switched<'e> {
    fn default() -> Self {
        Switched {
            short: None,
            long: None,
            aliases: None,
            disp_ord: 999,
            unified_ord: 999,
        }
    }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Switched<'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self {
        Switched {
            short: a.short,
            long: a.long,
            aliases: a.aliases.clone(),
            disp_ord: a.disp_ord,
            ..Default::default()
        }
    }
}

impl<'e> Clone for Switched<'e> {
    fn clone(&self) -> Self {
        Switched {
            short: self.short,
            long: self.long,
            aliases: self.aliases.clone(),
            disp_ord: self.disp_ord,
            unified_ord: self.unified_ord,
        }
    }
}
