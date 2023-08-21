#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub bt_select: BT_SELECT,
    #[doc = "0x04 - "]
    pub out: OUT,
    #[doc = "0x08 - "]
    pub out_w1ts: OUT_W1TS,
    #[doc = "0x0c - "]
    pub out_w1tc: OUT_W1TC,
    #[doc = "0x10 - "]
    pub out1: OUT1,
    #[doc = "0x14 - "]
    pub out1_w1ts: OUT1_W1TS,
    #[doc = "0x18 - "]
    pub out1_w1tc: OUT1_W1TC,
    #[doc = "0x1c - "]
    pub sdio_select: SDIO_SELECT,
    #[doc = "0x20 - "]
    pub enable: ENABLE,
    #[doc = "0x24 - "]
    pub enable_w1ts: ENABLE_W1TS,
    #[doc = "0x28 - "]
    pub enable_w1tc: ENABLE_W1TC,
    #[doc = "0x2c - "]
    pub enable1: ENABLE1,
    #[doc = "0x30 - "]
    pub enable1_w1ts: ENABLE1_W1TS,
    #[doc = "0x34 - "]
    pub enable1_w1tc: ENABLE1_W1TC,
    #[doc = "0x38 - "]
    pub strap: STRAP,
    #[doc = "0x3c - "]
    pub in_: IN,
    #[doc = "0x40 - "]
    pub in1: IN1,
    #[doc = "0x44 - "]
    pub status: STATUS,
    #[doc = "0x48 - "]
    pub status_w1ts: STATUS_W1TS,
    #[doc = "0x4c - "]
    pub status_w1tc: STATUS_W1TC,
    #[doc = "0x50 - "]
    pub status1: STATUS1,
    #[doc = "0x54 - "]
    pub status1_w1ts: STATUS1_W1TS,
    #[doc = "0x58 - "]
    pub status1_w1tc: STATUS1_W1TC,
    _reserved23: [u8; 0x04],
    #[doc = "0x60 - "]
    pub acpu_int: ACPU_INT,
    #[doc = "0x64 - "]
    pub acpu_nmi_int: ACPU_NMI_INT,
    #[doc = "0x68 - "]
    pub pcpu_int: PCPU_INT,
    #[doc = "0x6c - "]
    pub pcpu_nmi_int: PCPU_NMI_INT,
    #[doc = "0x70 - "]
    pub cpusdio_int: CPUSDIO_INT,
    #[doc = "0x74 - "]
    pub acpu_int1: ACPU_INT1,
    #[doc = "0x78 - "]
    pub acpu_nmi_int1: ACPU_NMI_INT1,
    #[doc = "0x7c - "]
    pub pcpu_int1: PCPU_INT1,
    #[doc = "0x80 - "]
    pub pcpu_nmi_int1: PCPU_NMI_INT1,
    #[doc = "0x84 - "]
    pub cpusdio_int1: CPUSDIO_INT1,
    #[doc = "0x88..0x128 - "]
    pub pin: [PIN; 40],
    #[doc = "0x128 - "]
    pub cali_conf: CALI_CONF,
    #[doc = "0x12c - "]
    pub cali_data: CALI_DATA,
    #[doc = "0x130..0x530 - "]
    pub func_in_sel_cfg: [FUNC_IN_SEL_CFG; 256],
    #[doc = "0x530..0x5d0 - "]
    pub func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 40],
}
#[doc = "BT_SELECT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`bt_select`] module"]
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
#[doc = ""]
pub mod bt_select;
#[doc = "OUT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out`] module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = ""]
pub mod out;
#[doc = "OUT_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_w1ts`] module"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = ""]
pub mod out_w1ts;
#[doc = "OUT_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_w1tc`] module"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = ""]
pub mod out_w1tc;
#[doc = "OUT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out1`] module"]
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
#[doc = ""]
pub mod out1;
#[doc = "OUT1_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out1_w1ts`] module"]
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
#[doc = ""]
pub mod out1_w1ts;
#[doc = "OUT1_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out1_w1tc`] module"]
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
#[doc = ""]
pub mod out1_w1tc;
#[doc = "SDIO_SELECT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sdio_select`] module"]
pub type SDIO_SELECT = crate::Reg<sdio_select::SDIO_SELECT_SPEC>;
#[doc = ""]
pub mod sdio_select;
#[doc = "ENABLE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable`] module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = ""]
pub mod enable;
#[doc = "ENABLE_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_w1ts`] module"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = ""]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = ""]
pub mod enable_w1tc;
#[doc = "ENABLE1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable1`] module"]
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
#[doc = ""]
pub mod enable1;
#[doc = "ENABLE1_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable1_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable1_w1ts`] module"]
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
#[doc = ""]
pub mod enable1_w1ts;
#[doc = "ENABLE1_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable1_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`enable1_w1tc`] module"]
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
#[doc = ""]
pub mod enable1_w1tc;
#[doc = "STRAP (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`strap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`strap`] module"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = ""]
pub mod strap;
#[doc = "IN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = ""]
pub mod in_;
#[doc = "IN1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in1`] module"]
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
#[doc = ""]
pub mod in1;
#[doc = "STATUS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "STATUS_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = ""]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = ""]
pub mod status_w1tc;
#[doc = "STATUS1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status1`] module"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = ""]
pub mod status1;
#[doc = "STATUS1_W1TS (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1_w1ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status1_w1ts`] module"]
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
#[doc = ""]
pub mod status1_w1ts;
#[doc = "STATUS1_W1TC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status1_w1tc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1tc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status1_w1tc`] module"]
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
#[doc = ""]
pub mod status1_w1tc;
#[doc = "ACPU_INT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpu_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`acpu_int`] module"]
pub type ACPU_INT = crate::Reg<acpu_int::ACPU_INT_SPEC>;
#[doc = ""]
pub mod acpu_int;
#[doc = "ACPU_NMI_INT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpu_nmi_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`acpu_nmi_int`] module"]
pub type ACPU_NMI_INT = crate::Reg<acpu_nmi_int::ACPU_NMI_INT_SPEC>;
#[doc = ""]
pub mod acpu_nmi_int;
#[doc = "PCPU_INT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcpu_int`] module"]
pub type PCPU_INT = crate::Reg<pcpu_int::PCPU_INT_SPEC>;
#[doc = ""]
pub mod pcpu_int;
#[doc = "PCPU_NMI_INT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcpu_nmi_int`] module"]
pub type PCPU_NMI_INT = crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>;
#[doc = ""]
pub mod pcpu_nmi_int;
#[doc = "CPUSDIO_INT (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpusdio_int`] module"]
pub type CPUSDIO_INT = crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>;
#[doc = ""]
pub mod cpusdio_int;
#[doc = "ACPU_INT1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpu_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`acpu_int1`] module"]
pub type ACPU_INT1 = crate::Reg<acpu_int1::ACPU_INT1_SPEC>;
#[doc = ""]
pub mod acpu_int1;
#[doc = "ACPU_NMI_INT1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpu_nmi_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`acpu_nmi_int1`] module"]
pub type ACPU_NMI_INT1 = crate::Reg<acpu_nmi_int1::ACPU_NMI_INT1_SPEC>;
#[doc = ""]
pub mod acpu_nmi_int1;
#[doc = "PCPU_INT1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcpu_int1`] module"]
pub type PCPU_INT1 = crate::Reg<pcpu_int1::PCPU_INT1_SPEC>;
#[doc = ""]
pub mod pcpu_int1;
#[doc = "PCPU_NMI_INT1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcpu_nmi_int1`] module"]
pub type PCPU_NMI_INT1 = crate::Reg<pcpu_nmi_int1::PCPU_NMI_INT1_SPEC>;
#[doc = ""]
pub mod pcpu_nmi_int1;
#[doc = "CPUSDIO_INT1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpusdio_int1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cpusdio_int1`] module"]
pub type CPUSDIO_INT1 = crate::Reg<cpusdio_int1::CPUSDIO_INT1_SPEC>;
#[doc = ""]
pub mod cpusdio_int1;
#[doc = "PIN (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = ""]
pub mod pin;
#[doc = "cali_conf (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cali_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cali_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cali_conf`] module"]
pub type CALI_CONF = crate::Reg<cali_conf::CALI_CONF_SPEC>;
#[doc = ""]
pub mod cali_conf;
#[doc = "cali_data (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cali_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cali_data`] module"]
pub type CALI_DATA = crate::Reg<cali_data::CALI_DATA_SPEC>;
#[doc = ""]
pub mod cali_data;
#[doc = "FUNC_IN_SEL_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`func_in_sel_cfg`] module"]
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
#[doc = ""]
pub mod func_in_sel_cfg;
#[doc = "FUNC_OUT_SEL_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`func_out_sel_cfg`] module"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = ""]
pub mod func_out_sel_cfg;
