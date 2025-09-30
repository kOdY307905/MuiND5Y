 * A Rust program using the Tokio framework to create a user interface component library.
 *
 * This library aims to provide a set of reusable UI components that can be easily
 * integrated into larger applications.
 */

// Use external crates
use tokio::sync::Mutex;

// Define a module for UI components
pub mod ui_components {
    // Define a struct for a basic button component
    #[derive(Debug, Clone)]
    pub struct Button {
        pub label: String,
        pub on_click: Box<dyn Fn() + Send + Sync>,
    }

    impl Button {
        // Constructor for the Button struct
        pub fn new(label: &str, on_click: impl Fn() + Send + Sync + 'static) -> Self {
            Button {
                label: label.to_string(),
                on_click: Box::new(on_click),
            }
        }

        // Method to simulate clicking the button
        pub fn click(&self) {
            (self.on_click)();
        }
    }

    // Define a struct for a text input component
    #[derive(Debug, Clone)]
    pub struct TextInput {
        pub placeholder: String,
        pub value: Mutex<String>,
    }

    impl TextInput {
        // Constructor for the TextInput struct
        pub fn new(placeholder: &str) -> Self {
            TextInput {
                placeholder: placeholder.to_string(),
                value: Mutex::new(String::new()),
            }
        }

        // Method to set the value of the text input
        pub fn set_value(&self, value: String) {
            let mut lock = self.value.lock().await;
            *lock = value;
        }

        // Method to get the value of the text input
        pub async fn get_value(&self) -> String {
            let lock = self.value.lock().await;
            lock.clone()
        }
    }
}

// Define a main function to demonstrate the usage of the UI components
#[tokio::main]
async fn main() {
    // Create a new button with an on-click action
    let button = ui_components::Button::new("But", || {
        println!("Button clicked!");
    });

    // Simulate clicking the button
    button.click();

    // Create a new text input with a placeholder
    let text_input = ui_components::TextInput::new("Enter text...");

    // Set the value of the text input
    text_input.set_value("Hello, world!".to_string()).await;

    // Get the value of the text input and print it
    let value = text_input.get_value().await;
    println!("Text input value: "{}"", value);
}
