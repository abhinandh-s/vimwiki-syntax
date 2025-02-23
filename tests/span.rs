#[cfg(test)]
mod test {
    use neorg_syntax::span::Span;
    use tower_lsp::lsp_types::Range;

    #[test]
    fn try_into_lsp_range_test_zero_based_01() {
        let input = "this is a string";
        let span = Span::new(0, input.len());
        assert_eq!(
            span.into_zero_based_lsp_range(input).unwrap(),
            Range {
                start: tower_lsp::lsp_types::Position {
                    line: 0,
                    character: 0
                },
                end: tower_lsp::lsp_types::Position {
                    line: 0,
                    character: 16
                }
            }
        )
    }

    #[test]
    fn try_into_lsp_range_test_zero_based_02() {
        let input = "this is a string \n and a newline";
        let span = Span::new(0, input.len());
        assert_eq!(
            span.into_zero_based_lsp_range(input).unwrap(),
            Range {
                start: tower_lsp::lsp_types::Position {
                    line: 0,
                    character: 0
                },
                end: tower_lsp::lsp_types::Position {
                    line: 1,
                    character: 14
                }
            }
        )
    }

    #[test]
    fn into_lsp_range_test_01() {
        let input = "this is a string";
        let span = Span::new(0, input.len());
        assert_eq!(
            span.into_lsp_range(input).unwrap(),
            Range {
                start: tower_lsp::lsp_types::Position {
                    line: 1,
                    character: 1
                },
                end: tower_lsp::lsp_types::Position {
                    line: 1,
                    character: 16
                }
            }
        )
    }

    #[test]
    fn into_lsp_range_test_02() {
        let input = "this is a string \n and a newline";
        let span = Span::new(0, input.len());
        assert_eq!(
            span.into_lsp_range(input).unwrap(),
            Range {
                start: tower_lsp::lsp_types::Position {
                    line: 1,
                    character: 1
                },
                end: tower_lsp::lsp_types::Position {
                    line: 2,
                    character: 14
                }
            }
        )
    }
}
