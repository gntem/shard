use std::collections::HashMap;

pub struct Engine {
    pub rules: HashMap<String, fn()>,
    pub facts: HashMap<String, HashMap<String, String>>
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            rules: HashMap::new(),
            facts: HashMap::new()
        }
    }

    pub fn add_rule(&mut self, rule_name: String, rule: fn()) {
        self.rules.insert(rule_name, rule);
    }

    pub fn add_fact(&mut self, fact_name: String, fact: HashMap<String, String>) {
        self.facts.insert(fact_name, fact);
    }

    pub fn run(&self) {
        for rule in self.rules.values() {
            rule();
        }
    }

    pub fn get_fact(&self, fact_name: String) -> Option<&HashMap<String, String>> {
        self.facts.get(&fact_name)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_rule() {
        let mut engine = Engine::new();
        engine.add_rule("test".to_string(), || {});
        assert_eq!(engine.rules.len(), 1);
    }

    #[test]
    fn test_add_fact() {
        let mut engine = Engine::new();
        let mut fact = HashMap::new();
        fact.insert("test".to_string(), "test".to_string());
        engine.add_fact("test".to_string(), fact);
        assert_eq!(engine.facts.len(), 1);
    }

    #[test]
    fn test_run() {
        let mut engine = Engine::new();
        let mut fact = HashMap::new();
        fact.insert("test".to_string(), "test".to_string());
        engine.add_fact("test".to_string(), fact);
        engine.add_rule("test".to_string(), || {});
        engine.run();
    }

    #[test]
    fn test_get_fact() {
        let mut engine = Engine::new();
        let mut fact = HashMap::new();
        fact.insert("test".to_string(), "test".to_string());
        engine.add_fact("test".to_string(), fact);
        assert_eq!(engine.get_fact("test".to_string()).unwrap().len(), 1);
    }
}
