#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    lp_ana_bod_mode0_cntl: LP_ANA_BOD_MODE0_CNTL,
    lp_ana_bod_mode1_cntl: LP_ANA_BOD_MODE1_CNTL,
    lp_ana_vdd_source_cntl: LP_ANA_VDD_SOURCE_CNTL,
    lp_ana_vddbat_bod_cntl: LP_ANA_VDDBAT_BOD_CNTL,
    lp_ana_vddbat_charge_cntl: LP_ANA_VDDBAT_CHARGE_CNTL,
    lp_ana_ck_glitch_cntl: LP_ANA_CK_GLITCH_CNTL,
    lp_ana_pg_glitch_cntl: LP_ANA_PG_GLITCH_CNTL,
    lp_ana_fib_enable: LP_ANA_FIB_ENABLE,
    lp_ana_int_raw: LP_ANA_INT_RAW,
    lp_ana_int_st: LP_ANA_INT_ST,
    lp_ana_int_ena: LP_ANA_INT_ENA,
    lp_ana_int_clr: LP_ANA_INT_CLR,
    lp_ana_lp_int_raw: LP_ANA_LP_INT_RAW,
    lp_ana_lp_int_st: LP_ANA_LP_INT_ST,
    lp_ana_lp_int_ena: LP_ANA_LP_INT_ENA,
    lp_ana_lp_int_clr: LP_ANA_LP_INT_CLR,
    _reserved16: [u8; 0xbc],
    lp_ana_touch_approach_work_meas_num: LP_ANA_TOUCH_APPROACH_WORK_MEAS_NUM,
    lp_ana_touch_scan_ctrl1: LP_ANA_TOUCH_SCAN_CTRL1,
    lp_ana_touch_scan_ctrl2: LP_ANA_TOUCH_SCAN_CTRL2,
    lp_ana_touch_work: LP_ANA_TOUCH_WORK,
    lp_ana_touch_work_meas_num: LP_ANA_TOUCH_WORK_MEAS_NUM,
    lp_ana_touch_filter1: LP_ANA_TOUCH_FILTER1,
    lp_ana_touch_filter2: LP_ANA_TOUCH_FILTER2,
    lp_ana_touch_filter3: LP_ANA_TOUCH_FILTER3,
    lp_ana_touch_slp0: LP_ANA_TOUCH_SLP0,
    lp_ana_touch_slp1: LP_ANA_TOUCH_SLP1,
    lp_ana_touch_clr: LP_ANA_TOUCH_CLR,
    lp_ana_touch_approach: LP_ANA_TOUCH_APPROACH,
    lp_ana_touch_freq0_scan_para: LP_ANA_TOUCH_FREQ0_SCAN_PARA,
    lp_ana_touch_freq1_scan_para: LP_ANA_TOUCH_FREQ1_SCAN_PARA,
    lp_ana_touch_freq2_scan_para: LP_ANA_TOUCH_FREQ2_SCAN_PARA,
    lp_ana_touch_ana_para: LP_ANA_TOUCH_ANA_PARA,
    lp_ana_touch_mux0: LP_ANA_TOUCH_MUX0,
    lp_ana_touch_mux1: LP_ANA_TOUCH_MUX1,
    lp_ana_touch_pad0_th0: LP_ANA_TOUCH_PAD0_TH0,
    lp_ana_touch_pad0_th1: LP_ANA_TOUCH_PAD0_TH1,
    lp_ana_touch_pad0_th2: LP_ANA_TOUCH_PAD0_TH2,
    lp_ana_touch_pad1_th0: LP_ANA_TOUCH_PAD1_TH0,
    lp_ana_touch_pad1_th1: LP_ANA_TOUCH_PAD1_TH1,
    lp_ana_touch_pad1_th2: LP_ANA_TOUCH_PAD1_TH2,
    lp_ana_touch_pad2_th0: LP_ANA_TOUCH_PAD2_TH0,
    lp_ana_touch_pad2_th1: LP_ANA_TOUCH_PAD2_TH1,
    lp_ana_touch_pad2_th2: LP_ANA_TOUCH_PAD2_TH2,
    lp_ana_touch_pad3_th0: LP_ANA_TOUCH_PAD3_TH0,
    lp_ana_touch_pad3_th1: LP_ANA_TOUCH_PAD3_TH1,
    lp_ana_touch_pad3_th2: LP_ANA_TOUCH_PAD3_TH2,
    lp_ana_touch_pad4_th0: LP_ANA_TOUCH_PAD4_TH0,
    lp_ana_touch_pad4_th1: LP_ANA_TOUCH_PAD4_TH1,
    lp_ana_touch_pad4_th2: LP_ANA_TOUCH_PAD4_TH2,
    lp_ana_touch_pad5_th0: LP_ANA_TOUCH_PAD5_TH0,
    lp_ana_touch_pad5_th1: LP_ANA_TOUCH_PAD5_TH1,
    lp_ana_touch_pad5_th2: LP_ANA_TOUCH_PAD5_TH2,
    lp_ana_touch_pad6_th0: LP_ANA_TOUCH_PAD6_TH0,
    lp_ana_touch_pad6_th1: LP_ANA_TOUCH_PAD6_TH1,
    lp_ana_touch_pad6_th2: LP_ANA_TOUCH_PAD6_TH2,
    lp_ana_touch_pad7_th0: LP_ANA_TOUCH_PAD7_TH0,
    lp_ana_touch_pad7_th1: LP_ANA_TOUCH_PAD7_TH1,
    lp_ana_touch_pad7_th2: LP_ANA_TOUCH_PAD7_TH2,
    lp_ana_touch_pad8_th0: LP_ANA_TOUCH_PAD8_TH0,
    lp_ana_touch_pad8_th1: LP_ANA_TOUCH_PAD8_TH1,
    lp_ana_touch_pad8_th2: LP_ANA_TOUCH_PAD8_TH2,
    lp_ana_touch_pad9_th0: LP_ANA_TOUCH_PAD9_TH0,
    lp_ana_touch_pad9_th1: LP_ANA_TOUCH_PAD9_TH1,
    lp_ana_touch_pad9_th2: LP_ANA_TOUCH_PAD9_TH2,
    lp_ana_touch_pad10_th0: LP_ANA_TOUCH_PAD10_TH0,
    lp_ana_touch_pad10_th1: LP_ANA_TOUCH_PAD10_TH1,
    lp_ana_touch_pad10_th2: LP_ANA_TOUCH_PAD10_TH2,
    lp_ana_touch_pad11_th0: LP_ANA_TOUCH_PAD11_TH0,
    lp_ana_touch_pad11_th1: LP_ANA_TOUCH_PAD11_TH1,
    lp_ana_touch_pad11_th2: LP_ANA_TOUCH_PAD11_TH2,
    lp_ana_touch_pad12_th0: LP_ANA_TOUCH_PAD12_TH0,
    lp_ana_touch_pad12_th1: LP_ANA_TOUCH_PAD12_TH1,
    lp_ana_touch_pad12_th2: LP_ANA_TOUCH_PAD12_TH2,
    lp_ana_touch_pad13_th0: LP_ANA_TOUCH_PAD13_TH0,
    lp_ana_touch_pad13_th1: LP_ANA_TOUCH_PAD13_TH1,
    lp_ana_touch_pad13_th2: LP_ANA_TOUCH_PAD13_TH2,
    lp_ana_touch_pad14_th0: LP_ANA_TOUCH_PAD14_TH0,
    lp_ana_touch_pad14_th1: LP_ANA_TOUCH_PAD14_TH1,
    lp_ana_touch_pad14_th2: LP_ANA_TOUCH_PAD14_TH2,
    _reserved79: [u8; 0x0204],
    lp_ana_date: LP_ANA_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_bod_mode0_cntl(&self) -> &LP_ANA_BOD_MODE0_CNTL {
        &self.lp_ana_bod_mode0_cntl
    }
    #[doc = "0x04 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_bod_mode1_cntl(&self) -> &LP_ANA_BOD_MODE1_CNTL {
        &self.lp_ana_bod_mode1_cntl
    }
    #[doc = "0x08 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_vdd_source_cntl(&self) -> &LP_ANA_VDD_SOURCE_CNTL {
        &self.lp_ana_vdd_source_cntl
    }
    #[doc = "0x0c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_vddbat_bod_cntl(&self) -> &LP_ANA_VDDBAT_BOD_CNTL {
        &self.lp_ana_vddbat_bod_cntl
    }
    #[doc = "0x10 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_vddbat_charge_cntl(&self) -> &LP_ANA_VDDBAT_CHARGE_CNTL {
        &self.lp_ana_vddbat_charge_cntl
    }
    #[doc = "0x14 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_ck_glitch_cntl(&self) -> &LP_ANA_CK_GLITCH_CNTL {
        &self.lp_ana_ck_glitch_cntl
    }
    #[doc = "0x18 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_pg_glitch_cntl(&self) -> &LP_ANA_PG_GLITCH_CNTL {
        &self.lp_ana_pg_glitch_cntl
    }
    #[doc = "0x1c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_fib_enable(&self) -> &LP_ANA_FIB_ENABLE {
        &self.lp_ana_fib_enable
    }
    #[doc = "0x20 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_int_raw(&self) -> &LP_ANA_INT_RAW {
        &self.lp_ana_int_raw
    }
    #[doc = "0x24 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_int_st(&self) -> &LP_ANA_INT_ST {
        &self.lp_ana_int_st
    }
    #[doc = "0x28 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_int_ena(&self) -> &LP_ANA_INT_ENA {
        &self.lp_ana_int_ena
    }
    #[doc = "0x2c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_int_clr(&self) -> &LP_ANA_INT_CLR {
        &self.lp_ana_int_clr
    }
    #[doc = "0x30 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_lp_int_raw(&self) -> &LP_ANA_LP_INT_RAW {
        &self.lp_ana_lp_int_raw
    }
    #[doc = "0x34 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_lp_int_st(&self) -> &LP_ANA_LP_INT_ST {
        &self.lp_ana_lp_int_st
    }
    #[doc = "0x38 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_lp_int_ena(&self) -> &LP_ANA_LP_INT_ENA {
        &self.lp_ana_lp_int_ena
    }
    #[doc = "0x3c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_lp_int_clr(&self) -> &LP_ANA_LP_INT_CLR {
        &self.lp_ana_lp_int_clr
    }
    #[doc = "0xfc - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_approach_work_meas_num(
        &self,
    ) -> &LP_ANA_TOUCH_APPROACH_WORK_MEAS_NUM {
        &self.lp_ana_touch_approach_work_meas_num
    }
    #[doc = "0x100 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_scan_ctrl1(&self) -> &LP_ANA_TOUCH_SCAN_CTRL1 {
        &self.lp_ana_touch_scan_ctrl1
    }
    #[doc = "0x104 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_scan_ctrl2(&self) -> &LP_ANA_TOUCH_SCAN_CTRL2 {
        &self.lp_ana_touch_scan_ctrl2
    }
    #[doc = "0x108 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_work(&self) -> &LP_ANA_TOUCH_WORK {
        &self.lp_ana_touch_work
    }
    #[doc = "0x10c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_work_meas_num(&self) -> &LP_ANA_TOUCH_WORK_MEAS_NUM {
        &self.lp_ana_touch_work_meas_num
    }
    #[doc = "0x110 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_filter1(&self) -> &LP_ANA_TOUCH_FILTER1 {
        &self.lp_ana_touch_filter1
    }
    #[doc = "0x114 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_filter2(&self) -> &LP_ANA_TOUCH_FILTER2 {
        &self.lp_ana_touch_filter2
    }
    #[doc = "0x118 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_filter3(&self) -> &LP_ANA_TOUCH_FILTER3 {
        &self.lp_ana_touch_filter3
    }
    #[doc = "0x11c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_slp0(&self) -> &LP_ANA_TOUCH_SLP0 {
        &self.lp_ana_touch_slp0
    }
    #[doc = "0x120 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_slp1(&self) -> &LP_ANA_TOUCH_SLP1 {
        &self.lp_ana_touch_slp1
    }
    #[doc = "0x124 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_clr(&self) -> &LP_ANA_TOUCH_CLR {
        &self.lp_ana_touch_clr
    }
    #[doc = "0x128 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_approach(&self) -> &LP_ANA_TOUCH_APPROACH {
        &self.lp_ana_touch_approach
    }
    #[doc = "0x12c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_freq0_scan_para(&self) -> &LP_ANA_TOUCH_FREQ0_SCAN_PARA {
        &self.lp_ana_touch_freq0_scan_para
    }
    #[doc = "0x130 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_freq1_scan_para(&self) -> &LP_ANA_TOUCH_FREQ1_SCAN_PARA {
        &self.lp_ana_touch_freq1_scan_para
    }
    #[doc = "0x134 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_freq2_scan_para(&self) -> &LP_ANA_TOUCH_FREQ2_SCAN_PARA {
        &self.lp_ana_touch_freq2_scan_para
    }
    #[doc = "0x138 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_ana_para(&self) -> &LP_ANA_TOUCH_ANA_PARA {
        &self.lp_ana_touch_ana_para
    }
    #[doc = "0x13c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_mux0(&self) -> &LP_ANA_TOUCH_MUX0 {
        &self.lp_ana_touch_mux0
    }
    #[doc = "0x140 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_mux1(&self) -> &LP_ANA_TOUCH_MUX1 {
        &self.lp_ana_touch_mux1
    }
    #[doc = "0x144 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad0_th0(&self) -> &LP_ANA_TOUCH_PAD0_TH0 {
        &self.lp_ana_touch_pad0_th0
    }
    #[doc = "0x148 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad0_th1(&self) -> &LP_ANA_TOUCH_PAD0_TH1 {
        &self.lp_ana_touch_pad0_th1
    }
    #[doc = "0x14c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad0_th2(&self) -> &LP_ANA_TOUCH_PAD0_TH2 {
        &self.lp_ana_touch_pad0_th2
    }
    #[doc = "0x150 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad1_th0(&self) -> &LP_ANA_TOUCH_PAD1_TH0 {
        &self.lp_ana_touch_pad1_th0
    }
    #[doc = "0x154 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad1_th1(&self) -> &LP_ANA_TOUCH_PAD1_TH1 {
        &self.lp_ana_touch_pad1_th1
    }
    #[doc = "0x158 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad1_th2(&self) -> &LP_ANA_TOUCH_PAD1_TH2 {
        &self.lp_ana_touch_pad1_th2
    }
    #[doc = "0x15c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad2_th0(&self) -> &LP_ANA_TOUCH_PAD2_TH0 {
        &self.lp_ana_touch_pad2_th0
    }
    #[doc = "0x160 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad2_th1(&self) -> &LP_ANA_TOUCH_PAD2_TH1 {
        &self.lp_ana_touch_pad2_th1
    }
    #[doc = "0x164 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad2_th2(&self) -> &LP_ANA_TOUCH_PAD2_TH2 {
        &self.lp_ana_touch_pad2_th2
    }
    #[doc = "0x168 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad3_th0(&self) -> &LP_ANA_TOUCH_PAD3_TH0 {
        &self.lp_ana_touch_pad3_th0
    }
    #[doc = "0x16c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad3_th1(&self) -> &LP_ANA_TOUCH_PAD3_TH1 {
        &self.lp_ana_touch_pad3_th1
    }
    #[doc = "0x170 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad3_th2(&self) -> &LP_ANA_TOUCH_PAD3_TH2 {
        &self.lp_ana_touch_pad3_th2
    }
    #[doc = "0x174 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad4_th0(&self) -> &LP_ANA_TOUCH_PAD4_TH0 {
        &self.lp_ana_touch_pad4_th0
    }
    #[doc = "0x178 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad4_th1(&self) -> &LP_ANA_TOUCH_PAD4_TH1 {
        &self.lp_ana_touch_pad4_th1
    }
    #[doc = "0x17c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad4_th2(&self) -> &LP_ANA_TOUCH_PAD4_TH2 {
        &self.lp_ana_touch_pad4_th2
    }
    #[doc = "0x180 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad5_th0(&self) -> &LP_ANA_TOUCH_PAD5_TH0 {
        &self.lp_ana_touch_pad5_th0
    }
    #[doc = "0x184 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad5_th1(&self) -> &LP_ANA_TOUCH_PAD5_TH1 {
        &self.lp_ana_touch_pad5_th1
    }
    #[doc = "0x188 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad5_th2(&self) -> &LP_ANA_TOUCH_PAD5_TH2 {
        &self.lp_ana_touch_pad5_th2
    }
    #[doc = "0x18c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad6_th0(&self) -> &LP_ANA_TOUCH_PAD6_TH0 {
        &self.lp_ana_touch_pad6_th0
    }
    #[doc = "0x190 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad6_th1(&self) -> &LP_ANA_TOUCH_PAD6_TH1 {
        &self.lp_ana_touch_pad6_th1
    }
    #[doc = "0x194 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad6_th2(&self) -> &LP_ANA_TOUCH_PAD6_TH2 {
        &self.lp_ana_touch_pad6_th2
    }
    #[doc = "0x198 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad7_th0(&self) -> &LP_ANA_TOUCH_PAD7_TH0 {
        &self.lp_ana_touch_pad7_th0
    }
    #[doc = "0x19c - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad7_th1(&self) -> &LP_ANA_TOUCH_PAD7_TH1 {
        &self.lp_ana_touch_pad7_th1
    }
    #[doc = "0x1a0 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad7_th2(&self) -> &LP_ANA_TOUCH_PAD7_TH2 {
        &self.lp_ana_touch_pad7_th2
    }
    #[doc = "0x1a4 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad8_th0(&self) -> &LP_ANA_TOUCH_PAD8_TH0 {
        &self.lp_ana_touch_pad8_th0
    }
    #[doc = "0x1a8 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad8_th1(&self) -> &LP_ANA_TOUCH_PAD8_TH1 {
        &self.lp_ana_touch_pad8_th1
    }
    #[doc = "0x1ac - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad8_th2(&self) -> &LP_ANA_TOUCH_PAD8_TH2 {
        &self.lp_ana_touch_pad8_th2
    }
    #[doc = "0x1b0 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad9_th0(&self) -> &LP_ANA_TOUCH_PAD9_TH0 {
        &self.lp_ana_touch_pad9_th0
    }
    #[doc = "0x1b4 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad9_th1(&self) -> &LP_ANA_TOUCH_PAD9_TH1 {
        &self.lp_ana_touch_pad9_th1
    }
    #[doc = "0x1b8 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad9_th2(&self) -> &LP_ANA_TOUCH_PAD9_TH2 {
        &self.lp_ana_touch_pad9_th2
    }
    #[doc = "0x1bc - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad10_th0(&self) -> &LP_ANA_TOUCH_PAD10_TH0 {
        &self.lp_ana_touch_pad10_th0
    }
    #[doc = "0x1c0 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad10_th1(&self) -> &LP_ANA_TOUCH_PAD10_TH1 {
        &self.lp_ana_touch_pad10_th1
    }
    #[doc = "0x1c4 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad10_th2(&self) -> &LP_ANA_TOUCH_PAD10_TH2 {
        &self.lp_ana_touch_pad10_th2
    }
    #[doc = "0x1c8 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad11_th0(&self) -> &LP_ANA_TOUCH_PAD11_TH0 {
        &self.lp_ana_touch_pad11_th0
    }
    #[doc = "0x1cc - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad11_th1(&self) -> &LP_ANA_TOUCH_PAD11_TH1 {
        &self.lp_ana_touch_pad11_th1
    }
    #[doc = "0x1d0 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad11_th2(&self) -> &LP_ANA_TOUCH_PAD11_TH2 {
        &self.lp_ana_touch_pad11_th2
    }
    #[doc = "0x1d4 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad12_th0(&self) -> &LP_ANA_TOUCH_PAD12_TH0 {
        &self.lp_ana_touch_pad12_th0
    }
    #[doc = "0x1d8 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad12_th1(&self) -> &LP_ANA_TOUCH_PAD12_TH1 {
        &self.lp_ana_touch_pad12_th1
    }
    #[doc = "0x1dc - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad12_th2(&self) -> &LP_ANA_TOUCH_PAD12_TH2 {
        &self.lp_ana_touch_pad12_th2
    }
    #[doc = "0x1e0 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad13_th0(&self) -> &LP_ANA_TOUCH_PAD13_TH0 {
        &self.lp_ana_touch_pad13_th0
    }
    #[doc = "0x1e4 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad13_th1(&self) -> &LP_ANA_TOUCH_PAD13_TH1 {
        &self.lp_ana_touch_pad13_th1
    }
    #[doc = "0x1e8 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad13_th2(&self) -> &LP_ANA_TOUCH_PAD13_TH2 {
        &self.lp_ana_touch_pad13_th2
    }
    #[doc = "0x1ec - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad14_th0(&self) -> &LP_ANA_TOUCH_PAD14_TH0 {
        &self.lp_ana_touch_pad14_th0
    }
    #[doc = "0x1f0 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad14_th1(&self) -> &LP_ANA_TOUCH_PAD14_TH1 {
        &self.lp_ana_touch_pad14_th1
    }
    #[doc = "0x1f4 - need_des"]
    #[inline(always)]
    pub const fn lp_ana_touch_pad14_th2(&self) -> &LP_ANA_TOUCH_PAD14_TH2 {
        &self.lp_ana_touch_pad14_th2
    }
    #[doc = "0x3fc - need_des"]
    #[inline(always)]
    pub const fn lp_ana_date(&self) -> &LP_ANA_DATE {
        &self.lp_ana_date
    }
}
#[doc = "LP_ANA_BOD_MODE0_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_bod_mode0_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_bod_mode0_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_bod_mode0_cntl`] module"]
pub type LP_ANA_BOD_MODE0_CNTL = crate::Reg<lp_ana_bod_mode0_cntl::LP_ANA_BOD_MODE0_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_bod_mode0_cntl;
#[doc = "LP_ANA_BOD_MODE1_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_bod_mode1_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_bod_mode1_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_bod_mode1_cntl`] module"]
pub type LP_ANA_BOD_MODE1_CNTL = crate::Reg<lp_ana_bod_mode1_cntl::LP_ANA_BOD_MODE1_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_bod_mode1_cntl;
#[doc = "LP_ANA_VDD_SOURCE_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_vdd_source_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_vdd_source_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_vdd_source_cntl`] module"]
pub type LP_ANA_VDD_SOURCE_CNTL = crate::Reg<lp_ana_vdd_source_cntl::LP_ANA_VDD_SOURCE_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_vdd_source_cntl;
#[doc = "LP_ANA_VDDBAT_BOD_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_vddbat_bod_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_vddbat_bod_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_vddbat_bod_cntl`] module"]
pub type LP_ANA_VDDBAT_BOD_CNTL = crate::Reg<lp_ana_vddbat_bod_cntl::LP_ANA_VDDBAT_BOD_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_vddbat_bod_cntl;
#[doc = "LP_ANA_VDDBAT_CHARGE_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_vddbat_charge_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_vddbat_charge_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_vddbat_charge_cntl`] module"]
pub type LP_ANA_VDDBAT_CHARGE_CNTL =
    crate::Reg<lp_ana_vddbat_charge_cntl::LP_ANA_VDDBAT_CHARGE_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_vddbat_charge_cntl;
#[doc = "LP_ANA_CK_GLITCH_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_ck_glitch_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_ck_glitch_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_ck_glitch_cntl`] module"]
pub type LP_ANA_CK_GLITCH_CNTL = crate::Reg<lp_ana_ck_glitch_cntl::LP_ANA_CK_GLITCH_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_ck_glitch_cntl;
#[doc = "LP_ANA_PG_GLITCH_CNTL (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_pg_glitch_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_pg_glitch_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_pg_glitch_cntl`] module"]
pub type LP_ANA_PG_GLITCH_CNTL = crate::Reg<lp_ana_pg_glitch_cntl::LP_ANA_PG_GLITCH_CNTL_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_pg_glitch_cntl;
#[doc = "LP_ANA_FIB_ENABLE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_fib_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_fib_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_fib_enable`] module"]
pub type LP_ANA_FIB_ENABLE = crate::Reg<lp_ana_fib_enable::LP_ANA_FIB_ENABLE_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_fib_enable;
#[doc = "LP_ANA_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_int_raw`] module"]
pub type LP_ANA_INT_RAW = crate::Reg<lp_ana_int_raw::LP_ANA_INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_int_raw;
#[doc = "LP_ANA_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_int_st`] module"]
pub type LP_ANA_INT_ST = crate::Reg<lp_ana_int_st::LP_ANA_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_int_st;
#[doc = "LP_ANA_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_int_ena`] module"]
pub type LP_ANA_INT_ENA = crate::Reg<lp_ana_int_ena::LP_ANA_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_int_ena;
#[doc = "LP_ANA_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_int_clr`] module"]
pub type LP_ANA_INT_CLR = crate::Reg<lp_ana_int_clr::LP_ANA_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_int_clr;
#[doc = "LP_ANA_LP_INT_RAW (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_lp_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_lp_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_lp_int_raw`] module"]
pub type LP_ANA_LP_INT_RAW = crate::Reg<lp_ana_lp_int_raw::LP_ANA_LP_INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_lp_int_raw;
#[doc = "LP_ANA_LP_INT_ST (r) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_lp_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_lp_int_st`] module"]
pub type LP_ANA_LP_INT_ST = crate::Reg<lp_ana_lp_int_st::LP_ANA_LP_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_lp_int_st;
#[doc = "LP_ANA_LP_INT_ENA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_lp_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_lp_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_lp_int_ena`] module"]
pub type LP_ANA_LP_INT_ENA = crate::Reg<lp_ana_lp_int_ena::LP_ANA_LP_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_lp_int_ena;
#[doc = "LP_ANA_LP_INT_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_lp_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_lp_int_clr`] module"]
pub type LP_ANA_LP_INT_CLR = crate::Reg<lp_ana_lp_int_clr::LP_ANA_LP_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_lp_int_clr;
#[doc = "LP_ANA_TOUCH_APPROACH_WORK_MEAS_NUM (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_approach_work_meas_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_approach_work_meas_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_approach_work_meas_num`] module"]
pub type LP_ANA_TOUCH_APPROACH_WORK_MEAS_NUM =
    crate::Reg<lp_ana_touch_approach_work_meas_num::LP_ANA_TOUCH_APPROACH_WORK_MEAS_NUM_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_approach_work_meas_num;
#[doc = "LP_ANA_TOUCH_SCAN_CTRL1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_scan_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_scan_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_scan_ctrl1`] module"]
pub type LP_ANA_TOUCH_SCAN_CTRL1 =
    crate::Reg<lp_ana_touch_scan_ctrl1::LP_ANA_TOUCH_SCAN_CTRL1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_scan_ctrl1;
#[doc = "LP_ANA_TOUCH_SCAN_CTRL2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_scan_ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_scan_ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_scan_ctrl2`] module"]
pub type LP_ANA_TOUCH_SCAN_CTRL2 =
    crate::Reg<lp_ana_touch_scan_ctrl2::LP_ANA_TOUCH_SCAN_CTRL2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_scan_ctrl2;
#[doc = "LP_ANA_TOUCH_WORK (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_work::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_work::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_work`] module"]
pub type LP_ANA_TOUCH_WORK = crate::Reg<lp_ana_touch_work::LP_ANA_TOUCH_WORK_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_work;
#[doc = "LP_ANA_TOUCH_WORK_MEAS_NUM (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_work_meas_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_work_meas_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_work_meas_num`] module"]
pub type LP_ANA_TOUCH_WORK_MEAS_NUM =
    crate::Reg<lp_ana_touch_work_meas_num::LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_work_meas_num;
#[doc = "LP_ANA_TOUCH_FILTER1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_filter1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_filter1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_filter1`] module"]
pub type LP_ANA_TOUCH_FILTER1 = crate::Reg<lp_ana_touch_filter1::LP_ANA_TOUCH_FILTER1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_filter1;
#[doc = "LP_ANA_TOUCH_FILTER2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_filter2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_filter2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_filter2`] module"]
pub type LP_ANA_TOUCH_FILTER2 = crate::Reg<lp_ana_touch_filter2::LP_ANA_TOUCH_FILTER2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_filter2;
#[doc = "LP_ANA_TOUCH_FILTER3 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_filter3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_filter3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_filter3`] module"]
pub type LP_ANA_TOUCH_FILTER3 = crate::Reg<lp_ana_touch_filter3::LP_ANA_TOUCH_FILTER3_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_filter3;
#[doc = "LP_ANA_TOUCH_SLP0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_slp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_slp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_slp0`] module"]
pub type LP_ANA_TOUCH_SLP0 = crate::Reg<lp_ana_touch_slp0::LP_ANA_TOUCH_SLP0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_slp0;
#[doc = "LP_ANA_TOUCH_SLP1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_slp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_slp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_slp1`] module"]
pub type LP_ANA_TOUCH_SLP1 = crate::Reg<lp_ana_touch_slp1::LP_ANA_TOUCH_SLP1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_slp1;
#[doc = "LP_ANA_TOUCH_CLR (w) register accessor: need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_clr`] module"]
pub type LP_ANA_TOUCH_CLR = crate::Reg<lp_ana_touch_clr::LP_ANA_TOUCH_CLR_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_clr;
#[doc = "LP_ANA_TOUCH_APPROACH (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_approach::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_approach::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_approach`] module"]
pub type LP_ANA_TOUCH_APPROACH = crate::Reg<lp_ana_touch_approach::LP_ANA_TOUCH_APPROACH_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_approach;
#[doc = "LP_ANA_TOUCH_FREQ0_SCAN_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_freq0_scan_para::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_freq0_scan_para::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_freq0_scan_para`] module"]
pub type LP_ANA_TOUCH_FREQ0_SCAN_PARA =
    crate::Reg<lp_ana_touch_freq0_scan_para::LP_ANA_TOUCH_FREQ0_SCAN_PARA_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_freq0_scan_para;
#[doc = "LP_ANA_TOUCH_FREQ1_SCAN_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_freq1_scan_para::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_freq1_scan_para::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_freq1_scan_para`] module"]
pub type LP_ANA_TOUCH_FREQ1_SCAN_PARA =
    crate::Reg<lp_ana_touch_freq1_scan_para::LP_ANA_TOUCH_FREQ1_SCAN_PARA_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_freq1_scan_para;
#[doc = "LP_ANA_TOUCH_FREQ2_SCAN_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_freq2_scan_para::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_freq2_scan_para::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_freq2_scan_para`] module"]
pub type LP_ANA_TOUCH_FREQ2_SCAN_PARA =
    crate::Reg<lp_ana_touch_freq2_scan_para::LP_ANA_TOUCH_FREQ2_SCAN_PARA_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_freq2_scan_para;
#[doc = "LP_ANA_TOUCH_ANA_PARA (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_ana_para::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_ana_para::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_ana_para`] module"]
pub type LP_ANA_TOUCH_ANA_PARA = crate::Reg<lp_ana_touch_ana_para::LP_ANA_TOUCH_ANA_PARA_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_ana_para;
#[doc = "LP_ANA_TOUCH_MUX0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_mux0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_mux0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_mux0`] module"]
pub type LP_ANA_TOUCH_MUX0 = crate::Reg<lp_ana_touch_mux0::LP_ANA_TOUCH_MUX0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_mux0;
#[doc = "LP_ANA_TOUCH_MUX1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_mux1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_mux1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_mux1`] module"]
pub type LP_ANA_TOUCH_MUX1 = crate::Reg<lp_ana_touch_mux1::LP_ANA_TOUCH_MUX1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_mux1;
#[doc = "LP_ANA_TOUCH_PAD0_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad0_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad0_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad0_th0`] module"]
pub type LP_ANA_TOUCH_PAD0_TH0 = crate::Reg<lp_ana_touch_pad0_th0::LP_ANA_TOUCH_PAD0_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad0_th0;
#[doc = "LP_ANA_TOUCH_PAD0_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad0_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad0_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad0_th1`] module"]
pub type LP_ANA_TOUCH_PAD0_TH1 = crate::Reg<lp_ana_touch_pad0_th1::LP_ANA_TOUCH_PAD0_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad0_th1;
#[doc = "LP_ANA_TOUCH_PAD0_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad0_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad0_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad0_th2`] module"]
pub type LP_ANA_TOUCH_PAD0_TH2 = crate::Reg<lp_ana_touch_pad0_th2::LP_ANA_TOUCH_PAD0_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad0_th2;
#[doc = "LP_ANA_TOUCH_PAD1_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad1_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad1_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad1_th0`] module"]
pub type LP_ANA_TOUCH_PAD1_TH0 = crate::Reg<lp_ana_touch_pad1_th0::LP_ANA_TOUCH_PAD1_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad1_th0;
#[doc = "LP_ANA_TOUCH_PAD1_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad1_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad1_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad1_th1`] module"]
pub type LP_ANA_TOUCH_PAD1_TH1 = crate::Reg<lp_ana_touch_pad1_th1::LP_ANA_TOUCH_PAD1_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad1_th1;
#[doc = "LP_ANA_TOUCH_PAD1_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad1_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad1_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad1_th2`] module"]
pub type LP_ANA_TOUCH_PAD1_TH2 = crate::Reg<lp_ana_touch_pad1_th2::LP_ANA_TOUCH_PAD1_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad1_th2;
#[doc = "LP_ANA_TOUCH_PAD2_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad2_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad2_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad2_th0`] module"]
pub type LP_ANA_TOUCH_PAD2_TH0 = crate::Reg<lp_ana_touch_pad2_th0::LP_ANA_TOUCH_PAD2_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad2_th0;
#[doc = "LP_ANA_TOUCH_PAD2_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad2_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad2_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad2_th1`] module"]
pub type LP_ANA_TOUCH_PAD2_TH1 = crate::Reg<lp_ana_touch_pad2_th1::LP_ANA_TOUCH_PAD2_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad2_th1;
#[doc = "LP_ANA_TOUCH_PAD2_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad2_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad2_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad2_th2`] module"]
pub type LP_ANA_TOUCH_PAD2_TH2 = crate::Reg<lp_ana_touch_pad2_th2::LP_ANA_TOUCH_PAD2_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad2_th2;
#[doc = "LP_ANA_TOUCH_PAD3_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad3_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad3_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad3_th0`] module"]
pub type LP_ANA_TOUCH_PAD3_TH0 = crate::Reg<lp_ana_touch_pad3_th0::LP_ANA_TOUCH_PAD3_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad3_th0;
#[doc = "LP_ANA_TOUCH_PAD3_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad3_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad3_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad3_th1`] module"]
pub type LP_ANA_TOUCH_PAD3_TH1 = crate::Reg<lp_ana_touch_pad3_th1::LP_ANA_TOUCH_PAD3_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad3_th1;
#[doc = "LP_ANA_TOUCH_PAD3_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad3_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad3_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad3_th2`] module"]
pub type LP_ANA_TOUCH_PAD3_TH2 = crate::Reg<lp_ana_touch_pad3_th2::LP_ANA_TOUCH_PAD3_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad3_th2;
#[doc = "LP_ANA_TOUCH_PAD4_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad4_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad4_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad4_th0`] module"]
pub type LP_ANA_TOUCH_PAD4_TH0 = crate::Reg<lp_ana_touch_pad4_th0::LP_ANA_TOUCH_PAD4_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad4_th0;
#[doc = "LP_ANA_TOUCH_PAD4_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad4_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad4_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad4_th1`] module"]
pub type LP_ANA_TOUCH_PAD4_TH1 = crate::Reg<lp_ana_touch_pad4_th1::LP_ANA_TOUCH_PAD4_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad4_th1;
#[doc = "LP_ANA_TOUCH_PAD4_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad4_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad4_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad4_th2`] module"]
pub type LP_ANA_TOUCH_PAD4_TH2 = crate::Reg<lp_ana_touch_pad4_th2::LP_ANA_TOUCH_PAD4_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad4_th2;
#[doc = "LP_ANA_TOUCH_PAD5_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad5_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad5_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad5_th0`] module"]
pub type LP_ANA_TOUCH_PAD5_TH0 = crate::Reg<lp_ana_touch_pad5_th0::LP_ANA_TOUCH_PAD5_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad5_th0;
#[doc = "LP_ANA_TOUCH_PAD5_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad5_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad5_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad5_th1`] module"]
pub type LP_ANA_TOUCH_PAD5_TH1 = crate::Reg<lp_ana_touch_pad5_th1::LP_ANA_TOUCH_PAD5_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad5_th1;
#[doc = "LP_ANA_TOUCH_PAD5_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad5_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad5_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad5_th2`] module"]
pub type LP_ANA_TOUCH_PAD5_TH2 = crate::Reg<lp_ana_touch_pad5_th2::LP_ANA_TOUCH_PAD5_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad5_th2;
#[doc = "LP_ANA_TOUCH_PAD6_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad6_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad6_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad6_th0`] module"]
pub type LP_ANA_TOUCH_PAD6_TH0 = crate::Reg<lp_ana_touch_pad6_th0::LP_ANA_TOUCH_PAD6_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad6_th0;
#[doc = "LP_ANA_TOUCH_PAD6_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad6_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad6_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad6_th1`] module"]
pub type LP_ANA_TOUCH_PAD6_TH1 = crate::Reg<lp_ana_touch_pad6_th1::LP_ANA_TOUCH_PAD6_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad6_th1;
#[doc = "LP_ANA_TOUCH_PAD6_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad6_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad6_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad6_th2`] module"]
pub type LP_ANA_TOUCH_PAD6_TH2 = crate::Reg<lp_ana_touch_pad6_th2::LP_ANA_TOUCH_PAD6_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad6_th2;
#[doc = "LP_ANA_TOUCH_PAD7_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad7_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad7_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad7_th0`] module"]
pub type LP_ANA_TOUCH_PAD7_TH0 = crate::Reg<lp_ana_touch_pad7_th0::LP_ANA_TOUCH_PAD7_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad7_th0;
#[doc = "LP_ANA_TOUCH_PAD7_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad7_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad7_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad7_th1`] module"]
pub type LP_ANA_TOUCH_PAD7_TH1 = crate::Reg<lp_ana_touch_pad7_th1::LP_ANA_TOUCH_PAD7_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad7_th1;
#[doc = "LP_ANA_TOUCH_PAD7_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad7_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad7_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad7_th2`] module"]
pub type LP_ANA_TOUCH_PAD7_TH2 = crate::Reg<lp_ana_touch_pad7_th2::LP_ANA_TOUCH_PAD7_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad7_th2;
#[doc = "LP_ANA_TOUCH_PAD8_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad8_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad8_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad8_th0`] module"]
pub type LP_ANA_TOUCH_PAD8_TH0 = crate::Reg<lp_ana_touch_pad8_th0::LP_ANA_TOUCH_PAD8_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad8_th0;
#[doc = "LP_ANA_TOUCH_PAD8_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad8_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad8_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad8_th1`] module"]
pub type LP_ANA_TOUCH_PAD8_TH1 = crate::Reg<lp_ana_touch_pad8_th1::LP_ANA_TOUCH_PAD8_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad8_th1;
#[doc = "LP_ANA_TOUCH_PAD8_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad8_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad8_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad8_th2`] module"]
pub type LP_ANA_TOUCH_PAD8_TH2 = crate::Reg<lp_ana_touch_pad8_th2::LP_ANA_TOUCH_PAD8_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad8_th2;
#[doc = "LP_ANA_TOUCH_PAD9_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad9_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad9_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad9_th0`] module"]
pub type LP_ANA_TOUCH_PAD9_TH0 = crate::Reg<lp_ana_touch_pad9_th0::LP_ANA_TOUCH_PAD9_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad9_th0;
#[doc = "LP_ANA_TOUCH_PAD9_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad9_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad9_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad9_th1`] module"]
pub type LP_ANA_TOUCH_PAD9_TH1 = crate::Reg<lp_ana_touch_pad9_th1::LP_ANA_TOUCH_PAD9_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad9_th1;
#[doc = "LP_ANA_TOUCH_PAD9_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad9_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad9_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad9_th2`] module"]
pub type LP_ANA_TOUCH_PAD9_TH2 = crate::Reg<lp_ana_touch_pad9_th2::LP_ANA_TOUCH_PAD9_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad9_th2;
#[doc = "LP_ANA_TOUCH_PAD10_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad10_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad10_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad10_th0`] module"]
pub type LP_ANA_TOUCH_PAD10_TH0 = crate::Reg<lp_ana_touch_pad10_th0::LP_ANA_TOUCH_PAD10_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad10_th0;
#[doc = "LP_ANA_TOUCH_PAD10_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad10_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad10_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad10_th1`] module"]
pub type LP_ANA_TOUCH_PAD10_TH1 = crate::Reg<lp_ana_touch_pad10_th1::LP_ANA_TOUCH_PAD10_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad10_th1;
#[doc = "LP_ANA_TOUCH_PAD10_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad10_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad10_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad10_th2`] module"]
pub type LP_ANA_TOUCH_PAD10_TH2 = crate::Reg<lp_ana_touch_pad10_th2::LP_ANA_TOUCH_PAD10_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad10_th2;
#[doc = "LP_ANA_TOUCH_PAD11_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad11_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad11_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad11_th0`] module"]
pub type LP_ANA_TOUCH_PAD11_TH0 = crate::Reg<lp_ana_touch_pad11_th0::LP_ANA_TOUCH_PAD11_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad11_th0;
#[doc = "LP_ANA_TOUCH_PAD11_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad11_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad11_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad11_th1`] module"]
pub type LP_ANA_TOUCH_PAD11_TH1 = crate::Reg<lp_ana_touch_pad11_th1::LP_ANA_TOUCH_PAD11_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad11_th1;
#[doc = "LP_ANA_TOUCH_PAD11_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad11_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad11_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad11_th2`] module"]
pub type LP_ANA_TOUCH_PAD11_TH2 = crate::Reg<lp_ana_touch_pad11_th2::LP_ANA_TOUCH_PAD11_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad11_th2;
#[doc = "LP_ANA_TOUCH_PAD12_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad12_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad12_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad12_th0`] module"]
pub type LP_ANA_TOUCH_PAD12_TH0 = crate::Reg<lp_ana_touch_pad12_th0::LP_ANA_TOUCH_PAD12_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad12_th0;
#[doc = "LP_ANA_TOUCH_PAD12_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad12_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad12_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad12_th1`] module"]
pub type LP_ANA_TOUCH_PAD12_TH1 = crate::Reg<lp_ana_touch_pad12_th1::LP_ANA_TOUCH_PAD12_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad12_th1;
#[doc = "LP_ANA_TOUCH_PAD12_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad12_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad12_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad12_th2`] module"]
pub type LP_ANA_TOUCH_PAD12_TH2 = crate::Reg<lp_ana_touch_pad12_th2::LP_ANA_TOUCH_PAD12_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad12_th2;
#[doc = "LP_ANA_TOUCH_PAD13_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad13_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad13_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad13_th0`] module"]
pub type LP_ANA_TOUCH_PAD13_TH0 = crate::Reg<lp_ana_touch_pad13_th0::LP_ANA_TOUCH_PAD13_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad13_th0;
#[doc = "LP_ANA_TOUCH_PAD13_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad13_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad13_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad13_th1`] module"]
pub type LP_ANA_TOUCH_PAD13_TH1 = crate::Reg<lp_ana_touch_pad13_th1::LP_ANA_TOUCH_PAD13_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad13_th1;
#[doc = "LP_ANA_TOUCH_PAD13_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad13_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad13_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad13_th2`] module"]
pub type LP_ANA_TOUCH_PAD13_TH2 = crate::Reg<lp_ana_touch_pad13_th2::LP_ANA_TOUCH_PAD13_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad13_th2;
#[doc = "LP_ANA_TOUCH_PAD14_TH0 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad14_th0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad14_th0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad14_th0`] module"]
pub type LP_ANA_TOUCH_PAD14_TH0 = crate::Reg<lp_ana_touch_pad14_th0::LP_ANA_TOUCH_PAD14_TH0_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad14_th0;
#[doc = "LP_ANA_TOUCH_PAD14_TH1 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad14_th1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad14_th1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad14_th1`] module"]
pub type LP_ANA_TOUCH_PAD14_TH1 = crate::Reg<lp_ana_touch_pad14_th1::LP_ANA_TOUCH_PAD14_TH1_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad14_th1;
#[doc = "LP_ANA_TOUCH_PAD14_TH2 (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_pad14_th2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_pad14_th2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_touch_pad14_th2`] module"]
pub type LP_ANA_TOUCH_PAD14_TH2 = crate::Reg<lp_ana_touch_pad14_th2::LP_ANA_TOUCH_PAD14_TH2_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_touch_pad14_th2;
#[doc = "LP_ANA_DATE (rw) register accessor: need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_ana_date`] module"]
pub type LP_ANA_DATE = crate::Reg<lp_ana_date::LP_ANA_DATE_SPEC>;
#[doc = "need_des"]
pub mod lp_ana_date;
