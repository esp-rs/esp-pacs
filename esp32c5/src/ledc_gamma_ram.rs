#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch_gamma_range: [CH_GAMMA_RANGE; 96],
}
impl RegisterBlock {
    #[doc = "0x00..0x180 - LEDC gamma fade configuration RAM entry %s."]
    #[inline(always)]
    pub const fn ch_gamma_range(&self, n: usize) -> &CH_GAMMA_RANGE {
        &self.ch_gamma_range[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x180 - LEDC gamma fade configuration RAM entry %s."]
    #[inline(always)]
    pub fn ch_gamma_range_iter(&self) -> impl Iterator<Item = &CH_GAMMA_RANGE> {
        self.ch_gamma_range.iter()
    }
}
#[doc = "CH_GAMMA_RANGE (rw) register accessor: LEDC gamma fade configuration RAM entry %s.\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_gamma_range::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_gamma_range::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_gamma_range`] module"]
pub type CH_GAMMA_RANGE = crate::Reg<ch_gamma_range::CH_GAMMA_RANGE_SPEC>;
#[doc = "LEDC gamma fade configuration RAM entry %s."]
pub mod ch_gamma_range;
