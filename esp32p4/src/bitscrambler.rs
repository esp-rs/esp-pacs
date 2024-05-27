#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    tx_inst_cfg0: TX_INST_CFG0,
    tx_inst_cfg1: TX_INST_CFG1,
    rx_inst_cfg0: RX_INST_CFG0,
    rx_inst_cfg1: RX_INST_CFG1,
    tx_lut_cfg0: TX_LUT_CFG0,
    tx_lut_cfg1: TX_LUT_CFG1,
    rx_lut_cfg0: RX_LUT_CFG0,
    rx_lut_cfg1: RX_LUT_CFG1,
    tx_tailing_bits: TX_TAILING_BITS,
    rx_tailing_bits: RX_TAILING_BITS,
    tx_ctrl: TX_CTRL,
    rx_ctrl: RX_CTRL,
    tx_state: TX_STATE,
    rx_state: RX_STATE,
    _reserved14: [u8; 0xc0],
    sys: SYS,
    version: VERSION,
}
impl RegisterBlock {
    ///0x00 - Control and configuration registers
    #[inline(always)]
    pub const fn tx_inst_cfg0(&self) -> &TX_INST_CFG0 {
        &self.tx_inst_cfg0
    }
    ///0x04 - Control and configuration registers
    #[inline(always)]
    pub const fn tx_inst_cfg1(&self) -> &TX_INST_CFG1 {
        &self.tx_inst_cfg1
    }
    ///0x08 - Control and configuration registers
    #[inline(always)]
    pub const fn rx_inst_cfg0(&self) -> &RX_INST_CFG0 {
        &self.rx_inst_cfg0
    }
    ///0x0c - Control and configuration registers
    #[inline(always)]
    pub const fn rx_inst_cfg1(&self) -> &RX_INST_CFG1 {
        &self.rx_inst_cfg1
    }
    ///0x10 - Control and configuration registers
    #[inline(always)]
    pub const fn tx_lut_cfg0(&self) -> &TX_LUT_CFG0 {
        &self.tx_lut_cfg0
    }
    ///0x14 - Control and configuration registers
    #[inline(always)]
    pub const fn tx_lut_cfg1(&self) -> &TX_LUT_CFG1 {
        &self.tx_lut_cfg1
    }
    ///0x18 - Control and configuration registers
    #[inline(always)]
    pub const fn rx_lut_cfg0(&self) -> &RX_LUT_CFG0 {
        &self.rx_lut_cfg0
    }
    ///0x1c - Control and configuration registers
    #[inline(always)]
    pub const fn rx_lut_cfg1(&self) -> &RX_LUT_CFG1 {
        &self.rx_lut_cfg1
    }
    ///0x20 - Control and configuration registers
    #[inline(always)]
    pub const fn tx_tailing_bits(&self) -> &TX_TAILING_BITS {
        &self.tx_tailing_bits
    }
    ///0x24 - Control and configuration registers
    #[inline(always)]
    pub const fn rx_tailing_bits(&self) -> &RX_TAILING_BITS {
        &self.rx_tailing_bits
    }
    ///0x28 - Control and configuration registers
    #[inline(always)]
    pub const fn tx_ctrl(&self) -> &TX_CTRL {
        &self.tx_ctrl
    }
    ///0x2c - Control and configuration registers
    #[inline(always)]
    pub const fn rx_ctrl(&self) -> &RX_CTRL {
        &self.rx_ctrl
    }
    ///0x30 - Status registers
    #[inline(always)]
    pub const fn tx_state(&self) -> &TX_STATE {
        &self.tx_state
    }
    ///0x34 - Status registers
    #[inline(always)]
    pub const fn rx_state(&self) -> &RX_STATE {
        &self.rx_state
    }
    ///0xf8 - Control and configuration registers
    #[inline(always)]
    pub const fn sys(&self) -> &SYS {
        &self.sys
    }
    ///0xfc - Control and configuration registers
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
/**TX_INST_CFG0 (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`tx_inst_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_inst_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_inst_cfg0`] module*/
pub type TX_INST_CFG0 = crate::Reg<tx_inst_cfg0::TX_INST_CFG0_SPEC>;
///Control and configuration registers
pub mod tx_inst_cfg0;
/**TX_INST_CFG1 (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`tx_inst_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_inst_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_inst_cfg1`] module*/
pub type TX_INST_CFG1 = crate::Reg<tx_inst_cfg1::TX_INST_CFG1_SPEC>;
///Control and configuration registers
pub mod tx_inst_cfg1;
/**RX_INST_CFG0 (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`rx_inst_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_inst_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_inst_cfg0`] module*/
pub type RX_INST_CFG0 = crate::Reg<rx_inst_cfg0::RX_INST_CFG0_SPEC>;
///Control and configuration registers
pub mod rx_inst_cfg0;
/**RX_INST_CFG1 (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`rx_inst_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_inst_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_inst_cfg1`] module*/
pub type RX_INST_CFG1 = crate::Reg<rx_inst_cfg1::RX_INST_CFG1_SPEC>;
///Control and configuration registers
pub mod rx_inst_cfg1;
/**TX_LUT_CFG0 (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`tx_lut_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_lut_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_lut_cfg0`] module*/
pub type TX_LUT_CFG0 = crate::Reg<tx_lut_cfg0::TX_LUT_CFG0_SPEC>;
///Control and configuration registers
pub mod tx_lut_cfg0;
/**TX_LUT_CFG1 (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`tx_lut_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_lut_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_lut_cfg1`] module*/
pub type TX_LUT_CFG1 = crate::Reg<tx_lut_cfg1::TX_LUT_CFG1_SPEC>;
///Control and configuration registers
pub mod tx_lut_cfg1;
/**RX_LUT_CFG0 (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`rx_lut_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_lut_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_lut_cfg0`] module*/
pub type RX_LUT_CFG0 = crate::Reg<rx_lut_cfg0::RX_LUT_CFG0_SPEC>;
///Control and configuration registers
pub mod rx_lut_cfg0;
/**RX_LUT_CFG1 (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`rx_lut_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_lut_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_lut_cfg1`] module*/
pub type RX_LUT_CFG1 = crate::Reg<rx_lut_cfg1::RX_LUT_CFG1_SPEC>;
///Control and configuration registers
pub mod rx_lut_cfg1;
/**TX_TAILING_BITS (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`tx_tailing_bits::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_tailing_bits::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_tailing_bits`] module*/
pub type TX_TAILING_BITS = crate::Reg<tx_tailing_bits::TX_TAILING_BITS_SPEC>;
///Control and configuration registers
pub mod tx_tailing_bits;
/**RX_TAILING_BITS (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`rx_tailing_bits::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_tailing_bits::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_tailing_bits`] module*/
pub type RX_TAILING_BITS = crate::Reg<rx_tailing_bits::RX_TAILING_BITS_SPEC>;
///Control and configuration registers
pub mod rx_tailing_bits;
/**TX_CTRL (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`tx_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_ctrl`] module*/
pub type TX_CTRL = crate::Reg<tx_ctrl::TX_CTRL_SPEC>;
///Control and configuration registers
pub mod tx_ctrl;
/**RX_CTRL (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`rx_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_ctrl`] module*/
pub type RX_CTRL = crate::Reg<rx_ctrl::RX_CTRL_SPEC>;
///Control and configuration registers
pub mod rx_ctrl;
/**TX_STATE (rw) register accessor: Status registers

You can [`read`](crate::generic::Reg::read) this register and get [`tx_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_state`] module*/
pub type TX_STATE = crate::Reg<tx_state::TX_STATE_SPEC>;
///Status registers
pub mod tx_state;
/**RX_STATE (rw) register accessor: Status registers

You can [`read`](crate::generic::Reg::read) this register and get [`rx_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_state`] module*/
pub type RX_STATE = crate::Reg<rx_state::RX_STATE_SPEC>;
///Status registers
pub mod rx_state;
/**SYS (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`sys::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sys`] module*/
pub type SYS = crate::Reg<sys::SYS_SPEC>;
///Control and configuration registers
pub mod sys;
/**VERSION (rw) register accessor: Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@version`] module*/
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
///Control and configuration registers
pub mod version;
