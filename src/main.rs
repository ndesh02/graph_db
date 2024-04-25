use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Clone)]
struct Node {
    id: u32,
    label: String,
    properties: HashMap<String, String>,
}

#[derive(Debug, Clone)]
struct Relationship {
    id: u32,
    label: String,
    start_node: u32,
    end_node: u32,
    properties: HashMap<String, String>,
}

#[derive(Debug)]
struct GraphDatabase {
    nodes: HashMap<u32, Node>,
    relationships: HashMap<u32, Relationship>,
}

impl GraphDatabase{
    fn new() -> Self{
        GraphDatabase {
            nodes: HashMap::new(),
            relationships: HashMap::new(),
        }
    }

    fn add_node(&mut self, id: u32, label: String, properties: HashMap<String, String>){
        let node = Node { id, label, properties };
        self.nodes.insert(id, node);
    }

    fn add_relationship(&mut self, id: u32, label: String, start_node: u32, end_node: u32, properties: HashMap<String, String>){
        let relationship = Relationship { id, label, start_node, end_node, properties };
        self.relationships.insert(id, relationship);
    }

    fn get_node_by_id(&self, id: u32) -> Option<&Node> {
        self.nodes.get(&id)
    }

    fn get_relationships_of_node(&self, node_id: u32) -> Vec<&Relationship> {
        self.relationships.values().filter(|&rel| rel.start_node == node_id || rel.end_node == node_id).collect()
    }

    fn get_nodes_by_property(&self, key: &str, value: &str) -> Vec<&Node> {
        self.nodes.values().filter(|&node| node.properties.get(key) == Some(&value.to_string())).collect()
    }
}

use colored::*;
use std::{io, thread};
use std::time::Duration;

fn clear_screen(){
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

fn main(){
    println!("{}", "Neha's Rust Graph".green().bold());
    thread::sleep(Duration::from_secs(3));
    clear_screen();
    let mut db = GraphDatabase::new();
    loop {
        print!("{} ", "graphdb>".blue().bold());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["exit"] => break,

            ["add", "node", id, label, properties] => {
                let properties: HashMap<_, _> = properties.split(',')
                    .map(|s| {
                        let parts: Vec<_> = s.split(':').collect();
                        (parts[0].to_string(), parts[1].to_string())
                    })
                    .collect();
                db.add_node(id.parse().unwrap(), label.to_string(), properties);
                println!("Node successfully added.");
            },

            ["add", "relationship", id, label, start, end, properties] => {
                let properties: HashMap<_, _> = properties.split(',')
                    .map(|s| {
                        let parts: Vec<_> = s.split(':').collect();
                        (parts[0].to_string(), parts[1].to_string())
                    })
                    .collect();
                db.add_relationship(id.parse().unwrap(), label.to_string(), start.parse().unwrap(), end.parse().unwrap(), properties);
                println!("Relationship successfully added.")
            },

            ["query", "node", "id", id] => {
                match db.get_node_by_id(id.parse().unwrap()) {
                    Some(node) => println!("{:?}", node),
                    None => println!("Node has not been found."),
                }
            }

            ["query", "node", "property", key, value] => {
                let nodes = db.get_nodes_by_property(key, value);
                for node in nodes {
                    println!("{:?}", node);
                }
            },

            _ => println!("Command not known.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_add_and_get_node() {
        let mut db = GraphDatabase::new();
        let properties: HashMap<String, String> = HashMap::new();
        db.add_node(1, "User".to_string(), properties.clone());

        let node = db.get_node_by_id(1).unwrap();
        assert_eq!(node.label, "User");
        assert_eq!(node.properties, properties);
    }

#[test]
    fn test_add_and_get_relationship() {
        let mut db = GraphDatabase::new();
        let properties = HashMap::new();

        db.add_node(1, "User".to_string(), properties.clone());
        db.add_node(2, "Post".to_string(), properties.clone());

        db.add_relationship(1, "WROTE".to_string(), 1, 2, properties.clone());

        let relationship = db.relationships.get(&1).unwrap();
        assert_eq!(relationship.start_node, 1);
        assert_eq!(relationship.end_node, 2);
        assert_eq!(relationship.label, "WROTE");
    }

#[test]
    fn test_query_node_by_property() {
        let mut db = GraphDatabase::new();
        let mut properties = HashMap::new();
        properties.insert("username".to_string(), "alice".to_string());

        db.add_node(1, "User".to_string(), properties);

        let nodes = db.get_nodes_by_property("username", "alice");
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].id, 1);
    }
}


