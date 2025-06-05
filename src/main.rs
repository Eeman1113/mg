use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use colored::*;
use regex::Regex;

//======================================================================
// PART 1: SPELLER LOGIC
// Integrated from the `spellcheck` library (src/lib.rs)
//======================================================================

pub struct Speller {
    pub letters: String,
    pub n_words: HashMap<String, u32>,
}

impl Speller {
    /// Creates a new, untrained Speller instance.
    pub fn new() -> Self {
        Speller {
            letters: "abcdefghijklmnopqrstuvwxyz".to_string(),
            n_words: HashMap::new(),
        }
    }

    /// Trains the language model with the words in the supplied text.
    pub fn train(&mut self, text: &str) {
        let re = Regex::new(r"[a-z]+").unwrap();
        let lc_text = text.to_lowercase();
        for m in re.find_iter(&lc_text) {
            let count = self.n_words.entry(m.as_str().to_string()).or_insert(0);
            *count += 1;
        }
    }

    /// Returns the set of possible corrections of the specified word.
    fn edits(&mut self, word: &str) -> Vec<String> {
        let mut results = Vec::new();
        // Deletion
        for i in 0..word.len() {
            let (first, last) = word.split_at(i);
            results.push([first, &last[1..]].concat());
        }
        // Transposition
        for i in 0..word.len() - 1 {
            let (first, last) = word.split_at(i);
            results.push([first, &last[1..2], &last[..1], &last[2..]].concat());
        }
        // Alteration
        for i in 0..word.len() {
            for c in self.letters.chars() {
                let (first, last) = word.split_at(i);
                let mut buffer = [0; 1];
                let result = c.encode_utf8(&mut buffer);
                results.push([first, result, &last[1..]].concat());
            }
        }
        // Insertion
        for i in 0..word.len() + 1 {
            for c in self.letters.chars() {
                let (first, last) = word.split_at(i);
                let mut buffer = [0; 1];
                let result = c.encode_utf8(&mut buffer);
                results.push([first, result, last].concat());
            }
        }
        results
    }

    /// Returns the correction for the specified word.
    pub fn correct(&mut self, word: &str) -> String {
        if self.n_words.contains_key(word) {
            return word.to_string();
        }

        let mut candidates: HashMap<u32, String> = HashMap::new();
        let list = self.edits(word);

        for edit in &list {
            if let Some(value) = self.n_words.get(edit) {
                candidates.insert(*value, edit.to_string());
            }
        }
        if let Some(c) = candidates.iter().max_by_key(|&entry| entry.0) {
            return c.1.to_string();
        }

        for edit in &list {
            for w in self.edits(&edit) {
                if let Some(value) = self.n_words.get(&w) {
                    candidates.insert(*value, w);
                }
            }
        }
        if let Some(c) = candidates.iter().max_by_key(|&entry| entry.0) {
            return c.1.to_string();
        }

        word.to_string()
    }
}


//======================================================================
// PART 2: WRITING CHECKER LOGIC
// From the original `writing-checker` project, now with spelling.
//======================================================================

#[derive(Debug)]
struct Match {
    line_num: usize,
    line: String,
    matched_text: String,
    file: String,
}

struct WritingChecker {
    weasel_words: Vec<String>,
    passive_irregulars: Vec<String>,
}

impl WritingChecker {
    fn new() -> Self {
        let default_weasels = vec![
            "many", "various", "very", "fairly", "several", "extremely",
            "exceedingly", "quite", "remarkably", "few", "surprisingly",
            "mostly", "largely", "huge", "tiny", "are a number", "is a number",
            "excellent", "interestingly", "significantly", "substantially",
            "clearly", "vast", "relatively", "completely"
        ];
        let irregulars = vec![
            "awoken", "been", "born", "beat", "become", "begun", "bent", "beset", "bet", "bid", "bidden", "bound", "bitten", "bled", "blown", "broken", "bred", "brought", "broadcast", "built", "burnt", "burst", "bought", "cast", "caught", "chosen", "clung", "come", "cost", "crept", "cut", "dealt", "dug", "dived", "done", "drawn", "dreamt", "driven", "drunk", "eaten", "fallen", "fed", "felt", "fought", "found", "fit", "fled", "flung", "flown", "forbidden", "forgotten", "foregone", "forgiven", "forsaken", "frozen", "gotten", "given", "gone", "ground", "grown", "hung", "heard", "hidden", "hit", "held", "hurt", "kept", "knelt", "knit", "known", "laid", "led", "leapt", "learnt", "left", "lent", "let", "lain", "lighted", "lost", "made", "meant", "met", "misspelt", "mistaken", "mown", "overcome", "overdone", "overtaken", "overthrown", "paid", "pled", "proven", "put", "quit", "read", "rid", "ridden", "rung", "risen", "run", "sawn", "said", "seen", "sought", "sold", "sent", "set", "sewn", "shaken", "shaven", "shorn", "shed", "shone", "shod", "shot", "shown", "shrunk", "shut", "sung", "sunk", "sat", "slept", "slain", "slid", "slung", "slit", "smitten", "sown", "spoken", "sped", "spent", "spilt", "spun", "spit", "split", "spread", "sprung", "stood", "stolen", "stuck", "stung", "stunk", "stridden", "struck", "strung", "striven", "sworn", "swept", "swollen", "swum", "swung", "taken", "taught", "torn", "told", "thought", "thrived", "thrown", "thrust", "trodden", "understood", "upheld", "upset", "woken", "worn", "woven", "wed", "wept", "wound", "won", "withheld", "withstood", "wrung", "written"
        ];
        WritingChecker {
            weasel_words: default_weasels.iter().map(|s| s.to_string()).collect(),
            passive_irregulars: irregulars.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn load_custom_weasels(&mut self) {
        let paths = vec![
            format!("{}/.config/writing-checker/weasels", env::var("HOME").unwrap_or_default()),
            "weasels.txt".to_string(),
        ];

        for path in paths {
            if Path::new(&path).exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    self.weasel_words = content
                        .lines()
                        .filter(|line| !line.trim().is_empty())
                        .map(|line| line.trim().to_string())
                        .collect();
                    println!("Loaded custom weasel words from: {}", path);
                    break;
                }
            }
        }
    }

    fn check_weasel_words(&self, content: &str, filename: &str) -> Vec<Match> {
        let mut matches = Vec::new();
        let pattern = format!(r"\b({})\b", self.weasel_words.join("|"));
        let re = Regex::new(&pattern).unwrap();

        for (line_num, line) in content.lines().enumerate() {
            for cap in re.find_iter(line) {
                matches.push(Match {
                    line_num: line_num + 1,
                    line: line.to_string(),
                    matched_text: cap.as_str().to_string(),
                    file: filename.to_string(),
                });
            }
        }
        matches
    }

    fn check_passive_voice(&self, content: &str, filename: &str) -> Vec<Match> {
        let mut matches = Vec::new();
        let irregulars_pattern = self.passive_irregulars.join("|");
        let pattern = format!(
            r"\b(am|are|were|being|is|been|was|be)\s+(\w+ed|({}))\b",
            irregulars_pattern
        );
        let re = Regex::new(&pattern).unwrap();

        for (line_num, line) in content.lines().enumerate() {
            for cap in re.find_iter(line) {
                matches.push(Match {
                    line_num: line_num + 1,
                    line: line.to_string(),
                    matched_text: cap.as_str().to_string(),
                    file: filename.to_string(),
                });
            }
        }
        matches
    }

    fn check_duplicates(&self, content: &str, filename: &str) -> Vec<Match> {
        let mut matches = Vec::new();
        let word_re = Regex::new(r"\b\w+\b").unwrap();

        for (line_num, line) in content.lines().enumerate() {
            let words: Vec<&str> = word_re.find_iter(line).map(|m| m.as_str()).collect();
            for i in 0..words.len().saturating_sub(1) {
                if words[i].to_lowercase() == words[i + 1].to_lowercase() {
                    matches.push(Match {
                        line_num: line_num + 1,
                        line: line.to_string(),
                        matched_text: format!("{} {}", words[i], words[i + 1]),
                        file: filename.to_string(),
                    });
                }
            }
        }
        matches
    }

    /// **NEW**: Checks for spelling errors using the trained Speller.
    fn check_spelling(&self, content: &str, filename: &str, speller: &mut Speller) -> Vec<Match> {
        let mut matches = Vec::new();
        let word_re = Regex::new(r"[a-zA-Z]+").unwrap();

        for (line_num, line) in content.lines().enumerate() {
            for mat in word_re.find_iter(line) {
                let original_word = mat.as_str();
                let lower_word = original_word.to_lowercase();
                
                // Don't check single-letter words like "a" or "I"
                if lower_word.len() <= 1 {
                    continue;
                }

                let suggestion = speller.correct(&lower_word);

                if suggestion != lower_word {
                    matches.push(Match {
                        line_num: line_num + 1,
                        line: line.to_string(),
                        matched_text: format!("'{}' -> '{}'", original_word, suggestion),
                        file: filename.to_string(),
                    });
                }
            }
        }
        matches
    }

    fn print_matches(&self, matches: &[Match], check_type: &str) {
        if matches.is_empty() {
            println!("{}: {}", check_type.green().bold(), "No issues found!".green());
            return;
        }

        println!("{}: {} issues found", check_type.yellow().bold(), matches.len());
        for m in matches {
            println!(
                "  {}:{} {} - {}",
                m.file.blue(),
                m.line_num.to_string().cyan(),
                m.matched_text.red().bold(),
                m.line.trim()
            );
        }
    }
    
    fn analyze_file(&self, filename: &str, speller: &mut Speller, spell_check_enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(filename)?;
        
        println!("\n{} {}", "Analyzing:".bold(), filename.blue().bold());
        println!("{}", "=".repeat(50));

        // Check for weasel words
        let weasel_matches = self.check_weasel_words(&content, filename);
        self.print_matches(&weasel_matches, "Weasel Words");
        println!();

        // Check for passive voice
        let passive_matches = self.check_passive_voice(&content, filename);
        self.print_matches(&passive_matches, "Passive Voice");
        println!();

        // Check for duplicates
        let duplicate_matches = self.check_duplicates(&content, filename);
        self.print_matches(&duplicate_matches, "Duplicate Words");
        println!();
        
        // **NEW**: Check for spelling
        if spell_check_enabled {
            let spelling_matches = self.check_spelling(&content, filename, speller);
            self.print_matches(&spelling_matches, "Spelling Suggestions");
        }

        println!();
        Ok(())
    }
}


//======================================================================
// PART 3: MAIN EXECUTION AND UI
// Merged and updated `main` and `print_usage` functions.
//======================================================================

fn print_usage() {
    println!("{}", "Comprehensive Writing Checker".green().bold());
    println!("Checks for weasel words, passive voice, duplicate words, and spelling errors.\n");
    println!("{}", "Usage:".bold());
    println!("  writing-checker <file1> [file2] [file3] ...");
    println!("\n{}", "Checks performed:".bold());
    println!("  • {} - Words that obscure precision", "Weasel words".yellow());
    println!("  • {} - Overuse of passive voice", "Passive voice".yellow());
    println!("  • {} - Accidentally duplicated words", "Duplicate words".yellow());
    println!("  • {} - Potential spelling errors", "Spelling".yellow());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let mut checker = WritingChecker::new();
    checker.load_custom_weasels();

    // --- Initialize and train the Speller ---
    let mut speller = Speller::new();
    let spell_check_enabled: bool;

    println!("\nChecking for spell checker training file ('training.txt')...");
    match fs::read_to_string("training.txt") {
        Ok(contents) => {
            print!("Training data found. Training spell checker... ");
            speller.train(&contents);
            spell_check_enabled = true;
            println!("{}", "Done.".green());
        }
        Err(_) => {
            eprintln!("{}", "Warning: 'training.txt' not found in the project root. Spell checking will be disabled.".yellow());
            spell_check_enabled = false;
        }
    }

    let mut files_processed = 0;
    for filename in &args[1..] {
        if Path::new(filename).exists() {
             match checker.analyze_file(filename, &mut speller, spell_check_enabled) {
                Ok(()) => {
                    files_processed += 1;
                }
                Err(e) => {
                    eprintln!("{} {}: {}", "Error processing".red(), filename, e);
                }
            }
        } else {
            eprintln!("{} File not found: {}", "Error:".red().bold(), filename);
        }
    }

    println!("{}", "=".repeat(50));
    println!(
        "{} {} files processed",
        "Summary:".green().bold(),
        files_processed
    );
}