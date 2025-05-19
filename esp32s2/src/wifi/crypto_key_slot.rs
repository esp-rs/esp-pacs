#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cryptographic keys for MPDU encapsulation and decapsulation"]
pub struct CRYPTO_KEY_SLOT {
    addr_low: ADDR_LOW,
    addr_high: ADDR_HIGH,
    key_value: [KEY_VALUE; 8],
}
impl CRYPTO_KEY_SLOT {
    #[doc = "0x00 - Lower four octets of the MAC address"]
    #[inline(always)]
    pub const fn addr_low(&self) -> &ADDR_LOW {
        &self.addr_low
    }
    #[doc = "0x04 - Higher two octets of the MAC address and config bits"]
    #[inline(always)]
    pub const fn addr_high(&self) -> &ADDR_HIGH {
        &self.addr_high
    }
    #[doc = "0x08..0x28 - Actual key data"]
    #[inline(always)]
    pub const fn key_value(&self, n: usize) -> &KEY_VALUE {
        &self.key_value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x28 - Actual key data"]
    #[inline(always)]
    pub fn key_value_iter(&self) -> impl Iterator<Item = &KEY_VALUE> {
        self.key_value.iter()
    }
}
#[doc = "ADDR_LOW (rw) register accessor: Lower four octets of the MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr_low`] module"]
pub type ADDR_LOW = crate::Reg<addr_low::ADDR_LOW_SPEC>;
#[doc = "Lower four octets of the MAC address"]
pub mod addr_low;
#[doc = "ADDR_HIGH (rw) register accessor: Higher two octets of the MAC address and config bits\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr_high`] module"]
pub type ADDR_HIGH = crate::Reg<addr_high::ADDR_HIGH_SPEC>;
#[doc = "Higher two octets of the MAC address and config bits"]
pub mod addr_high;
#[doc = "KEY_VALUE (rw) register accessor: Actual key data\n\nYou can [`read`](crate::Reg::read) this register and get [`key_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_value`] module"]
pub type KEY_VALUE = crate::Reg<key_value::KEY_VALUE_SPEC>;
#[doc = "Actual key data"]
pub mod key_value;
