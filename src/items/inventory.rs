use crate::prelude::{Player, Fonts};

use super::ItemDescriptor;
use bevy::prelude::*;

/// Represents the inventory of an entity
#[derive(Debug, Clone, Component)]
pub struct Inventory {
    // The inventory's capacity
    pub capacity: usize,
    /// The items in the inventory
    pub items: Vec<ItemDescriptor>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            capacity: 10,
            items: Vec::with_capacity(10.0 as usize),
        }
    }
}

impl Inventory {
    /// Creates a new [`inventory`] with a capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            items: Vec::with_capacity(capacity),
        }
    }

    /// Add an item to the inventory
    pub fn add_item(&mut self, item: ItemDescriptor) -> Option<()> {
        if self.items.len() < self.capacity {
            self.items.push(item);
            return Some(());
        }
        None
    }

    /// Remove an item from the inventory
    pub fn remove_item(&mut self, index: usize) -> Option<()> {
        if index < self.items.len() {
            self.items.remove(index);
            return Some(());
        }
        None
    }

    /// Get an item from the inventory
    pub fn get_item(&self, index: usize) -> Option<&ItemDescriptor> {
        self.items.get(index)
    }

    /// Insert an item into the inventory
    pub fn insert_item(&mut self, index: usize, item: ItemDescriptor) -> Option<()> {
        if index < self.items.len() {
            self.items.insert(index, item);
            return Some(());
        }
        None
    }
}

/// Spawns the inentory menu of the player
pub fn spawn_inventory_menu(
    fonts: Res<Fonts>,
    inventory: Query<&Inventory, With<Player>>, 
    mut commands: Commands,
) {
    // Only show the players inventory
    if let Some(inventory) = inventory.iter().next() {
        commands.spawn_bundle(NodeBundle {
            style : Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: Rect::all(Val::Px(300.0)),
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        }).with_children(|parent| {
            // Spawn the title text
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Inventory",
                    TextStyle {
                        font: fonts.fira_sans.clone(),
                        font_size: 100.0,
                        ..Default::default()
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
    }
}