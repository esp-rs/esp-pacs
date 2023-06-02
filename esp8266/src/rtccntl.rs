#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - PLL I2C Register"]
    pub pll: PLL,
}
#[doc = "PLL (rw) register accessor: an alias for `Reg<PLL_SPEC>`"]
pub type PLL = crate::Reg<pll::PLL_SPEC>;
#[doc = "PLL I2C Register"]
pub mod pll;
