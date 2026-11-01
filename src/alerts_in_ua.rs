#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompactStatus {
    NoAlert,
    Red,
    Yellow,
    Partial,
    NoData,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidCode(char),
    WrongOblastCount { expected: usize, actual: usize },
}

pub const OBLAST_ORDER_UK: [&str; 27] = [
    "Автономна Республіка Крим",
    "Волинська область",
    "Вінницька область",
    "Дніпропетровська область",
    "Донецька область",
    "Житомирська область",
    "Закарпатська область",
    "Запорізька область",
    "Івано-Франківська область",
    "м. Київ",
    "Київська область",
    "Кіровоградська область",
    "Луганська область",
    "Львівська область",
    "Миколаївська область",
    "Одеська область",
    "Полтавська область",
    "Рівненська область",
    "м. Севастополь",
    "Сумська область",
    "Тернопільська область",
    "Харківська область",
    "Херсонська область",
    "Хмельницька область",
    "Черкаська область",
    "Чернівецька область",
    "Чернігівська область",
];

fn parse_code(code: char) -> Result<CompactStatus, ParseError> {
    match code {
        'N' => Ok(CompactStatus::NoAlert),
        'R' => Ok(CompactStatus::Red),
        'Y' => Ok(CompactStatus::Yellow),
        'P' => Ok(CompactStatus::Partial),
        ' ' => Ok(CompactStatus::NoData),
        other => Err(ParseError::InvalidCode(other)),
    }
}

pub fn parse_uid_compact(value: &str) -> Result<Vec<CompactStatus>, ParseError> {
    value.chars().map(parse_code).collect()
}

pub fn parse_oblast_v2(value: &str) -> Result<Vec<(&'static str, CompactStatus)>, ParseError> {
    let codes = parse_uid_compact(value)?;
    if codes.len() != OBLAST_ORDER_UK.len() {
        return Err(ParseError::WrongOblastCount {
            expected: OBLAST_ORDER_UK.len(),
            actual: codes.len(),
        });
    }

    Ok(OBLAST_ORDER_UK
        .iter()
        .copied()
        .zip(codes)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_documented_v2_oblast_example() {
        let parsed = parse_oblast_v2("RNNNNNNNNYNNRNNNNNNNNNNNNNN").unwrap();

        assert_eq!(parsed.len(), 27);
        assert_eq!(parsed[0], ("Автономна Республіка Крим", CompactStatus::Red));
        assert_eq!(parsed[9], ("м. Київ", CompactStatus::Yellow));
        assert_eq!(parsed[12], ("Луганська область", CompactStatus::Red));
        assert_eq!(parsed[13].0, "Львівська область");
    }

    #[test]
    fn distinguishes_partial_and_no_data() {
        let parsed = parse_uid_compact("NP YR").unwrap();
        assert_eq!(
            parsed,
            vec![
                CompactStatus::NoAlert,
                CompactStatus::Partial,
                CompactStatus::NoData,
                CompactStatus::Yellow,
                CompactStatus::Red,
            ]
        );
    }

    #[test]
    fn rejects_unknown_status_code() {
        assert_eq!(parse_uid_compact("NX"), Err(ParseError::InvalidCode('X')));
    }

    #[test]
    fn oblast_endpoint_requires_exactly_27_positions() {
        assert_eq!(
            parse_oblast_v2("R"),
            Err(ParseError::WrongOblastCount {
                expected: 27,
                actual: 1,
            })
        );
    }

    #[test]
    fn parser_does_not_create_official_events() {
        let parsed = parse_uid_compact("N").unwrap();
        assert_eq!(parsed, vec![CompactStatus::NoAlert]);
        // CompactStatus intentionally has no conversion to OfficialEvent:
        // aggregator observations must pass the source trust boundary first.
    }
}
