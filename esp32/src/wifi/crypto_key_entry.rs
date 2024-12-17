#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "The cryptographic keys, to be used by the MAC"]
pub struct CRYPTO_KEY_ENTRY {
    addr_low: ADDR_LOW,
    addr_high: ADDR_HIGH,
    _reserved_end: [u8; 0x20],
}
impl CRYPTO_KEY_ENTRY {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn addr_low(&self) -> &ADDR_LOW {
        &self.addr_low
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn addr_high(&self) -> &ADDR_HIGH {
        &self.addr_high
    }
}
#[doc = "ADDR_LOW (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`addr_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr_low`] module"]
pub type ADDR_LOW = crate::Reg<addr_low::ADDR_LOW_SPEC>;
#[doc = ""]
pub mod addr_low;
#[doc = "ADDR_HIGH (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`addr_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr_high`] module"]
pub type ADDR_HIGH = crate::Reg<addr_high::ADDR_HIGH_SPEC>;
#[doc = ""]
pub mod addr_high;
