#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KatottgStagingRow {
    /// First, second, third, fourth and additional level as published by the source.
    pub levels: [Option<String>; 5],
    /// Preserved as source data. No automatic semantic mapping is performed here.
    pub category: String,
    pub name_uk: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KatottgRowError {
    MissingFirstLevel,
    GapInLevels,
    InvalidCode,
    BlankCategory,
    BlankName,
}

/// Current accepted source shape (2026-11-01): "UA" + 17 ASCII digits.
/// If the Ministry changes the code shape, ingestion should fail closed until
/// a reviewed importer revision explicitly accepts the new format.
fn valid_source_code(code: &str) -> bool {
    let value = code.trim();
    value.len() == 19
        && value.starts_with("UA")
        && value[2..].chars().all(|c| c.is_ascii_digit())
}

impl KatottgStagingRow {
    pub fn validate(&self) -> Result<(), KatottgRowError> {
        if self.name_uk.trim().is_empty() {
            return Err(KatottgRowError::BlankName);
        }
        if self.category.trim().is_empty() {
            return Err(KatottgRowError::BlankCategory);
        }

        if self.levels[0]
            .as_deref()
            .map(str::trim)
            .unwrap_or("")
            .is_empty()
        {
            return Err(KatottgRowError::MissingFirstLevel);
        }

        let mut saw_gap = false;
        for level in &self.levels {
            match level.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
                Some(code) => {
                    if saw_gap {
                        return Err(KatottgRowError::GapInLevels);
                    }
                    if !valid_source_code(code) {
                        return Err(KatottgRowError::InvalidCode);
                    }
                }
                None => saw_gap = true,
            }
        }

        Ok(())
    }

    pub fn deepest_code(&self) -> Result<&str, KatottgRowError> {
        self.validate()?;
        self.levels
            .iter()
            .rev()
            .find_map(|value| {
                value
                    .as_deref()
                    .map(str::trim)
                    .filter(|v| !v.is_empty())
            })
            .ok_or(KatottgRowError::MissingFirstLevel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(levels: [&str; 5], category: &str, name: &str) -> KatottgStagingRow {
        KatottgStagingRow {
            levels: levels.map(|v| {
                let trimmed = v.trim();
                (!trimmed.is_empty()).then(|| trimmed.to_owned())
            }),
            category: category.into(),
            name_uk: name.into(),
        }
    }

    #[test]
    fn accepts_published_four_level_shape_without_interpreting_category() {
        let value = row(
            [
                "UA01000000000013043",
                "UA01020000000022387",
                "UA01020010000048857",
                "UA01020010010075540",
                "",
            ],
            "C",
            "Андріївка",
        );

        assert_eq!(value.validate(), Ok(()));
        assert_eq!(value.deepest_code(), Ok("UA01020010010075540"));
        assert_eq!(value.category, "C");
    }

    #[test]
    fn accepts_published_additional_level_shape() {
        let value = row(
            [
                "UA14000000000091971",
                "UA14060000000024357",
                "UA14060030000084305",
                "UA14060030010082815",
                "UA14060030010124146",
            ],
            "B",
            "Кіндратівський",
        );

        assert_eq!(value.validate(), Ok(()));
        assert_eq!(value.deepest_code(), Ok("UA14060030010124146"));
        assert_eq!(value.category, "B");
    }

    #[test]
    fn rejects_hierarchy_gap() {
        let value = row(
            [
                "UA01000000000013043",
                "",
                "UA01020010000048857",
                "",
                "",
            ],
            "H",
            "Андріївська",
        );

        assert_eq!(value.validate(), Err(KatottgRowError::GapInLevels));
    }

    #[test]
    fn rejects_non_ua_non_numeric_short_or_long_code() {
        let bad_prefix = row(["XX01000000000013043", "", "", "", ""], "O", "Тест");
        let bad_digits = row(["UA01ABC00000013043", "", "", "", ""], "O", "Тест");
        let short = row(["UA123", "", "", "", ""], "O", "Тест");
        let long = row(["UA010000000000130430", "", "", "", ""], "O", "Тест");

        assert_eq!(bad_prefix.validate(), Err(KatottgRowError::InvalidCode));
        assert_eq!(bad_digits.validate(), Err(KatottgRowError::InvalidCode));
        assert_eq!(short.validate(), Err(KatottgRowError::InvalidCode));
        assert_eq!(long.validate(), Err(KatottgRowError::InvalidCode));
    }

    #[test]
    fn category_is_preserved_as_opaque_source_value() {
        let value = row(
            ["UA01000000000013043", "", "", "", ""],
            "FUTURE",
            "Тест",
        );

        assert_eq!(value.validate(), Ok(()));
        assert_eq!(value.category, "FUTURE");
    }

    #[test]
    fn rejects_blank_name_and_category() {
        assert_eq!(
            row(
                ["UA01000000000013043", "", "", "", ""],
                "",
                "Тест"
            )
            .validate(),
            Err(KatottgRowError::BlankCategory)
        );
        assert_eq!(
            row(
                ["UA01000000000013043", "", "", "", ""],
                "O",
                " "
            )
            .validate(),
            Err(KatottgRowError::BlankName)
        );
    }
}
