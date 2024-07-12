#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    id0: ID0,
    _reserved1: [u8; 0x04],
    compver0: COMPVER0,
    _reserved2: [u8; 0x04],
    cfg0: CFG0,
    _reserved3: [u8; 0x04],
    chen0: CHEN0,
    chen1: CHEN1,
    _reserved5: [u8; 0x10],
    intstatus0: INTSTATUS0,
    _reserved6: [u8; 0x04],
    commonreg_intclear0: COMMONREG_INTCLEAR0,
    _reserved7: [u8; 0x04],
    commonreg_intstatus_enable0: COMMONREG_INTSTATUS_ENABLE0,
    _reserved8: [u8; 0x04],
    commonreg_intsignal_enable0: COMMONREG_INTSIGNAL_ENABLE0,
    _reserved9: [u8; 0x04],
    commonreg_intstatus0: COMMONREG_INTSTATUS0,
    _reserved10: [u8; 0x04],
    reset0: RESET0,
    _reserved11: [u8; 0x04],
    lowpower_cfg0: LOWPOWER_CFG0,
    lowpower_cfg1: LOWPOWER_CFG1,
    _reserved13: [u8; 0x98],
    ch: [CH; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - NA"]
    #[inline(always)]
    pub const fn id0(&self) -> &ID0 {
        &self.id0
    }
    #[doc = "0x08 - NA"]
    #[inline(always)]
    pub const fn compver0(&self) -> &COMPVER0 {
        &self.compver0
    }
    #[doc = "0x10 - NA"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &CFG0 {
        &self.cfg0
    }
    #[doc = "0x18 - NA"]
    #[inline(always)]
    pub const fn chen0(&self) -> &CHEN0 {
        &self.chen0
    }
    #[doc = "0x1c - NA"]
    #[inline(always)]
    pub const fn chen1(&self) -> &CHEN1 {
        &self.chen1
    }
    #[doc = "0x30 - NA"]
    #[inline(always)]
    pub const fn intstatus0(&self) -> &INTSTATUS0 {
        &self.intstatus0
    }
    #[doc = "0x38 - NA"]
    #[inline(always)]
    pub const fn commonreg_intclear0(&self) -> &COMMONREG_INTCLEAR0 {
        &self.commonreg_intclear0
    }
    #[doc = "0x40 - NA"]
    #[inline(always)]
    pub const fn commonreg_intstatus_enable0(&self) -> &COMMONREG_INTSTATUS_ENABLE0 {
        &self.commonreg_intstatus_enable0
    }
    #[doc = "0x48 - NA"]
    #[inline(always)]
    pub const fn commonreg_intsignal_enable0(&self) -> &COMMONREG_INTSIGNAL_ENABLE0 {
        &self.commonreg_intsignal_enable0
    }
    #[doc = "0x50 - NA"]
    #[inline(always)]
    pub const fn commonreg_intstatus0(&self) -> &COMMONREG_INTSTATUS0 {
        &self.commonreg_intstatus0
    }
    #[doc = "0x58 - NA"]
    #[inline(always)]
    pub const fn reset0(&self) -> &RESET0 {
        &self.reset0
    }
    #[doc = "0x60 - NA"]
    #[inline(always)]
    pub const fn lowpower_cfg0(&self) -> &LOWPOWER_CFG0 {
        &self.lowpower_cfg0
    }
    #[doc = "0x64 - NA"]
    #[inline(always)]
    pub const fn lowpower_cfg1(&self) -> &LOWPOWER_CFG1 {
        &self.lowpower_cfg1
    }
    #[doc = "0x100..0x500 - Cluster CH%s, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x500 - Cluster CH%s, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x100..0x200 - Cluster CH1, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[inline(always)]
    pub const fn ch1(&self) -> &CH {
        self.ch(0)
    }
    #[doc = "0x200..0x300 - Cluster CH2, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[inline(always)]
    pub const fn ch2(&self) -> &CH {
        self.ch(1)
    }
    #[doc = "0x300..0x400 - Cluster CH3, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[inline(always)]
    pub const fn ch3(&self) -> &CH {
        self.ch(2)
    }
    #[doc = "0x400..0x500 - Cluster CH4, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
    #[inline(always)]
    pub const fn ch4(&self) -> &CH {
        self.ch(3)
    }
}
#[doc = "ID0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`id0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0`] module"]
pub type ID0 = crate::Reg<id0::ID0_SPEC>;
#[doc = "NA"]
pub mod id0;
#[doc = "COMPVER0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`compver0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compver0`] module"]
pub type COMPVER0 = crate::Reg<compver0::COMPVER0_SPEC>;
#[doc = "NA"]
pub mod compver0;
#[doc = "CFG0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`] module"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "NA"]
pub mod cfg0;
#[doc = "CHEN0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`chen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen0`] module"]
pub type CHEN0 = crate::Reg<chen0::CHEN0_SPEC>;
#[doc = "NA"]
pub mod chen0;
#[doc = "CHEN1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`chen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen1`] module"]
pub type CHEN1 = crate::Reg<chen1::CHEN1_SPEC>;
#[doc = "NA"]
pub mod chen1;
#[doc = "INTSTATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus0`] module"]
pub type INTSTATUS0 = crate::Reg<intstatus0::INTSTATUS0_SPEC>;
#[doc = "NA"]
pub mod intstatus0;
#[doc = "COMMONREG_INTCLEAR0 (w) register accessor: NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`commonreg_intclear0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intclear0`] module"]
pub type COMMONREG_INTCLEAR0 = crate::Reg<commonreg_intclear0::COMMONREG_INTCLEAR0_SPEC>;
#[doc = "NA"]
pub mod commonreg_intclear0;
#[doc = "COMMONREG_INTSTATUS_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`commonreg_intstatus_enable0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`commonreg_intstatus_enable0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intstatus_enable0`] module"]
pub type COMMONREG_INTSTATUS_ENABLE0 =
    crate::Reg<commonreg_intstatus_enable0::COMMONREG_INTSTATUS_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod commonreg_intstatus_enable0;
#[doc = "COMMONREG_INTSIGNAL_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`commonreg_intsignal_enable0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`commonreg_intsignal_enable0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intsignal_enable0`] module"]
pub type COMMONREG_INTSIGNAL_ENABLE0 =
    crate::Reg<commonreg_intsignal_enable0::COMMONREG_INTSIGNAL_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod commonreg_intsignal_enable0;
#[doc = "COMMONREG_INTSTATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`commonreg_intstatus0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intstatus0`] module"]
pub type COMMONREG_INTSTATUS0 = crate::Reg<commonreg_intstatus0::COMMONREG_INTSTATUS0_SPEC>;
#[doc = "NA"]
pub mod commonreg_intstatus0;
#[doc = "RESET0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`reset0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset0`] module"]
pub type RESET0 = crate::Reg<reset0::RESET0_SPEC>;
#[doc = "NA"]
pub mod reset0;
#[doc = "LOWPOWER_CFG0 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpower_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpower_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowpower_cfg0`] module"]
pub type LOWPOWER_CFG0 = crate::Reg<lowpower_cfg0::LOWPOWER_CFG0_SPEC>;
#[doc = "NA"]
pub mod lowpower_cfg0;
#[doc = "LOWPOWER_CFG1 (rw) register accessor: NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpower_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpower_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowpower_cfg1`] module"]
pub type LOWPOWER_CFG1 = crate::Reg<lowpower_cfg1::LOWPOWER_CFG1_SPEC>;
#[doc = "NA"]
pub mod lowpower_cfg1;
#[doc = "Cluster CH%s, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing CH?_SAR0, CH?_SAR1, CH?_DAR0, CH?_DAR1, CH?_BLOCK_TS0, CH?_CTL0, CH?_CTL1, CH?_CFG0, CH?_CFG1, CH?_LLP0, CH?_LLP1, CH?_STATUS0, CH?_STATUS1, CH?_SWHSSRC0, CH?_SWHSDST0, CH?_BLK_TFR_RESUMEREQ0, CH?_AXI_ID0, CH?_AXI_QOS0, CH?_SSTAT0, CH?_DSTAT0, CH?_SSTATAR0, CH?_SSTATAR1, CH?_DSTATAR0, CH?_DSTATAR1, CH?_INTSTATUS_ENABLE0, CH?_INTSTATUS_ENABLE1, CH?_INTSTATUS0, CH?_INTSTATUS1, CH?_INTSIGNAL_ENABLE0, CH?_INTSIGNAL_ENABLE1, CH?_INTCLEAR0, CH?_INTCLEAR1"]
pub mod ch;
