pub mod symbol_table;
use crate::parser::ast::{Program, Statement, Expression, Operator, BlockStatement};
use symbol_table::SymbolTable;

pub struct Analyzer { pub symbol_table: SymbolTable }

impl Analyzer {
    pub fn new() -> Self { Analyzer { symbol_table: SymbolTable::new() } }

    pub fn analyze_program(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements { self.analyze_statement(stmt)?; }
        Ok(())
    }

    fn analyze_statement(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::AssetDeclaration { ticker, precision } => {
                self.symbol_table.define_asset(ticker.clone(), *precision);
                println!("✅ Auditor: Registered legal asset '{}' with precision {}", ticker, precision);
                Ok(())
            }
            Statement::Assignment { identifier, value } => {
                let resulting_type = self.analyze_expression(value)?;
                self.symbol_table.define_variable(identifier.clone(), resulting_type.clone());
                println!("✅ Auditor: Validated assignment. '{}' securely holds type '{}'.", identifier, resulting_type);
                Ok(())
            }
            Statement::IfStatement { condition, consequence, alternative } => {
                let cond_type = self.analyze_expression(condition)?;
                if cond_type != "BOOLEAN" {
                    return Err(format!("❌ CRITICAL ERROR: 'if' condition must be a Boolean, found '{}'.", cond_type));
                }
                self.analyze_block(consequence)?;
                if let Some(alt_block) = alternative { self.analyze_block(alt_block)?; }
                Ok(())
            }
            // ---------------------------------------------------------
            // THE FIX: Register the STRICT Type, not "ANY"!
            // ---------------------------------------------------------
            Statement::TransactionDeclaration { name, parameters, body } => {
                self.symbol_table.define_transaction(name.clone(), parameters.clone());
                println!("✅ Auditor: Registered strictly-typed Smart Contract '{}'.", name);
                
                for (p_name, p_type) in parameters {
                    self.symbol_table.define_variable(p_name.clone(), p_type.clone());
                }
                self.analyze_block(body)?;
                Ok(())
            }
            Statement::TransactionCall { name, arguments } => {
                if let Some(tx_sig) = self.symbol_table.lookup_transaction(name).cloned() {
                    if tx_sig.expected_parameters.len() != arguments.len() {
                        return Err(format!("❌ CRITICAL ERROR: Transaction '{}' expects {} arguments.", name, tx_sig.expected_parameters.len()));
                    }
                    // Validate that the caller is passing the correct currency to the typed parameter
                    for (i, arg) in arguments.iter().enumerate() {
                        let arg_type = self.analyze_expression(arg)?;
                        let expected_type = &tx_sig.expected_parameters[i].1;
                        if &arg_type != expected_type {
                            return Err(format!("❌ CRITICAL ERROR: Contract parameter '{}' requires '{}', but you passed '{}'.", tx_sig.expected_parameters[i].0, expected_type, arg_type));
                        }
                    }
                    println!("✅ Auditor: Validated execution call for Transaction '{}'. All currencies match.", name);
                    Ok(())
                } else { Err(format!("❌ CRITICAL ERROR: Transaction '{}' is called but undefined.", name)) }
            }
        }
    }

    fn analyze_block(&mut self, block: &BlockStatement) -> Result<(), String> {
        for stmt in &block.statements { self.analyze_statement(stmt)?; }
        Ok(())
    }

    fn analyze_expression(&self, expr: &Expression) -> Result<String, String> {
        match expr {
            Expression::MoneyLiteral { amount: _, currency } => {
                if self.symbol_table.lookup_asset(currency).is_some() { Ok(currency.clone()) } 
                else { Err(format!("❌ CRITICAL ERROR: Unregistered asset '{}'.", currency)) }
            }
            Expression::BooleanLiteral(_) => Ok("BOOLEAN".to_string()),
            Expression::Identifier(name) => {
                if let Some(var_type) = self.symbol_table.lookup_variable(name) { Ok(var_type.clone()) } 
                else { Err(format!("❌ CRITICAL ERROR: Variable '{}' is used before defined.", name)) }
            }
            Expression::BinaryOperation { left, operator, right } => {
                let left_type = self.analyze_expression(left)?;
                let right_type = self.analyze_expression(right)?;

                if left_type != right_type {
                    return Err(format!("❌ CRITICAL ERROR: Type mismatch! Cannot operate between '{}' and '{}'.", left_type, right_type));
                }
                match operator {
                    Operator::Plus | Operator::Minus | Operator::Multiply | Operator::Divide => {
                        if left_type == "BOOLEAN" { return Err("❌ CRITICAL ERROR: Cannot do math on Booleans.".to_string()); }
                        Ok(left_type) 
                    }
                    Operator::LessThan | Operator::GreaterThan | Operator::Equal | Operator::NotEqual => { Ok("BOOLEAN".to_string()) }
                }
            }
        }
    }
}