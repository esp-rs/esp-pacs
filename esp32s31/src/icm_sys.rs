#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    date: DATE,
    clk_en: CLK_EN,
    interrupt_raw: INTERRUPT_RAW,
    interrupt_clr: INTERRUPT_CLR,
    interrupt_ena: INTERRUPT_ENA,
    interrupt_st: INTERRUPT_ST,
    timeout_max_cnt: TIMEOUT_MAX_CNT,
    timeout_info: TIMEOUT_INFO,
    dec_failure_info: DEC_FAILURE_INFO,
    qos_debug_mst_sel: QOS_DEBUG_MST_SEL,
    mst_arqos_reg0: MST_ARQOS_REG0,
    _reserved11: [u8; 0x04],
    mst_awqos_reg0: MST_AWQOS_REG0,
    _reserved12: [u8; 0x4c],
    dec_failure_addr: DEC_FAILURE_ADDR,
    reg_slave_is_secure: REG_SLAVE_IS_SECURE,
    slave_security_override: SLAVE_SECURITY_OVERRIDE,
    master_is_secure: MASTER_IS_SECURE,
    master_security_override: MASTER_SECURITY_OVERRIDE,
    aw_qos_value: [AW_QOS_VALUE; 6],
    ar_qos_value: [AR_QOS_VALUE; 6],
    b_prior_value: [B_PRIOR_VALUE; 3],
    r_prior_value: [R_PRIOR_VALUE; 3],
    aw_qos_sel: [AW_QOS_SEL; 6],
    ar_qos_sel: [AR_QOS_SEL; 6],
    aw_qos_recovery: AW_QOS_RECOVERY,
    ar_qos_recovery: AR_QOS_RECOVERY,
    aw_qos_id_mask: [AW_QOS_ID_MASK; 6],
    ar_qos_id_mask: [AR_QOS_ID_MASK; 6],
    aw_qos_id_filter: [AW_QOS_ID_FILTER; 6],
    ar_qos_id_filter: [AR_QOS_ID_FILTER; 6],
    aw_qos_ctrl_cfg: [AW_QOS_CTRL_CFG; 6],
    ar_qos_ctrl_cfg: [AR_QOS_CTRL_CFG; 6],
    aw_qos_trans_rate: [AW_QOS_TRANS_RATE; 6],
    ar_qos_trans_rate: [AR_QOS_TRANS_RATE; 6],
    aw_qos_peak_rate: [AW_QOS_PEAK_RATE; 6],
    ar_qos_peak_rate: [AR_QOS_PEAK_RATE; 6],
    aw_qos_burstiness: [AW_QOS_BURSTINESS; 6],
    ar_qos_burstiness: [AR_QOS_BURSTINESS; 6],
    aw_qos_cfg_vlu: [AW_QOS_CFG_VLU; 6],
    ar_qos_cfg_vlu: [AR_QOS_CFG_VLU; 6],
    aw_qos_dly_thr0: [AW_QOS_DLY_THR0; 6],
    aw_qos_dly_thr1: [AW_QOS_DLY_THR1; 6],
    aw_qos_dly_thr2: [AW_QOS_DLY_THR2; 6],
    aw_qos_dly_thr3: [AW_QOS_DLY_THR3; 6],
    ar_qos_dly_thr0: [AR_QOS_DLY_THR0; 6],
    ar_qos_dly_thr1: [AR_QOS_DLY_THR1; 6],
    ar_qos_dly_thr2: [AR_QOS_DLY_THR2; 6],
    ar_qos_dly_thr3: [AR_QOS_DLY_THR3; 6],
    aw_qos_bur_len_thr: [AW_QOS_BUR_LEN_THR; 6],
    ar_qos_bur_len_thr: [AR_QOS_BUR_LEN_THR; 6],
    aw_qos_dbg_tk_thr: [AW_QOS_DBG_TK_THR; 6],
    ar_qos_dbg_tk_thr: [AR_QOS_DBG_TK_THR; 6],
    max_ost_aw_s: [MAX_OST_AW_S; 3],
    max_ost_ar_s: [MAX_OST_AR_S; 3],
    aw_qos_peak_rate_mode: AW_QOS_PEAK_RATE_MODE,
    ar_qos_peak_rate_mode: AR_QOS_PEAK_RATE_MODE,
}
impl RegisterBlock {
    #[doc = "0x400 - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x404 - "]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x408 - "]
    #[inline(always)]
    pub const fn interrupt_raw(&self) -> &INTERRUPT_RAW {
        &self.interrupt_raw
    }
    #[doc = "0x40c - "]
    #[inline(always)]
    pub const fn interrupt_clr(&self) -> &INTERRUPT_CLR {
        &self.interrupt_clr
    }
    #[doc = "0x410 - "]
    #[inline(always)]
    pub const fn interrupt_ena(&self) -> &INTERRUPT_ENA {
        &self.interrupt_ena
    }
    #[doc = "0x414 - "]
    #[inline(always)]
    pub const fn interrupt_st(&self) -> &INTERRUPT_ST {
        &self.interrupt_st
    }
    #[doc = "0x418 - "]
    #[inline(always)]
    pub const fn timeout_max_cnt(&self) -> &TIMEOUT_MAX_CNT {
        &self.timeout_max_cnt
    }
    #[doc = "0x41c - "]
    #[inline(always)]
    pub const fn timeout_info(&self) -> &TIMEOUT_INFO {
        &self.timeout_info
    }
    #[doc = "0x420 - "]
    #[inline(always)]
    pub const fn dec_failure_info(&self) -> &DEC_FAILURE_INFO {
        &self.dec_failure_info
    }
    #[doc = "0x424 - "]
    #[inline(always)]
    pub const fn qos_debug_mst_sel(&self) -> &QOS_DEBUG_MST_SEL {
        &self.qos_debug_mst_sel
    }
    #[doc = "0x428 - "]
    #[inline(always)]
    pub const fn mst_arqos_reg0(&self) -> &MST_ARQOS_REG0 {
        &self.mst_arqos_reg0
    }
    #[doc = "0x430 - "]
    #[inline(always)]
    pub const fn mst_awqos_reg0(&self) -> &MST_AWQOS_REG0 {
        &self.mst_awqos_reg0
    }
    #[doc = "0x480 - "]
    #[inline(always)]
    pub const fn dec_failure_addr(&self) -> &DEC_FAILURE_ADDR {
        &self.dec_failure_addr
    }
    #[doc = "0x484 - "]
    #[inline(always)]
    pub const fn reg_slave_is_secure(&self) -> &REG_SLAVE_IS_SECURE {
        &self.reg_slave_is_secure
    }
    #[doc = "0x488 - "]
    #[inline(always)]
    pub const fn slave_security_override(&self) -> &SLAVE_SECURITY_OVERRIDE {
        &self.slave_security_override
    }
    #[doc = "0x48c - "]
    #[inline(always)]
    pub const fn master_is_secure(&self) -> &MASTER_IS_SECURE {
        &self.master_is_secure
    }
    #[doc = "0x490 - "]
    #[inline(always)]
    pub const fn master_security_override(&self) -> &MASTER_SECURITY_OVERRIDE {
        &self.master_security_override
    }
    #[doc = "0x494..0x4ac - "]
    #[inline(always)]
    pub const fn aw_qos_value(&self, n: usize) -> &AW_QOS_VALUE {
        &self.aw_qos_value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x494..0x4ac - "]
    #[inline(always)]
    pub fn aw_qos_value_iter(&self) -> impl Iterator<Item = &AW_QOS_VALUE> {
        self.aw_qos_value.iter()
    }
    #[doc = "0x4ac..0x4c4 - "]
    #[inline(always)]
    pub const fn ar_qos_value(&self, n: usize) -> &AR_QOS_VALUE {
        &self.ar_qos_value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4ac..0x4c4 - "]
    #[inline(always)]
    pub fn ar_qos_value_iter(&self) -> impl Iterator<Item = &AR_QOS_VALUE> {
        self.ar_qos_value.iter()
    }
    #[doc = "0x4c4..0x4d0 - "]
    #[inline(always)]
    pub const fn b_prior_value(&self, n: usize) -> &B_PRIOR_VALUE {
        &self.b_prior_value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4c4..0x4d0 - "]
    #[inline(always)]
    pub fn b_prior_value_iter(&self) -> impl Iterator<Item = &B_PRIOR_VALUE> {
        self.b_prior_value.iter()
    }
    #[doc = "0x4d0..0x4dc - "]
    #[inline(always)]
    pub const fn r_prior_value(&self, n: usize) -> &R_PRIOR_VALUE {
        &self.r_prior_value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4d0..0x4dc - "]
    #[inline(always)]
    pub fn r_prior_value_iter(&self) -> impl Iterator<Item = &R_PRIOR_VALUE> {
        self.r_prior_value.iter()
    }
    #[doc = "0x4dc..0x4f4 - "]
    #[inline(always)]
    pub const fn aw_qos_sel(&self, n: usize) -> &AW_QOS_SEL {
        &self.aw_qos_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4dc..0x4f4 - "]
    #[inline(always)]
    pub fn aw_qos_sel_iter(&self) -> impl Iterator<Item = &AW_QOS_SEL> {
        self.aw_qos_sel.iter()
    }
    #[doc = "0x4f4..0x50c - "]
    #[inline(always)]
    pub const fn ar_qos_sel(&self, n: usize) -> &AR_QOS_SEL {
        &self.ar_qos_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4f4..0x50c - "]
    #[inline(always)]
    pub fn ar_qos_sel_iter(&self) -> impl Iterator<Item = &AR_QOS_SEL> {
        self.ar_qos_sel.iter()
    }
    #[doc = "0x50c - "]
    #[inline(always)]
    pub const fn aw_qos_recovery(&self) -> &AW_QOS_RECOVERY {
        &self.aw_qos_recovery
    }
    #[doc = "0x510 - "]
    #[inline(always)]
    pub const fn ar_qos_recovery(&self) -> &AR_QOS_RECOVERY {
        &self.ar_qos_recovery
    }
    #[doc = "0x514..0x52c - "]
    #[inline(always)]
    pub const fn aw_qos_id_mask(&self, n: usize) -> &AW_QOS_ID_MASK {
        &self.aw_qos_id_mask[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x514..0x52c - "]
    #[inline(always)]
    pub fn aw_qos_id_mask_iter(&self) -> impl Iterator<Item = &AW_QOS_ID_MASK> {
        self.aw_qos_id_mask.iter()
    }
    #[doc = "0x52c..0x544 - "]
    #[inline(always)]
    pub const fn ar_qos_id_mask(&self, n: usize) -> &AR_QOS_ID_MASK {
        &self.ar_qos_id_mask[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x52c..0x544 - "]
    #[inline(always)]
    pub fn ar_qos_id_mask_iter(&self) -> impl Iterator<Item = &AR_QOS_ID_MASK> {
        self.ar_qos_id_mask.iter()
    }
    #[doc = "0x544..0x55c - "]
    #[inline(always)]
    pub const fn aw_qos_id_filter(&self, n: usize) -> &AW_QOS_ID_FILTER {
        &self.aw_qos_id_filter[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x544..0x55c - "]
    #[inline(always)]
    pub fn aw_qos_id_filter_iter(&self) -> impl Iterator<Item = &AW_QOS_ID_FILTER> {
        self.aw_qos_id_filter.iter()
    }
    #[doc = "0x55c..0x574 - "]
    #[inline(always)]
    pub const fn ar_qos_id_filter(&self, n: usize) -> &AR_QOS_ID_FILTER {
        &self.ar_qos_id_filter[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x55c..0x574 - "]
    #[inline(always)]
    pub fn ar_qos_id_filter_iter(&self) -> impl Iterator<Item = &AR_QOS_ID_FILTER> {
        self.ar_qos_id_filter.iter()
    }
    #[doc = "0x574..0x58c - "]
    #[inline(always)]
    pub const fn aw_qos_ctrl_cfg(&self, n: usize) -> &AW_QOS_CTRL_CFG {
        &self.aw_qos_ctrl_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x574..0x58c - "]
    #[inline(always)]
    pub fn aw_qos_ctrl_cfg_iter(&self) -> impl Iterator<Item = &AW_QOS_CTRL_CFG> {
        self.aw_qos_ctrl_cfg.iter()
    }
    #[doc = "0x58c..0x5a4 - "]
    #[inline(always)]
    pub const fn ar_qos_ctrl_cfg(&self, n: usize) -> &AR_QOS_CTRL_CFG {
        &self.ar_qos_ctrl_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58c..0x5a4 - "]
    #[inline(always)]
    pub fn ar_qos_ctrl_cfg_iter(&self) -> impl Iterator<Item = &AR_QOS_CTRL_CFG> {
        self.ar_qos_ctrl_cfg.iter()
    }
    #[doc = "0x5a4..0x5bc - "]
    #[inline(always)]
    pub const fn aw_qos_trans_rate(&self, n: usize) -> &AW_QOS_TRANS_RATE {
        &self.aw_qos_trans_rate[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5a4..0x5bc - "]
    #[inline(always)]
    pub fn aw_qos_trans_rate_iter(&self) -> impl Iterator<Item = &AW_QOS_TRANS_RATE> {
        self.aw_qos_trans_rate.iter()
    }
    #[doc = "0x5bc..0x5d4 - "]
    #[inline(always)]
    pub const fn ar_qos_trans_rate(&self, n: usize) -> &AR_QOS_TRANS_RATE {
        &self.ar_qos_trans_rate[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5bc..0x5d4 - "]
    #[inline(always)]
    pub fn ar_qos_trans_rate_iter(&self) -> impl Iterator<Item = &AR_QOS_TRANS_RATE> {
        self.ar_qos_trans_rate.iter()
    }
    #[doc = "0x5d4..0x5ec - "]
    #[inline(always)]
    pub const fn aw_qos_peak_rate(&self, n: usize) -> &AW_QOS_PEAK_RATE {
        &self.aw_qos_peak_rate[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5d4..0x5ec - "]
    #[inline(always)]
    pub fn aw_qos_peak_rate_iter(&self) -> impl Iterator<Item = &AW_QOS_PEAK_RATE> {
        self.aw_qos_peak_rate.iter()
    }
    #[doc = "0x5ec..0x604 - "]
    #[inline(always)]
    pub const fn ar_qos_peak_rate(&self, n: usize) -> &AR_QOS_PEAK_RATE {
        &self.ar_qos_peak_rate[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5ec..0x604 - "]
    #[inline(always)]
    pub fn ar_qos_peak_rate_iter(&self) -> impl Iterator<Item = &AR_QOS_PEAK_RATE> {
        self.ar_qos_peak_rate.iter()
    }
    #[doc = "0x604..0x61c - "]
    #[inline(always)]
    pub const fn aw_qos_burstiness(&self, n: usize) -> &AW_QOS_BURSTINESS {
        &self.aw_qos_burstiness[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x604..0x61c - "]
    #[inline(always)]
    pub fn aw_qos_burstiness_iter(&self) -> impl Iterator<Item = &AW_QOS_BURSTINESS> {
        self.aw_qos_burstiness.iter()
    }
    #[doc = "0x61c..0x634 - "]
    #[inline(always)]
    pub const fn ar_qos_burstiness(&self, n: usize) -> &AR_QOS_BURSTINESS {
        &self.ar_qos_burstiness[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x61c..0x634 - "]
    #[inline(always)]
    pub fn ar_qos_burstiness_iter(&self) -> impl Iterator<Item = &AR_QOS_BURSTINESS> {
        self.ar_qos_burstiness.iter()
    }
    #[doc = "0x634..0x64c - "]
    #[inline(always)]
    pub const fn aw_qos_cfg_vlu(&self, n: usize) -> &AW_QOS_CFG_VLU {
        &self.aw_qos_cfg_vlu[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x634..0x64c - "]
    #[inline(always)]
    pub fn aw_qos_cfg_vlu_iter(&self) -> impl Iterator<Item = &AW_QOS_CFG_VLU> {
        self.aw_qos_cfg_vlu.iter()
    }
    #[doc = "0x64c..0x664 - "]
    #[inline(always)]
    pub const fn ar_qos_cfg_vlu(&self, n: usize) -> &AR_QOS_CFG_VLU {
        &self.ar_qos_cfg_vlu[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x64c..0x664 - "]
    #[inline(always)]
    pub fn ar_qos_cfg_vlu_iter(&self) -> impl Iterator<Item = &AR_QOS_CFG_VLU> {
        self.ar_qos_cfg_vlu.iter()
    }
    #[doc = "0x664..0x67c - "]
    #[inline(always)]
    pub const fn aw_qos_dly_thr0(&self, n: usize) -> &AW_QOS_DLY_THR0 {
        &self.aw_qos_dly_thr0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x664..0x67c - "]
    #[inline(always)]
    pub fn aw_qos_dly_thr0_iter(&self) -> impl Iterator<Item = &AW_QOS_DLY_THR0> {
        self.aw_qos_dly_thr0.iter()
    }
    #[doc = "0x67c..0x694 - "]
    #[inline(always)]
    pub const fn aw_qos_dly_thr1(&self, n: usize) -> &AW_QOS_DLY_THR1 {
        &self.aw_qos_dly_thr1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x67c..0x694 - "]
    #[inline(always)]
    pub fn aw_qos_dly_thr1_iter(&self) -> impl Iterator<Item = &AW_QOS_DLY_THR1> {
        self.aw_qos_dly_thr1.iter()
    }
    #[doc = "0x694..0x6ac - "]
    #[inline(always)]
    pub const fn aw_qos_dly_thr2(&self, n: usize) -> &AW_QOS_DLY_THR2 {
        &self.aw_qos_dly_thr2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x694..0x6ac - "]
    #[inline(always)]
    pub fn aw_qos_dly_thr2_iter(&self) -> impl Iterator<Item = &AW_QOS_DLY_THR2> {
        self.aw_qos_dly_thr2.iter()
    }
    #[doc = "0x6ac..0x6c4 - "]
    #[inline(always)]
    pub const fn aw_qos_dly_thr3(&self, n: usize) -> &AW_QOS_DLY_THR3 {
        &self.aw_qos_dly_thr3[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x6ac..0x6c4 - "]
    #[inline(always)]
    pub fn aw_qos_dly_thr3_iter(&self) -> impl Iterator<Item = &AW_QOS_DLY_THR3> {
        self.aw_qos_dly_thr3.iter()
    }
    #[doc = "0x6c4..0x6dc - "]
    #[inline(always)]
    pub const fn ar_qos_dly_thr0(&self, n: usize) -> &AR_QOS_DLY_THR0 {
        &self.ar_qos_dly_thr0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x6c4..0x6dc - "]
    #[inline(always)]
    pub fn ar_qos_dly_thr0_iter(&self) -> impl Iterator<Item = &AR_QOS_DLY_THR0> {
        self.ar_qos_dly_thr0.iter()
    }
    #[doc = "0x6dc..0x6f4 - "]
    #[inline(always)]
    pub const fn ar_qos_dly_thr1(&self, n: usize) -> &AR_QOS_DLY_THR1 {
        &self.ar_qos_dly_thr1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x6dc..0x6f4 - "]
    #[inline(always)]
    pub fn ar_qos_dly_thr1_iter(&self) -> impl Iterator<Item = &AR_QOS_DLY_THR1> {
        self.ar_qos_dly_thr1.iter()
    }
    #[doc = "0x6f4..0x70c - "]
    #[inline(always)]
    pub const fn ar_qos_dly_thr2(&self, n: usize) -> &AR_QOS_DLY_THR2 {
        &self.ar_qos_dly_thr2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x6f4..0x70c - "]
    #[inline(always)]
    pub fn ar_qos_dly_thr2_iter(&self) -> impl Iterator<Item = &AR_QOS_DLY_THR2> {
        self.ar_qos_dly_thr2.iter()
    }
    #[doc = "0x70c..0x724 - "]
    #[inline(always)]
    pub const fn ar_qos_dly_thr3(&self, n: usize) -> &AR_QOS_DLY_THR3 {
        &self.ar_qos_dly_thr3[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70c..0x724 - "]
    #[inline(always)]
    pub fn ar_qos_dly_thr3_iter(&self) -> impl Iterator<Item = &AR_QOS_DLY_THR3> {
        self.ar_qos_dly_thr3.iter()
    }
    #[doc = "0x724..0x73c - "]
    #[inline(always)]
    pub const fn aw_qos_bur_len_thr(&self, n: usize) -> &AW_QOS_BUR_LEN_THR {
        &self.aw_qos_bur_len_thr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x724..0x73c - "]
    #[inline(always)]
    pub fn aw_qos_bur_len_thr_iter(&self) -> impl Iterator<Item = &AW_QOS_BUR_LEN_THR> {
        self.aw_qos_bur_len_thr.iter()
    }
    #[doc = "0x73c..0x754 - "]
    #[inline(always)]
    pub const fn ar_qos_bur_len_thr(&self, n: usize) -> &AR_QOS_BUR_LEN_THR {
        &self.ar_qos_bur_len_thr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x73c..0x754 - "]
    #[inline(always)]
    pub fn ar_qos_bur_len_thr_iter(&self) -> impl Iterator<Item = &AR_QOS_BUR_LEN_THR> {
        self.ar_qos_bur_len_thr.iter()
    }
    #[doc = "0x754..0x76c - "]
    #[inline(always)]
    pub const fn aw_qos_dbg_tk_thr(&self, n: usize) -> &AW_QOS_DBG_TK_THR {
        &self.aw_qos_dbg_tk_thr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x754..0x76c - "]
    #[inline(always)]
    pub fn aw_qos_dbg_tk_thr_iter(&self) -> impl Iterator<Item = &AW_QOS_DBG_TK_THR> {
        self.aw_qos_dbg_tk_thr.iter()
    }
    #[doc = "0x76c..0x784 - "]
    #[inline(always)]
    pub const fn ar_qos_dbg_tk_thr(&self, n: usize) -> &AR_QOS_DBG_TK_THR {
        &self.ar_qos_dbg_tk_thr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x76c..0x784 - "]
    #[inline(always)]
    pub fn ar_qos_dbg_tk_thr_iter(&self) -> impl Iterator<Item = &AR_QOS_DBG_TK_THR> {
        self.ar_qos_dbg_tk_thr.iter()
    }
    #[doc = "0x784..0x790 - "]
    #[inline(always)]
    pub const fn max_ost_aw_s(&self, n: usize) -> &MAX_OST_AW_S {
        &self.max_ost_aw_s[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x784..0x790 - "]
    #[inline(always)]
    pub fn max_ost_aw_s_iter(&self) -> impl Iterator<Item = &MAX_OST_AW_S> {
        self.max_ost_aw_s.iter()
    }
    #[doc = "0x790..0x79c - "]
    #[inline(always)]
    pub const fn max_ost_ar_s(&self, n: usize) -> &MAX_OST_AR_S {
        &self.max_ost_ar_s[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x790..0x79c - "]
    #[inline(always)]
    pub fn max_ost_ar_s_iter(&self) -> impl Iterator<Item = &MAX_OST_AR_S> {
        self.max_ost_ar_s.iter()
    }
    #[doc = "0x79c - "]
    #[inline(always)]
    pub const fn aw_qos_peak_rate_mode(&self) -> &AW_QOS_PEAK_RATE_MODE {
        &self.aw_qos_peak_rate_mode
    }
    #[doc = "0x7a0 - "]
    #[inline(always)]
    pub const fn ar_qos_peak_rate_mode(&self) -> &AR_QOS_PEAK_RATE_MODE {
        &self.ar_qos_peak_rate_mode
    }
}
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
#[doc = "CLK_EN (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = ""]
pub mod clk_en;
#[doc = "INTERRUPT_RAW (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_raw`] module"]
pub type INTERRUPT_RAW = crate::Reg<interrupt_raw::INTERRUPT_RAW_SPEC>;
#[doc = ""]
pub mod interrupt_raw;
#[doc = "INTERRUPT_CLR (w) register accessor: \n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_clr`] module"]
pub type INTERRUPT_CLR = crate::Reg<interrupt_clr::INTERRUPT_CLR_SPEC>;
#[doc = ""]
pub mod interrupt_clr;
#[doc = "INTERRUPT_ENA (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_ena`] module"]
pub type INTERRUPT_ENA = crate::Reg<interrupt_ena::INTERRUPT_ENA_SPEC>;
#[doc = ""]
pub mod interrupt_ena;
#[doc = "INTERRUPT_ST (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt_st`] module"]
pub type INTERRUPT_ST = crate::Reg<interrupt_st::INTERRUPT_ST_SPEC>;
#[doc = ""]
pub mod interrupt_st;
#[doc = "TIMEOUT_MAX_CNT (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_max_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_max_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_max_cnt`] module"]
pub type TIMEOUT_MAX_CNT = crate::Reg<timeout_max_cnt::TIMEOUT_MAX_CNT_SPEC>;
#[doc = ""]
pub mod timeout_max_cnt;
#[doc = "TIMEOUT_INFO (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_info`] module"]
pub type TIMEOUT_INFO = crate::Reg<timeout_info::TIMEOUT_INFO_SPEC>;
#[doc = ""]
pub mod timeout_info;
#[doc = "DEC_FAILURE_INFO (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dec_failure_info::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dec_failure_info::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dec_failure_info`] module"]
pub type DEC_FAILURE_INFO = crate::Reg<dec_failure_info::DEC_FAILURE_INFO_SPEC>;
#[doc = ""]
pub mod dec_failure_info;
#[doc = "QOS_DEBUG_MST_SEL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`qos_debug_mst_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qos_debug_mst_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qos_debug_mst_sel`] module"]
pub type QOS_DEBUG_MST_SEL = crate::Reg<qos_debug_mst_sel::QOS_DEBUG_MST_SEL_SPEC>;
#[doc = ""]
pub mod qos_debug_mst_sel;
#[doc = "MST_ARQOS_REG0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mst_arqos_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mst_arqos_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mst_arqos_reg0`] module"]
pub type MST_ARQOS_REG0 = crate::Reg<mst_arqos_reg0::MST_ARQOS_REG0_SPEC>;
#[doc = ""]
pub mod mst_arqos_reg0;
#[doc = "MST_AWQOS_REG0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`mst_awqos_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mst_awqos_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mst_awqos_reg0`] module"]
pub type MST_AWQOS_REG0 = crate::Reg<mst_awqos_reg0::MST_AWQOS_REG0_SPEC>;
#[doc = ""]
pub mod mst_awqos_reg0;
#[doc = "DEC_FAILURE_ADDR (r) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dec_failure_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dec_failure_addr`] module"]
pub type DEC_FAILURE_ADDR = crate::Reg<dec_failure_addr::DEC_FAILURE_ADDR_SPEC>;
#[doc = ""]
pub mod dec_failure_addr;
#[doc = "REG_SLAVE_IS_SECURE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reg_slave_is_secure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_slave_is_secure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_slave_is_secure`] module"]
pub type REG_SLAVE_IS_SECURE = crate::Reg<reg_slave_is_secure::REG_SLAVE_IS_SECURE_SPEC>;
#[doc = ""]
pub mod reg_slave_is_secure;
#[doc = "SLAVE_SECURITY_OVERRIDE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`slave_security_override::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slave_security_override::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slave_security_override`] module"]
pub type SLAVE_SECURITY_OVERRIDE =
    crate::Reg<slave_security_override::SLAVE_SECURITY_OVERRIDE_SPEC>;
#[doc = ""]
pub mod slave_security_override;
#[doc = "MASTER_IS_SECURE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`master_is_secure::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_is_secure::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master_is_secure`] module"]
pub type MASTER_IS_SECURE = crate::Reg<master_is_secure::MASTER_IS_SECURE_SPEC>;
#[doc = ""]
pub mod master_is_secure;
#[doc = "MASTER_SECURITY_OVERRIDE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`master_security_override::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master_security_override::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master_security_override`] module"]
pub type MASTER_SECURITY_OVERRIDE =
    crate::Reg<master_security_override::MASTER_SECURITY_OVERRIDE_SPEC>;
#[doc = ""]
pub mod master_security_override;
#[doc = "AW_QOS_VALUE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_value`] module"]
pub type AW_QOS_VALUE = crate::Reg<aw_qos_value::AW_QOS_VALUE_SPEC>;
#[doc = ""]
pub mod aw_qos_value;
#[doc = "AR_QOS_VALUE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_value`] module"]
pub type AR_QOS_VALUE = crate::Reg<ar_qos_value::AR_QOS_VALUE_SPEC>;
#[doc = ""]
pub mod ar_qos_value;
#[doc = "B_PRIOR_VALUE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`b_prior_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_prior_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b_prior_value`] module"]
pub type B_PRIOR_VALUE = crate::Reg<b_prior_value::B_PRIOR_VALUE_SPEC>;
#[doc = ""]
pub mod b_prior_value;
#[doc = "R_PRIOR_VALUE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`r_prior_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_prior_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r_prior_value`] module"]
pub type R_PRIOR_VALUE = crate::Reg<r_prior_value::R_PRIOR_VALUE_SPEC>;
#[doc = ""]
pub mod r_prior_value;
#[doc = "AW_QOS_SEL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_sel`] module"]
pub type AW_QOS_SEL = crate::Reg<aw_qos_sel::AW_QOS_SEL_SPEC>;
#[doc = ""]
pub mod aw_qos_sel;
#[doc = "AR_QOS_SEL (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_sel`] module"]
pub type AR_QOS_SEL = crate::Reg<ar_qos_sel::AR_QOS_SEL_SPEC>;
#[doc = ""]
pub mod ar_qos_sel;
#[doc = "AW_QOS_RECOVERY (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_recovery::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_recovery::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_recovery`] module"]
pub type AW_QOS_RECOVERY = crate::Reg<aw_qos_recovery::AW_QOS_RECOVERY_SPEC>;
#[doc = ""]
pub mod aw_qos_recovery;
#[doc = "AR_QOS_RECOVERY (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_recovery::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_recovery::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_recovery`] module"]
pub type AR_QOS_RECOVERY = crate::Reg<ar_qos_recovery::AR_QOS_RECOVERY_SPEC>;
#[doc = ""]
pub mod ar_qos_recovery;
#[doc = "AW_QOS_ID_MASK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_id_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_id_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_id_mask`] module"]
pub type AW_QOS_ID_MASK = crate::Reg<aw_qos_id_mask::AW_QOS_ID_MASK_SPEC>;
#[doc = ""]
pub mod aw_qos_id_mask;
#[doc = "AR_QOS_ID_MASK (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_id_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_id_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_id_mask`] module"]
pub type AR_QOS_ID_MASK = crate::Reg<ar_qos_id_mask::AR_QOS_ID_MASK_SPEC>;
#[doc = ""]
pub mod ar_qos_id_mask;
#[doc = "AW_QOS_ID_FILTER (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_id_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_id_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_id_filter`] module"]
pub type AW_QOS_ID_FILTER = crate::Reg<aw_qos_id_filter::AW_QOS_ID_FILTER_SPEC>;
#[doc = ""]
pub mod aw_qos_id_filter;
#[doc = "AR_QOS_ID_FILTER (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_id_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_id_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_id_filter`] module"]
pub type AR_QOS_ID_FILTER = crate::Reg<ar_qos_id_filter::AR_QOS_ID_FILTER_SPEC>;
#[doc = ""]
pub mod ar_qos_id_filter;
#[doc = "AW_QOS_CTRL_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_ctrl_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_ctrl_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_ctrl_cfg`] module"]
pub type AW_QOS_CTRL_CFG = crate::Reg<aw_qos_ctrl_cfg::AW_QOS_CTRL_CFG_SPEC>;
#[doc = ""]
pub mod aw_qos_ctrl_cfg;
#[doc = "AR_QOS_CTRL_CFG (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_ctrl_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_ctrl_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_ctrl_cfg`] module"]
pub type AR_QOS_CTRL_CFG = crate::Reg<ar_qos_ctrl_cfg::AR_QOS_CTRL_CFG_SPEC>;
#[doc = ""]
pub mod ar_qos_ctrl_cfg;
#[doc = "AW_QOS_TRANS_RATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_trans_rate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_trans_rate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_trans_rate`] module"]
pub type AW_QOS_TRANS_RATE = crate::Reg<aw_qos_trans_rate::AW_QOS_TRANS_RATE_SPEC>;
#[doc = ""]
pub mod aw_qos_trans_rate;
#[doc = "AR_QOS_TRANS_RATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_trans_rate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_trans_rate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_trans_rate`] module"]
pub type AR_QOS_TRANS_RATE = crate::Reg<ar_qos_trans_rate::AR_QOS_TRANS_RATE_SPEC>;
#[doc = ""]
pub mod ar_qos_trans_rate;
#[doc = "AW_QOS_PEAK_RATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_peak_rate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_peak_rate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_peak_rate`] module"]
pub type AW_QOS_PEAK_RATE = crate::Reg<aw_qos_peak_rate::AW_QOS_PEAK_RATE_SPEC>;
#[doc = ""]
pub mod aw_qos_peak_rate;
#[doc = "AR_QOS_PEAK_RATE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_peak_rate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_peak_rate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_peak_rate`] module"]
pub type AR_QOS_PEAK_RATE = crate::Reg<ar_qos_peak_rate::AR_QOS_PEAK_RATE_SPEC>;
#[doc = ""]
pub mod ar_qos_peak_rate;
#[doc = "AW_QOS_BURSTINESS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_burstiness::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_burstiness::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_burstiness`] module"]
pub type AW_QOS_BURSTINESS = crate::Reg<aw_qos_burstiness::AW_QOS_BURSTINESS_SPEC>;
#[doc = ""]
pub mod aw_qos_burstiness;
#[doc = "AR_QOS_BURSTINESS (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_burstiness::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_burstiness::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_burstiness`] module"]
pub type AR_QOS_BURSTINESS = crate::Reg<ar_qos_burstiness::AR_QOS_BURSTINESS_SPEC>;
#[doc = ""]
pub mod ar_qos_burstiness;
#[doc = "AW_QOS_CFG_VLU (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_cfg_vlu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_cfg_vlu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_cfg_vlu`] module"]
pub type AW_QOS_CFG_VLU = crate::Reg<aw_qos_cfg_vlu::AW_QOS_CFG_VLU_SPEC>;
#[doc = ""]
pub mod aw_qos_cfg_vlu;
#[doc = "AR_QOS_CFG_VLU (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_cfg_vlu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_cfg_vlu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_cfg_vlu`] module"]
pub type AR_QOS_CFG_VLU = crate::Reg<ar_qos_cfg_vlu::AR_QOS_CFG_VLU_SPEC>;
#[doc = ""]
pub mod ar_qos_cfg_vlu;
#[doc = "AW_QOS_DLY_THR0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_dly_thr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_dly_thr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_dly_thr0`] module"]
pub type AW_QOS_DLY_THR0 = crate::Reg<aw_qos_dly_thr0::AW_QOS_DLY_THR0_SPEC>;
#[doc = ""]
pub mod aw_qos_dly_thr0;
#[doc = "AW_QOS_DLY_THR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_dly_thr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_dly_thr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_dly_thr1`] module"]
pub type AW_QOS_DLY_THR1 = crate::Reg<aw_qos_dly_thr1::AW_QOS_DLY_THR1_SPEC>;
#[doc = ""]
pub mod aw_qos_dly_thr1;
#[doc = "AW_QOS_DLY_THR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_dly_thr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_dly_thr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_dly_thr2`] module"]
pub type AW_QOS_DLY_THR2 = crate::Reg<aw_qos_dly_thr2::AW_QOS_DLY_THR2_SPEC>;
#[doc = ""]
pub mod aw_qos_dly_thr2;
#[doc = "AW_QOS_DLY_THR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_dly_thr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_dly_thr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_dly_thr3`] module"]
pub type AW_QOS_DLY_THR3 = crate::Reg<aw_qos_dly_thr3::AW_QOS_DLY_THR3_SPEC>;
#[doc = ""]
pub mod aw_qos_dly_thr3;
#[doc = "AR_QOS_DLY_THR0 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_dly_thr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_dly_thr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_dly_thr0`] module"]
pub type AR_QOS_DLY_THR0 = crate::Reg<ar_qos_dly_thr0::AR_QOS_DLY_THR0_SPEC>;
#[doc = ""]
pub mod ar_qos_dly_thr0;
#[doc = "AR_QOS_DLY_THR1 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_dly_thr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_dly_thr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_dly_thr1`] module"]
pub type AR_QOS_DLY_THR1 = crate::Reg<ar_qos_dly_thr1::AR_QOS_DLY_THR1_SPEC>;
#[doc = ""]
pub mod ar_qos_dly_thr1;
#[doc = "AR_QOS_DLY_THR2 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_dly_thr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_dly_thr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_dly_thr2`] module"]
pub type AR_QOS_DLY_THR2 = crate::Reg<ar_qos_dly_thr2::AR_QOS_DLY_THR2_SPEC>;
#[doc = ""]
pub mod ar_qos_dly_thr2;
#[doc = "AR_QOS_DLY_THR3 (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_dly_thr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_dly_thr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_dly_thr3`] module"]
pub type AR_QOS_DLY_THR3 = crate::Reg<ar_qos_dly_thr3::AR_QOS_DLY_THR3_SPEC>;
#[doc = ""]
pub mod ar_qos_dly_thr3;
#[doc = "AW_QOS_BUR_LEN_THR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_bur_len_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_bur_len_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_bur_len_thr`] module"]
pub type AW_QOS_BUR_LEN_THR = crate::Reg<aw_qos_bur_len_thr::AW_QOS_BUR_LEN_THR_SPEC>;
#[doc = ""]
pub mod aw_qos_bur_len_thr;
#[doc = "AR_QOS_BUR_LEN_THR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_bur_len_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_bur_len_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_bur_len_thr`] module"]
pub type AR_QOS_BUR_LEN_THR = crate::Reg<ar_qos_bur_len_thr::AR_QOS_BUR_LEN_THR_SPEC>;
#[doc = ""]
pub mod ar_qos_bur_len_thr;
#[doc = "AW_QOS_DBG_TK_THR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_dbg_tk_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_dbg_tk_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_dbg_tk_thr`] module"]
pub type AW_QOS_DBG_TK_THR = crate::Reg<aw_qos_dbg_tk_thr::AW_QOS_DBG_TK_THR_SPEC>;
#[doc = ""]
pub mod aw_qos_dbg_tk_thr;
#[doc = "AR_QOS_DBG_TK_THR (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_dbg_tk_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_dbg_tk_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_dbg_tk_thr`] module"]
pub type AR_QOS_DBG_TK_THR = crate::Reg<ar_qos_dbg_tk_thr::AR_QOS_DBG_TK_THR_SPEC>;
#[doc = ""]
pub mod ar_qos_dbg_tk_thr;
#[doc = "MAX_OST_AW_S (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`max_ost_aw_s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max_ost_aw_s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max_ost_aw_s`] module"]
pub type MAX_OST_AW_S = crate::Reg<max_ost_aw_s::MAX_OST_AW_S_SPEC>;
#[doc = ""]
pub mod max_ost_aw_s;
#[doc = "MAX_OST_AR_S (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`max_ost_ar_s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max_ost_ar_s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@max_ost_ar_s`] module"]
pub type MAX_OST_AR_S = crate::Reg<max_ost_ar_s::MAX_OST_AR_S_SPEC>;
#[doc = ""]
pub mod max_ost_ar_s;
#[doc = "AW_QOS_PEAK_RATE_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`aw_qos_peak_rate_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aw_qos_peak_rate_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aw_qos_peak_rate_mode`] module"]
pub type AW_QOS_PEAK_RATE_MODE = crate::Reg<aw_qos_peak_rate_mode::AW_QOS_PEAK_RATE_MODE_SPEC>;
#[doc = ""]
pub mod aw_qos_peak_rate_mode;
#[doc = "AR_QOS_PEAK_RATE_MODE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`ar_qos_peak_rate_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar_qos_peak_rate_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar_qos_peak_rate_mode`] module"]
pub type AR_QOS_PEAK_RATE_MODE = crate::Reg<ar_qos_peak_rate_mode::AR_QOS_PEAK_RATE_MODE_SPEC>;
#[doc = ""]
pub mod ar_qos_peak_rate_mode;
