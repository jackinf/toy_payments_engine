mod helpers;

#[cfg(test)]
mod test {
    use rstest::rstest;
    use toy_payments_engine::run_transactions_from_file;

    #[rstest]
    #[case("empty")]
    #[case("simple")]
    #[case("big")]
    // #[case("full")]
    fn test_use_case(#[case] name: &str) {
        // Arrange
        let input_file = crate::helpers::get_test_file_path(&format!("inputs/{}.csv", name));
        let output_file = crate::helpers::get_test_file_path(&format!("outputs/{}.csv", name));
        let output_lines_raw = crate::helpers::read_csv(&output_file).unwrap();
        let mut output_lines = crate::helpers::deserialize_output_lines(output_lines_raw);

        // Act
        let clients = run_transactions_from_file(input_file).unwrap();

        // Assert
        crate::helpers::compare_expected_output_with_actual(&mut output_lines, clients);
    }
}
