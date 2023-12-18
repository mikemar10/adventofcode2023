pub struct EngineSchematic<'a> {
    rows: Vec<&'a str>,
}

impl<'a> EngineSchematic<'a> {
    pub fn new(s: &'a str) -> Self {
        let rows: Vec<&str> = s.lines().collect();
        Self { rows }
    }

    /// Parses through each row of data, returns a vector of numbers that are adjacent to symbols
    ///
    /// Symbols exclude the '.' character
    /// Adjacency is considered in the 8 cardinal directions NW, N, NE, E, SE, S, SW, W
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
                    // When a numeric character is encountered, append it to the buffer and search for adjacent symbols
                    // Symbols are defined as not the '.' character and not a numeric character
                    (Some((idx, chr)), _) if chr.is_numeric() => {
                        buffer.push(chr);
                        // for the row above, the current row, and the next row
                        // try to grab the characters at the indices -1, 0, 1 relative to the current char
                        // this technically includes the character itself but that isn't a problem
                        // other than an extra comparison that will always fail
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
                        // Latch true a flag for an adjacent symbol found
                        symbol_adjacent = symbol_adjacent
                            || above
                                .chain(around)
                                .chain(below)
                                .any(|dir| dir != '.' && !dir.is_numeric());
                    }
                    // If the current character is non-numeric or non-existent (because we reached the end of the row)
                    (curr, next) => {
                        // append to the buffer and reset number detection state
                        if symbol_adjacent {
                            if let Ok(n) = buffer.parse() {
                                results.push(n)
                            }
                        }
                        buffer.clear();
                        symbol_adjacent = false;
                        // if we've completed the row by iterating off the end, break the loop
                        if (curr, next) == (None, None) {
                            break;
                        }
                    }
                }
            }

            // shift down a row
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
