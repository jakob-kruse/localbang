pub mod Shortcuts {
  use serde::Deserialize;
  use urlencoding::encode;

  #[derive(Debug, Deserialize)]
  pub struct SearchShortcut {
    keyword: String,
    url: String,
  }

  impl SearchShortcut {
    fn query_matches(&self, query: &str) -> bool {
      query.contains(&self.keyword)
    }

    fn query(&self, query: &str) -> String {
      let query = query.replace(&self.keyword, "");
      let query = query.trim();
      let encoded_query = encode(&query);
      self.url.replace("%s", encoded_query.trim())
    }
  }

  struct Shortcuts {
    pub shortcuts: Vec<Shortcuts::Shortcuts::ShortCut>,
  }

  fn read_shortcuts() -> Result<Vec<SearchShortcut>, String> {
    let mut reader = csv::Reader::from_path("shortcuts.csv").expect("Failed to read shortcuts.csv");

    let mut shortcuts = Vec::new();
    for result in reader.deserialize() {
      let shortcut: SearchShortcut = result.expect("Failed to parse shortcut");
      shortcuts.push(shortcut);
    }
    Ok(shortcuts)
  }

  pub fn find_shortcut(query: &str) -> Option<SearchShortcut> {
    let shortcuts = read_shortcuts().expect("Failed to read shortcuts");

    for shortcut in shortcuts {
      if shortcut.query_matches(query) {
        println!("Matching shortcut: {}", shortcut.keyword);
        return Some(shortcut);
      }
    }

    None
  }
}
