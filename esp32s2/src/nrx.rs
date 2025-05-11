#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xd4],
    nrxpd_ctrl: NRXPD_CTRL,
    _reserved1: [u8; 0x37],
    chan_dump_cfg: CHAN_DUMP_CFG,
}
impl RegisterBlock {
    #[doc = "0xd4 - NRX Power Down Control Register"]
    #[inline(always)]
    pub const fn nrxpd_ctrl(&self) -> &NRXPD_CTRL {
        &self.nrxpd_ctrl
    }
    #[doc = "0x10c - Configuration of channel info dumping"]
    #[inline(always)]
    pub const fn chan_dump_cfg(&self) -> &CHAN_DUMP_CFG {
        &self.chan_dump_cfg
    }
}
#[doc = "NRXPD_CTRL (rw) register accessor: NRX Power Down Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`nrxpd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrxpd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nrxpd_ctrl`] module"]
pub type NRXPD_CTRL = crate::Reg<nrxpd_ctrl::NRXPD_CTRL_SPEC>;
#[doc = "NRX Power Down Control Register"]
pub mod nrxpd_ctrl;
#[doc = "CHAN_DUMP_CFG (rw) register accessor: Configuration of channel info dumping\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_dump_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chan_dump_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_dump_cfg`] module"]
pub type CHAN_DUMP_CFG = crate::Reg<chan_dump_cfg::CHAN_DUMP_CFG_SPEC>;
#[doc = "Configuration of channel info dumping"]
pub mod chan_dump_cfg;
