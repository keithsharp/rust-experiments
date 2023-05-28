use std::sync::{Arc, Mutex};

struct Container {
    delay: u64,
    items: Arc<Mutex<Option<Vec<String>>>>,
}

impl Container {
    fn new(delay: u64) -> Self {
        Self {
            delay,
            items: Arc::new(Mutex::new(None)),
        }
    }

    fn items(&self) -> Vec<String> {
        let mut items = self.items.lock().unwrap();
        items.get_or_insert_with(|| self.create_items()).clone()
    }

    fn create_items(&self) -> Vec<String> {
        // This is an artificial delay to simulate a complex compute operation
        std::thread::sleep(std::time::Duration::from_secs(self.delay));
        vec!["Apple".to_string(), "Orange".to_string()]
    }
}

fn main() {
    let container = Container::new(2);

    let handle = std::thread::spawn(move || {
        let start = std::time::SystemTime::now();
        let items = container.items();
        println!(
            "First run, got {} items in {}ms.",
            items.len(),
            start.elapsed().unwrap().as_millis()
        );

        let start = std::time::SystemTime::now();
        let items = container.items();
        println!(
            "Second run, got {} items in {}ms.",
            items.len(),
            start.elapsed().unwrap().as_millis()
        );
    });

    let _ = handle.join();
}
