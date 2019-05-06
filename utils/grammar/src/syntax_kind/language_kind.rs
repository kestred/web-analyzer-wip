use rowan::SyntaxKind;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LanguageKind(pub u8);

impl LanguageKind {
    const PREFIX_BITS: u8 = 5;
    const SYNTAX_BITS: u8 = 16 - LanguageKind::PREFIX_BITS;

    pub const MAX: u8 = (1 << LanguageKind::PREFIX_BITS) - 1;

    pub const fn syntax_kind(self, suffix: u16) -> SyntaxKind {
        let prefix = (self.0 as u16) << LanguageKind::SYNTAX_BITS;
        SyntaxKind(prefix | suffix)
    }
}

impl From<SyntaxKind> for LanguageKind {
    fn from(s: SyntaxKind) -> LanguageKind {
        LanguageKind((s.0 >> LanguageKind::SYNTAX_BITS) as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_kind() {
        let l = LanguageKind(2);
        let k = l.syntax_kind(1234);
        assert_eq!(k.0, 4096 + 1234);
        let o =  LanguageKind::from(k);
        assert_eq!(o, l);
    }
}