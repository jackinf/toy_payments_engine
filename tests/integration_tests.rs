use crate::helpers::{
    compare_expected_output_with_actual, deserialize_output_lines, get_test_file_path, read_csv,
};
use std::error::Error;
use std::result;
use toy_payments_engine::run_transactions_from_file;

mod helpers;

type Result<T> = result::Result<T, Box<dyn Error>>;

#[test]
fn empty_case() {
    // Arrange
    let input_file = get_test_file_path(&format!("inputs/{}", "empty.csv"));
    let output_file = get_test_file_path(&format!("outputs/{}", "empty.csv"));
    let output_lines_raw = read_csv(&output_file).unwrap();
    let mut output_lines = deserialize_output_lines(output_lines_raw);

    // Act
    let clients = run_transactions_from_file(input_file).unwrap();

    // Assert
    compare_expected_output_with_actual(&mut output_lines, clients);
}

#[test]
fn simple_case() {
    // Arrange
    let input_file = get_test_file_path(&format!("inputs/{}", "simple.csv"));
    let output_file = get_test_file_path(&format!("outputs/{}", "simple.csv"));
    let output_lines_raw = read_csv(&output_file).unwrap();
    let mut output_lines = deserialize_output_lines(output_lines_raw);

    // Act
    let clients = run_transactions_from_file(input_file).unwrap();

    // Assert
    compare_expected_output_with_actual(&mut output_lines, clients);
}

#[test]
fn large_case() {
    // Arrange
    let input_file = get_test_file_path(&format!("inputs/{}", "large.csv"));
    let output_file = get_test_file_path(&format!("outputs/{}", "large.csv"));
    let output_lines_raw = read_csv(&output_file).unwrap();
    let mut output_lines = deserialize_output_lines(output_lines_raw);

    // Act
    let clients = run_transactions_from_file(input_file).unwrap();

    // Assert
    compare_expected_output_with_actual(&mut output_lines, clients);
}
