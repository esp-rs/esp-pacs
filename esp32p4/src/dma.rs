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
    ch1_sar0: CH1_SAR0,
    ch1_sar1: CH1_SAR1,
    ch1_dar0: CH1_DAR0,
    ch1_dar1: CH1_DAR1,
    ch1_block_ts0: CH1_BLOCK_TS0,
    _reserved18: [u8; 0x04],
    ch1_ctl0: CH1_CTL0,
    ch1_ctl1: CH1_CTL1,
    ch1_cfg0: CH1_CFG0,
    ch1_cfg1: CH1_CFG1,
    ch1_llp0: CH1_LLP0,
    ch1_llp1: CH1_LLP1,
    ch1_status0: CH1_STATUS0,
    ch1_status1: CH1_STATUS1,
    ch1_swhssrc0: CH1_SWHSSRC0,
    _reserved27: [u8; 0x04],
    ch1_swhsdst0: CH1_SWHSDST0,
    _reserved28: [u8; 0x04],
    ch1_blk_tfr_resumereq0: CH1_BLK_TFR_RESUMEREQ0,
    _reserved29: [u8; 0x04],
    ch1_axi_id0: CH1_AXI_ID0,
    _reserved30: [u8; 0x04],
    ch1_axi_qos0: CH1_AXI_QOS0,
    _reserved31: [u8; 0x04],
    ch1_sstat0: CH1_SSTAT0,
    _reserved32: [u8; 0x04],
    ch1_dstat0: CH1_DSTAT0,
    _reserved33: [u8; 0x04],
    ch1_sstatar0: CH1_SSTATAR0,
    ch1_sstatar1: CH1_SSTATAR1,
    ch1_dstatar0: CH1_DSTATAR0,
    ch1_dstatar1: CH1_DSTATAR1,
    ch1_intstatus_enable0: CH1_INTSTATUS_ENABLE0,
    ch1_intstatus_enable1: CH1_INTSTATUS_ENABLE1,
    ch1_intstatus0: CH1_INTSTATUS0,
    ch1_intstatus1: CH1_INTSTATUS1,
    ch1_intsignal_enable0: CH1_INTSIGNAL_ENABLE0,
    ch1_intsignal_enable1: CH1_INTSIGNAL_ENABLE1,
    ch1_intclear0: CH1_INTCLEAR0,
    ch1_intclear1: CH1_INTCLEAR1,
    _reserved45: [u8; 0x60],
    ch2_sar0: CH2_SAR0,
    ch2_sar1: CH2_SAR1,
    ch2_dar0: CH2_DAR0,
    ch2_dar1: CH2_DAR1,
    ch2_block_ts0: CH2_BLOCK_TS0,
    _reserved50: [u8; 0x04],
    ch2_ctl0: CH2_CTL0,
    ch2_ctl1: CH2_CTL1,
    ch2_cfg0: CH2_CFG0,
    ch2_cfg1: CH2_CFG1,
    ch2_llp0: CH2_LLP0,
    ch2_llp1: CH2_LLP1,
    ch2_status0: CH2_STATUS0,
    ch2_status1: CH2_STATUS1,
    ch2_swhssrc0: CH2_SWHSSRC0,
    _reserved59: [u8; 0x04],
    ch2_swhsdst0: CH2_SWHSDST0,
    _reserved60: [u8; 0x04],
    ch2_blk_tfr_resumereq0: CH2_BLK_TFR_RESUMEREQ0,
    _reserved61: [u8; 0x04],
    ch2_axi_id0: CH2_AXI_ID0,
    _reserved62: [u8; 0x04],
    ch2_axi_qos0: CH2_AXI_QOS0,
    _reserved63: [u8; 0x04],
    ch2_sstat0: CH2_SSTAT0,
    _reserved64: [u8; 0x04],
    ch2_dstat0: CH2_DSTAT0,
    _reserved65: [u8; 0x04],
    ch2_sstatar0: CH2_SSTATAR0,
    ch2_sstatar1: CH2_SSTATAR1,
    ch2_dstatar0: CH2_DSTATAR0,
    ch2_dstatar1: CH2_DSTATAR1,
    ch2_intstatus_enable0: CH2_INTSTATUS_ENABLE0,
    ch2_intstatus_enable1: CH2_INTSTATUS_ENABLE1,
    ch2_intstatus0: CH2_INTSTATUS0,
    ch2_intstatus1: CH2_INTSTATUS1,
    ch2_intsignal_enable0: CH2_INTSIGNAL_ENABLE0,
    ch2_intsignal_enable1: CH2_INTSIGNAL_ENABLE1,
    ch2_intclear0: CH2_INTCLEAR0,
    ch2_intclear1: CH2_INTCLEAR1,
    _reserved77: [u8; 0x60],
    ch3_sar0: CH3_SAR0,
    ch3_sar1: CH3_SAR1,
    ch3_dar0: CH3_DAR0,
    ch3_dar1: CH3_DAR1,
    ch3_block_ts0: CH3_BLOCK_TS0,
    _reserved82: [u8; 0x04],
    ch3_ctl0: CH3_CTL0,
    ch3_ctl1: CH3_CTL1,
    ch3_cfg0: CH3_CFG0,
    ch3_cfg1: CH3_CFG1,
    ch3_llp0: CH3_LLP0,
    ch3_llp1: CH3_LLP1,
    ch3_status0: CH3_STATUS0,
    ch3_status1: CH3_STATUS1,
    ch3_swhssrc0: CH3_SWHSSRC0,
    _reserved91: [u8; 0x04],
    ch3_swhsdst0: CH3_SWHSDST0,
    _reserved92: [u8; 0x04],
    ch3_blk_tfr_resumereq0: CH3_BLK_TFR_RESUMEREQ0,
    _reserved93: [u8; 0x04],
    ch3_axi_id0: CH3_AXI_ID0,
    _reserved94: [u8; 0x04],
    ch3_axi_qos0: CH3_AXI_QOS0,
    _reserved95: [u8; 0x04],
    ch3_sstat0: CH3_SSTAT0,
    _reserved96: [u8; 0x04],
    ch3_dstat0: CH3_DSTAT0,
    _reserved97: [u8; 0x04],
    ch3_sstatar0: CH3_SSTATAR0,
    ch3_sstatar1: CH3_SSTATAR1,
    ch3_dstatar0: CH3_DSTATAR0,
    ch3_dstatar1: CH3_DSTATAR1,
    ch3_intstatus_enable0: CH3_INTSTATUS_ENABLE0,
    ch3_intstatus_enable1: CH3_INTSTATUS_ENABLE1,
    ch3_intstatus0: CH3_INTSTATUS0,
    ch3_intstatus1: CH3_INTSTATUS1,
    ch3_intsignal_enable0: CH3_INTSIGNAL_ENABLE0,
    ch3_intsignal_enable1: CH3_INTSIGNAL_ENABLE1,
    ch3_intclear0: CH3_INTCLEAR0,
    ch3_intclear1: CH3_INTCLEAR1,
    _reserved109: [u8; 0x60],
    ch4_sar0: CH4_SAR0,
    ch4_sar1: CH4_SAR1,
    ch4_dar0: CH4_DAR0,
    ch4_dar1: CH4_DAR1,
    ch4_block_ts0: CH4_BLOCK_TS0,
    _reserved114: [u8; 0x04],
    ch4_ctl0: CH4_CTL0,
    ch4_ctl1: CH4_CTL1,
    ch4_cfg0: CH4_CFG0,
    ch4_cfg1: CH4_CFG1,
    ch4_llp0: CH4_LLP0,
    ch4_llp1: CH4_LLP1,
    ch4_status0: CH4_STATUS0,
    ch4_status1: CH4_STATUS1,
    ch4_swhssrc0: CH4_SWHSSRC0,
    _reserved123: [u8; 0x04],
    ch4_swhsdst0: CH4_SWHSDST0,
    _reserved124: [u8; 0x04],
    ch4_blk_tfr_resumereq0: CH4_BLK_TFR_RESUMEREQ0,
    _reserved125: [u8; 0x04],
    ch4_axi_id0: CH4_AXI_ID0,
    _reserved126: [u8; 0x04],
    ch4_axi_qos0: CH4_AXI_QOS0,
    _reserved127: [u8; 0x04],
    ch4_sstat0: CH4_SSTAT0,
    _reserved128: [u8; 0x04],
    ch4_dstat0: CH4_DSTAT0,
    _reserved129: [u8; 0x04],
    ch4_sstatar0: CH4_SSTATAR0,
    ch4_sstatar1: CH4_SSTATAR1,
    ch4_dstatar0: CH4_DSTATAR0,
    ch4_dstatar1: CH4_DSTATAR1,
    ch4_intstatus_enable0: CH4_INTSTATUS_ENABLE0,
    ch4_intstatus_enable1: CH4_INTSTATUS_ENABLE1,
    ch4_intstatus0: CH4_INTSTATUS0,
    ch4_intstatus1: CH4_INTSTATUS1,
    ch4_intsignal_enable0: CH4_INTSIGNAL_ENABLE0,
    ch4_intsignal_enable1: CH4_INTSIGNAL_ENABLE1,
    ch4_intclear0: CH4_INTCLEAR0,
    ch4_intclear1: CH4_INTCLEAR1,
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
    #[doc = "0x100 - NA"]
    #[inline(always)]
    pub const fn ch1_sar0(&self) -> &CH1_SAR0 {
        &self.ch1_sar0
    }
    #[doc = "0x104 - NA"]
    #[inline(always)]
    pub const fn ch1_sar1(&self) -> &CH1_SAR1 {
        &self.ch1_sar1
    }
    #[doc = "0x108 - NA"]
    #[inline(always)]
    pub const fn ch1_dar0(&self) -> &CH1_DAR0 {
        &self.ch1_dar0
    }
    #[doc = "0x10c - NA"]
    #[inline(always)]
    pub const fn ch1_dar1(&self) -> &CH1_DAR1 {
        &self.ch1_dar1
    }
    #[doc = "0x110 - NA"]
    #[inline(always)]
    pub const fn ch1_block_ts0(&self) -> &CH1_BLOCK_TS0 {
        &self.ch1_block_ts0
    }
    #[doc = "0x118 - NA"]
    #[inline(always)]
    pub const fn ch1_ctl0(&self) -> &CH1_CTL0 {
        &self.ch1_ctl0
    }
    #[doc = "0x11c - NA"]
    #[inline(always)]
    pub const fn ch1_ctl1(&self) -> &CH1_CTL1 {
        &self.ch1_ctl1
    }
    #[doc = "0x120 - NA"]
    #[inline(always)]
    pub const fn ch1_cfg0(&self) -> &CH1_CFG0 {
        &self.ch1_cfg0
    }
    #[doc = "0x124 - NA"]
    #[inline(always)]
    pub const fn ch1_cfg1(&self) -> &CH1_CFG1 {
        &self.ch1_cfg1
    }
    #[doc = "0x128 - NA"]
    #[inline(always)]
    pub const fn ch1_llp0(&self) -> &CH1_LLP0 {
        &self.ch1_llp0
    }
    #[doc = "0x12c - NA"]
    #[inline(always)]
    pub const fn ch1_llp1(&self) -> &CH1_LLP1 {
        &self.ch1_llp1
    }
    #[doc = "0x130 - NA"]
    #[inline(always)]
    pub const fn ch1_status0(&self) -> &CH1_STATUS0 {
        &self.ch1_status0
    }
    #[doc = "0x134 - NA"]
    #[inline(always)]
    pub const fn ch1_status1(&self) -> &CH1_STATUS1 {
        &self.ch1_status1
    }
    #[doc = "0x138 - NA"]
    #[inline(always)]
    pub const fn ch1_swhssrc0(&self) -> &CH1_SWHSSRC0 {
        &self.ch1_swhssrc0
    }
    #[doc = "0x140 - NA"]
    #[inline(always)]
    pub const fn ch1_swhsdst0(&self) -> &CH1_SWHSDST0 {
        &self.ch1_swhsdst0
    }
    #[doc = "0x148 - NA"]
    #[inline(always)]
    pub const fn ch1_blk_tfr_resumereq0(&self) -> &CH1_BLK_TFR_RESUMEREQ0 {
        &self.ch1_blk_tfr_resumereq0
    }
    #[doc = "0x150 - NA"]
    #[inline(always)]
    pub const fn ch1_axi_id0(&self) -> &CH1_AXI_ID0 {
        &self.ch1_axi_id0
    }
    #[doc = "0x158 - NA"]
    #[inline(always)]
    pub const fn ch1_axi_qos0(&self) -> &CH1_AXI_QOS0 {
        &self.ch1_axi_qos0
    }
    #[doc = "0x160 - NA"]
    #[inline(always)]
    pub const fn ch1_sstat0(&self) -> &CH1_SSTAT0 {
        &self.ch1_sstat0
    }
    #[doc = "0x168 - NA"]
    #[inline(always)]
    pub const fn ch1_dstat0(&self) -> &CH1_DSTAT0 {
        &self.ch1_dstat0
    }
    #[doc = "0x170 - NA"]
    #[inline(always)]
    pub const fn ch1_sstatar0(&self) -> &CH1_SSTATAR0 {
        &self.ch1_sstatar0
    }
    #[doc = "0x174 - NA"]
    #[inline(always)]
    pub const fn ch1_sstatar1(&self) -> &CH1_SSTATAR1 {
        &self.ch1_sstatar1
    }
    #[doc = "0x178 - NA"]
    #[inline(always)]
    pub const fn ch1_dstatar0(&self) -> &CH1_DSTATAR0 {
        &self.ch1_dstatar0
    }
    #[doc = "0x17c - NA"]
    #[inline(always)]
    pub const fn ch1_dstatar1(&self) -> &CH1_DSTATAR1 {
        &self.ch1_dstatar1
    }
    #[doc = "0x180 - NA"]
    #[inline(always)]
    pub const fn ch1_intstatus_enable0(&self) -> &CH1_INTSTATUS_ENABLE0 {
        &self.ch1_intstatus_enable0
    }
    #[doc = "0x184 - NA"]
    #[inline(always)]
    pub const fn ch1_intstatus_enable1(&self) -> &CH1_INTSTATUS_ENABLE1 {
        &self.ch1_intstatus_enable1
    }
    #[doc = "0x188 - NA"]
    #[inline(always)]
    pub const fn ch1_intstatus0(&self) -> &CH1_INTSTATUS0 {
        &self.ch1_intstatus0
    }
    #[doc = "0x18c - NA"]
    #[inline(always)]
    pub const fn ch1_intstatus1(&self) -> &CH1_INTSTATUS1 {
        &self.ch1_intstatus1
    }
    #[doc = "0x190 - NA"]
    #[inline(always)]
    pub const fn ch1_intsignal_enable0(&self) -> &CH1_INTSIGNAL_ENABLE0 {
        &self.ch1_intsignal_enable0
    }
    #[doc = "0x194 - NA"]
    #[inline(always)]
    pub const fn ch1_intsignal_enable1(&self) -> &CH1_INTSIGNAL_ENABLE1 {
        &self.ch1_intsignal_enable1
    }
    #[doc = "0x198 - NA"]
    #[inline(always)]
    pub const fn ch1_intclear0(&self) -> &CH1_INTCLEAR0 {
        &self.ch1_intclear0
    }
    #[doc = "0x19c - NA"]
    #[inline(always)]
    pub const fn ch1_intclear1(&self) -> &CH1_INTCLEAR1 {
        &self.ch1_intclear1
    }
    #[doc = "0x200 - NA"]
    #[inline(always)]
    pub const fn ch2_sar0(&self) -> &CH2_SAR0 {
        &self.ch2_sar0
    }
    #[doc = "0x204 - NA"]
    #[inline(always)]
    pub const fn ch2_sar1(&self) -> &CH2_SAR1 {
        &self.ch2_sar1
    }
    #[doc = "0x208 - NA"]
    #[inline(always)]
    pub const fn ch2_dar0(&self) -> &CH2_DAR0 {
        &self.ch2_dar0
    }
    #[doc = "0x20c - NA"]
    #[inline(always)]
    pub const fn ch2_dar1(&self) -> &CH2_DAR1 {
        &self.ch2_dar1
    }
    #[doc = "0x210 - NA"]
    #[inline(always)]
    pub const fn ch2_block_ts0(&self) -> &CH2_BLOCK_TS0 {
        &self.ch2_block_ts0
    }
    #[doc = "0x218 - NA"]
    #[inline(always)]
    pub const fn ch2_ctl0(&self) -> &CH2_CTL0 {
        &self.ch2_ctl0
    }
    #[doc = "0x21c - NA"]
    #[inline(always)]
    pub const fn ch2_ctl1(&self) -> &CH2_CTL1 {
        &self.ch2_ctl1
    }
    #[doc = "0x220 - NA"]
    #[inline(always)]
    pub const fn ch2_cfg0(&self) -> &CH2_CFG0 {
        &self.ch2_cfg0
    }
    #[doc = "0x224 - NA"]
    #[inline(always)]
    pub const fn ch2_cfg1(&self) -> &CH2_CFG1 {
        &self.ch2_cfg1
    }
    #[doc = "0x228 - NA"]
    #[inline(always)]
    pub const fn ch2_llp0(&self) -> &CH2_LLP0 {
        &self.ch2_llp0
    }
    #[doc = "0x22c - NA"]
    #[inline(always)]
    pub const fn ch2_llp1(&self) -> &CH2_LLP1 {
        &self.ch2_llp1
    }
    #[doc = "0x230 - NA"]
    #[inline(always)]
    pub const fn ch2_status0(&self) -> &CH2_STATUS0 {
        &self.ch2_status0
    }
    #[doc = "0x234 - NA"]
    #[inline(always)]
    pub const fn ch2_status1(&self) -> &CH2_STATUS1 {
        &self.ch2_status1
    }
    #[doc = "0x238 - NA"]
    #[inline(always)]
    pub const fn ch2_swhssrc0(&self) -> &CH2_SWHSSRC0 {
        &self.ch2_swhssrc0
    }
    #[doc = "0x240 - NA"]
    #[inline(always)]
    pub const fn ch2_swhsdst0(&self) -> &CH2_SWHSDST0 {
        &self.ch2_swhsdst0
    }
    #[doc = "0x248 - NA"]
    #[inline(always)]
    pub const fn ch2_blk_tfr_resumereq0(&self) -> &CH2_BLK_TFR_RESUMEREQ0 {
        &self.ch2_blk_tfr_resumereq0
    }
    #[doc = "0x250 - NA"]
    #[inline(always)]
    pub const fn ch2_axi_id0(&self) -> &CH2_AXI_ID0 {
        &self.ch2_axi_id0
    }
    #[doc = "0x258 - NA"]
    #[inline(always)]
    pub const fn ch2_axi_qos0(&self) -> &CH2_AXI_QOS0 {
        &self.ch2_axi_qos0
    }
    #[doc = "0x260 - NA"]
    #[inline(always)]
    pub const fn ch2_sstat0(&self) -> &CH2_SSTAT0 {
        &self.ch2_sstat0
    }
    #[doc = "0x268 - NA"]
    #[inline(always)]
    pub const fn ch2_dstat0(&self) -> &CH2_DSTAT0 {
        &self.ch2_dstat0
    }
    #[doc = "0x270 - NA"]
    #[inline(always)]
    pub const fn ch2_sstatar0(&self) -> &CH2_SSTATAR0 {
        &self.ch2_sstatar0
    }
    #[doc = "0x274 - NA"]
    #[inline(always)]
    pub const fn ch2_sstatar1(&self) -> &CH2_SSTATAR1 {
        &self.ch2_sstatar1
    }
    #[doc = "0x278 - NA"]
    #[inline(always)]
    pub const fn ch2_dstatar0(&self) -> &CH2_DSTATAR0 {
        &self.ch2_dstatar0
    }
    #[doc = "0x27c - NA"]
    #[inline(always)]
    pub const fn ch2_dstatar1(&self) -> &CH2_DSTATAR1 {
        &self.ch2_dstatar1
    }
    #[doc = "0x280 - NA"]
    #[inline(always)]
    pub const fn ch2_intstatus_enable0(&self) -> &CH2_INTSTATUS_ENABLE0 {
        &self.ch2_intstatus_enable0
    }
    #[doc = "0x284 - NA"]
    #[inline(always)]
    pub const fn ch2_intstatus_enable1(&self) -> &CH2_INTSTATUS_ENABLE1 {
        &self.ch2_intstatus_enable1
    }
    #[doc = "0x288 - NA"]
    #[inline(always)]
    pub const fn ch2_intstatus0(&self) -> &CH2_INTSTATUS0 {
        &self.ch2_intstatus0
    }
    #[doc = "0x28c - NA"]
    #[inline(always)]
    pub const fn ch2_intstatus1(&self) -> &CH2_INTSTATUS1 {
        &self.ch2_intstatus1
    }
    #[doc = "0x290 - NA"]
    #[inline(always)]
    pub const fn ch2_intsignal_enable0(&self) -> &CH2_INTSIGNAL_ENABLE0 {
        &self.ch2_intsignal_enable0
    }
    #[doc = "0x294 - NA"]
    #[inline(always)]
    pub const fn ch2_intsignal_enable1(&self) -> &CH2_INTSIGNAL_ENABLE1 {
        &self.ch2_intsignal_enable1
    }
    #[doc = "0x298 - NA"]
    #[inline(always)]
    pub const fn ch2_intclear0(&self) -> &CH2_INTCLEAR0 {
        &self.ch2_intclear0
    }
    #[doc = "0x29c - NA"]
    #[inline(always)]
    pub const fn ch2_intclear1(&self) -> &CH2_INTCLEAR1 {
        &self.ch2_intclear1
    }
    #[doc = "0x300 - NA"]
    #[inline(always)]
    pub const fn ch3_sar0(&self) -> &CH3_SAR0 {
        &self.ch3_sar0
    }
    #[doc = "0x304 - NA"]
    #[inline(always)]
    pub const fn ch3_sar1(&self) -> &CH3_SAR1 {
        &self.ch3_sar1
    }
    #[doc = "0x308 - NA"]
    #[inline(always)]
    pub const fn ch3_dar0(&self) -> &CH3_DAR0 {
        &self.ch3_dar0
    }
    #[doc = "0x30c - NA"]
    #[inline(always)]
    pub const fn ch3_dar1(&self) -> &CH3_DAR1 {
        &self.ch3_dar1
    }
    #[doc = "0x310 - NA"]
    #[inline(always)]
    pub const fn ch3_block_ts0(&self) -> &CH3_BLOCK_TS0 {
        &self.ch3_block_ts0
    }
    #[doc = "0x318 - NA"]
    #[inline(always)]
    pub const fn ch3_ctl0(&self) -> &CH3_CTL0 {
        &self.ch3_ctl0
    }
    #[doc = "0x31c - NA"]
    #[inline(always)]
    pub const fn ch3_ctl1(&self) -> &CH3_CTL1 {
        &self.ch3_ctl1
    }
    #[doc = "0x320 - NA"]
    #[inline(always)]
    pub const fn ch3_cfg0(&self) -> &CH3_CFG0 {
        &self.ch3_cfg0
    }
    #[doc = "0x324 - NA"]
    #[inline(always)]
    pub const fn ch3_cfg1(&self) -> &CH3_CFG1 {
        &self.ch3_cfg1
    }
    #[doc = "0x328 - NA"]
    #[inline(always)]
    pub const fn ch3_llp0(&self) -> &CH3_LLP0 {
        &self.ch3_llp0
    }
    #[doc = "0x32c - NA"]
    #[inline(always)]
    pub const fn ch3_llp1(&self) -> &CH3_LLP1 {
        &self.ch3_llp1
    }
    #[doc = "0x330 - NA"]
    #[inline(always)]
    pub const fn ch3_status0(&self) -> &CH3_STATUS0 {
        &self.ch3_status0
    }
    #[doc = "0x334 - NA"]
    #[inline(always)]
    pub const fn ch3_status1(&self) -> &CH3_STATUS1 {
        &self.ch3_status1
    }
    #[doc = "0x338 - NA"]
    #[inline(always)]
    pub const fn ch3_swhssrc0(&self) -> &CH3_SWHSSRC0 {
        &self.ch3_swhssrc0
    }
    #[doc = "0x340 - NA"]
    #[inline(always)]
    pub const fn ch3_swhsdst0(&self) -> &CH3_SWHSDST0 {
        &self.ch3_swhsdst0
    }
    #[doc = "0x348 - NA"]
    #[inline(always)]
    pub const fn ch3_blk_tfr_resumereq0(&self) -> &CH3_BLK_TFR_RESUMEREQ0 {
        &self.ch3_blk_tfr_resumereq0
    }
    #[doc = "0x350 - NA"]
    #[inline(always)]
    pub const fn ch3_axi_id0(&self) -> &CH3_AXI_ID0 {
        &self.ch3_axi_id0
    }
    #[doc = "0x358 - NA"]
    #[inline(always)]
    pub const fn ch3_axi_qos0(&self) -> &CH3_AXI_QOS0 {
        &self.ch3_axi_qos0
    }
    #[doc = "0x360 - NA"]
    #[inline(always)]
    pub const fn ch3_sstat0(&self) -> &CH3_SSTAT0 {
        &self.ch3_sstat0
    }
    #[doc = "0x368 - NA"]
    #[inline(always)]
    pub const fn ch3_dstat0(&self) -> &CH3_DSTAT0 {
        &self.ch3_dstat0
    }
    #[doc = "0x370 - NA"]
    #[inline(always)]
    pub const fn ch3_sstatar0(&self) -> &CH3_SSTATAR0 {
        &self.ch3_sstatar0
    }
    #[doc = "0x374 - NA"]
    #[inline(always)]
    pub const fn ch3_sstatar1(&self) -> &CH3_SSTATAR1 {
        &self.ch3_sstatar1
    }
    #[doc = "0x378 - NA"]
    #[inline(always)]
    pub const fn ch3_dstatar0(&self) -> &CH3_DSTATAR0 {
        &self.ch3_dstatar0
    }
    #[doc = "0x37c - NA"]
    #[inline(always)]
    pub const fn ch3_dstatar1(&self) -> &CH3_DSTATAR1 {
        &self.ch3_dstatar1
    }
    #[doc = "0x380 - NA"]
    #[inline(always)]
    pub const fn ch3_intstatus_enable0(&self) -> &CH3_INTSTATUS_ENABLE0 {
        &self.ch3_intstatus_enable0
    }
    #[doc = "0x384 - NA"]
    #[inline(always)]
    pub const fn ch3_intstatus_enable1(&self) -> &CH3_INTSTATUS_ENABLE1 {
        &self.ch3_intstatus_enable1
    }
    #[doc = "0x388 - NA"]
    #[inline(always)]
    pub const fn ch3_intstatus0(&self) -> &CH3_INTSTATUS0 {
        &self.ch3_intstatus0
    }
    #[doc = "0x38c - NA"]
    #[inline(always)]
    pub const fn ch3_intstatus1(&self) -> &CH3_INTSTATUS1 {
        &self.ch3_intstatus1
    }
    #[doc = "0x390 - NA"]
    #[inline(always)]
    pub const fn ch3_intsignal_enable0(&self) -> &CH3_INTSIGNAL_ENABLE0 {
        &self.ch3_intsignal_enable0
    }
    #[doc = "0x394 - NA"]
    #[inline(always)]
    pub const fn ch3_intsignal_enable1(&self) -> &CH3_INTSIGNAL_ENABLE1 {
        &self.ch3_intsignal_enable1
    }
    #[doc = "0x398 - NA"]
    #[inline(always)]
    pub const fn ch3_intclear0(&self) -> &CH3_INTCLEAR0 {
        &self.ch3_intclear0
    }
    #[doc = "0x39c - NA"]
    #[inline(always)]
    pub const fn ch3_intclear1(&self) -> &CH3_INTCLEAR1 {
        &self.ch3_intclear1
    }
    #[doc = "0x400 - NA"]
    #[inline(always)]
    pub const fn ch4_sar0(&self) -> &CH4_SAR0 {
        &self.ch4_sar0
    }
    #[doc = "0x404 - NA"]
    #[inline(always)]
    pub const fn ch4_sar1(&self) -> &CH4_SAR1 {
        &self.ch4_sar1
    }
    #[doc = "0x408 - NA"]
    #[inline(always)]
    pub const fn ch4_dar0(&self) -> &CH4_DAR0 {
        &self.ch4_dar0
    }
    #[doc = "0x40c - NA"]
    #[inline(always)]
    pub const fn ch4_dar1(&self) -> &CH4_DAR1 {
        &self.ch4_dar1
    }
    #[doc = "0x410 - NA"]
    #[inline(always)]
    pub const fn ch4_block_ts0(&self) -> &CH4_BLOCK_TS0 {
        &self.ch4_block_ts0
    }
    #[doc = "0x418 - NA"]
    #[inline(always)]
    pub const fn ch4_ctl0(&self) -> &CH4_CTL0 {
        &self.ch4_ctl0
    }
    #[doc = "0x41c - NA"]
    #[inline(always)]
    pub const fn ch4_ctl1(&self) -> &CH4_CTL1 {
        &self.ch4_ctl1
    }
    #[doc = "0x420 - NA"]
    #[inline(always)]
    pub const fn ch4_cfg0(&self) -> &CH4_CFG0 {
        &self.ch4_cfg0
    }
    #[doc = "0x424 - NA"]
    #[inline(always)]
    pub const fn ch4_cfg1(&self) -> &CH4_CFG1 {
        &self.ch4_cfg1
    }
    #[doc = "0x428 - NA"]
    #[inline(always)]
    pub const fn ch4_llp0(&self) -> &CH4_LLP0 {
        &self.ch4_llp0
    }
    #[doc = "0x42c - NA"]
    #[inline(always)]
    pub const fn ch4_llp1(&self) -> &CH4_LLP1 {
        &self.ch4_llp1
    }
    #[doc = "0x430 - NA"]
    #[inline(always)]
    pub const fn ch4_status0(&self) -> &CH4_STATUS0 {
        &self.ch4_status0
    }
    #[doc = "0x434 - NA"]
    #[inline(always)]
    pub const fn ch4_status1(&self) -> &CH4_STATUS1 {
        &self.ch4_status1
    }
    #[doc = "0x438 - NA"]
    #[inline(always)]
    pub const fn ch4_swhssrc0(&self) -> &CH4_SWHSSRC0 {
        &self.ch4_swhssrc0
    }
    #[doc = "0x440 - NA"]
    #[inline(always)]
    pub const fn ch4_swhsdst0(&self) -> &CH4_SWHSDST0 {
        &self.ch4_swhsdst0
    }
    #[doc = "0x448 - NA"]
    #[inline(always)]
    pub const fn ch4_blk_tfr_resumereq0(&self) -> &CH4_BLK_TFR_RESUMEREQ0 {
        &self.ch4_blk_tfr_resumereq0
    }
    #[doc = "0x450 - NA"]
    #[inline(always)]
    pub const fn ch4_axi_id0(&self) -> &CH4_AXI_ID0 {
        &self.ch4_axi_id0
    }
    #[doc = "0x458 - NA"]
    #[inline(always)]
    pub const fn ch4_axi_qos0(&self) -> &CH4_AXI_QOS0 {
        &self.ch4_axi_qos0
    }
    #[doc = "0x460 - NA"]
    #[inline(always)]
    pub const fn ch4_sstat0(&self) -> &CH4_SSTAT0 {
        &self.ch4_sstat0
    }
    #[doc = "0x468 - NA"]
    #[inline(always)]
    pub const fn ch4_dstat0(&self) -> &CH4_DSTAT0 {
        &self.ch4_dstat0
    }
    #[doc = "0x470 - NA"]
    #[inline(always)]
    pub const fn ch4_sstatar0(&self) -> &CH4_SSTATAR0 {
        &self.ch4_sstatar0
    }
    #[doc = "0x474 - NA"]
    #[inline(always)]
    pub const fn ch4_sstatar1(&self) -> &CH4_SSTATAR1 {
        &self.ch4_sstatar1
    }
    #[doc = "0x478 - NA"]
    #[inline(always)]
    pub const fn ch4_dstatar0(&self) -> &CH4_DSTATAR0 {
        &self.ch4_dstatar0
    }
    #[doc = "0x47c - NA"]
    #[inline(always)]
    pub const fn ch4_dstatar1(&self) -> &CH4_DSTATAR1 {
        &self.ch4_dstatar1
    }
    #[doc = "0x480 - NA"]
    #[inline(always)]
    pub const fn ch4_intstatus_enable0(&self) -> &CH4_INTSTATUS_ENABLE0 {
        &self.ch4_intstatus_enable0
    }
    #[doc = "0x484 - NA"]
    #[inline(always)]
    pub const fn ch4_intstatus_enable1(&self) -> &CH4_INTSTATUS_ENABLE1 {
        &self.ch4_intstatus_enable1
    }
    #[doc = "0x488 - NA"]
    #[inline(always)]
    pub const fn ch4_intstatus0(&self) -> &CH4_INTSTATUS0 {
        &self.ch4_intstatus0
    }
    #[doc = "0x48c - NA"]
    #[inline(always)]
    pub const fn ch4_intstatus1(&self) -> &CH4_INTSTATUS1 {
        &self.ch4_intstatus1
    }
    #[doc = "0x490 - NA"]
    #[inline(always)]
    pub const fn ch4_intsignal_enable0(&self) -> &CH4_INTSIGNAL_ENABLE0 {
        &self.ch4_intsignal_enable0
    }
    #[doc = "0x494 - NA"]
    #[inline(always)]
    pub const fn ch4_intsignal_enable1(&self) -> &CH4_INTSIGNAL_ENABLE1 {
        &self.ch4_intsignal_enable1
    }
    #[doc = "0x498 - NA"]
    #[inline(always)]
    pub const fn ch4_intclear0(&self) -> &CH4_INTCLEAR0 {
        &self.ch4_intclear0
    }
    #[doc = "0x49c - NA"]
    #[inline(always)]
    pub const fn ch4_intclear1(&self) -> &CH4_INTCLEAR1 {
        &self.ch4_intclear1
    }
}
#[doc = "ID0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id0`] module"]
pub type ID0 = crate::Reg<id0::ID0_SPEC>;
#[doc = "NA"]
pub mod id0;
#[doc = "COMPVER0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compver0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compver0`] module"]
pub type COMPVER0 = crate::Reg<compver0::COMPVER0_SPEC>;
#[doc = "NA"]
pub mod compver0;
#[doc = "CFG0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`] module"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "NA"]
pub mod cfg0;
#[doc = "CHEN0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen0`] module"]
pub type CHEN0 = crate::Reg<chen0::CHEN0_SPEC>;
#[doc = "NA"]
pub mod chen0;
#[doc = "CHEN1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen1`] module"]
pub type CHEN1 = crate::Reg<chen1::CHEN1_SPEC>;
#[doc = "NA"]
pub mod chen1;
#[doc = "INTSTATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus0`] module"]
pub type INTSTATUS0 = crate::Reg<intstatus0::INTSTATUS0_SPEC>;
#[doc = "NA"]
pub mod intstatus0;
#[doc = "COMMONREG_INTCLEAR0 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`commonreg_intclear0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intclear0`] module"]
pub type COMMONREG_INTCLEAR0 = crate::Reg<commonreg_intclear0::COMMONREG_INTCLEAR0_SPEC>;
#[doc = "NA"]
pub mod commonreg_intclear0;
#[doc = "COMMONREG_INTSTATUS_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`commonreg_intstatus_enable0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`commonreg_intstatus_enable0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intstatus_enable0`] module"]
pub type COMMONREG_INTSTATUS_ENABLE0 =
    crate::Reg<commonreg_intstatus_enable0::COMMONREG_INTSTATUS_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod commonreg_intstatus_enable0;
#[doc = "COMMONREG_INTSIGNAL_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`commonreg_intsignal_enable0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`commonreg_intsignal_enable0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intsignal_enable0`] module"]
pub type COMMONREG_INTSIGNAL_ENABLE0 =
    crate::Reg<commonreg_intsignal_enable0::COMMONREG_INTSIGNAL_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod commonreg_intsignal_enable0;
#[doc = "COMMONREG_INTSTATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`commonreg_intstatus0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@commonreg_intstatus0`] module"]
pub type COMMONREG_INTSTATUS0 = crate::Reg<commonreg_intstatus0::COMMONREG_INTSTATUS0_SPEC>;
#[doc = "NA"]
pub mod commonreg_intstatus0;
#[doc = "RESET0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset0`] module"]
pub type RESET0 = crate::Reg<reset0::RESET0_SPEC>;
#[doc = "NA"]
pub mod reset0;
#[doc = "LOWPOWER_CFG0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lowpower_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lowpower_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowpower_cfg0`] module"]
pub type LOWPOWER_CFG0 = crate::Reg<lowpower_cfg0::LOWPOWER_CFG0_SPEC>;
#[doc = "NA"]
pub mod lowpower_cfg0;
#[doc = "LOWPOWER_CFG1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lowpower_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lowpower_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowpower_cfg1`] module"]
pub type LOWPOWER_CFG1 = crate::Reg<lowpower_cfg1::LOWPOWER_CFG1_SPEC>;
#[doc = "NA"]
pub mod lowpower_cfg1;
#[doc = "CH1_SAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_sar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_sar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_sar0`] module"]
pub type CH1_SAR0 = crate::Reg<ch1_sar0::CH1_SAR0_SPEC>;
#[doc = "NA"]
pub mod ch1_sar0;
#[doc = "CH1_SAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_sar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_sar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_sar1`] module"]
pub type CH1_SAR1 = crate::Reg<ch1_sar1::CH1_SAR1_SPEC>;
#[doc = "NA"]
pub mod ch1_sar1;
#[doc = "CH1_DAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_dar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dar0`] module"]
pub type CH1_DAR0 = crate::Reg<ch1_dar0::CH1_DAR0_SPEC>;
#[doc = "NA"]
pub mod ch1_dar0;
#[doc = "CH1_DAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_dar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dar1`] module"]
pub type CH1_DAR1 = crate::Reg<ch1_dar1::CH1_DAR1_SPEC>;
#[doc = "NA"]
pub mod ch1_dar1;
#[doc = "CH1_BLOCK_TS0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_block_ts0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_block_ts0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_block_ts0`] module"]
pub type CH1_BLOCK_TS0 = crate::Reg<ch1_block_ts0::CH1_BLOCK_TS0_SPEC>;
#[doc = "NA"]
pub mod ch1_block_ts0;
#[doc = "CH1_CTL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ctl0`] module"]
pub type CH1_CTL0 = crate::Reg<ch1_ctl0::CH1_CTL0_SPEC>;
#[doc = "NA"]
pub mod ch1_ctl0;
#[doc = "CH1_CTL1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ctl1`] module"]
pub type CH1_CTL1 = crate::Reg<ch1_ctl1::CH1_CTL1_SPEC>;
#[doc = "NA"]
pub mod ch1_ctl1;
#[doc = "CH1_CFG0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_cfg0`] module"]
pub type CH1_CFG0 = crate::Reg<ch1_cfg0::CH1_CFG0_SPEC>;
#[doc = "NA"]
pub mod ch1_cfg0;
#[doc = "CH1_CFG1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_cfg1`] module"]
pub type CH1_CFG1 = crate::Reg<ch1_cfg1::CH1_CFG1_SPEC>;
#[doc = "NA"]
pub mod ch1_cfg1;
#[doc = "CH1_LLP0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_llp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_llp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_llp0`] module"]
pub type CH1_LLP0 = crate::Reg<ch1_llp0::CH1_LLP0_SPEC>;
#[doc = "NA"]
pub mod ch1_llp0;
#[doc = "CH1_LLP1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_llp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_llp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_llp1`] module"]
pub type CH1_LLP1 = crate::Reg<ch1_llp1::CH1_LLP1_SPEC>;
#[doc = "NA"]
pub mod ch1_llp1;
#[doc = "CH1_STATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_status0`] module"]
pub type CH1_STATUS0 = crate::Reg<ch1_status0::CH1_STATUS0_SPEC>;
#[doc = "NA"]
pub mod ch1_status0;
#[doc = "CH1_STATUS1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_status1`] module"]
pub type CH1_STATUS1 = crate::Reg<ch1_status1::CH1_STATUS1_SPEC>;
#[doc = "NA"]
pub mod ch1_status1;
#[doc = "CH1_SWHSSRC0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_swhssrc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_swhssrc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_swhssrc0`] module"]
pub type CH1_SWHSSRC0 = crate::Reg<ch1_swhssrc0::CH1_SWHSSRC0_SPEC>;
#[doc = "NA"]
pub mod ch1_swhssrc0;
#[doc = "CH1_SWHSDST0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_swhsdst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_swhsdst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_swhsdst0`] module"]
pub type CH1_SWHSDST0 = crate::Reg<ch1_swhsdst0::CH1_SWHSDST0_SPEC>;
#[doc = "NA"]
pub mod ch1_swhsdst0;
#[doc = "CH1_BLK_TFR_RESUMEREQ0 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_blk_tfr_resumereq0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_blk_tfr_resumereq0`] module"]
pub type CH1_BLK_TFR_RESUMEREQ0 = crate::Reg<ch1_blk_tfr_resumereq0::CH1_BLK_TFR_RESUMEREQ0_SPEC>;
#[doc = "NA"]
pub mod ch1_blk_tfr_resumereq0;
#[doc = "CH1_AXI_ID0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_axi_id0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_axi_id0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_axi_id0`] module"]
pub type CH1_AXI_ID0 = crate::Reg<ch1_axi_id0::CH1_AXI_ID0_SPEC>;
#[doc = "NA"]
pub mod ch1_axi_id0;
#[doc = "CH1_AXI_QOS0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_axi_qos0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_axi_qos0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_axi_qos0`] module"]
pub type CH1_AXI_QOS0 = crate::Reg<ch1_axi_qos0::CH1_AXI_QOS0_SPEC>;
#[doc = "NA"]
pub mod ch1_axi_qos0;
#[doc = "CH1_SSTAT0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_sstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_sstat0`] module"]
pub type CH1_SSTAT0 = crate::Reg<ch1_sstat0::CH1_SSTAT0_SPEC>;
#[doc = "NA"]
pub mod ch1_sstat0;
#[doc = "CH1_DSTAT0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dstat0`] module"]
pub type CH1_DSTAT0 = crate::Reg<ch1_dstat0::CH1_DSTAT0_SPEC>;
#[doc = "NA"]
pub mod ch1_dstat0;
#[doc = "CH1_SSTATAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_sstatar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_sstatar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_sstatar0`] module"]
pub type CH1_SSTATAR0 = crate::Reg<ch1_sstatar0::CH1_SSTATAR0_SPEC>;
#[doc = "NA"]
pub mod ch1_sstatar0;
#[doc = "CH1_SSTATAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_sstatar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_sstatar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_sstatar1`] module"]
pub type CH1_SSTATAR1 = crate::Reg<ch1_sstatar1::CH1_SSTATAR1_SPEC>;
#[doc = "NA"]
pub mod ch1_sstatar1;
#[doc = "CH1_DSTATAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dstatar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_dstatar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dstatar0`] module"]
pub type CH1_DSTATAR0 = crate::Reg<ch1_dstatar0::CH1_DSTATAR0_SPEC>;
#[doc = "NA"]
pub mod ch1_dstatar0;
#[doc = "CH1_DSTATAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dstatar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_dstatar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dstatar1`] module"]
pub type CH1_DSTATAR1 = crate::Reg<ch1_dstatar1::CH1_DSTATAR1_SPEC>;
#[doc = "NA"]
pub mod ch1_dstatar1;
#[doc = "CH1_INTSTATUS_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_intstatus_enable0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_intstatus_enable0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_intstatus_enable0`] module"]
pub type CH1_INTSTATUS_ENABLE0 = crate::Reg<ch1_intstatus_enable0::CH1_INTSTATUS_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod ch1_intstatus_enable0;
#[doc = "CH1_INTSTATUS_ENABLE1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_intstatus_enable1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_intstatus_enable1`] module"]
pub type CH1_INTSTATUS_ENABLE1 = crate::Reg<ch1_intstatus_enable1::CH1_INTSTATUS_ENABLE1_SPEC>;
#[doc = "NA"]
pub mod ch1_intstatus_enable1;
#[doc = "CH1_INTSTATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_intstatus0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_intstatus0`] module"]
pub type CH1_INTSTATUS0 = crate::Reg<ch1_intstatus0::CH1_INTSTATUS0_SPEC>;
#[doc = "NA"]
pub mod ch1_intstatus0;
#[doc = "CH1_INTSTATUS1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_intstatus1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_intstatus1`] module"]
pub type CH1_INTSTATUS1 = crate::Reg<ch1_intstatus1::CH1_INTSTATUS1_SPEC>;
#[doc = "NA"]
pub mod ch1_intstatus1;
#[doc = "CH1_INTSIGNAL_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_intsignal_enable0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_intsignal_enable0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_intsignal_enable0`] module"]
pub type CH1_INTSIGNAL_ENABLE0 = crate::Reg<ch1_intsignal_enable0::CH1_INTSIGNAL_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod ch1_intsignal_enable0;
#[doc = "CH1_INTSIGNAL_ENABLE1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_intsignal_enable1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_intsignal_enable1`] module"]
pub type CH1_INTSIGNAL_ENABLE1 = crate::Reg<ch1_intsignal_enable1::CH1_INTSIGNAL_ENABLE1_SPEC>;
#[doc = "NA"]
pub mod ch1_intsignal_enable1;
#[doc = "CH1_INTCLEAR0 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_intclear0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_intclear0`] module"]
pub type CH1_INTCLEAR0 = crate::Reg<ch1_intclear0::CH1_INTCLEAR0_SPEC>;
#[doc = "NA"]
pub mod ch1_intclear0;
#[doc = "CH1_INTCLEAR1 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_intclear1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_intclear1`] module"]
pub type CH1_INTCLEAR1 = crate::Reg<ch1_intclear1::CH1_INTCLEAR1_SPEC>;
#[doc = "NA"]
pub mod ch1_intclear1;
#[doc = "CH2_SAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_sar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_sar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_sar0`] module"]
pub type CH2_SAR0 = crate::Reg<ch2_sar0::CH2_SAR0_SPEC>;
#[doc = "NA"]
pub mod ch2_sar0;
#[doc = "CH2_SAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_sar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_sar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_sar1`] module"]
pub type CH2_SAR1 = crate::Reg<ch2_sar1::CH2_SAR1_SPEC>;
#[doc = "NA"]
pub mod ch2_sar1;
#[doc = "CH2_DAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_dar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_dar0`] module"]
pub type CH2_DAR0 = crate::Reg<ch2_dar0::CH2_DAR0_SPEC>;
#[doc = "NA"]
pub mod ch2_dar0;
#[doc = "CH2_DAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_dar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_dar1`] module"]
pub type CH2_DAR1 = crate::Reg<ch2_dar1::CH2_DAR1_SPEC>;
#[doc = "NA"]
pub mod ch2_dar1;
#[doc = "CH2_BLOCK_TS0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_block_ts0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_block_ts0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_block_ts0`] module"]
pub type CH2_BLOCK_TS0 = crate::Reg<ch2_block_ts0::CH2_BLOCK_TS0_SPEC>;
#[doc = "NA"]
pub mod ch2_block_ts0;
#[doc = "CH2_CTL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ctl0`] module"]
pub type CH2_CTL0 = crate::Reg<ch2_ctl0::CH2_CTL0_SPEC>;
#[doc = "NA"]
pub mod ch2_ctl0;
#[doc = "CH2_CTL1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ctl1`] module"]
pub type CH2_CTL1 = crate::Reg<ch2_ctl1::CH2_CTL1_SPEC>;
#[doc = "NA"]
pub mod ch2_ctl1;
#[doc = "CH2_CFG0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_cfg0`] module"]
pub type CH2_CFG0 = crate::Reg<ch2_cfg0::CH2_CFG0_SPEC>;
#[doc = "NA"]
pub mod ch2_cfg0;
#[doc = "CH2_CFG1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_cfg1`] module"]
pub type CH2_CFG1 = crate::Reg<ch2_cfg1::CH2_CFG1_SPEC>;
#[doc = "NA"]
pub mod ch2_cfg1;
#[doc = "CH2_LLP0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_llp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_llp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_llp0`] module"]
pub type CH2_LLP0 = crate::Reg<ch2_llp0::CH2_LLP0_SPEC>;
#[doc = "NA"]
pub mod ch2_llp0;
#[doc = "CH2_LLP1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_llp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_llp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_llp1`] module"]
pub type CH2_LLP1 = crate::Reg<ch2_llp1::CH2_LLP1_SPEC>;
#[doc = "NA"]
pub mod ch2_llp1;
#[doc = "CH2_STATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_status0`] module"]
pub type CH2_STATUS0 = crate::Reg<ch2_status0::CH2_STATUS0_SPEC>;
#[doc = "NA"]
pub mod ch2_status0;
#[doc = "CH2_STATUS1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_status1`] module"]
pub type CH2_STATUS1 = crate::Reg<ch2_status1::CH2_STATUS1_SPEC>;
#[doc = "NA"]
pub mod ch2_status1;
#[doc = "CH2_SWHSSRC0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_swhssrc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_swhssrc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_swhssrc0`] module"]
pub type CH2_SWHSSRC0 = crate::Reg<ch2_swhssrc0::CH2_SWHSSRC0_SPEC>;
#[doc = "NA"]
pub mod ch2_swhssrc0;
#[doc = "CH2_SWHSDST0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_swhsdst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_swhsdst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_swhsdst0`] module"]
pub type CH2_SWHSDST0 = crate::Reg<ch2_swhsdst0::CH2_SWHSDST0_SPEC>;
#[doc = "NA"]
pub mod ch2_swhsdst0;
#[doc = "CH2_BLK_TFR_RESUMEREQ0 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_blk_tfr_resumereq0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_blk_tfr_resumereq0`] module"]
pub type CH2_BLK_TFR_RESUMEREQ0 = crate::Reg<ch2_blk_tfr_resumereq0::CH2_BLK_TFR_RESUMEREQ0_SPEC>;
#[doc = "NA"]
pub mod ch2_blk_tfr_resumereq0;
#[doc = "CH2_AXI_ID0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_axi_id0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_axi_id0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_axi_id0`] module"]
pub type CH2_AXI_ID0 = crate::Reg<ch2_axi_id0::CH2_AXI_ID0_SPEC>;
#[doc = "NA"]
pub mod ch2_axi_id0;
#[doc = "CH2_AXI_QOS0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_axi_qos0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_axi_qos0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_axi_qos0`] module"]
pub type CH2_AXI_QOS0 = crate::Reg<ch2_axi_qos0::CH2_AXI_QOS0_SPEC>;
#[doc = "NA"]
pub mod ch2_axi_qos0;
#[doc = "CH2_SSTAT0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_sstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_sstat0`] module"]
pub type CH2_SSTAT0 = crate::Reg<ch2_sstat0::CH2_SSTAT0_SPEC>;
#[doc = "NA"]
pub mod ch2_sstat0;
#[doc = "CH2_DSTAT0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_dstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_dstat0`] module"]
pub type CH2_DSTAT0 = crate::Reg<ch2_dstat0::CH2_DSTAT0_SPEC>;
#[doc = "NA"]
pub mod ch2_dstat0;
#[doc = "CH2_SSTATAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_sstatar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_sstatar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_sstatar0`] module"]
pub type CH2_SSTATAR0 = crate::Reg<ch2_sstatar0::CH2_SSTATAR0_SPEC>;
#[doc = "NA"]
pub mod ch2_sstatar0;
#[doc = "CH2_SSTATAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_sstatar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_sstatar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_sstatar1`] module"]
pub type CH2_SSTATAR1 = crate::Reg<ch2_sstatar1::CH2_SSTATAR1_SPEC>;
#[doc = "NA"]
pub mod ch2_sstatar1;
#[doc = "CH2_DSTATAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_dstatar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dstatar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_dstatar0`] module"]
pub type CH2_DSTATAR0 = crate::Reg<ch2_dstatar0::CH2_DSTATAR0_SPEC>;
#[doc = "NA"]
pub mod ch2_dstatar0;
#[doc = "CH2_DSTATAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_dstatar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_dstatar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_dstatar1`] module"]
pub type CH2_DSTATAR1 = crate::Reg<ch2_dstatar1::CH2_DSTATAR1_SPEC>;
#[doc = "NA"]
pub mod ch2_dstatar1;
#[doc = "CH2_INTSTATUS_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_intstatus_enable0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_intstatus_enable0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_intstatus_enable0`] module"]
pub type CH2_INTSTATUS_ENABLE0 = crate::Reg<ch2_intstatus_enable0::CH2_INTSTATUS_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod ch2_intstatus_enable0;
#[doc = "CH2_INTSTATUS_ENABLE1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_intstatus_enable1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_intstatus_enable1`] module"]
pub type CH2_INTSTATUS_ENABLE1 = crate::Reg<ch2_intstatus_enable1::CH2_INTSTATUS_ENABLE1_SPEC>;
#[doc = "NA"]
pub mod ch2_intstatus_enable1;
#[doc = "CH2_INTSTATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_intstatus0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_intstatus0`] module"]
pub type CH2_INTSTATUS0 = crate::Reg<ch2_intstatus0::CH2_INTSTATUS0_SPEC>;
#[doc = "NA"]
pub mod ch2_intstatus0;
#[doc = "CH2_INTSTATUS1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_intstatus1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_intstatus1`] module"]
pub type CH2_INTSTATUS1 = crate::Reg<ch2_intstatus1::CH2_INTSTATUS1_SPEC>;
#[doc = "NA"]
pub mod ch2_intstatus1;
#[doc = "CH2_INTSIGNAL_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_intsignal_enable0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_intsignal_enable0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_intsignal_enable0`] module"]
pub type CH2_INTSIGNAL_ENABLE0 = crate::Reg<ch2_intsignal_enable0::CH2_INTSIGNAL_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod ch2_intsignal_enable0;
#[doc = "CH2_INTSIGNAL_ENABLE1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_intsignal_enable1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_intsignal_enable1`] module"]
pub type CH2_INTSIGNAL_ENABLE1 = crate::Reg<ch2_intsignal_enable1::CH2_INTSIGNAL_ENABLE1_SPEC>;
#[doc = "NA"]
pub mod ch2_intsignal_enable1;
#[doc = "CH2_INTCLEAR0 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_intclear0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_intclear0`] module"]
pub type CH2_INTCLEAR0 = crate::Reg<ch2_intclear0::CH2_INTCLEAR0_SPEC>;
#[doc = "NA"]
pub mod ch2_intclear0;
#[doc = "CH2_INTCLEAR1 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_intclear1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_intclear1`] module"]
pub type CH2_INTCLEAR1 = crate::Reg<ch2_intclear1::CH2_INTCLEAR1_SPEC>;
#[doc = "NA"]
pub mod ch2_intclear1;
#[doc = "CH3_SAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_sar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_sar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_sar0`] module"]
pub type CH3_SAR0 = crate::Reg<ch3_sar0::CH3_SAR0_SPEC>;
#[doc = "NA"]
pub mod ch3_sar0;
#[doc = "CH3_SAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_sar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_sar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_sar1`] module"]
pub type CH3_SAR1 = crate::Reg<ch3_sar1::CH3_SAR1_SPEC>;
#[doc = "NA"]
pub mod ch3_sar1;
#[doc = "CH3_DAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_dar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_dar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_dar0`] module"]
pub type CH3_DAR0 = crate::Reg<ch3_dar0::CH3_DAR0_SPEC>;
#[doc = "NA"]
pub mod ch3_dar0;
#[doc = "CH3_DAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_dar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_dar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_dar1`] module"]
pub type CH3_DAR1 = crate::Reg<ch3_dar1::CH3_DAR1_SPEC>;
#[doc = "NA"]
pub mod ch3_dar1;
#[doc = "CH3_BLOCK_TS0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_block_ts0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_block_ts0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_block_ts0`] module"]
pub type CH3_BLOCK_TS0 = crate::Reg<ch3_block_ts0::CH3_BLOCK_TS0_SPEC>;
#[doc = "NA"]
pub mod ch3_block_ts0;
#[doc = "CH3_CTL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ctl0`] module"]
pub type CH3_CTL0 = crate::Reg<ch3_ctl0::CH3_CTL0_SPEC>;
#[doc = "NA"]
pub mod ch3_ctl0;
#[doc = "CH3_CTL1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ctl1`] module"]
pub type CH3_CTL1 = crate::Reg<ch3_ctl1::CH3_CTL1_SPEC>;
#[doc = "NA"]
pub mod ch3_ctl1;
#[doc = "CH3_CFG0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_cfg0`] module"]
pub type CH3_CFG0 = crate::Reg<ch3_cfg0::CH3_CFG0_SPEC>;
#[doc = "NA"]
pub mod ch3_cfg0;
#[doc = "CH3_CFG1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_cfg1`] module"]
pub type CH3_CFG1 = crate::Reg<ch3_cfg1::CH3_CFG1_SPEC>;
#[doc = "NA"]
pub mod ch3_cfg1;
#[doc = "CH3_LLP0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_llp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_llp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_llp0`] module"]
pub type CH3_LLP0 = crate::Reg<ch3_llp0::CH3_LLP0_SPEC>;
#[doc = "NA"]
pub mod ch3_llp0;
#[doc = "CH3_LLP1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_llp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_llp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_llp1`] module"]
pub type CH3_LLP1 = crate::Reg<ch3_llp1::CH3_LLP1_SPEC>;
#[doc = "NA"]
pub mod ch3_llp1;
#[doc = "CH3_STATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_status0`] module"]
pub type CH3_STATUS0 = crate::Reg<ch3_status0::CH3_STATUS0_SPEC>;
#[doc = "NA"]
pub mod ch3_status0;
#[doc = "CH3_STATUS1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_status1`] module"]
pub type CH3_STATUS1 = crate::Reg<ch3_status1::CH3_STATUS1_SPEC>;
#[doc = "NA"]
pub mod ch3_status1;
#[doc = "CH3_SWHSSRC0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_swhssrc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_swhssrc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_swhssrc0`] module"]
pub type CH3_SWHSSRC0 = crate::Reg<ch3_swhssrc0::CH3_SWHSSRC0_SPEC>;
#[doc = "NA"]
pub mod ch3_swhssrc0;
#[doc = "CH3_SWHSDST0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_swhsdst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_swhsdst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_swhsdst0`] module"]
pub type CH3_SWHSDST0 = crate::Reg<ch3_swhsdst0::CH3_SWHSDST0_SPEC>;
#[doc = "NA"]
pub mod ch3_swhsdst0;
#[doc = "CH3_BLK_TFR_RESUMEREQ0 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_blk_tfr_resumereq0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_blk_tfr_resumereq0`] module"]
pub type CH3_BLK_TFR_RESUMEREQ0 = crate::Reg<ch3_blk_tfr_resumereq0::CH3_BLK_TFR_RESUMEREQ0_SPEC>;
#[doc = "NA"]
pub mod ch3_blk_tfr_resumereq0;
#[doc = "CH3_AXI_ID0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_axi_id0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_axi_id0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_axi_id0`] module"]
pub type CH3_AXI_ID0 = crate::Reg<ch3_axi_id0::CH3_AXI_ID0_SPEC>;
#[doc = "NA"]
pub mod ch3_axi_id0;
#[doc = "CH3_AXI_QOS0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_axi_qos0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_axi_qos0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_axi_qos0`] module"]
pub type CH3_AXI_QOS0 = crate::Reg<ch3_axi_qos0::CH3_AXI_QOS0_SPEC>;
#[doc = "NA"]
pub mod ch3_axi_qos0;
#[doc = "CH3_SSTAT0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_sstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_sstat0`] module"]
pub type CH3_SSTAT0 = crate::Reg<ch3_sstat0::CH3_SSTAT0_SPEC>;
#[doc = "NA"]
pub mod ch3_sstat0;
#[doc = "CH3_DSTAT0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_dstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_dstat0`] module"]
pub type CH3_DSTAT0 = crate::Reg<ch3_dstat0::CH3_DSTAT0_SPEC>;
#[doc = "NA"]
pub mod ch3_dstat0;
#[doc = "CH3_SSTATAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_sstatar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_sstatar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_sstatar0`] module"]
pub type CH3_SSTATAR0 = crate::Reg<ch3_sstatar0::CH3_SSTATAR0_SPEC>;
#[doc = "NA"]
pub mod ch3_sstatar0;
#[doc = "CH3_SSTATAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_sstatar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_sstatar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_sstatar1`] module"]
pub type CH3_SSTATAR1 = crate::Reg<ch3_sstatar1::CH3_SSTATAR1_SPEC>;
#[doc = "NA"]
pub mod ch3_sstatar1;
#[doc = "CH3_DSTATAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_dstatar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_dstatar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_dstatar0`] module"]
pub type CH3_DSTATAR0 = crate::Reg<ch3_dstatar0::CH3_DSTATAR0_SPEC>;
#[doc = "NA"]
pub mod ch3_dstatar0;
#[doc = "CH3_DSTATAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_dstatar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_dstatar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_dstatar1`] module"]
pub type CH3_DSTATAR1 = crate::Reg<ch3_dstatar1::CH3_DSTATAR1_SPEC>;
#[doc = "NA"]
pub mod ch3_dstatar1;
#[doc = "CH3_INTSTATUS_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_intstatus_enable0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_intstatus_enable0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_intstatus_enable0`] module"]
pub type CH3_INTSTATUS_ENABLE0 = crate::Reg<ch3_intstatus_enable0::CH3_INTSTATUS_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod ch3_intstatus_enable0;
#[doc = "CH3_INTSTATUS_ENABLE1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_intstatus_enable1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_intstatus_enable1`] module"]
pub type CH3_INTSTATUS_ENABLE1 = crate::Reg<ch3_intstatus_enable1::CH3_INTSTATUS_ENABLE1_SPEC>;
#[doc = "NA"]
pub mod ch3_intstatus_enable1;
#[doc = "CH3_INTSTATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_intstatus0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_intstatus0`] module"]
pub type CH3_INTSTATUS0 = crate::Reg<ch3_intstatus0::CH3_INTSTATUS0_SPEC>;
#[doc = "NA"]
pub mod ch3_intstatus0;
#[doc = "CH3_INTSTATUS1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_intstatus1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_intstatus1`] module"]
pub type CH3_INTSTATUS1 = crate::Reg<ch3_intstatus1::CH3_INTSTATUS1_SPEC>;
#[doc = "NA"]
pub mod ch3_intstatus1;
#[doc = "CH3_INTSIGNAL_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_intsignal_enable0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_intsignal_enable0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_intsignal_enable0`] module"]
pub type CH3_INTSIGNAL_ENABLE0 = crate::Reg<ch3_intsignal_enable0::CH3_INTSIGNAL_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod ch3_intsignal_enable0;
#[doc = "CH3_INTSIGNAL_ENABLE1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3_intsignal_enable1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_intsignal_enable1`] module"]
pub type CH3_INTSIGNAL_ENABLE1 = crate::Reg<ch3_intsignal_enable1::CH3_INTSIGNAL_ENABLE1_SPEC>;
#[doc = "NA"]
pub mod ch3_intsignal_enable1;
#[doc = "CH3_INTCLEAR0 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_intclear0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_intclear0`] module"]
pub type CH3_INTCLEAR0 = crate::Reg<ch3_intclear0::CH3_INTCLEAR0_SPEC>;
#[doc = "NA"]
pub mod ch3_intclear0;
#[doc = "CH3_INTCLEAR1 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_intclear1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_intclear1`] module"]
pub type CH3_INTCLEAR1 = crate::Reg<ch3_intclear1::CH3_INTCLEAR1_SPEC>;
#[doc = "NA"]
pub mod ch3_intclear1;
#[doc = "CH4_SAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_sar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_sar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_sar0`] module"]
pub type CH4_SAR0 = crate::Reg<ch4_sar0::CH4_SAR0_SPEC>;
#[doc = "NA"]
pub mod ch4_sar0;
#[doc = "CH4_SAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_sar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_sar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_sar1`] module"]
pub type CH4_SAR1 = crate::Reg<ch4_sar1::CH4_SAR1_SPEC>;
#[doc = "NA"]
pub mod ch4_sar1;
#[doc = "CH4_DAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_dar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_dar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_dar0`] module"]
pub type CH4_DAR0 = crate::Reg<ch4_dar0::CH4_DAR0_SPEC>;
#[doc = "NA"]
pub mod ch4_dar0;
#[doc = "CH4_DAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_dar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_dar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_dar1`] module"]
pub type CH4_DAR1 = crate::Reg<ch4_dar1::CH4_DAR1_SPEC>;
#[doc = "NA"]
pub mod ch4_dar1;
#[doc = "CH4_BLOCK_TS0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_block_ts0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_block_ts0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_block_ts0`] module"]
pub type CH4_BLOCK_TS0 = crate::Reg<ch4_block_ts0::CH4_BLOCK_TS0_SPEC>;
#[doc = "NA"]
pub mod ch4_block_ts0;
#[doc = "CH4_CTL0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ctl0`] module"]
pub type CH4_CTL0 = crate::Reg<ch4_ctl0::CH4_CTL0_SPEC>;
#[doc = "NA"]
pub mod ch4_ctl0;
#[doc = "CH4_CTL1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ctl1`] module"]
pub type CH4_CTL1 = crate::Reg<ch4_ctl1::CH4_CTL1_SPEC>;
#[doc = "NA"]
pub mod ch4_ctl1;
#[doc = "CH4_CFG0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_cfg0`] module"]
pub type CH4_CFG0 = crate::Reg<ch4_cfg0::CH4_CFG0_SPEC>;
#[doc = "NA"]
pub mod ch4_cfg0;
#[doc = "CH4_CFG1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_cfg1`] module"]
pub type CH4_CFG1 = crate::Reg<ch4_cfg1::CH4_CFG1_SPEC>;
#[doc = "NA"]
pub mod ch4_cfg1;
#[doc = "CH4_LLP0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_llp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_llp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_llp0`] module"]
pub type CH4_LLP0 = crate::Reg<ch4_llp0::CH4_LLP0_SPEC>;
#[doc = "NA"]
pub mod ch4_llp0;
#[doc = "CH4_LLP1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_llp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_llp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_llp1`] module"]
pub type CH4_LLP1 = crate::Reg<ch4_llp1::CH4_LLP1_SPEC>;
#[doc = "NA"]
pub mod ch4_llp1;
#[doc = "CH4_STATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_status0`] module"]
pub type CH4_STATUS0 = crate::Reg<ch4_status0::CH4_STATUS0_SPEC>;
#[doc = "NA"]
pub mod ch4_status0;
#[doc = "CH4_STATUS1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_status1`] module"]
pub type CH4_STATUS1 = crate::Reg<ch4_status1::CH4_STATUS1_SPEC>;
#[doc = "NA"]
pub mod ch4_status1;
#[doc = "CH4_SWHSSRC0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_swhssrc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_swhssrc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_swhssrc0`] module"]
pub type CH4_SWHSSRC0 = crate::Reg<ch4_swhssrc0::CH4_SWHSSRC0_SPEC>;
#[doc = "NA"]
pub mod ch4_swhssrc0;
#[doc = "CH4_SWHSDST0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_swhsdst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_swhsdst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_swhsdst0`] module"]
pub type CH4_SWHSDST0 = crate::Reg<ch4_swhsdst0::CH4_SWHSDST0_SPEC>;
#[doc = "NA"]
pub mod ch4_swhsdst0;
#[doc = "CH4_BLK_TFR_RESUMEREQ0 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_blk_tfr_resumereq0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_blk_tfr_resumereq0`] module"]
pub type CH4_BLK_TFR_RESUMEREQ0 = crate::Reg<ch4_blk_tfr_resumereq0::CH4_BLK_TFR_RESUMEREQ0_SPEC>;
#[doc = "NA"]
pub mod ch4_blk_tfr_resumereq0;
#[doc = "CH4_AXI_ID0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_axi_id0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_axi_id0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_axi_id0`] module"]
pub type CH4_AXI_ID0 = crate::Reg<ch4_axi_id0::CH4_AXI_ID0_SPEC>;
#[doc = "NA"]
pub mod ch4_axi_id0;
#[doc = "CH4_AXI_QOS0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_axi_qos0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_axi_qos0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_axi_qos0`] module"]
pub type CH4_AXI_QOS0 = crate::Reg<ch4_axi_qos0::CH4_AXI_QOS0_SPEC>;
#[doc = "NA"]
pub mod ch4_axi_qos0;
#[doc = "CH4_SSTAT0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_sstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_sstat0`] module"]
pub type CH4_SSTAT0 = crate::Reg<ch4_sstat0::CH4_SSTAT0_SPEC>;
#[doc = "NA"]
pub mod ch4_sstat0;
#[doc = "CH4_DSTAT0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_dstat0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_dstat0`] module"]
pub type CH4_DSTAT0 = crate::Reg<ch4_dstat0::CH4_DSTAT0_SPEC>;
#[doc = "NA"]
pub mod ch4_dstat0;
#[doc = "CH4_SSTATAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_sstatar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_sstatar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_sstatar0`] module"]
pub type CH4_SSTATAR0 = crate::Reg<ch4_sstatar0::CH4_SSTATAR0_SPEC>;
#[doc = "NA"]
pub mod ch4_sstatar0;
#[doc = "CH4_SSTATAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_sstatar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_sstatar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_sstatar1`] module"]
pub type CH4_SSTATAR1 = crate::Reg<ch4_sstatar1::CH4_SSTATAR1_SPEC>;
#[doc = "NA"]
pub mod ch4_sstatar1;
#[doc = "CH4_DSTATAR0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_dstatar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_dstatar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_dstatar0`] module"]
pub type CH4_DSTATAR0 = crate::Reg<ch4_dstatar0::CH4_DSTATAR0_SPEC>;
#[doc = "NA"]
pub mod ch4_dstatar0;
#[doc = "CH4_DSTATAR1 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_dstatar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_dstatar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_dstatar1`] module"]
pub type CH4_DSTATAR1 = crate::Reg<ch4_dstatar1::CH4_DSTATAR1_SPEC>;
#[doc = "NA"]
pub mod ch4_dstatar1;
#[doc = "CH4_INTSTATUS_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_intstatus_enable0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_intstatus_enable0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_intstatus_enable0`] module"]
pub type CH4_INTSTATUS_ENABLE0 = crate::Reg<ch4_intstatus_enable0::CH4_INTSTATUS_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod ch4_intstatus_enable0;
#[doc = "CH4_INTSTATUS_ENABLE1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_intstatus_enable1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_intstatus_enable1`] module"]
pub type CH4_INTSTATUS_ENABLE1 = crate::Reg<ch4_intstatus_enable1::CH4_INTSTATUS_ENABLE1_SPEC>;
#[doc = "NA"]
pub mod ch4_intstatus_enable1;
#[doc = "CH4_INTSTATUS0 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_intstatus0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_intstatus0`] module"]
pub type CH4_INTSTATUS0 = crate::Reg<ch4_intstatus0::CH4_INTSTATUS0_SPEC>;
#[doc = "NA"]
pub mod ch4_intstatus0;
#[doc = "CH4_INTSTATUS1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_intstatus1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_intstatus1`] module"]
pub type CH4_INTSTATUS1 = crate::Reg<ch4_intstatus1::CH4_INTSTATUS1_SPEC>;
#[doc = "NA"]
pub mod ch4_intstatus1;
#[doc = "CH4_INTSIGNAL_ENABLE0 (rw) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_intsignal_enable0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_intsignal_enable0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_intsignal_enable0`] module"]
pub type CH4_INTSIGNAL_ENABLE0 = crate::Reg<ch4_intsignal_enable0::CH4_INTSIGNAL_ENABLE0_SPEC>;
#[doc = "NA"]
pub mod ch4_intsignal_enable0;
#[doc = "CH4_INTSIGNAL_ENABLE1 (r) register accessor: NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch4_intsignal_enable1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_intsignal_enable1`] module"]
pub type CH4_INTSIGNAL_ENABLE1 = crate::Reg<ch4_intsignal_enable1::CH4_INTSIGNAL_ENABLE1_SPEC>;
#[doc = "NA"]
pub mod ch4_intsignal_enable1;
#[doc = "CH4_INTCLEAR0 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_intclear0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_intclear0`] module"]
pub type CH4_INTCLEAR0 = crate::Reg<ch4_intclear0::CH4_INTCLEAR0_SPEC>;
#[doc = "NA"]
pub mod ch4_intclear0;
#[doc = "CH4_INTCLEAR1 (w) register accessor: NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_intclear1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_intclear1`] module"]
pub type CH4_INTCLEAR1 = crate::Reg<ch4_intclear1::CH4_INTCLEAR1_SPEC>;
#[doc = "NA"]
pub mod ch4_intclear1;
