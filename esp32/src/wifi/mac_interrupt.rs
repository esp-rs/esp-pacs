#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Status and clear for the Wi-Fi MAC interrupt"]
pub struct MAC_INTERRUPT {
    wifi_int_status: WIFI_INT_STATUS,
    wifi_int_clear: WIFI_INT_CLEAR,
}
impl MAC_INTERRUPT {
    #[doc = "0x00 - Interrupt status of WIFI peripheral"]
    #[inline(always)]
    pub const fn wifi_int_status(&self) -> &WIFI_INT_STATUS {
        &self.wifi_int_status
    }
    #[doc = "0x04 - Interrupt status clear of WIFI peripheral"]
    #[inline(always)]
    pub const fn wifi_int_clear(&self) -> &WIFI_INT_CLEAR {
        &self.wifi_int_clear
    }
}
#[doc = "WIFI_INT_STATUS (rw) register accessor: Interrupt status of WIFI peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_int_status`] module"]
pub type WIFI_INT_STATUS = crate::Reg<wifi_int_status::WIFI_INT_STATUS_SPEC>;
#[doc = "Interrupt status of WIFI peripheral"]
pub mod wifi_int_status;
#[doc = "WIFI_INT_CLEAR (rw) register accessor: Interrupt status clear of WIFI peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_int_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_int_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_int_clear`] module"]
pub type WIFI_INT_CLEAR = crate::Reg<wifi_int_clear::WIFI_INT_CLEAR_SPEC>;
#[doc = "Interrupt status clear of WIFI peripheral"]
pub mod wifi_int_clear;
