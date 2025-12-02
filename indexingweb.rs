use std::collections::HashMap;

struct InvertedWebPages {
    pages: Vec<String>,
    page_map: HashMap<String, Vec<usize>>,
}

impl InvertedWebPages {
    fn new(pages: &Vec<String>) -> Self {
        InvertedWebPages {
            pages: pages.clone(),
            page_map: HashMap::new(),
        }
    }

    fn populate_index(&mut self) {
        for (num, page) in self.pages.iter().enumerate() {
            for word in page.split_whitespace() {
                self.page_map
                    .entry(word.to_string())
                    .or_insert(Vec::new())
                    .push(num);
            }
        }
    }

    fn search_word(&self, query: &str) {
        let result = self.page_map.get(query);
        println!("Search results for '{}':", query);
        match result {
            Some(indices) => {
                for idx in indices {
                    println!(" - Found in page {}: \"{}\"", idx, self.pages[*idx]);
                }
            }
            None => {
                println!(" - Not found.");
            }
        }
    }


    fn find_common_elements(list_a: &Vec<usize>, list_b: &Vec<usize>) -> Vec<usize> {
        let mut i = 0;
        let mut j = 0;
        let mut res = Vec::new();

        while i < list_a.len() && j < list_b.len() {
            if list_a[i] == list_b[j] {
                res.push(list_a[i]);
                i += 1;
                j += 1;
            } else if list_a[i] < list_b[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
        res
    }
}

fn main() {
    let pages = vec![
        "rust is fast and safe".to_string(),
        "rust has a great compiler".to_string(),
        "python is slow but easy".to_string(),
        "java is verbose".to_string(),
    ];

    let mut index = InvertedWebPages::new(&pages);
    index.populate_index();

    println!("--- Single Word Search ---");
    index.search_word("rust");
    index.search_word("is");
    index.search_word("slow");
    index.search_word("missing");

    println!("\n--- Intersection Search (rust AND is) ---");
    // Manually getting lists to test the static function
    // In a real app, this would be inside a method like `search_multi_word`
    if let (Some(list_a), Some(list_b)) = (index.page_map.get("rust"), index.page_map.get("is")) {
        let common = InvertedWebPages::find_common_elements(list_a, list_b);
        println!("Pages containing both 'rust' and 'is': {:?}", common);
        for idx in common {
             println!(" - Page {}: \"{}\"", idx, pages[idx]);
        }
    } else {
        println!("One of the words was not found.");
    }
}