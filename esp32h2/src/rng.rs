#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Random number data"]
    pub data: DATA,
}
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Random number data"]
pub mod data;
