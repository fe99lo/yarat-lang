use std::collections::HashMap;
use crate::parser::ast::{Program, Statement, Expression, Operator, BlockStatement};

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Money { amount: f64, currency: String },
    Boolean(bool),
}

pub struct Evaluator {
    pub memory: HashMap<String, RuntimeValue>,
    pub transaction_vault: HashMap<String, (Vec<(String, String)>, BlockStatement)>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { memory: HashMap::new(), transaction_vault: HashMap::new() }
    }

    pub fn evaluate_program(&mut self, program: &Program) {
        for stmt in &program.statements {
            self.evaluate_statement(stmt);
        }
    }

    // ---------------------------------------------------------
    // STATEMENT EXECUTION
    // ---------------------------------------------------------
    fn evaluate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::AssetDeclaration { ticker, precision } => {
                println!("🏦 Execution: Initializing system for asset '{}' with {} decimal places.", ticker, precision);
            }
            Statement::Assignment { identifier, value } => {
                let evaluated_val = self.evaluate_expression(value);
                self.memory.insert(identifier.clone(), evaluated_val.clone());
                
                match evaluated_val {
                    RuntimeValue::Money { amount, currency } => {
                        println!("💰 Execution: Deposited {} {} into account '{}'.", amount, currency, identifier);
                    }
                    RuntimeValue::Boolean(b) => {
                        println!("⚙️ Execution: Set logical flag '{}' to {}.", identifier, b);
                    }
                }
            }
            Statement::IfStatement { condition, consequence, alternative } => {
                let cond_val = self.evaluate_expression(condition);
                if let RuntimeValue::Boolean(is_true) = cond_val {
                    if is_true {
                        self.evaluate_block(consequence);
                    } else if let Some(alt_block) = alternative {
                        self.evaluate_block(alt_block);
                    }
                }
            }
            Statement::TransactionDeclaration { name, parameters, body } => {
                self.transaction_vault.insert(name.clone(), (parameters.clone(), body.clone()));
                println!("📜 Execution: Saved Smart Contract '{}' into the vault.", name);
            }
            
            // ---------------------------------------------------------
            // THE FIX: SECURE STATE WRITE-BACK (PASS-BY-REFERENCE)
            // ---------------------------------------------------------
            Statement::TransactionCall { name, arguments } => {
                println!("🚀 Execution: Triggering Smart Contract '{}'...", name);
                if let Some((params, body)) = self.transaction_vault.get(name).cloned() {
                    
                    // We need to track which original accounts to update when the contract finishes
                    let mut write_backs = Vec::new();

                    // Step 1: Inject the balances into the contract's local parameters
                    for (i, arg_expr) in arguments.iter().enumerate() {
                        let param_name = params[i].0.clone();
                        let val = self.evaluate_expression(arg_expr);
                        self.memory.insert(param_name.clone(), val);

                        // If the argument was an account (like 'narok_wallet'), map it to the parameter (like 'sender')
                        if let Expression::Identifier(orig_account) = arg_expr {
                            write_backs.push((orig_account.clone(), param_name));
                        }
                    }

                    // Step 2: Run the smart contract logic
                    self.evaluate_block(&body);

                    // Step 3: Write the updated balances BACK to the original accounts!
                    for (orig_account, param_name) in write_backs {
                        if let Some(updated_val) = self.memory.get(&param_name).cloned() {
                            self.memory.insert(orig_account, updated_val);
                        }
                        // Security Cleanup: Remove the temporary parameter from the ledger so it doesn't clutter memory
                        self.memory.remove(&param_name);
                    }
                    
                    println!("✅ Execution: Smart Contract '{}' finished. Main ledger updated.", name);
                }
            }
        }
    }

    fn evaluate_block(&mut self, block: &BlockStatement) {
        for stmt in &block.statements {
            self.evaluate_statement(stmt);
        }
    }

    // ---------------------------------------------------------
    // EXPRESSION CALCULATION
    // ---------------------------------------------------------
    fn evaluate_expression(&self, expr: &Expression) -> RuntimeValue {
        match expr {
            Expression::MoneyLiteral { amount, currency } => {
                RuntimeValue::Money { amount: *amount, currency: currency.clone() }
            }
            Expression::BooleanLiteral(b) => {
                RuntimeValue::Boolean(*b)
            }
            Expression::Identifier(name) => {
                self.memory.get(name).cloned().unwrap_or(RuntimeValue::Boolean(false)) 
            }
            Expression::BinaryOperation { left, operator, right } => {
                let left_val = self.evaluate_expression(left);
                let right_val = self.evaluate_expression(right);

                match (left_val, right_val) {
                    (RuntimeValue::Money { amount: l_amt, currency: l_cur }, RuntimeValue::Money { amount: r_amt, currency: _ }) => {
                        match operator {
                            Operator::Plus => RuntimeValue::Money { amount: l_amt + r_amt, currency: l_cur },
                            Operator::Minus => RuntimeValue::Money { amount: l_amt - r_amt, currency: l_cur },
                            Operator::Multiply => RuntimeValue::Money { amount: l_amt * r_amt, currency: l_cur },
                            Operator::Divide => RuntimeValue::Money { amount: l_amt / r_amt, currency: l_cur },
                            Operator::LessThan => RuntimeValue::Boolean(l_amt < r_amt),
                            Operator::GreaterThan => RuntimeValue::Boolean(l_amt > r_amt),
                            Operator::Equal => RuntimeValue::Boolean(l_amt == r_amt),
                            Operator::NotEqual => RuntimeValue::Boolean(l_amt != r_amt),
                        }
                    }
                    (RuntimeValue::Boolean(l_bool), RuntimeValue::Boolean(r_bool)) => {
                        match operator {
                            Operator::Equal => RuntimeValue::Boolean(l_bool == r_bool),
                            Operator::NotEqual => RuntimeValue::Boolean(l_bool != r_bool),
                            _ => RuntimeValue::Boolean(false),
                        }
                    }
                    _ => RuntimeValue::Boolean(false),
                }
            }
        }
    }
}