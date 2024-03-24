#[macro_export]
macro_rules! assert_instruction_error {
    ($error:expr, $matcher:pat) => {
        match $error {
            BanksClientError::TransactionError(TransactionError::InstructionError(_, $matcher)) => {
                assert!(true)
            }
            err => assert!(false, "Expected instruction error but got '{:#?}'", err),
        };
    };
}
