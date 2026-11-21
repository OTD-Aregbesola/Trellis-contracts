#![no_std]
// Exercise production sources without building unrelated workspace modules.
#[path = "../../../shared/src/accessibility.rs"]
pub mod accessibility;
#[path = "../../../shared/src/errors.rs"]
pub mod errors;
#[path = "../../../shared/src/storage.rs"]
pub mod storage;

#[cfg(test)]
mod accessibility_tests;
