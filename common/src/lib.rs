use rusty_ulid::generate_ulid_string;

pub mod error;
pub mod log;

pub fn ulid_genrate()-> String {
    generate_ulid_string()
}
