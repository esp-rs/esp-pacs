#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    verid_fileds: VERID_FILEDS,
    hw_cfg: HW_CFG,
    cmd: CMD,
    data: DATA,
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn verid_fileds(&self) -> &VERID_FILEDS {
        &self.verid_fileds
    }
    #[doc = "0x04 - NA"]
    #[inline(always)]
    pub const fn hw_cfg(&self) -> &HW_CFG {
        &self.hw_cfg
    }
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x0c - NA"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
}
#[doc = "VERID_FILEDS (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verid_fileds::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid_fileds`] module"]
pub type VERID_FILEDS = crate::Reg<verid_fileds::VERID_FILEDS_SPEC>;
#[doc = "NA"]
pub mod verid_fileds;
#[doc = "HW_CFG (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_cfg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hw_cfg`] module"]
pub type HW_CFG = crate::Reg<hw_cfg::HW_CFG_SPEC>;
#[doc = "NA"]
pub mod hw_cfg;
#[doc = "CMD (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "NA"]
pub mod cmd;
#[doc = "DATA (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "NA"]
pub mod data;
