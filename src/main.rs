use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum TransactionType {
    Income,
    Expense,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionType::Income => write!(f, "Income"),
            TransactionType::Expense => write!(f, "Expense"),
        }
    }
}

impl From<&str> for TransactionType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "income" => TransactionType::Income,
            "expense" => TransactionType::Expense,
            _ => TransactionType::Expense,
        }
    }
}

#[derive(Debug, Clone)]
struct Transaction {
    id: u32,
    description: String,
    amount: f64,
    is_recurring: bool,
    date: String,
    transaction_type: TransactionType,
    category: String,
}

#[derive(Debug)]
struct FinanceTracker {
    transactions: Vec<Transaction>,
    category_totals: HashMap<String, f64>,
    unique_categories: HashSet<String>,
    next_id: u32,
}

impl FinanceTracker {
    pub fn new() -> Self {
        FinanceTracker {
            transactions: Vec::new(),
            category_totals: HashMap::new(),
            unique_categories: HashSet::new(),
            next_id: 1,
        }
    }

    pub fn add_transaction(
        &mut self,
        description: String,
        amount: f64,
        is_recurring: bool,
        date: String,
        transaction_type: TransactionType,
        category: String,
    ) {
        let transaction = Transaction {
            id: self.next_id,
            description,
            amount,
            is_recurring,
            date,
            transaction_type,
            category: category.clone(),
        };

        self.transactions.push(transaction);

        self.category_totals
            .entry(category.clone())
            .and_modify(|total| *total += amount)
            .or_insert(amount);

        self.unique_categories.insert(category);

        self.next_id += 1;
    }

    pub fn total_income(&self) -> f64 {
        self.transactions
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Income)
            .map(|t| t.amount)
            .sum()
    }

    pub fn total_expense(&self) -> f64 {
        self.transactions
            .iter()
            .filter(|t| t.transaction_type == TransactionType::Expense)
            .map(|t| t.amount)
            .sum()
    }

    pub fn net_balance(&self) -> f64 {
        self.total_income() - self.total_expense()
    }

    pub fn average_transaction(&self) -> f64 {
        if self.transactions.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.transactions.iter().map(|t| t.amount).sum();

        let count = self.transactions.len() as f64;

        sum / count
    }

    pub fn category_breakdown(&self) -> &HashMap<String, f64> {
        &self.category_totals
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}

fn get_user_input(promt: &str) -> String {
    use std::io::{self, Write};

    print!("{}", promt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn parse_amount(input: &str) -> Result<f64, std::num::ParseFloatError> {
    input.parse::<f64>()
}

fn parse_bool(input: &str) -> bool {
    match input.to_lowercase().as_str() {
        "yes" | "y" => true,
        _ => false,
    }
}

fn display_menu() {
    println!("\n=== Finance Tracker Menu ===");
    println!("1) Add Transaction");
    println!("2) View Summary");
    println!("3) View Category Report");
    println!("4) View All Transactions");
    println!("5) Quit");
    println!("===========================")
}

fn add_transaction_interactive(tracker: &mut FinanceTracker) {
    let description = get_user_input("Enter description: ");

    let amount = loop {
        let input = get_user_input("Enter amount: ");
        match parse_amount(&input) {
            Ok(amt) => break amt,
            Err(_) => println!("Invalid amount. Please enter a number."),
        }
    };

    let is_recurring_input = get_user_input("Is this recurring? (yes/no): ");
    let is_recurring = parse_bool(&is_recurring_input);

    let date = get_user_input("Enter date (YYYY-MM-DD): ");

    let type_input = get_user_input("Enter type (income/expense): ");
    let transaction_type = TransactionType::from(type_input.as_str());

    let category = get_user_input("Enter category: ");

    tracker.add_transaction(
        description,
        amount,
        is_recurring,
        date,
        transaction_type,
        category,
    );

    println!("Transaction added successfully!")
}

fn display_summary(tracker: &FinanceTracker) {
    println!("\n=== Financial Summary ===");
    println!("Total Income: ${:.2}", tracker.total_income());
    println!("Total Expense: ${:.2}", tracker.total_expense());
    println!("Net Balance: ${:.2}", tracker.net_balance());
    println!("Average Transaction ${:.2}", tracker.average_transaction());
    println!("======================\n")
}

fn display_category_report(tracker: &FinanceTracker) {
    println!("\n=== Category Breakdown ===");
    let breakdown = tracker.category_breakdown();
    for (categoty, total) in breakdown.iter() {
        println!("{} ${:.2}", categoty, total)
    }
    println!("=========================\n")
}

fn display_all_transactions(tracker: &FinanceTracker) {
    println!("\n=== All Transaction ===");
    let transactions = tracker.get_transactions();
    for transaction in transactions.iter() {
        println!(
            "ID: {} | {} | ${:.2} | {} | {} | {} | Recurring: {}",
            transaction.id,
            transaction.description,
            transaction.amount,
            transaction.transaction_type,
            transaction.category,
            transaction.date,
            transaction.is_recurring
        );
    }
    println!("=======================\n");
}

fn main() {
    let mut tracker = FinanceTracker::new();

    loop {
        display_menu();
        let choice = get_user_input("Enter choice: ");

        match choice.as_str() {
            "1" => add_transaction_interactive(&mut tracker),
            "2" => display_summary(&tracker),
            "3" => display_category_report(&tracker),
            "4" => display_all_transactions(&tracker),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn create_test_tracker() -> FinanceTracker {
        let mut tracker = FinanceTracker::new();

        tracker.add_transaction(
            String::from("Salary"),
            5000.0,
            true,
            String::from("2025-01-04"),
            TransactionType::Income,
            String::from("Work"),
        );

        tracker.add_transaction(
            String::from("Freelance"),
            1500.0,
            false,
            String::from("2024-01-20"),
            TransactionType::Income,
            String::from("Work"),
        );

        tracker.add_transaction(
            String::from("Rent"),
            2000.0,
            true,
            String::from("2024-01-01"),
            TransactionType::Expense,
            String::from("Housing"),
        );

        tracker.add_transaction(
            String::from("Groceries"),
            500.0,
            false,
            String::from("2024-01-10"),
            TransactionType::Expense,
            String::from("Food"),
        );

        tracker
    }

    #[test]
    fn test_total_income() {
        let tracker = create_test_tracker();
        assert_eq!(tracker.total_income(), 6500.0);
    }
    
    #[test]
    fn test_total_expense() {
        let tracker = create_test_tracker();
        assert_eq!(tracker.total_expense(), 2500.0)
    }

    #[test]
    fn test_net_balance() {
        let tracker = create_test_tracker();
        assert_eq!(tracker.net_balance(), 4000.0);
    }

    #[test]
    fn test_average_transaction() {
        let tracker = create_test_tracker();
        assert_eq!(tracker.average_transaction(), 2250.0);
    }

    #[test]
    fn test_average_empty() {
        let tracker = FinanceTracker::new();
        assert_eq!(tracker.average_transaction(), 0.0)
    }

    #[test]
    fn test_category_totals() {
        let tracker = create_test_tracker();
        let breakdown = tracker.category_breakdown();

        assert_eq!(breakdown.get("Work"), Some(&6500.0));
        assert_eq!(breakdown.get("Housing"), Some(&2000.0));
        assert_eq!(breakdown.get("Food"), Some(&500.0));
    }

    #[test]
    fn test_parse_amount_valid() {
        let result = parse_amount("123.45");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 123.45);
    }

    #[test]
    fn test_parse_amount_invald() {
        let result = parse_amount("abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_transaction_type_from_str() {
        assert_eq!(TransactionType::from("income"), TransactionType::Income);
        assert_eq!(TransactionType::from("INCOME"), TransactionType::Income);
        assert_eq!(TransactionType::from("expense"), TransactionType::Expense);
        assert_eq!(TransactionType::from("EXPENSE"), TransactionType::Expense);
    }
}
