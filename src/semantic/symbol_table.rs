use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AssetInfo {
    pub precision: u32,
}

#[derive(Debug, Clone)]
pub struct TransactionSignature {
    // THE FIX: The registry now securely stores BOTH the Name and the Type
    pub expected_parameters: Vec<(String, String)>, 
}

#[derive(Debug)]
pub struct SymbolTable {
    pub assets: HashMap<String, AssetInfo>,
    pub variables: HashMap<String, String>, 
    pub transactions: HashMap<String, TransactionSignature>, 
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            assets: HashMap::new(),
            variables: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn define_asset(&mut self, ticker: String, precision: u32) {
        self.assets.insert(ticker, AssetInfo { precision });
    }

    pub fn lookup_asset(&self, ticker: &str) -> Option<&AssetInfo> {
        self.assets.get(ticker)
    }

    pub fn define_variable(&mut self, name: String, exact_type: String) {
        self.variables.insert(name, exact_type);
    }

    pub fn lookup_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    // THE FIX: Now accepts the (Name, Type) tuple from the Auditor
    pub fn define_transaction(&mut self, name: String, params: Vec<(String, String)>) {
        self.transactions.insert(name, TransactionSignature { expected_parameters: params });
    }

    pub fn lookup_transaction(&self, name: &str) -> Option<&TransactionSignature> {
        self.transactions.get(name)
    }
}