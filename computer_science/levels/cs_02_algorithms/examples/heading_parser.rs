#[derive(Debug, PartialEq, Eq)]
struct Heading<'a> {
    level: usize,
    title: &'a str,
}

fn parse_headings(markdown: &str) -> Vec<Heading<'_>> {
    markdown
        .lines()
        .filter_map(|line| {
            let hashes = line.chars().take_while(|ch| *ch == '#').count();
            if hashes == 0 || hashes > 6 {
                return None;
            }

            let rest = &line[hashes..];
            let title = rest.strip_prefix(' ')?;
            if title.is_empty() {
                return None;
            }

            Some(Heading {
                level: hashes,
                title,
            })
        })
        .collect()
}

fn main() {
    let markdown = "# Title\ntext\n## Section\n### Child\n#### Details\n";

    for heading in parse_headings(markdown) {
        println!("{} {}", heading.level, heading.title);
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_headings, Heading};

    #[test]
    fn parses_headings() {
        let markdown = "# Title\ntext\n## Section\n### Child\n";
        assert_eq!(
            parse_headings(markdown),
            vec![
                Heading {
                    level: 1,
                    title: "Title"
                },
                Heading {
                    level: 2,
                    title: "Section"
                },
                Heading {
                    level: 3,
                    title: "Child"
                }
            ]
        );
    }
}
