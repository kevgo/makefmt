use crate::makefile::Makefile;

/// removes multiple empty lines from the given Makefile content
pub fn double_empty_lines(makefile: Makefile) -> Makefile {
    makefile
}

#[cfg(test)]
mod tests {
    use crate::fixes::double_empty_lines;
    use crate::makefile::Makefile;

    #[test]
    fn normal() {
        let give = Makefile::parse("alpha: beta\n\techo one\n\nbeta:\n\techo two\n");
        let want = "alpha: beta\n\techo one\n\nbeta:\n\techo two\n";
        let have = double_empty_lines(give.into()).serialize();
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn fix_double_empty_lines() {
        let give = Makefile::parse("alpha: beta\n\techo one\n\n\nbeta:\n\techo two\n");
        let want = "alpha: beta\n\techo one\n\nbeta:\n\techo two\n";
        let have = double_empty_lines(give.into()).serialize();
        pretty::assert_eq!(have, want);
    }

    #[test]
    fn fix_triple_empty_lines() {
        let give = Makefile::parse("alpha: beta\n\techo one\n\n\n\nbeta:\n\techo two\n");
        let want = "alpha: beta\n\techo one\n\nbeta:\n\techo two\n";
        let have = double_empty_lines(give.into()).serialize();
        pretty::assert_eq!(have, want);
    }
}
