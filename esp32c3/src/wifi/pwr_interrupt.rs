#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Status and clear for the WIFI_PWR interrupt"]
pub struct PWR_INTERRUPT {
    pwr_int_status: PWR_INT_STATUS,
    pwr_int_clear: PWR_INT_CLEAR,
}
impl PWR_INTERRUPT {
    #[doc = "0x00 - Interrupt status for the WIFI_PWR interrupt"]
    #[inline(always)]
    pub const fn pwr_int_status(&self) -> &PWR_INT_STATUS {
        &self.pwr_int_status
    }
    #[doc = "0x04 - Interrupt status clear for the WIFI_PWR interrupt"]
    #[inline(always)]
    pub const fn pwr_int_clear(&self) -> &PWR_INT_CLEAR {
        &self.pwr_int_clear
    }
}
#[doc = "PWR_INT_STATUS (rw) register accessor: Interrupt status for the WIFI_PWR interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_int_status`] module"]
pub type PWR_INT_STATUS = crate::Reg<pwr_int_status::PWR_INT_STATUS_SPEC>;
#[doc = "Interrupt status for the WIFI_PWR interrupt"]
pub mod pwr_int_status;
#[doc = "PWR_INT_CLEAR (rw) register accessor: Interrupt status clear for the WIFI_PWR interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_int_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_int_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_int_clear`] module"]
pub type PWR_INT_CLEAR = crate::Reg<pwr_int_clear::PWR_INT_CLEAR_SPEC>;
#[doc = "Interrupt status clear for the WIFI_PWR interrupt"]
pub mod pwr_int_clear;
