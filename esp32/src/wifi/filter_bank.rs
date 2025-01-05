#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for two interfaces."]
pub struct FILTER_BANK {
    dummy: DUMMY,
    _reserved_end: [u8; 0x3c],
}
impl FILTER_BANK {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn dummy(&self) -> &DUMMY {
        &self.dummy
    }
}
#[doc = "DUMMY (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dummy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dummy`] module"]
pub type DUMMY = crate::Reg<dummy::DUMMY_SPEC>;
#[doc = ""]
pub mod dummy;
