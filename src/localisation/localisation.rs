use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    French,
    German,
    Polski,
    Spanish,
    Italian,
    Swedish,
    Czech,
    Hungarian,
    Dutch,
    Portuguese,
    Russian,
    Finnish,
    Chinese,
}

impl Language {
    pub fn csv_header(&self) -> &'static str {
        match self {
            Language::English => "ENGLISH",
            Language::French => "FRENCH",
            Language::German => "GERMAN",
            Language::Polski => "POLSKI",
            Language::Spanish => "SPANISH",
            Language::Italian => "ITALIAN",
            Language::Swedish => "SWEDISH",
            Language::Czech => "CZECH",
            Language::Hungarian => "HUNGARIAN",
            Language::Dutch => "DUTCH",
            Language::Portuguese => "PORTUGUESE",
            Language::Russian => "RUSSIAN",
            Language::Finnish => "FINNISH",
            Language::Chinese => "CHINESE",
        }
    }

    pub fn translation_key(&self) -> &'static str {
        match self {
            Language::English => "lang_english",
            Language::French => "lang_french",
            Language::German => "lang_german",
            Language::Polski => "lang_polski",
            Language::Spanish => "lang_spanish",
            Language::Italian => "lang_italian",
            Language::Swedish => "lang_swedish",
            Language::Czech => "lang_czech",
            Language::Hungarian => "lang_hungarian",
            Language::Dutch => "lang_dutch",
            Language::Portuguese => "lang_portuguese",
            Language::Russian => "lang_russian",
            Language::Finnish => "lang_finnish",
            Language::Chinese => "lang_chinese",
        }
    }

    pub fn all() -> Vec<Language> {
        vec![
            Language::English,
            Language::French,
            Language::German,
            Language::Polski,
            Language::Spanish,
            Language::Italian,
            Language::Swedish,
            Language::Czech,
            Language::Hungarian,
            Language::Dutch,
            Language::Portuguese,
            Language::Russian,
            Language::Finnish,
            Language::Chinese,
        ]
    }
}

pub struct Localisation {
    current_language: Language,
    translations: HashMap<String, String>,
}

impl Localisation {
    pub fn new(language: Language) -> Result<Self, Box<dyn Error>> {
        let mut loc = Self {
            current_language: language,
            translations: HashMap::new(),
        };
        loc.load_translations()?;
        Ok(loc)
    }

    pub fn load_translations(&mut self) -> Result<(), Box<dyn Error>> {
        const CSV_DATA: &str = include_str!("../../assets/localisation.csv");

        let mut reader = ReaderBuilder::new()
            .delimiter(b';')
            .from_reader(CSV_DATA.as_bytes());

        let headers = reader.headers()?.clone();
        let lang_index = headers
            .iter()
            .position(|h| h == self.current_language.csv_header())
            .ok_or(format!(
                "Language '{}' not found",
                self.get_language_name(self.current_language)
            ))?;

        self.translations.clear();
        for result in reader.records() {
            let record = result?;
            let key = record.get(0).unwrap_or("").to_string();
            let value = record.get(lang_index).unwrap_or("").to_string();
            self.translations.insert(key, value);
        }

        Ok(())
    }

    pub fn set_language(&mut self, language: Language) -> Result<(), Box<dyn Error>> {
        self.current_language = language;
        self.load_translations()
    }

    pub fn get<'a>(&'a self, key: &'a str) -> &'a str {
        self.translations
            .get(key)
            .map(|s| s.as_str())
            .unwrap_or(key)
    }

    pub fn get_language_name(&self, language: Language) -> &str {
        self.get(language.translation_key())
    }

    pub fn title(&self) -> &str {
        self.get("title")
    }

    pub fn font_settings_title(&self) -> &str {
        self.get("font_settings_title")
    }

    pub fn add_font_label(&self) -> &str {
        self.get("add_font_label")
    }

    pub fn size_label(&self) -> &str {
        self.get("size_label")
    }

    pub fn size_suffix(&self) -> &str {
        self.get("size_suffix")
    }

    pub fn select_font_button(&self) -> &str {
        self.get("select_font_button")
    }

    pub fn configured_fonts_label(&self) -> &str {
        self.get("configured_fonts_label")
    }

    pub fn no_fonts_label(&self) -> &str {
        self.get("no_fonts_label")
    }

    pub fn generate_atlas_button(&self) -> &str {
        self.get("generate_atlas_button")
    }

    pub fn delete_button(&self) -> &str {
        self.get("delete_button")
    }

    pub fn open_button(&self) -> &str {
        self.get("open_button")
    }

    pub fn save_button(&self) -> &str {
        self.get("save_button")
    }

    pub fn save_as_button(&self) -> &str {
        self.get("save_as_button")
    }

    pub fn language_label(&self) -> &str {
        self.get("language_label")
    }

    pub fn preview_panel_title(&self) -> &str {
        self.get("preview_panel_title")
    }

    pub fn no_file_label(&self) -> &str {
        self.get("no_file_label")
    }
}
