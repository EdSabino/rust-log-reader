use std::collections::HashMap;

pub struct Transaction {
    pub commited: bool,
    pub finalized: bool,
    transaction_state: HashMap<String, i32>
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            commited: false,
            finalized: false,
            transaction_state: HashMap::new()
        }
    }

    pub fn commit(&mut self, state: &mut HashMap<String, i32>) {
        for (key, value) in self.transaction_state.clone().into_iter() {
            state.insert(key, value);
        }
    }

    pub fn mark_commited(&mut self) {
        self.commited = true;
    }

    pub fn change(&mut self, key: String, value: i32) {
        self.transaction_state.insert(key.to_lowercase(), value);
    }

    pub fn finalize(&mut self) {
        self.finalized = true;
    }
}