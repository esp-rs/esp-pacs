#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    verid_fileds: VERID_FILEDS,
    hw_cfg: HW_CFG,
    cmd: CMD,
    data: DATA,
}
impl RegisterBlock {
    ///0x00 - NA
    #[inline(always)]
    pub const fn verid_fileds(&self) -> &VERID_FILEDS {
        &self.verid_fileds
    }
    ///0x04 - NA
    #[inline(always)]
    pub const fn hw_cfg(&self) -> &HW_CFG {
        &self.hw_cfg
    }
    ///0x08 - NA
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    ///0x0c - NA
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
}
/**VERID_FILEDS (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`verid_fileds::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@verid_fileds`] module*/
pub type VERID_FILEDS = crate::Reg<verid_fileds::VERID_FILEDS_SPEC>;
///NA
pub mod verid_fileds;
/**HW_CFG (r) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`hw_cfg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hw_cfg`] module*/
pub type HW_CFG = crate::Reg<hw_cfg::HW_CFG_SPEC>;
///NA
pub mod hw_cfg;
/**CMD (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmd`] module*/
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///NA
pub mod cmd;
/**DATA (rw) register accessor: NA

You can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@data`] module*/
pub type DATA = crate::Reg<data::DATA_SPEC>;
///NA
pub mod data;
