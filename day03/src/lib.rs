pub struct EngineSchematic<'a> {
    rows: Vec<&'a str>,
}

impl<'a> EngineSchematic<'a> {
    pub fn new(s: &'a str) -> Self {
        let rows: Vec<&str> = s.lines().collect();
        Self { rows }
    }

    pub fn find_engine_parts(&self) -> Vec<usize> {
        let mut results: Vec<usize> = vec![];
        let mut buffer = String::new();
        let mut rows = self.rows.iter();
        let mut prev_row = None;
        let mut curr_row = rows.next();
        let mut next_row = rows.next();
        let mut symbol_adjacent = false;
        while curr_row.is_some() {
            let mut cols = curr_row.unwrap().char_indices().peekable();
            loop {
                match (cols.next(), cols.peek()) {
                    (Some((idx, chr)), _) if chr.is_numeric() => {
                        buffer.push(chr);
                        let above = prev_row
                            .get_or_insert(&"")
                            .chars()
                            .skip(idx.saturating_sub(1))
                            .take(3);
                        let around = curr_row
                            .get_or_insert(&"")
                            .chars()
                            .skip(idx.saturating_sub(1))
                            .take(3);
                        let below = next_row
                            .get_or_insert(&"")
                            .chars()
                            .skip(idx.saturating_sub(1))
                            .take(3);
                        symbol_adjacent = symbol_adjacent
                            || above
                                .chain(around)
                                .chain(below)
                                .any(|dir| dir != '.' && !dir.is_numeric());
                    }
                    (curr, next) => {
                        if symbol_adjacent {
                            if let Ok(n) = buffer.parse() {
                                results.push(n)
                            }
                        }
                        buffer.clear();
                        symbol_adjacent = false;
                        if (curr, next) == (None, None) {
                            break;
                        }
                    }
                }
            }

            prev_row = curr_row;
            curr_row = next_row;
            next_row = rows.next();
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_engine_parts() {
        let input = "467..114..\n\
                     ...*......\n\
                     ..35..633.\n\
                     ......#...\n\
                     617*......\n\
                     .....+.58.\n\
                     ..592.....\n\
                     ......755.\n\
                     ...$.*....\n\
                     .664.598..";
        let schematic = EngineSchematic::new(input);
        assert_eq!(schematic.find_engine_parts().iter().sum::<usize>(), 4361);
    }
}
