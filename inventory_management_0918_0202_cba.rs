use tokio::sync::{Mutex, MutexGuard};
use std::sync::Arc;

/// Represents an item in the inventory.
#[derive(Debug, Clone)]
struct Item {
    id: u32,
    name: String,
    quantity: u32,
}

/// Represents the inventory management system.
struct InventoryManager {
    items: Mutex<Vec<Item>>,
}

impl InventoryManager {
    /// Creates a new InventoryManager.
    fn new() -> Self {
        InventoryManager {
            items: Mutex::new(Vec::new()),
        }
    }

    /// Adds an item to the inventory.
    async fn add_item(&self, item: Item) -> Result<(), String> {
        let mut items = self.items.lock().await;
        items.push(item);
        Ok(())
    }

    /// Removes an item from the inventory.
    async fn remove_item(&self, item_id: u32) -> Result<(), String> {
        let mut items = self.items.lock().await;
        if let Some(index) = items.iter().position(|x| x.id == item_id) {
            items.remove(index);
            Ok(())
        } else {
            Err("Item not found".to_string())
        }
    }

    /// Updates the quantity of an item in the inventory.
    async fn update_quantity(&self, item_id: u32, new_quantity: u32) -> Result<(), String> {
        let mut items = self.items.lock().await;
        if let Some(item) = items.iter_mut().find(|x| x.id == item_id) {
            item.quantity = new_quantity;
            Ok(())
        } else {
            Err("Item not found".to_string())
        }
    }

    /// Retrieves an item from the inventory by ID.
    async fn get_item(&self, item_id: u32) -> Option<Item> {
        let items = self.items.lock().await;
        items.iter().find(|x| x.id == item_id).cloned()
    }
}

#[tokio::main]
async fn main() {
    let manager = Arc::new(InventoryManager::new());

    // Adding items to the inventory.
    let item1 = Item { id: 1, name: "Apple".to_string(), quantity: 100 };
    let item2 = Item { id: 2, name: "Banana".to_string(), quantity: 50 };
    manager.add_item(item1).await.unwrap();
    manager.add_item(item2).await.unwrap();

    // Updating the quantity of an item.
    manager.update_quantity(1, 150).await.unwrap();

    // Retrieving an item from the inventory.
    if let Some(item) = manager.get_item(1).await {
        println!("Item: {:?}", item);
    }
}
