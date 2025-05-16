#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Control registers for hardware crypto"]
pub struct CRYPTO_CONTROL {
    interface_crypto_control: [INTERFACE_CRYPTO_CONTROL; 4],
    general_crypto_control: GENERAL_CRYPTO_CONTROL,
    crypto_key_slot_state: CRYPTO_KEY_SLOT_STATE,
}
impl CRYPTO_CONTROL {
    #[doc = "0x00..0x10 - Control over cryptographic parameters for a specific interface"]
    #[inline(always)]
    pub const fn interface_crypto_control(&self, n: usize) -> &INTERFACE_CRYPTO_CONTROL {
        &self.interface_crypto_control[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Control over cryptographic parameters for a specific interface"]
    #[inline(always)]
    pub fn interface_crypto_control_iter(&self) -> impl Iterator<Item = &INTERFACE_CRYPTO_CONTROL> {
        self.interface_crypto_control.iter()
    }
    #[doc = "0x10 - Control over the whole crypto unit"]
    #[inline(always)]
    pub const fn general_crypto_control(&self) -> &GENERAL_CRYPTO_CONTROL {
        &self.general_crypto_control
    }
    #[doc = "0x14 - State of cryptographic key slots"]
    #[inline(always)]
    pub const fn crypto_key_slot_state(&self) -> &CRYPTO_KEY_SLOT_STATE {
        &self.crypto_key_slot_state
    }
}
#[doc = "INTERFACE_CRYPTO_CONTROL (rw) register accessor: Control over cryptographic parameters for a specific interface\n\nYou can [`read`](crate::Reg::read) this register and get [`interface_crypto_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interface_crypto_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interface_crypto_control`] module"]
pub type INTERFACE_CRYPTO_CONTROL =
    crate::Reg<interface_crypto_control::INTERFACE_CRYPTO_CONTROL_SPEC>;
#[doc = "Control over cryptographic parameters for a specific interface"]
pub mod interface_crypto_control;
#[doc = "GENERAL_CRYPTO_CONTROL (rw) register accessor: Control over the whole crypto unit\n\nYou can [`read`](crate::Reg::read) this register and get [`general_crypto_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`general_crypto_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@general_crypto_control`] module"]
pub type GENERAL_CRYPTO_CONTROL = crate::Reg<general_crypto_control::GENERAL_CRYPTO_CONTROL_SPEC>;
#[doc = "Control over the whole crypto unit"]
pub mod general_crypto_control;
#[doc = "CRYPTO_KEY_SLOT_STATE (rw) register accessor: State of cryptographic key slots\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_key_slot_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_key_slot_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_key_slot_state`] module"]
pub type CRYPTO_KEY_SLOT_STATE = crate::Reg<crypto_key_slot_state::CRYPTO_KEY_SLOT_STATE_SPEC>;
#[doc = "State of cryptographic key slots"]
pub mod crypto_key_slot_state;
