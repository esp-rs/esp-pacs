#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    pmup_bitmap_high0: PMUP_BITMAP_HIGH0,
    pmup_bitmap_high1: PMUP_BITMAP_HIGH1,
    pmup_bitmap_high2: PMUP_BITMAP_HIGH2,
    pmup_bitmap_high3: PMUP_BITMAP_HIGH3,
    pmup_bitmap_high4: PMUP_BITMAP_HIGH4,
    pmup_bitmap_low0: PMUP_BITMAP_LOW0,
    pmup_bitmap_low1: PMUP_BITMAP_LOW1,
    pmup_bitmap_low2: PMUP_BITMAP_LOW2,
    pmup_bitmap_low3: PMUP_BITMAP_LOW3,
    pmup_bitmap_low4: PMUP_BITMAP_LOW4,
    pmup_drv_cfg: PMUP_DRV_CFG,
    pmup_channel_cfg: PMUP_CHANNEL_CFG,
    clk_cfg: CLK_CFG,
    dbias_channel_sel0: DBIAS_CHANNEL_SEL0,
    dbias_channel_sel1: DBIAS_CHANNEL_SEL1,
    dbias_channel0_sel: DBIAS_CHANNEL0_SEL,
    dbias_channel1_sel: DBIAS_CHANNEL1_SEL,
    dbias_channel2_sel: DBIAS_CHANNEL2_SEL,
    dbias_channel3_sel: DBIAS_CHANNEL3_SEL,
    dbias_channel4_sel: DBIAS_CHANNEL4_SEL,
    dbias_cmd0: DBIAS_CMD0,
    dbias_cmd1: DBIAS_CMD1,
    dbias_cmd2: DBIAS_CMD2,
    dbias_cmd3: DBIAS_CMD3,
    dbias_cmd4: DBIAS_CMD4,
    dbias_timer: DBIAS_TIMER,
    comb_pd_site0_unit0_vt0_conf1: COMB_PD_SITE0_UNIT0_VT0_CONF1,
    comb_pd_site0_unit1_vt0_conf1: COMB_PD_SITE0_UNIT1_VT0_CONF1,
    comb_pd_site0_unit2_vt0_conf1: COMB_PD_SITE0_UNIT2_VT0_CONF1,
    comb_pd_site0_unit3_vt0_conf1: COMB_PD_SITE0_UNIT3_VT0_CONF1,
    comb_pd_site0_unit0_vt1_conf1: COMB_PD_SITE0_UNIT0_VT1_CONF1,
    comb_pd_site0_unit1_vt1_conf1: COMB_PD_SITE0_UNIT1_VT1_CONF1,
    comb_pd_site0_unit2_vt1_conf1: COMB_PD_SITE0_UNIT2_VT1_CONF1,
    comb_pd_site0_unit3_vt1_conf1: COMB_PD_SITE0_UNIT3_VT1_CONF1,
    comb_pd_site0_unit0_vt2_conf1: COMB_PD_SITE0_UNIT0_VT2_CONF1,
    comb_pd_site0_unit1_vt2_conf1: COMB_PD_SITE0_UNIT1_VT2_CONF1,
    comb_pd_site0_unit2_vt2_conf1: COMB_PD_SITE0_UNIT2_VT2_CONF1,
    comb_pd_site0_unit3_vt2_conf1: COMB_PD_SITE0_UNIT3_VT2_CONF1,
    comb_pd_site1_unit0_vt0_conf1: COMB_PD_SITE1_UNIT0_VT0_CONF1,
    comb_pd_site1_unit1_vt0_conf1: COMB_PD_SITE1_UNIT1_VT0_CONF1,
    comb_pd_site1_unit2_vt0_conf1: COMB_PD_SITE1_UNIT2_VT0_CONF1,
    comb_pd_site1_unit3_vt0_conf1: COMB_PD_SITE1_UNIT3_VT0_CONF1,
    comb_pd_site1_unit0_vt1_conf1: COMB_PD_SITE1_UNIT0_VT1_CONF1,
    comb_pd_site1_unit1_vt1_conf1: COMB_PD_SITE1_UNIT1_VT1_CONF1,
    comb_pd_site1_unit2_vt1_conf1: COMB_PD_SITE1_UNIT2_VT1_CONF1,
    comb_pd_site1_unit3_vt1_conf1: COMB_PD_SITE1_UNIT3_VT1_CONF1,
    comb_pd_site1_unit0_vt2_conf1: COMB_PD_SITE1_UNIT0_VT2_CONF1,
    comb_pd_site1_unit1_vt2_conf1: COMB_PD_SITE1_UNIT1_VT2_CONF1,
    comb_pd_site1_unit2_vt2_conf1: COMB_PD_SITE1_UNIT2_VT2_CONF1,
    comb_pd_site1_unit3_vt2_conf1: COMB_PD_SITE1_UNIT3_VT2_CONF1,
    comb_pd_site2_unit0_vt0_conf1: COMB_PD_SITE2_UNIT0_VT0_CONF1,
    comb_pd_site2_unit1_vt0_conf1: COMB_PD_SITE2_UNIT1_VT0_CONF1,
    comb_pd_site2_unit2_vt0_conf1: COMB_PD_SITE2_UNIT2_VT0_CONF1,
    comb_pd_site2_unit3_vt0_conf1: COMB_PD_SITE2_UNIT3_VT0_CONF1,
    comb_pd_site2_unit0_vt1_conf1: COMB_PD_SITE2_UNIT0_VT1_CONF1,
    comb_pd_site2_unit1_vt1_conf1: COMB_PD_SITE2_UNIT1_VT1_CONF1,
    comb_pd_site2_unit2_vt1_conf1: COMB_PD_SITE2_UNIT2_VT1_CONF1,
    comb_pd_site2_unit3_vt1_conf1: COMB_PD_SITE2_UNIT3_VT1_CONF1,
    comb_pd_site2_unit0_vt2_conf1: COMB_PD_SITE2_UNIT0_VT2_CONF1,
    comb_pd_site2_unit1_vt2_conf1: COMB_PD_SITE2_UNIT1_VT2_CONF1,
    comb_pd_site2_unit2_vt2_conf1: COMB_PD_SITE2_UNIT2_VT2_CONF1,
    comb_pd_site2_unit3_vt2_conf1: COMB_PD_SITE2_UNIT3_VT2_CONF1,
    comb_pd_site3_unit0_vt0_conf1: COMB_PD_SITE3_UNIT0_VT0_CONF1,
    comb_pd_site3_unit1_vt0_conf1: COMB_PD_SITE3_UNIT1_VT0_CONF1,
    comb_pd_site3_unit2_vt0_conf1: COMB_PD_SITE3_UNIT2_VT0_CONF1,
    comb_pd_site3_unit3_vt0_conf1: COMB_PD_SITE3_UNIT3_VT0_CONF1,
    comb_pd_site3_unit0_vt1_conf1: COMB_PD_SITE3_UNIT0_VT1_CONF1,
    comb_pd_site3_unit1_vt1_conf1: COMB_PD_SITE3_UNIT1_VT1_CONF1,
    comb_pd_site3_unit2_vt1_conf1: COMB_PD_SITE3_UNIT2_VT1_CONF1,
    comb_pd_site3_unit3_vt1_conf1: COMB_PD_SITE3_UNIT3_VT1_CONF1,
    comb_pd_site3_unit0_vt2_conf1: COMB_PD_SITE3_UNIT0_VT2_CONF1,
    comb_pd_site3_unit1_vt2_conf1: COMB_PD_SITE3_UNIT1_VT2_CONF1,
    comb_pd_site3_unit2_vt2_conf1: COMB_PD_SITE3_UNIT2_VT2_CONF1,
    comb_pd_site3_unit3_vt2_conf1: COMB_PD_SITE3_UNIT3_VT2_CONF1,
    comb_pd_site0_unit0_vt0_conf2: COMB_PD_SITE0_UNIT0_VT0_CONF2,
    comb_pd_site0_unit1_vt0_conf2: COMB_PD_SITE0_UNIT1_VT0_CONF2,
    comb_pd_site0_unit2_vt0_conf2: COMB_PD_SITE0_UNIT2_VT0_CONF2,
    comb_pd_site0_unit3_vt0_conf2: COMB_PD_SITE0_UNIT3_VT0_CONF2,
    comb_pd_site0_unit0_vt1_conf2: COMB_PD_SITE0_UNIT0_VT1_CONF2,
    comb_pd_site0_unit1_vt1_conf2: COMB_PD_SITE0_UNIT1_VT1_CONF2,
    comb_pd_site0_unit2_vt1_conf2: COMB_PD_SITE0_UNIT2_VT1_CONF2,
    comb_pd_site0_unit3_vt1_conf2: COMB_PD_SITE0_UNIT3_VT1_CONF2,
    comb_pd_site0_unit0_vt2_conf2: COMB_PD_SITE0_UNIT0_VT2_CONF2,
    comb_pd_site0_unit1_vt2_conf2: COMB_PD_SITE0_UNIT1_VT2_CONF2,
    comb_pd_site0_unit2_vt2_conf2: COMB_PD_SITE0_UNIT2_VT2_CONF2,
    comb_pd_site0_unit3_vt2_conf2: COMB_PD_SITE0_UNIT3_VT2_CONF2,
    comb_pd_site1_unit0_vt0_conf2: COMB_PD_SITE1_UNIT0_VT0_CONF2,
    comb_pd_site1_unit1_vt0_conf2: COMB_PD_SITE1_UNIT1_VT0_CONF2,
    comb_pd_site1_unit2_vt0_conf2: COMB_PD_SITE1_UNIT2_VT0_CONF2,
    comb_pd_site1_unit3_vt0_conf2: COMB_PD_SITE1_UNIT3_VT0_CONF2,
    comb_pd_site1_unit0_vt1_conf2: COMB_PD_SITE1_UNIT0_VT1_CONF2,
    comb_pd_site1_unit1_vt1_conf2: COMB_PD_SITE1_UNIT1_VT1_CONF2,
    comb_pd_site1_unit2_vt1_conf2: COMB_PD_SITE1_UNIT2_VT1_CONF2,
    comb_pd_site1_unit3_vt1_conf2: COMB_PD_SITE1_UNIT3_VT1_CONF2,
    comb_pd_site1_unit0_vt2_conf2: COMB_PD_SITE1_UNIT0_VT2_CONF2,
    comb_pd_site1_unit1_vt2_conf2: COMB_PD_SITE1_UNIT1_VT2_CONF2,
    comb_pd_site1_unit2_vt2_conf2: COMB_PD_SITE1_UNIT2_VT2_CONF2,
    comb_pd_site1_unit3_vt2_conf2: COMB_PD_SITE1_UNIT3_VT2_CONF2,
    comb_pd_site2_unit0_vt0_conf2: COMB_PD_SITE2_UNIT0_VT0_CONF2,
    comb_pd_site2_unit1_vt0_conf2: COMB_PD_SITE2_UNIT1_VT0_CONF2,
    comb_pd_site2_unit2_vt0_conf2: COMB_PD_SITE2_UNIT2_VT0_CONF2,
    comb_pd_site2_unit3_vt0_conf2: COMB_PD_SITE2_UNIT3_VT0_CONF2,
    comb_pd_site2_unit0_vt1_conf2: COMB_PD_SITE2_UNIT0_VT1_CONF2,
    comb_pd_site2_unit1_vt1_conf2: COMB_PD_SITE2_UNIT1_VT1_CONF2,
    comb_pd_site2_unit2_vt1_conf2: COMB_PD_SITE2_UNIT2_VT1_CONF2,
    comb_pd_site2_unit3_vt1_conf2: COMB_PD_SITE2_UNIT3_VT1_CONF2,
    comb_pd_site2_unit0_vt2_conf2: COMB_PD_SITE2_UNIT0_VT2_CONF2,
    comb_pd_site2_unit1_vt2_conf2: COMB_PD_SITE2_UNIT1_VT2_CONF2,
    comb_pd_site2_unit2_vt2_conf2: COMB_PD_SITE2_UNIT2_VT2_CONF2,
    comb_pd_site2_unit3_vt2_conf2: COMB_PD_SITE2_UNIT3_VT2_CONF2,
    comb_pd_site3_unit0_vt0_conf2: COMB_PD_SITE3_UNIT0_VT0_CONF2,
    comb_pd_site3_unit1_vt0_conf2: COMB_PD_SITE3_UNIT1_VT0_CONF2,
    comb_pd_site3_unit2_vt0_conf2: COMB_PD_SITE3_UNIT2_VT0_CONF2,
    comb_pd_site3_unit3_vt0_conf2: COMB_PD_SITE3_UNIT3_VT0_CONF2,
    comb_pd_site3_unit0_vt1_conf2: COMB_PD_SITE3_UNIT0_VT1_CONF2,
    comb_pd_site3_unit1_vt1_conf2: COMB_PD_SITE3_UNIT1_VT1_CONF2,
    comb_pd_site3_unit2_vt1_conf2: COMB_PD_SITE3_UNIT2_VT1_CONF2,
    comb_pd_site3_unit3_vt1_conf2: COMB_PD_SITE3_UNIT3_VT1_CONF2,
    comb_pd_site3_unit0_vt2_conf2: COMB_PD_SITE3_UNIT0_VT2_CONF2,
    comb_pd_site3_unit1_vt2_conf2: COMB_PD_SITE3_UNIT1_VT2_CONF2,
    comb_pd_site3_unit2_vt2_conf2: COMB_PD_SITE3_UNIT2_VT2_CONF2,
    comb_pd_site3_unit3_vt2_conf2: COMB_PD_SITE3_UNIT3_VT2_CONF2,
    value_update: VALUE_UPDATE,
    _reserved123: [u8; 0x0e10],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - select valid pvt channel"]
    #[inline(always)]
    pub const fn pmup_bitmap_high0(&self) -> &PMUP_BITMAP_HIGH0 {
        &self.pmup_bitmap_high0
    }
    #[doc = "0x04 - select valid pvt channel"]
    #[inline(always)]
    pub const fn pmup_bitmap_high1(&self) -> &PMUP_BITMAP_HIGH1 {
        &self.pmup_bitmap_high1
    }
    #[doc = "0x08 - select valid pvt channel"]
    #[inline(always)]
    pub const fn pmup_bitmap_high2(&self) -> &PMUP_BITMAP_HIGH2 {
        &self.pmup_bitmap_high2
    }
    #[doc = "0x0c - select valid pvt channel"]
    #[inline(always)]
    pub const fn pmup_bitmap_high3(&self) -> &PMUP_BITMAP_HIGH3 {
        &self.pmup_bitmap_high3
    }
    #[doc = "0x10 - select valid pvt channel"]
    #[inline(always)]
    pub const fn pmup_bitmap_high4(&self) -> &PMUP_BITMAP_HIGH4 {
        &self.pmup_bitmap_high4
    }
    #[doc = "0x14 - select valid pvt channel"]
    #[inline(always)]
    pub const fn pmup_bitmap_low0(&self) -> &PMUP_BITMAP_LOW0 {
        &self.pmup_bitmap_low0
    }
    #[doc = "0x18 - select valid pvt channel"]
    #[inline(always)]
    pub const fn pmup_bitmap_low1(&self) -> &PMUP_BITMAP_LOW1 {
        &self.pmup_bitmap_low1
    }
    #[doc = "0x1c - select valid pvt channel"]
    #[inline(always)]
    pub const fn pmup_bitmap_low2(&self) -> &PMUP_BITMAP_LOW2 {
        &self.pmup_bitmap_low2
    }
    #[doc = "0x20 - select valid pvt channel"]
    #[inline(always)]
    pub const fn pmup_bitmap_low3(&self) -> &PMUP_BITMAP_LOW3 {
        &self.pmup_bitmap_low3
    }
    #[doc = "0x24 - select valid pvt channel"]
    #[inline(always)]
    pub const fn pmup_bitmap_low4(&self) -> &PMUP_BITMAP_LOW4 {
        &self.pmup_bitmap_low4
    }
    #[doc = "0x28 - configure pump drv"]
    #[inline(always)]
    pub const fn pmup_drv_cfg(&self) -> &PMUP_DRV_CFG {
        &self.pmup_drv_cfg
    }
    #[doc = "0x2c - configure the code of valid pump channel code"]
    #[inline(always)]
    pub const fn pmup_channel_cfg(&self) -> &PMUP_CHANNEL_CFG {
        &self.pmup_channel_cfg
    }
    #[doc = "0x30 - configure pvt clk"]
    #[inline(always)]
    pub const fn clk_cfg(&self) -> &CLK_CFG {
        &self.clk_cfg
    }
    #[doc = "0x34 - needs desc"]
    #[inline(always)]
    pub const fn dbias_channel_sel0(&self) -> &DBIAS_CHANNEL_SEL0 {
        &self.dbias_channel_sel0
    }
    #[doc = "0x38 - needs desc"]
    #[inline(always)]
    pub const fn dbias_channel_sel1(&self) -> &DBIAS_CHANNEL_SEL1 {
        &self.dbias_channel_sel1
    }
    #[doc = "0x3c - needs desc"]
    #[inline(always)]
    pub const fn dbias_channel0_sel(&self) -> &DBIAS_CHANNEL0_SEL {
        &self.dbias_channel0_sel
    }
    #[doc = "0x40 - needs desc"]
    #[inline(always)]
    pub const fn dbias_channel1_sel(&self) -> &DBIAS_CHANNEL1_SEL {
        &self.dbias_channel1_sel
    }
    #[doc = "0x44 - needs desc"]
    #[inline(always)]
    pub const fn dbias_channel2_sel(&self) -> &DBIAS_CHANNEL2_SEL {
        &self.dbias_channel2_sel
    }
    #[doc = "0x48 - needs desc"]
    #[inline(always)]
    pub const fn dbias_channel3_sel(&self) -> &DBIAS_CHANNEL3_SEL {
        &self.dbias_channel3_sel
    }
    #[doc = "0x4c - needs desc"]
    #[inline(always)]
    pub const fn dbias_channel4_sel(&self) -> &DBIAS_CHANNEL4_SEL {
        &self.dbias_channel4_sel
    }
    #[doc = "0x50 - needs desc"]
    #[inline(always)]
    pub const fn dbias_cmd0(&self) -> &DBIAS_CMD0 {
        &self.dbias_cmd0
    }
    #[doc = "0x54 - needs desc"]
    #[inline(always)]
    pub const fn dbias_cmd1(&self) -> &DBIAS_CMD1 {
        &self.dbias_cmd1
    }
    #[doc = "0x58 - needs desc"]
    #[inline(always)]
    pub const fn dbias_cmd2(&self) -> &DBIAS_CMD2 {
        &self.dbias_cmd2
    }
    #[doc = "0x5c - needs desc"]
    #[inline(always)]
    pub const fn dbias_cmd3(&self) -> &DBIAS_CMD3 {
        &self.dbias_cmd3
    }
    #[doc = "0x60 - needs desc"]
    #[inline(always)]
    pub const fn dbias_cmd4(&self) -> &DBIAS_CMD4 {
        &self.dbias_cmd4
    }
    #[doc = "0x64 - needs desc"]
    #[inline(always)]
    pub const fn dbias_timer(&self) -> &DBIAS_TIMER {
        &self.dbias_timer
    }
    #[doc = "0x68 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit0_vt0_conf1(&self) -> &COMB_PD_SITE0_UNIT0_VT0_CONF1 {
        &self.comb_pd_site0_unit0_vt0_conf1
    }
    #[doc = "0x6c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit1_vt0_conf1(&self) -> &COMB_PD_SITE0_UNIT1_VT0_CONF1 {
        &self.comb_pd_site0_unit1_vt0_conf1
    }
    #[doc = "0x70 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit2_vt0_conf1(&self) -> &COMB_PD_SITE0_UNIT2_VT0_CONF1 {
        &self.comb_pd_site0_unit2_vt0_conf1
    }
    #[doc = "0x74 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit3_vt0_conf1(&self) -> &COMB_PD_SITE0_UNIT3_VT0_CONF1 {
        &self.comb_pd_site0_unit3_vt0_conf1
    }
    #[doc = "0x78 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit0_vt1_conf1(&self) -> &COMB_PD_SITE0_UNIT0_VT1_CONF1 {
        &self.comb_pd_site0_unit0_vt1_conf1
    }
    #[doc = "0x7c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit1_vt1_conf1(&self) -> &COMB_PD_SITE0_UNIT1_VT1_CONF1 {
        &self.comb_pd_site0_unit1_vt1_conf1
    }
    #[doc = "0x80 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit2_vt1_conf1(&self) -> &COMB_PD_SITE0_UNIT2_VT1_CONF1 {
        &self.comb_pd_site0_unit2_vt1_conf1
    }
    #[doc = "0x84 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit3_vt1_conf1(&self) -> &COMB_PD_SITE0_UNIT3_VT1_CONF1 {
        &self.comb_pd_site0_unit3_vt1_conf1
    }
    #[doc = "0x88 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit0_vt2_conf1(&self) -> &COMB_PD_SITE0_UNIT0_VT2_CONF1 {
        &self.comb_pd_site0_unit0_vt2_conf1
    }
    #[doc = "0x8c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit1_vt2_conf1(&self) -> &COMB_PD_SITE0_UNIT1_VT2_CONF1 {
        &self.comb_pd_site0_unit1_vt2_conf1
    }
    #[doc = "0x90 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit2_vt2_conf1(&self) -> &COMB_PD_SITE0_UNIT2_VT2_CONF1 {
        &self.comb_pd_site0_unit2_vt2_conf1
    }
    #[doc = "0x94 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit3_vt2_conf1(&self) -> &COMB_PD_SITE0_UNIT3_VT2_CONF1 {
        &self.comb_pd_site0_unit3_vt2_conf1
    }
    #[doc = "0x98 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit0_vt0_conf1(&self) -> &COMB_PD_SITE1_UNIT0_VT0_CONF1 {
        &self.comb_pd_site1_unit0_vt0_conf1
    }
    #[doc = "0x9c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit1_vt0_conf1(&self) -> &COMB_PD_SITE1_UNIT1_VT0_CONF1 {
        &self.comb_pd_site1_unit1_vt0_conf1
    }
    #[doc = "0xa0 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit2_vt0_conf1(&self) -> &COMB_PD_SITE1_UNIT2_VT0_CONF1 {
        &self.comb_pd_site1_unit2_vt0_conf1
    }
    #[doc = "0xa4 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit3_vt0_conf1(&self) -> &COMB_PD_SITE1_UNIT3_VT0_CONF1 {
        &self.comb_pd_site1_unit3_vt0_conf1
    }
    #[doc = "0xa8 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit0_vt1_conf1(&self) -> &COMB_PD_SITE1_UNIT0_VT1_CONF1 {
        &self.comb_pd_site1_unit0_vt1_conf1
    }
    #[doc = "0xac - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit1_vt1_conf1(&self) -> &COMB_PD_SITE1_UNIT1_VT1_CONF1 {
        &self.comb_pd_site1_unit1_vt1_conf1
    }
    #[doc = "0xb0 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit2_vt1_conf1(&self) -> &COMB_PD_SITE1_UNIT2_VT1_CONF1 {
        &self.comb_pd_site1_unit2_vt1_conf1
    }
    #[doc = "0xb4 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit3_vt1_conf1(&self) -> &COMB_PD_SITE1_UNIT3_VT1_CONF1 {
        &self.comb_pd_site1_unit3_vt1_conf1
    }
    #[doc = "0xb8 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit0_vt2_conf1(&self) -> &COMB_PD_SITE1_UNIT0_VT2_CONF1 {
        &self.comb_pd_site1_unit0_vt2_conf1
    }
    #[doc = "0xbc - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit1_vt2_conf1(&self) -> &COMB_PD_SITE1_UNIT1_VT2_CONF1 {
        &self.comb_pd_site1_unit1_vt2_conf1
    }
    #[doc = "0xc0 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit2_vt2_conf1(&self) -> &COMB_PD_SITE1_UNIT2_VT2_CONF1 {
        &self.comb_pd_site1_unit2_vt2_conf1
    }
    #[doc = "0xc4 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit3_vt2_conf1(&self) -> &COMB_PD_SITE1_UNIT3_VT2_CONF1 {
        &self.comb_pd_site1_unit3_vt2_conf1
    }
    #[doc = "0xc8 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit0_vt0_conf1(&self) -> &COMB_PD_SITE2_UNIT0_VT0_CONF1 {
        &self.comb_pd_site2_unit0_vt0_conf1
    }
    #[doc = "0xcc - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit1_vt0_conf1(&self) -> &COMB_PD_SITE2_UNIT1_VT0_CONF1 {
        &self.comb_pd_site2_unit1_vt0_conf1
    }
    #[doc = "0xd0 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit2_vt0_conf1(&self) -> &COMB_PD_SITE2_UNIT2_VT0_CONF1 {
        &self.comb_pd_site2_unit2_vt0_conf1
    }
    #[doc = "0xd4 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit3_vt0_conf1(&self) -> &COMB_PD_SITE2_UNIT3_VT0_CONF1 {
        &self.comb_pd_site2_unit3_vt0_conf1
    }
    #[doc = "0xd8 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit0_vt1_conf1(&self) -> &COMB_PD_SITE2_UNIT0_VT1_CONF1 {
        &self.comb_pd_site2_unit0_vt1_conf1
    }
    #[doc = "0xdc - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit1_vt1_conf1(&self) -> &COMB_PD_SITE2_UNIT1_VT1_CONF1 {
        &self.comb_pd_site2_unit1_vt1_conf1
    }
    #[doc = "0xe0 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit2_vt1_conf1(&self) -> &COMB_PD_SITE2_UNIT2_VT1_CONF1 {
        &self.comb_pd_site2_unit2_vt1_conf1
    }
    #[doc = "0xe4 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit3_vt1_conf1(&self) -> &COMB_PD_SITE2_UNIT3_VT1_CONF1 {
        &self.comb_pd_site2_unit3_vt1_conf1
    }
    #[doc = "0xe8 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit0_vt2_conf1(&self) -> &COMB_PD_SITE2_UNIT0_VT2_CONF1 {
        &self.comb_pd_site2_unit0_vt2_conf1
    }
    #[doc = "0xec - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit1_vt2_conf1(&self) -> &COMB_PD_SITE2_UNIT1_VT2_CONF1 {
        &self.comb_pd_site2_unit1_vt2_conf1
    }
    #[doc = "0xf0 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit2_vt2_conf1(&self) -> &COMB_PD_SITE2_UNIT2_VT2_CONF1 {
        &self.comb_pd_site2_unit2_vt2_conf1
    }
    #[doc = "0xf4 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit3_vt2_conf1(&self) -> &COMB_PD_SITE2_UNIT3_VT2_CONF1 {
        &self.comb_pd_site2_unit3_vt2_conf1
    }
    #[doc = "0xf8 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit0_vt0_conf1(&self) -> &COMB_PD_SITE3_UNIT0_VT0_CONF1 {
        &self.comb_pd_site3_unit0_vt0_conf1
    }
    #[doc = "0xfc - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit1_vt0_conf1(&self) -> &COMB_PD_SITE3_UNIT1_VT0_CONF1 {
        &self.comb_pd_site3_unit1_vt0_conf1
    }
    #[doc = "0x100 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit2_vt0_conf1(&self) -> &COMB_PD_SITE3_UNIT2_VT0_CONF1 {
        &self.comb_pd_site3_unit2_vt0_conf1
    }
    #[doc = "0x104 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit3_vt0_conf1(&self) -> &COMB_PD_SITE3_UNIT3_VT0_CONF1 {
        &self.comb_pd_site3_unit3_vt0_conf1
    }
    #[doc = "0x108 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit0_vt1_conf1(&self) -> &COMB_PD_SITE3_UNIT0_VT1_CONF1 {
        &self.comb_pd_site3_unit0_vt1_conf1
    }
    #[doc = "0x10c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit1_vt1_conf1(&self) -> &COMB_PD_SITE3_UNIT1_VT1_CONF1 {
        &self.comb_pd_site3_unit1_vt1_conf1
    }
    #[doc = "0x110 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit2_vt1_conf1(&self) -> &COMB_PD_SITE3_UNIT2_VT1_CONF1 {
        &self.comb_pd_site3_unit2_vt1_conf1
    }
    #[doc = "0x114 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit3_vt1_conf1(&self) -> &COMB_PD_SITE3_UNIT3_VT1_CONF1 {
        &self.comb_pd_site3_unit3_vt1_conf1
    }
    #[doc = "0x118 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit0_vt2_conf1(&self) -> &COMB_PD_SITE3_UNIT0_VT2_CONF1 {
        &self.comb_pd_site3_unit0_vt2_conf1
    }
    #[doc = "0x11c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit1_vt2_conf1(&self) -> &COMB_PD_SITE3_UNIT1_VT2_CONF1 {
        &self.comb_pd_site3_unit1_vt2_conf1
    }
    #[doc = "0x120 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit2_vt2_conf1(&self) -> &COMB_PD_SITE3_UNIT2_VT2_CONF1 {
        &self.comb_pd_site3_unit2_vt2_conf1
    }
    #[doc = "0x124 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit3_vt2_conf1(&self) -> &COMB_PD_SITE3_UNIT3_VT2_CONF1 {
        &self.comb_pd_site3_unit3_vt2_conf1
    }
    #[doc = "0x128 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit0_vt0_conf2(&self) -> &COMB_PD_SITE0_UNIT0_VT0_CONF2 {
        &self.comb_pd_site0_unit0_vt0_conf2
    }
    #[doc = "0x12c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit1_vt0_conf2(&self) -> &COMB_PD_SITE0_UNIT1_VT0_CONF2 {
        &self.comb_pd_site0_unit1_vt0_conf2
    }
    #[doc = "0x130 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit2_vt0_conf2(&self) -> &COMB_PD_SITE0_UNIT2_VT0_CONF2 {
        &self.comb_pd_site0_unit2_vt0_conf2
    }
    #[doc = "0x134 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit3_vt0_conf2(&self) -> &COMB_PD_SITE0_UNIT3_VT0_CONF2 {
        &self.comb_pd_site0_unit3_vt0_conf2
    }
    #[doc = "0x138 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit0_vt1_conf2(&self) -> &COMB_PD_SITE0_UNIT0_VT1_CONF2 {
        &self.comb_pd_site0_unit0_vt1_conf2
    }
    #[doc = "0x13c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit1_vt1_conf2(&self) -> &COMB_PD_SITE0_UNIT1_VT1_CONF2 {
        &self.comb_pd_site0_unit1_vt1_conf2
    }
    #[doc = "0x140 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit2_vt1_conf2(&self) -> &COMB_PD_SITE0_UNIT2_VT1_CONF2 {
        &self.comb_pd_site0_unit2_vt1_conf2
    }
    #[doc = "0x144 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit3_vt1_conf2(&self) -> &COMB_PD_SITE0_UNIT3_VT1_CONF2 {
        &self.comb_pd_site0_unit3_vt1_conf2
    }
    #[doc = "0x148 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit0_vt2_conf2(&self) -> &COMB_PD_SITE0_UNIT0_VT2_CONF2 {
        &self.comb_pd_site0_unit0_vt2_conf2
    }
    #[doc = "0x14c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit1_vt2_conf2(&self) -> &COMB_PD_SITE0_UNIT1_VT2_CONF2 {
        &self.comb_pd_site0_unit1_vt2_conf2
    }
    #[doc = "0x150 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit2_vt2_conf2(&self) -> &COMB_PD_SITE0_UNIT2_VT2_CONF2 {
        &self.comb_pd_site0_unit2_vt2_conf2
    }
    #[doc = "0x154 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site0_unit3_vt2_conf2(&self) -> &COMB_PD_SITE0_UNIT3_VT2_CONF2 {
        &self.comb_pd_site0_unit3_vt2_conf2
    }
    #[doc = "0x158 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit0_vt0_conf2(&self) -> &COMB_PD_SITE1_UNIT0_VT0_CONF2 {
        &self.comb_pd_site1_unit0_vt0_conf2
    }
    #[doc = "0x15c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit1_vt0_conf2(&self) -> &COMB_PD_SITE1_UNIT1_VT0_CONF2 {
        &self.comb_pd_site1_unit1_vt0_conf2
    }
    #[doc = "0x160 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit2_vt0_conf2(&self) -> &COMB_PD_SITE1_UNIT2_VT0_CONF2 {
        &self.comb_pd_site1_unit2_vt0_conf2
    }
    #[doc = "0x164 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit3_vt0_conf2(&self) -> &COMB_PD_SITE1_UNIT3_VT0_CONF2 {
        &self.comb_pd_site1_unit3_vt0_conf2
    }
    #[doc = "0x168 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit0_vt1_conf2(&self) -> &COMB_PD_SITE1_UNIT0_VT1_CONF2 {
        &self.comb_pd_site1_unit0_vt1_conf2
    }
    #[doc = "0x16c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit1_vt1_conf2(&self) -> &COMB_PD_SITE1_UNIT1_VT1_CONF2 {
        &self.comb_pd_site1_unit1_vt1_conf2
    }
    #[doc = "0x170 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit2_vt1_conf2(&self) -> &COMB_PD_SITE1_UNIT2_VT1_CONF2 {
        &self.comb_pd_site1_unit2_vt1_conf2
    }
    #[doc = "0x174 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit3_vt1_conf2(&self) -> &COMB_PD_SITE1_UNIT3_VT1_CONF2 {
        &self.comb_pd_site1_unit3_vt1_conf2
    }
    #[doc = "0x178 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit0_vt2_conf2(&self) -> &COMB_PD_SITE1_UNIT0_VT2_CONF2 {
        &self.comb_pd_site1_unit0_vt2_conf2
    }
    #[doc = "0x17c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit1_vt2_conf2(&self) -> &COMB_PD_SITE1_UNIT1_VT2_CONF2 {
        &self.comb_pd_site1_unit1_vt2_conf2
    }
    #[doc = "0x180 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit2_vt2_conf2(&self) -> &COMB_PD_SITE1_UNIT2_VT2_CONF2 {
        &self.comb_pd_site1_unit2_vt2_conf2
    }
    #[doc = "0x184 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site1_unit3_vt2_conf2(&self) -> &COMB_PD_SITE1_UNIT3_VT2_CONF2 {
        &self.comb_pd_site1_unit3_vt2_conf2
    }
    #[doc = "0x188 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit0_vt0_conf2(&self) -> &COMB_PD_SITE2_UNIT0_VT0_CONF2 {
        &self.comb_pd_site2_unit0_vt0_conf2
    }
    #[doc = "0x18c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit1_vt0_conf2(&self) -> &COMB_PD_SITE2_UNIT1_VT0_CONF2 {
        &self.comb_pd_site2_unit1_vt0_conf2
    }
    #[doc = "0x190 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit2_vt0_conf2(&self) -> &COMB_PD_SITE2_UNIT2_VT0_CONF2 {
        &self.comb_pd_site2_unit2_vt0_conf2
    }
    #[doc = "0x194 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit3_vt0_conf2(&self) -> &COMB_PD_SITE2_UNIT3_VT0_CONF2 {
        &self.comb_pd_site2_unit3_vt0_conf2
    }
    #[doc = "0x198 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit0_vt1_conf2(&self) -> &COMB_PD_SITE2_UNIT0_VT1_CONF2 {
        &self.comb_pd_site2_unit0_vt1_conf2
    }
    #[doc = "0x19c - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit1_vt1_conf2(&self) -> &COMB_PD_SITE2_UNIT1_VT1_CONF2 {
        &self.comb_pd_site2_unit1_vt1_conf2
    }
    #[doc = "0x1a0 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit2_vt1_conf2(&self) -> &COMB_PD_SITE2_UNIT2_VT1_CONF2 {
        &self.comb_pd_site2_unit2_vt1_conf2
    }
    #[doc = "0x1a4 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit3_vt1_conf2(&self) -> &COMB_PD_SITE2_UNIT3_VT1_CONF2 {
        &self.comb_pd_site2_unit3_vt1_conf2
    }
    #[doc = "0x1a8 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit0_vt2_conf2(&self) -> &COMB_PD_SITE2_UNIT0_VT2_CONF2 {
        &self.comb_pd_site2_unit0_vt2_conf2
    }
    #[doc = "0x1ac - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit1_vt2_conf2(&self) -> &COMB_PD_SITE2_UNIT1_VT2_CONF2 {
        &self.comb_pd_site2_unit1_vt2_conf2
    }
    #[doc = "0x1b0 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit2_vt2_conf2(&self) -> &COMB_PD_SITE2_UNIT2_VT2_CONF2 {
        &self.comb_pd_site2_unit2_vt2_conf2
    }
    #[doc = "0x1b4 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site2_unit3_vt2_conf2(&self) -> &COMB_PD_SITE2_UNIT3_VT2_CONF2 {
        &self.comb_pd_site2_unit3_vt2_conf2
    }
    #[doc = "0x1b8 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit0_vt0_conf2(&self) -> &COMB_PD_SITE3_UNIT0_VT0_CONF2 {
        &self.comb_pd_site3_unit0_vt0_conf2
    }
    #[doc = "0x1bc - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit1_vt0_conf2(&self) -> &COMB_PD_SITE3_UNIT1_VT0_CONF2 {
        &self.comb_pd_site3_unit1_vt0_conf2
    }
    #[doc = "0x1c0 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit2_vt0_conf2(&self) -> &COMB_PD_SITE3_UNIT2_VT0_CONF2 {
        &self.comb_pd_site3_unit2_vt0_conf2
    }
    #[doc = "0x1c4 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit3_vt0_conf2(&self) -> &COMB_PD_SITE3_UNIT3_VT0_CONF2 {
        &self.comb_pd_site3_unit3_vt0_conf2
    }
    #[doc = "0x1c8 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit0_vt1_conf2(&self) -> &COMB_PD_SITE3_UNIT0_VT1_CONF2 {
        &self.comb_pd_site3_unit0_vt1_conf2
    }
    #[doc = "0x1cc - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit1_vt1_conf2(&self) -> &COMB_PD_SITE3_UNIT1_VT1_CONF2 {
        &self.comb_pd_site3_unit1_vt1_conf2
    }
    #[doc = "0x1d0 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit2_vt1_conf2(&self) -> &COMB_PD_SITE3_UNIT2_VT1_CONF2 {
        &self.comb_pd_site3_unit2_vt1_conf2
    }
    #[doc = "0x1d4 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit3_vt1_conf2(&self) -> &COMB_PD_SITE3_UNIT3_VT1_CONF2 {
        &self.comb_pd_site3_unit3_vt1_conf2
    }
    #[doc = "0x1d8 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit0_vt2_conf2(&self) -> &COMB_PD_SITE3_UNIT0_VT2_CONF2 {
        &self.comb_pd_site3_unit0_vt2_conf2
    }
    #[doc = "0x1dc - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit1_vt2_conf2(&self) -> &COMB_PD_SITE3_UNIT1_VT2_CONF2 {
        &self.comb_pd_site3_unit1_vt2_conf2
    }
    #[doc = "0x1e0 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit2_vt2_conf2(&self) -> &COMB_PD_SITE3_UNIT2_VT2_CONF2 {
        &self.comb_pd_site3_unit2_vt2_conf2
    }
    #[doc = "0x1e4 - needs desc"]
    #[inline(always)]
    pub const fn comb_pd_site3_unit3_vt2_conf2(&self) -> &COMB_PD_SITE3_UNIT3_VT2_CONF2 {
        &self.comb_pd_site3_unit3_vt2_conf2
    }
    #[doc = "0x1e8 - needs field desc"]
    #[inline(always)]
    pub const fn value_update(&self) -> &VALUE_UPDATE {
        &self.value_update
    }
    #[doc = "0xffc - version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "PMUP_BITMAP_HIGH0 (rw) register accessor: select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_high0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_high0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_bitmap_high0`] module"]
pub type PMUP_BITMAP_HIGH0 = crate::Reg<pmup_bitmap_high0::PMUP_BITMAP_HIGH0_SPEC>;
#[doc = "select valid pvt channel"]
pub mod pmup_bitmap_high0;
#[doc = "PMUP_BITMAP_HIGH1 (rw) register accessor: select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_high1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_high1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_bitmap_high1`] module"]
pub type PMUP_BITMAP_HIGH1 = crate::Reg<pmup_bitmap_high1::PMUP_BITMAP_HIGH1_SPEC>;
#[doc = "select valid pvt channel"]
pub mod pmup_bitmap_high1;
#[doc = "PMUP_BITMAP_HIGH2 (rw) register accessor: select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_high2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_high2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_bitmap_high2`] module"]
pub type PMUP_BITMAP_HIGH2 = crate::Reg<pmup_bitmap_high2::PMUP_BITMAP_HIGH2_SPEC>;
#[doc = "select valid pvt channel"]
pub mod pmup_bitmap_high2;
#[doc = "PMUP_BITMAP_HIGH3 (rw) register accessor: select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_high3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_high3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_bitmap_high3`] module"]
pub type PMUP_BITMAP_HIGH3 = crate::Reg<pmup_bitmap_high3::PMUP_BITMAP_HIGH3_SPEC>;
#[doc = "select valid pvt channel"]
pub mod pmup_bitmap_high3;
#[doc = "PMUP_BITMAP_HIGH4 (rw) register accessor: select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_high4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_high4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_bitmap_high4`] module"]
pub type PMUP_BITMAP_HIGH4 = crate::Reg<pmup_bitmap_high4::PMUP_BITMAP_HIGH4_SPEC>;
#[doc = "select valid pvt channel"]
pub mod pmup_bitmap_high4;
#[doc = "PMUP_BITMAP_LOW0 (rw) register accessor: select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_low0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_low0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_bitmap_low0`] module"]
pub type PMUP_BITMAP_LOW0 = crate::Reg<pmup_bitmap_low0::PMUP_BITMAP_LOW0_SPEC>;
#[doc = "select valid pvt channel"]
pub mod pmup_bitmap_low0;
#[doc = "PMUP_BITMAP_LOW1 (rw) register accessor: select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_low1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_low1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_bitmap_low1`] module"]
pub type PMUP_BITMAP_LOW1 = crate::Reg<pmup_bitmap_low1::PMUP_BITMAP_LOW1_SPEC>;
#[doc = "select valid pvt channel"]
pub mod pmup_bitmap_low1;
#[doc = "PMUP_BITMAP_LOW2 (rw) register accessor: select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_low2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_low2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_bitmap_low2`] module"]
pub type PMUP_BITMAP_LOW2 = crate::Reg<pmup_bitmap_low2::PMUP_BITMAP_LOW2_SPEC>;
#[doc = "select valid pvt channel"]
pub mod pmup_bitmap_low2;
#[doc = "PMUP_BITMAP_LOW3 (rw) register accessor: select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_low3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_low3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_bitmap_low3`] module"]
pub type PMUP_BITMAP_LOW3 = crate::Reg<pmup_bitmap_low3::PMUP_BITMAP_LOW3_SPEC>;
#[doc = "select valid pvt channel"]
pub mod pmup_bitmap_low3;
#[doc = "PMUP_BITMAP_LOW4 (rw) register accessor: select valid pvt channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_bitmap_low4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_bitmap_low4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_bitmap_low4`] module"]
pub type PMUP_BITMAP_LOW4 = crate::Reg<pmup_bitmap_low4::PMUP_BITMAP_LOW4_SPEC>;
#[doc = "select valid pvt channel"]
pub mod pmup_bitmap_low4;
#[doc = "PMUP_DRV_CFG (rw) register accessor: configure pump drv\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_drv_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_drv_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_drv_cfg`] module"]
pub type PMUP_DRV_CFG = crate::Reg<pmup_drv_cfg::PMUP_DRV_CFG_SPEC>;
#[doc = "configure pump drv"]
pub mod pmup_drv_cfg;
#[doc = "PMUP_CHANNEL_CFG (rw) register accessor: configure the code of valid pump channel code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmup_channel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmup_channel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmup_channel_cfg`] module"]
pub type PMUP_CHANNEL_CFG = crate::Reg<pmup_channel_cfg::PMUP_CHANNEL_CFG_SPEC>;
#[doc = "configure the code of valid pump channel code"]
pub mod pmup_channel_cfg;
#[doc = "CLK_CFG (rw) register accessor: configure pvt clk\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cfg`] module"]
pub type CLK_CFG = crate::Reg<clk_cfg::CLK_CFG_SPEC>;
#[doc = "configure pvt clk"]
pub mod clk_cfg;
#[doc = "DBIAS_CHANNEL_SEL0 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_channel_sel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_channel_sel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_channel_sel0`] module"]
pub type DBIAS_CHANNEL_SEL0 = crate::Reg<dbias_channel_sel0::DBIAS_CHANNEL_SEL0_SPEC>;
#[doc = "needs desc"]
pub mod dbias_channel_sel0;
#[doc = "DBIAS_CHANNEL_SEL1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_channel_sel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_channel_sel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_channel_sel1`] module"]
pub type DBIAS_CHANNEL_SEL1 = crate::Reg<dbias_channel_sel1::DBIAS_CHANNEL_SEL1_SPEC>;
#[doc = "needs desc"]
pub mod dbias_channel_sel1;
#[doc = "DBIAS_CHANNEL0_SEL (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_channel0_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_channel0_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_channel0_sel`] module"]
pub type DBIAS_CHANNEL0_SEL = crate::Reg<dbias_channel0_sel::DBIAS_CHANNEL0_SEL_SPEC>;
#[doc = "needs desc"]
pub mod dbias_channel0_sel;
#[doc = "DBIAS_CHANNEL1_SEL (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_channel1_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_channel1_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_channel1_sel`] module"]
pub type DBIAS_CHANNEL1_SEL = crate::Reg<dbias_channel1_sel::DBIAS_CHANNEL1_SEL_SPEC>;
#[doc = "needs desc"]
pub mod dbias_channel1_sel;
#[doc = "DBIAS_CHANNEL2_SEL (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_channel2_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_channel2_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_channel2_sel`] module"]
pub type DBIAS_CHANNEL2_SEL = crate::Reg<dbias_channel2_sel::DBIAS_CHANNEL2_SEL_SPEC>;
#[doc = "needs desc"]
pub mod dbias_channel2_sel;
#[doc = "DBIAS_CHANNEL3_SEL (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_channel3_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_channel3_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_channel3_sel`] module"]
pub type DBIAS_CHANNEL3_SEL = crate::Reg<dbias_channel3_sel::DBIAS_CHANNEL3_SEL_SPEC>;
#[doc = "needs desc"]
pub mod dbias_channel3_sel;
#[doc = "DBIAS_CHANNEL4_SEL (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_channel4_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_channel4_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_channel4_sel`] module"]
pub type DBIAS_CHANNEL4_SEL = crate::Reg<dbias_channel4_sel::DBIAS_CHANNEL4_SEL_SPEC>;
#[doc = "needs desc"]
pub mod dbias_channel4_sel;
#[doc = "DBIAS_CMD0 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_cmd0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_cmd0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_cmd0`] module"]
pub type DBIAS_CMD0 = crate::Reg<dbias_cmd0::DBIAS_CMD0_SPEC>;
#[doc = "needs desc"]
pub mod dbias_cmd0;
#[doc = "DBIAS_CMD1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_cmd1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_cmd1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_cmd1`] module"]
pub type DBIAS_CMD1 = crate::Reg<dbias_cmd1::DBIAS_CMD1_SPEC>;
#[doc = "needs desc"]
pub mod dbias_cmd1;
#[doc = "DBIAS_CMD2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_cmd2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_cmd2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_cmd2`] module"]
pub type DBIAS_CMD2 = crate::Reg<dbias_cmd2::DBIAS_CMD2_SPEC>;
#[doc = "needs desc"]
pub mod dbias_cmd2;
#[doc = "DBIAS_CMD3 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_cmd3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_cmd3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_cmd3`] module"]
pub type DBIAS_CMD3 = crate::Reg<dbias_cmd3::DBIAS_CMD3_SPEC>;
#[doc = "needs desc"]
pub mod dbias_cmd3;
#[doc = "DBIAS_CMD4 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_cmd4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_cmd4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_cmd4`] module"]
pub type DBIAS_CMD4 = crate::Reg<dbias_cmd4::DBIAS_CMD4_SPEC>;
#[doc = "needs desc"]
pub mod dbias_cmd4;
#[doc = "DBIAS_TIMER (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_timer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_timer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbias_timer`] module"]
pub type DBIAS_TIMER = crate::Reg<dbias_timer::DBIAS_TIMER_SPEC>;
#[doc = "needs desc"]
pub mod dbias_timer;
#[doc = "COMB_PD_SITE0_UNIT0_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit0_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit0_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit0_vt0_conf1`] module"]
pub type COMB_PD_SITE0_UNIT0_VT0_CONF1 =
    crate::Reg<comb_pd_site0_unit0_vt0_conf1::COMB_PD_SITE0_UNIT0_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit0_vt0_conf1;
#[doc = "COMB_PD_SITE0_UNIT1_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit1_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit1_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit1_vt0_conf1`] module"]
pub type COMB_PD_SITE0_UNIT1_VT0_CONF1 =
    crate::Reg<comb_pd_site0_unit1_vt0_conf1::COMB_PD_SITE0_UNIT1_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit1_vt0_conf1;
#[doc = "COMB_PD_SITE0_UNIT2_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit2_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit2_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit2_vt0_conf1`] module"]
pub type COMB_PD_SITE0_UNIT2_VT0_CONF1 =
    crate::Reg<comb_pd_site0_unit2_vt0_conf1::COMB_PD_SITE0_UNIT2_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit2_vt0_conf1;
#[doc = "COMB_PD_SITE0_UNIT3_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit3_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit3_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit3_vt0_conf1`] module"]
pub type COMB_PD_SITE0_UNIT3_VT0_CONF1 =
    crate::Reg<comb_pd_site0_unit3_vt0_conf1::COMB_PD_SITE0_UNIT3_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit3_vt0_conf1;
#[doc = "COMB_PD_SITE0_UNIT0_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit0_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit0_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit0_vt1_conf1`] module"]
pub type COMB_PD_SITE0_UNIT0_VT1_CONF1 =
    crate::Reg<comb_pd_site0_unit0_vt1_conf1::COMB_PD_SITE0_UNIT0_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit0_vt1_conf1;
#[doc = "COMB_PD_SITE0_UNIT1_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit1_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit1_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit1_vt1_conf1`] module"]
pub type COMB_PD_SITE0_UNIT1_VT1_CONF1 =
    crate::Reg<comb_pd_site0_unit1_vt1_conf1::COMB_PD_SITE0_UNIT1_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit1_vt1_conf1;
#[doc = "COMB_PD_SITE0_UNIT2_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit2_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit2_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit2_vt1_conf1`] module"]
pub type COMB_PD_SITE0_UNIT2_VT1_CONF1 =
    crate::Reg<comb_pd_site0_unit2_vt1_conf1::COMB_PD_SITE0_UNIT2_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit2_vt1_conf1;
#[doc = "COMB_PD_SITE0_UNIT3_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit3_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit3_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit3_vt1_conf1`] module"]
pub type COMB_PD_SITE0_UNIT3_VT1_CONF1 =
    crate::Reg<comb_pd_site0_unit3_vt1_conf1::COMB_PD_SITE0_UNIT3_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit3_vt1_conf1;
#[doc = "COMB_PD_SITE0_UNIT0_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit0_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit0_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit0_vt2_conf1`] module"]
pub type COMB_PD_SITE0_UNIT0_VT2_CONF1 =
    crate::Reg<comb_pd_site0_unit0_vt2_conf1::COMB_PD_SITE0_UNIT0_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit0_vt2_conf1;
#[doc = "COMB_PD_SITE0_UNIT1_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit1_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit1_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit1_vt2_conf1`] module"]
pub type COMB_PD_SITE0_UNIT1_VT2_CONF1 =
    crate::Reg<comb_pd_site0_unit1_vt2_conf1::COMB_PD_SITE0_UNIT1_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit1_vt2_conf1;
#[doc = "COMB_PD_SITE0_UNIT2_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit2_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit2_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit2_vt2_conf1`] module"]
pub type COMB_PD_SITE0_UNIT2_VT2_CONF1 =
    crate::Reg<comb_pd_site0_unit2_vt2_conf1::COMB_PD_SITE0_UNIT2_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit2_vt2_conf1;
#[doc = "COMB_PD_SITE0_UNIT3_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit3_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit3_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit3_vt2_conf1`] module"]
pub type COMB_PD_SITE0_UNIT3_VT2_CONF1 =
    crate::Reg<comb_pd_site0_unit3_vt2_conf1::COMB_PD_SITE0_UNIT3_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit3_vt2_conf1;
#[doc = "COMB_PD_SITE1_UNIT0_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit0_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit0_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit0_vt0_conf1`] module"]
pub type COMB_PD_SITE1_UNIT0_VT0_CONF1 =
    crate::Reg<comb_pd_site1_unit0_vt0_conf1::COMB_PD_SITE1_UNIT0_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit0_vt0_conf1;
#[doc = "COMB_PD_SITE1_UNIT1_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit1_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit1_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit1_vt0_conf1`] module"]
pub type COMB_PD_SITE1_UNIT1_VT0_CONF1 =
    crate::Reg<comb_pd_site1_unit1_vt0_conf1::COMB_PD_SITE1_UNIT1_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit1_vt0_conf1;
#[doc = "COMB_PD_SITE1_UNIT2_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit2_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit2_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit2_vt0_conf1`] module"]
pub type COMB_PD_SITE1_UNIT2_VT0_CONF1 =
    crate::Reg<comb_pd_site1_unit2_vt0_conf1::COMB_PD_SITE1_UNIT2_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit2_vt0_conf1;
#[doc = "COMB_PD_SITE1_UNIT3_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit3_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit3_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit3_vt0_conf1`] module"]
pub type COMB_PD_SITE1_UNIT3_VT0_CONF1 =
    crate::Reg<comb_pd_site1_unit3_vt0_conf1::COMB_PD_SITE1_UNIT3_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit3_vt0_conf1;
#[doc = "COMB_PD_SITE1_UNIT0_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit0_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit0_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit0_vt1_conf1`] module"]
pub type COMB_PD_SITE1_UNIT0_VT1_CONF1 =
    crate::Reg<comb_pd_site1_unit0_vt1_conf1::COMB_PD_SITE1_UNIT0_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit0_vt1_conf1;
#[doc = "COMB_PD_SITE1_UNIT1_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit1_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit1_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit1_vt1_conf1`] module"]
pub type COMB_PD_SITE1_UNIT1_VT1_CONF1 =
    crate::Reg<comb_pd_site1_unit1_vt1_conf1::COMB_PD_SITE1_UNIT1_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit1_vt1_conf1;
#[doc = "COMB_PD_SITE1_UNIT2_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit2_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit2_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit2_vt1_conf1`] module"]
pub type COMB_PD_SITE1_UNIT2_VT1_CONF1 =
    crate::Reg<comb_pd_site1_unit2_vt1_conf1::COMB_PD_SITE1_UNIT2_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit2_vt1_conf1;
#[doc = "COMB_PD_SITE1_UNIT3_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit3_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit3_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit3_vt1_conf1`] module"]
pub type COMB_PD_SITE1_UNIT3_VT1_CONF1 =
    crate::Reg<comb_pd_site1_unit3_vt1_conf1::COMB_PD_SITE1_UNIT3_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit3_vt1_conf1;
#[doc = "COMB_PD_SITE1_UNIT0_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit0_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit0_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit0_vt2_conf1`] module"]
pub type COMB_PD_SITE1_UNIT0_VT2_CONF1 =
    crate::Reg<comb_pd_site1_unit0_vt2_conf1::COMB_PD_SITE1_UNIT0_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit0_vt2_conf1;
#[doc = "COMB_PD_SITE1_UNIT1_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit1_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit1_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit1_vt2_conf1`] module"]
pub type COMB_PD_SITE1_UNIT1_VT2_CONF1 =
    crate::Reg<comb_pd_site1_unit1_vt2_conf1::COMB_PD_SITE1_UNIT1_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit1_vt2_conf1;
#[doc = "COMB_PD_SITE1_UNIT2_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit2_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit2_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit2_vt2_conf1`] module"]
pub type COMB_PD_SITE1_UNIT2_VT2_CONF1 =
    crate::Reg<comb_pd_site1_unit2_vt2_conf1::COMB_PD_SITE1_UNIT2_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit2_vt2_conf1;
#[doc = "COMB_PD_SITE1_UNIT3_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit3_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit3_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit3_vt2_conf1`] module"]
pub type COMB_PD_SITE1_UNIT3_VT2_CONF1 =
    crate::Reg<comb_pd_site1_unit3_vt2_conf1::COMB_PD_SITE1_UNIT3_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit3_vt2_conf1;
#[doc = "COMB_PD_SITE2_UNIT0_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit0_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit0_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit0_vt0_conf1`] module"]
pub type COMB_PD_SITE2_UNIT0_VT0_CONF1 =
    crate::Reg<comb_pd_site2_unit0_vt0_conf1::COMB_PD_SITE2_UNIT0_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit0_vt0_conf1;
#[doc = "COMB_PD_SITE2_UNIT1_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit1_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit1_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit1_vt0_conf1`] module"]
pub type COMB_PD_SITE2_UNIT1_VT0_CONF1 =
    crate::Reg<comb_pd_site2_unit1_vt0_conf1::COMB_PD_SITE2_UNIT1_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit1_vt0_conf1;
#[doc = "COMB_PD_SITE2_UNIT2_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit2_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit2_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit2_vt0_conf1`] module"]
pub type COMB_PD_SITE2_UNIT2_VT0_CONF1 =
    crate::Reg<comb_pd_site2_unit2_vt0_conf1::COMB_PD_SITE2_UNIT2_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit2_vt0_conf1;
#[doc = "COMB_PD_SITE2_UNIT3_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit3_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit3_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit3_vt0_conf1`] module"]
pub type COMB_PD_SITE2_UNIT3_VT0_CONF1 =
    crate::Reg<comb_pd_site2_unit3_vt0_conf1::COMB_PD_SITE2_UNIT3_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit3_vt0_conf1;
#[doc = "COMB_PD_SITE2_UNIT0_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit0_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit0_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit0_vt1_conf1`] module"]
pub type COMB_PD_SITE2_UNIT0_VT1_CONF1 =
    crate::Reg<comb_pd_site2_unit0_vt1_conf1::COMB_PD_SITE2_UNIT0_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit0_vt1_conf1;
#[doc = "COMB_PD_SITE2_UNIT1_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit1_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit1_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit1_vt1_conf1`] module"]
pub type COMB_PD_SITE2_UNIT1_VT1_CONF1 =
    crate::Reg<comb_pd_site2_unit1_vt1_conf1::COMB_PD_SITE2_UNIT1_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit1_vt1_conf1;
#[doc = "COMB_PD_SITE2_UNIT2_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit2_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit2_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit2_vt1_conf1`] module"]
pub type COMB_PD_SITE2_UNIT2_VT1_CONF1 =
    crate::Reg<comb_pd_site2_unit2_vt1_conf1::COMB_PD_SITE2_UNIT2_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit2_vt1_conf1;
#[doc = "COMB_PD_SITE2_UNIT3_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit3_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit3_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit3_vt1_conf1`] module"]
pub type COMB_PD_SITE2_UNIT3_VT1_CONF1 =
    crate::Reg<comb_pd_site2_unit3_vt1_conf1::COMB_PD_SITE2_UNIT3_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit3_vt1_conf1;
#[doc = "COMB_PD_SITE2_UNIT0_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit0_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit0_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit0_vt2_conf1`] module"]
pub type COMB_PD_SITE2_UNIT0_VT2_CONF1 =
    crate::Reg<comb_pd_site2_unit0_vt2_conf1::COMB_PD_SITE2_UNIT0_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit0_vt2_conf1;
#[doc = "COMB_PD_SITE2_UNIT1_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit1_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit1_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit1_vt2_conf1`] module"]
pub type COMB_PD_SITE2_UNIT1_VT2_CONF1 =
    crate::Reg<comb_pd_site2_unit1_vt2_conf1::COMB_PD_SITE2_UNIT1_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit1_vt2_conf1;
#[doc = "COMB_PD_SITE2_UNIT2_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit2_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit2_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit2_vt2_conf1`] module"]
pub type COMB_PD_SITE2_UNIT2_VT2_CONF1 =
    crate::Reg<comb_pd_site2_unit2_vt2_conf1::COMB_PD_SITE2_UNIT2_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit2_vt2_conf1;
#[doc = "COMB_PD_SITE2_UNIT3_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit3_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit3_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit3_vt2_conf1`] module"]
pub type COMB_PD_SITE2_UNIT3_VT2_CONF1 =
    crate::Reg<comb_pd_site2_unit3_vt2_conf1::COMB_PD_SITE2_UNIT3_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit3_vt2_conf1;
#[doc = "COMB_PD_SITE3_UNIT0_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit0_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit0_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit0_vt0_conf1`] module"]
pub type COMB_PD_SITE3_UNIT0_VT0_CONF1 =
    crate::Reg<comb_pd_site3_unit0_vt0_conf1::COMB_PD_SITE3_UNIT0_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit0_vt0_conf1;
#[doc = "COMB_PD_SITE3_UNIT1_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit1_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit1_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit1_vt0_conf1`] module"]
pub type COMB_PD_SITE3_UNIT1_VT0_CONF1 =
    crate::Reg<comb_pd_site3_unit1_vt0_conf1::COMB_PD_SITE3_UNIT1_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit1_vt0_conf1;
#[doc = "COMB_PD_SITE3_UNIT2_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit2_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit2_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit2_vt0_conf1`] module"]
pub type COMB_PD_SITE3_UNIT2_VT0_CONF1 =
    crate::Reg<comb_pd_site3_unit2_vt0_conf1::COMB_PD_SITE3_UNIT2_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit2_vt0_conf1;
#[doc = "COMB_PD_SITE3_UNIT3_VT0_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit3_vt0_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit3_vt0_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit3_vt0_conf1`] module"]
pub type COMB_PD_SITE3_UNIT3_VT0_CONF1 =
    crate::Reg<comb_pd_site3_unit3_vt0_conf1::COMB_PD_SITE3_UNIT3_VT0_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit3_vt0_conf1;
#[doc = "COMB_PD_SITE3_UNIT0_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit0_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit0_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit0_vt1_conf1`] module"]
pub type COMB_PD_SITE3_UNIT0_VT1_CONF1 =
    crate::Reg<comb_pd_site3_unit0_vt1_conf1::COMB_PD_SITE3_UNIT0_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit0_vt1_conf1;
#[doc = "COMB_PD_SITE3_UNIT1_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit1_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit1_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit1_vt1_conf1`] module"]
pub type COMB_PD_SITE3_UNIT1_VT1_CONF1 =
    crate::Reg<comb_pd_site3_unit1_vt1_conf1::COMB_PD_SITE3_UNIT1_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit1_vt1_conf1;
#[doc = "COMB_PD_SITE3_UNIT2_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit2_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit2_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit2_vt1_conf1`] module"]
pub type COMB_PD_SITE3_UNIT2_VT1_CONF1 =
    crate::Reg<comb_pd_site3_unit2_vt1_conf1::COMB_PD_SITE3_UNIT2_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit2_vt1_conf1;
#[doc = "COMB_PD_SITE3_UNIT3_VT1_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit3_vt1_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit3_vt1_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit3_vt1_conf1`] module"]
pub type COMB_PD_SITE3_UNIT3_VT1_CONF1 =
    crate::Reg<comb_pd_site3_unit3_vt1_conf1::COMB_PD_SITE3_UNIT3_VT1_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit3_vt1_conf1;
#[doc = "COMB_PD_SITE3_UNIT0_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit0_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit0_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit0_vt2_conf1`] module"]
pub type COMB_PD_SITE3_UNIT0_VT2_CONF1 =
    crate::Reg<comb_pd_site3_unit0_vt2_conf1::COMB_PD_SITE3_UNIT0_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit0_vt2_conf1;
#[doc = "COMB_PD_SITE3_UNIT1_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit1_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit1_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit1_vt2_conf1`] module"]
pub type COMB_PD_SITE3_UNIT1_VT2_CONF1 =
    crate::Reg<comb_pd_site3_unit1_vt2_conf1::COMB_PD_SITE3_UNIT1_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit1_vt2_conf1;
#[doc = "COMB_PD_SITE3_UNIT2_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit2_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit2_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit2_vt2_conf1`] module"]
pub type COMB_PD_SITE3_UNIT2_VT2_CONF1 =
    crate::Reg<comb_pd_site3_unit2_vt2_conf1::COMB_PD_SITE3_UNIT2_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit2_vt2_conf1;
#[doc = "COMB_PD_SITE3_UNIT3_VT2_CONF1 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit3_vt2_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit3_vt2_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit3_vt2_conf1`] module"]
pub type COMB_PD_SITE3_UNIT3_VT2_CONF1 =
    crate::Reg<comb_pd_site3_unit3_vt2_conf1::COMB_PD_SITE3_UNIT3_VT2_CONF1_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit3_vt2_conf1;
#[doc = "COMB_PD_SITE0_UNIT0_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit0_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit0_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit0_vt0_conf2`] module"]
pub type COMB_PD_SITE0_UNIT0_VT0_CONF2 =
    crate::Reg<comb_pd_site0_unit0_vt0_conf2::COMB_PD_SITE0_UNIT0_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit0_vt0_conf2;
#[doc = "COMB_PD_SITE0_UNIT1_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit1_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit1_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit1_vt0_conf2`] module"]
pub type COMB_PD_SITE0_UNIT1_VT0_CONF2 =
    crate::Reg<comb_pd_site0_unit1_vt0_conf2::COMB_PD_SITE0_UNIT1_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit1_vt0_conf2;
#[doc = "COMB_PD_SITE0_UNIT2_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit2_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit2_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit2_vt0_conf2`] module"]
pub type COMB_PD_SITE0_UNIT2_VT0_CONF2 =
    crate::Reg<comb_pd_site0_unit2_vt0_conf2::COMB_PD_SITE0_UNIT2_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit2_vt0_conf2;
#[doc = "COMB_PD_SITE0_UNIT3_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit3_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit3_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit3_vt0_conf2`] module"]
pub type COMB_PD_SITE0_UNIT3_VT0_CONF2 =
    crate::Reg<comb_pd_site0_unit3_vt0_conf2::COMB_PD_SITE0_UNIT3_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit3_vt0_conf2;
#[doc = "COMB_PD_SITE0_UNIT0_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit0_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit0_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit0_vt1_conf2`] module"]
pub type COMB_PD_SITE0_UNIT0_VT1_CONF2 =
    crate::Reg<comb_pd_site0_unit0_vt1_conf2::COMB_PD_SITE0_UNIT0_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit0_vt1_conf2;
#[doc = "COMB_PD_SITE0_UNIT1_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit1_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit1_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit1_vt1_conf2`] module"]
pub type COMB_PD_SITE0_UNIT1_VT1_CONF2 =
    crate::Reg<comb_pd_site0_unit1_vt1_conf2::COMB_PD_SITE0_UNIT1_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit1_vt1_conf2;
#[doc = "COMB_PD_SITE0_UNIT2_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit2_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit2_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit2_vt1_conf2`] module"]
pub type COMB_PD_SITE0_UNIT2_VT1_CONF2 =
    crate::Reg<comb_pd_site0_unit2_vt1_conf2::COMB_PD_SITE0_UNIT2_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit2_vt1_conf2;
#[doc = "COMB_PD_SITE0_UNIT3_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit3_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit3_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit3_vt1_conf2`] module"]
pub type COMB_PD_SITE0_UNIT3_VT1_CONF2 =
    crate::Reg<comb_pd_site0_unit3_vt1_conf2::COMB_PD_SITE0_UNIT3_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit3_vt1_conf2;
#[doc = "COMB_PD_SITE0_UNIT0_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit0_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit0_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit0_vt2_conf2`] module"]
pub type COMB_PD_SITE0_UNIT0_VT2_CONF2 =
    crate::Reg<comb_pd_site0_unit0_vt2_conf2::COMB_PD_SITE0_UNIT0_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit0_vt2_conf2;
#[doc = "COMB_PD_SITE0_UNIT1_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit1_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit1_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit1_vt2_conf2`] module"]
pub type COMB_PD_SITE0_UNIT1_VT2_CONF2 =
    crate::Reg<comb_pd_site0_unit1_vt2_conf2::COMB_PD_SITE0_UNIT1_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit1_vt2_conf2;
#[doc = "COMB_PD_SITE0_UNIT2_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit2_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit2_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit2_vt2_conf2`] module"]
pub type COMB_PD_SITE0_UNIT2_VT2_CONF2 =
    crate::Reg<comb_pd_site0_unit2_vt2_conf2::COMB_PD_SITE0_UNIT2_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit2_vt2_conf2;
#[doc = "COMB_PD_SITE0_UNIT3_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site0_unit3_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site0_unit3_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site0_unit3_vt2_conf2`] module"]
pub type COMB_PD_SITE0_UNIT3_VT2_CONF2 =
    crate::Reg<comb_pd_site0_unit3_vt2_conf2::COMB_PD_SITE0_UNIT3_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site0_unit3_vt2_conf2;
#[doc = "COMB_PD_SITE1_UNIT0_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit0_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit0_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit0_vt0_conf2`] module"]
pub type COMB_PD_SITE1_UNIT0_VT0_CONF2 =
    crate::Reg<comb_pd_site1_unit0_vt0_conf2::COMB_PD_SITE1_UNIT0_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit0_vt0_conf2;
#[doc = "COMB_PD_SITE1_UNIT1_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit1_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit1_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit1_vt0_conf2`] module"]
pub type COMB_PD_SITE1_UNIT1_VT0_CONF2 =
    crate::Reg<comb_pd_site1_unit1_vt0_conf2::COMB_PD_SITE1_UNIT1_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit1_vt0_conf2;
#[doc = "COMB_PD_SITE1_UNIT2_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit2_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit2_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit2_vt0_conf2`] module"]
pub type COMB_PD_SITE1_UNIT2_VT0_CONF2 =
    crate::Reg<comb_pd_site1_unit2_vt0_conf2::COMB_PD_SITE1_UNIT2_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit2_vt0_conf2;
#[doc = "COMB_PD_SITE1_UNIT3_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit3_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit3_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit3_vt0_conf2`] module"]
pub type COMB_PD_SITE1_UNIT3_VT0_CONF2 =
    crate::Reg<comb_pd_site1_unit3_vt0_conf2::COMB_PD_SITE1_UNIT3_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit3_vt0_conf2;
#[doc = "COMB_PD_SITE1_UNIT0_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit0_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit0_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit0_vt1_conf2`] module"]
pub type COMB_PD_SITE1_UNIT0_VT1_CONF2 =
    crate::Reg<comb_pd_site1_unit0_vt1_conf2::COMB_PD_SITE1_UNIT0_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit0_vt1_conf2;
#[doc = "COMB_PD_SITE1_UNIT1_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit1_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit1_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit1_vt1_conf2`] module"]
pub type COMB_PD_SITE1_UNIT1_VT1_CONF2 =
    crate::Reg<comb_pd_site1_unit1_vt1_conf2::COMB_PD_SITE1_UNIT1_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit1_vt1_conf2;
#[doc = "COMB_PD_SITE1_UNIT2_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit2_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit2_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit2_vt1_conf2`] module"]
pub type COMB_PD_SITE1_UNIT2_VT1_CONF2 =
    crate::Reg<comb_pd_site1_unit2_vt1_conf2::COMB_PD_SITE1_UNIT2_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit2_vt1_conf2;
#[doc = "COMB_PD_SITE1_UNIT3_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit3_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit3_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit3_vt1_conf2`] module"]
pub type COMB_PD_SITE1_UNIT3_VT1_CONF2 =
    crate::Reg<comb_pd_site1_unit3_vt1_conf2::COMB_PD_SITE1_UNIT3_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit3_vt1_conf2;
#[doc = "COMB_PD_SITE1_UNIT0_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit0_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit0_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit0_vt2_conf2`] module"]
pub type COMB_PD_SITE1_UNIT0_VT2_CONF2 =
    crate::Reg<comb_pd_site1_unit0_vt2_conf2::COMB_PD_SITE1_UNIT0_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit0_vt2_conf2;
#[doc = "COMB_PD_SITE1_UNIT1_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit1_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit1_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit1_vt2_conf2`] module"]
pub type COMB_PD_SITE1_UNIT1_VT2_CONF2 =
    crate::Reg<comb_pd_site1_unit1_vt2_conf2::COMB_PD_SITE1_UNIT1_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit1_vt2_conf2;
#[doc = "COMB_PD_SITE1_UNIT2_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit2_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit2_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit2_vt2_conf2`] module"]
pub type COMB_PD_SITE1_UNIT2_VT2_CONF2 =
    crate::Reg<comb_pd_site1_unit2_vt2_conf2::COMB_PD_SITE1_UNIT2_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit2_vt2_conf2;
#[doc = "COMB_PD_SITE1_UNIT3_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site1_unit3_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site1_unit3_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site1_unit3_vt2_conf2`] module"]
pub type COMB_PD_SITE1_UNIT3_VT2_CONF2 =
    crate::Reg<comb_pd_site1_unit3_vt2_conf2::COMB_PD_SITE1_UNIT3_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site1_unit3_vt2_conf2;
#[doc = "COMB_PD_SITE2_UNIT0_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit0_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit0_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit0_vt0_conf2`] module"]
pub type COMB_PD_SITE2_UNIT0_VT0_CONF2 =
    crate::Reg<comb_pd_site2_unit0_vt0_conf2::COMB_PD_SITE2_UNIT0_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit0_vt0_conf2;
#[doc = "COMB_PD_SITE2_UNIT1_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit1_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit1_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit1_vt0_conf2`] module"]
pub type COMB_PD_SITE2_UNIT1_VT0_CONF2 =
    crate::Reg<comb_pd_site2_unit1_vt0_conf2::COMB_PD_SITE2_UNIT1_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit1_vt0_conf2;
#[doc = "COMB_PD_SITE2_UNIT2_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit2_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit2_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit2_vt0_conf2`] module"]
pub type COMB_PD_SITE2_UNIT2_VT0_CONF2 =
    crate::Reg<comb_pd_site2_unit2_vt0_conf2::COMB_PD_SITE2_UNIT2_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit2_vt0_conf2;
#[doc = "COMB_PD_SITE2_UNIT3_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit3_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit3_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit3_vt0_conf2`] module"]
pub type COMB_PD_SITE2_UNIT3_VT0_CONF2 =
    crate::Reg<comb_pd_site2_unit3_vt0_conf2::COMB_PD_SITE2_UNIT3_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit3_vt0_conf2;
#[doc = "COMB_PD_SITE2_UNIT0_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit0_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit0_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit0_vt1_conf2`] module"]
pub type COMB_PD_SITE2_UNIT0_VT1_CONF2 =
    crate::Reg<comb_pd_site2_unit0_vt1_conf2::COMB_PD_SITE2_UNIT0_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit0_vt1_conf2;
#[doc = "COMB_PD_SITE2_UNIT1_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit1_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit1_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit1_vt1_conf2`] module"]
pub type COMB_PD_SITE2_UNIT1_VT1_CONF2 =
    crate::Reg<comb_pd_site2_unit1_vt1_conf2::COMB_PD_SITE2_UNIT1_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit1_vt1_conf2;
#[doc = "COMB_PD_SITE2_UNIT2_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit2_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit2_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit2_vt1_conf2`] module"]
pub type COMB_PD_SITE2_UNIT2_VT1_CONF2 =
    crate::Reg<comb_pd_site2_unit2_vt1_conf2::COMB_PD_SITE2_UNIT2_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit2_vt1_conf2;
#[doc = "COMB_PD_SITE2_UNIT3_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit3_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit3_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit3_vt1_conf2`] module"]
pub type COMB_PD_SITE2_UNIT3_VT1_CONF2 =
    crate::Reg<comb_pd_site2_unit3_vt1_conf2::COMB_PD_SITE2_UNIT3_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit3_vt1_conf2;
#[doc = "COMB_PD_SITE2_UNIT0_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit0_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit0_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit0_vt2_conf2`] module"]
pub type COMB_PD_SITE2_UNIT0_VT2_CONF2 =
    crate::Reg<comb_pd_site2_unit0_vt2_conf2::COMB_PD_SITE2_UNIT0_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit0_vt2_conf2;
#[doc = "COMB_PD_SITE2_UNIT1_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit1_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit1_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit1_vt2_conf2`] module"]
pub type COMB_PD_SITE2_UNIT1_VT2_CONF2 =
    crate::Reg<comb_pd_site2_unit1_vt2_conf2::COMB_PD_SITE2_UNIT1_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit1_vt2_conf2;
#[doc = "COMB_PD_SITE2_UNIT2_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit2_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit2_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit2_vt2_conf2`] module"]
pub type COMB_PD_SITE2_UNIT2_VT2_CONF2 =
    crate::Reg<comb_pd_site2_unit2_vt2_conf2::COMB_PD_SITE2_UNIT2_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit2_vt2_conf2;
#[doc = "COMB_PD_SITE2_UNIT3_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site2_unit3_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site2_unit3_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site2_unit3_vt2_conf2`] module"]
pub type COMB_PD_SITE2_UNIT3_VT2_CONF2 =
    crate::Reg<comb_pd_site2_unit3_vt2_conf2::COMB_PD_SITE2_UNIT3_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site2_unit3_vt2_conf2;
#[doc = "COMB_PD_SITE3_UNIT0_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit0_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit0_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit0_vt0_conf2`] module"]
pub type COMB_PD_SITE3_UNIT0_VT0_CONF2 =
    crate::Reg<comb_pd_site3_unit0_vt0_conf2::COMB_PD_SITE3_UNIT0_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit0_vt0_conf2;
#[doc = "COMB_PD_SITE3_UNIT1_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit1_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit1_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit1_vt0_conf2`] module"]
pub type COMB_PD_SITE3_UNIT1_VT0_CONF2 =
    crate::Reg<comb_pd_site3_unit1_vt0_conf2::COMB_PD_SITE3_UNIT1_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit1_vt0_conf2;
#[doc = "COMB_PD_SITE3_UNIT2_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit2_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit2_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit2_vt0_conf2`] module"]
pub type COMB_PD_SITE3_UNIT2_VT0_CONF2 =
    crate::Reg<comb_pd_site3_unit2_vt0_conf2::COMB_PD_SITE3_UNIT2_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit2_vt0_conf2;
#[doc = "COMB_PD_SITE3_UNIT3_VT0_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit3_vt0_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit3_vt0_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit3_vt0_conf2`] module"]
pub type COMB_PD_SITE3_UNIT3_VT0_CONF2 =
    crate::Reg<comb_pd_site3_unit3_vt0_conf2::COMB_PD_SITE3_UNIT3_VT0_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit3_vt0_conf2;
#[doc = "COMB_PD_SITE3_UNIT0_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit0_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit0_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit0_vt1_conf2`] module"]
pub type COMB_PD_SITE3_UNIT0_VT1_CONF2 =
    crate::Reg<comb_pd_site3_unit0_vt1_conf2::COMB_PD_SITE3_UNIT0_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit0_vt1_conf2;
#[doc = "COMB_PD_SITE3_UNIT1_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit1_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit1_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit1_vt1_conf2`] module"]
pub type COMB_PD_SITE3_UNIT1_VT1_CONF2 =
    crate::Reg<comb_pd_site3_unit1_vt1_conf2::COMB_PD_SITE3_UNIT1_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit1_vt1_conf2;
#[doc = "COMB_PD_SITE3_UNIT2_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit2_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit2_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit2_vt1_conf2`] module"]
pub type COMB_PD_SITE3_UNIT2_VT1_CONF2 =
    crate::Reg<comb_pd_site3_unit2_vt1_conf2::COMB_PD_SITE3_UNIT2_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit2_vt1_conf2;
#[doc = "COMB_PD_SITE3_UNIT3_VT1_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit3_vt1_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit3_vt1_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit3_vt1_conf2`] module"]
pub type COMB_PD_SITE3_UNIT3_VT1_CONF2 =
    crate::Reg<comb_pd_site3_unit3_vt1_conf2::COMB_PD_SITE3_UNIT3_VT1_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit3_vt1_conf2;
#[doc = "COMB_PD_SITE3_UNIT0_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit0_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit0_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit0_vt2_conf2`] module"]
pub type COMB_PD_SITE3_UNIT0_VT2_CONF2 =
    crate::Reg<comb_pd_site3_unit0_vt2_conf2::COMB_PD_SITE3_UNIT0_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit0_vt2_conf2;
#[doc = "COMB_PD_SITE3_UNIT1_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit1_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit1_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit1_vt2_conf2`] module"]
pub type COMB_PD_SITE3_UNIT1_VT2_CONF2 =
    crate::Reg<comb_pd_site3_unit1_vt2_conf2::COMB_PD_SITE3_UNIT1_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit1_vt2_conf2;
#[doc = "COMB_PD_SITE3_UNIT2_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit2_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit2_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit2_vt2_conf2`] module"]
pub type COMB_PD_SITE3_UNIT2_VT2_CONF2 =
    crate::Reg<comb_pd_site3_unit2_vt2_conf2::COMB_PD_SITE3_UNIT2_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit2_vt2_conf2;
#[doc = "COMB_PD_SITE3_UNIT3_VT2_CONF2 (rw) register accessor: needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pd_site3_unit3_vt2_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comb_pd_site3_unit3_vt2_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comb_pd_site3_unit3_vt2_conf2`] module"]
pub type COMB_PD_SITE3_UNIT3_VT2_CONF2 =
    crate::Reg<comb_pd_site3_unit3_vt2_conf2::COMB_PD_SITE3_UNIT3_VT2_CONF2_SPEC>;
#[doc = "needs desc"]
pub mod comb_pd_site3_unit3_vt2_conf2;
#[doc = "VALUE_UPDATE (rw) register accessor: needs field desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`value_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value_update`] module"]
pub type VALUE_UPDATE = crate::Reg<value_update::VALUE_UPDATE_SPEC>;
#[doc = "needs field desc"]
pub mod value_update;
#[doc = "DATE (rw) register accessor: version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
