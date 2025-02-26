## Structure

This module contains mutliple test cases. The test cases are ordered in the following groups:
- `generator` contains all code generator related tests
- `optimizer` contains all optimizer related tests
- `feature` contains general tests for different features of the crate
- `schema` contains test for whole real life example schemas

Within the group directory, each case is a separate module and is located in its own directory. Each test case directory normally contains the following:
- `mod.rs` test case module with the different tests
- `schema.xsd` XML schema used for the tests in this test case
- `expected` directory with the expected output of the code generator
- `example` directory with different example XML files for serialize and deserialize tests

## How to make a new case

The easiest way is:
- copy existing case to a new directory `new_test`
- register `new_test` in `mod.rs`
- modify `new_test/schema.xsd` according to your case
- modify `new_test/examples/default.xml` according to your case
- modify `new_test/expected/default.rs` manually or cheat:
    - run test `cargo test --features update-expectations schema::new_test::`
- modify `new_test/mod.rs` to implement your needed tests
