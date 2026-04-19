#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    in_int_ch: [IN_INT_CH; 3],
    out_int_ch: [OUT_INT_CH; 3],
    ahb_test: AHB_TEST,
    misc_conf: MISC_CONF,
    date: DATE,
    _reserved5: [u8; 0x04],
    ch: [CH; 3],
    _reserved6: [u8; 0x0c],
    out_crc_init_data_ch: (),
    _reserved7: [u8; 0x04],
    tx_crc_width_ch: (),
    _reserved8: [u8; 0x04],
    out_crc_clear_ch: (),
    _reserved9: [u8; 0x04],
    out_crc_final_result_ch: (),
    _reserved10: [u8; 0x04],
    tx_crc_en_wr_data_ch: (),
    _reserved11: [u8; 0x04],
    tx_crc_en_addr_ch: (),
    _reserved12: [u8; 0x04],
    tx_crc_data_en_wr_data_ch: (),
    _reserved13: [u8; 0x04],
    tx_crc_data_en_addr_ch: (),
    _reserved14: [u8; 0x04],
    tx_ch_arb_weight_ch0: TX_CH_ARB_WEIGHT_CH0,
    tx_arb_weight_opt_dir_ch0: TX_ARB_WEIGHT_OPT_DIR_CH0,
    _reserved16: [u8; 0x20],
    tx_ch_arb_weight_ch1: TX_CH_ARB_WEIGHT_CH1,
    tx_arb_weight_opt_dir_ch1: TX_ARB_WEIGHT_OPT_DIR_CH1,
    _reserved18: [u8; 0x20],
    tx_ch_arb_weight_ch2: TX_CH_ARB_WEIGHT_CH2,
    tx_arb_weight_opt_dir_ch2: TX_ARB_WEIGHT_OPT_DIR_CH2,
    in_crc_init_data_ch: (),
    _reserved21: [u8; 0x04],
    rx_crc_width_ch: (),
    _reserved22: [u8; 0x04],
    in_crc_clear_ch: (),
    _reserved23: [u8; 0x04],
    in_crc_final_result_ch: (),
    _reserved24: [u8; 0x04],
    rx_crc_en_wr_data_ch: (),
    _reserved25: [u8; 0x04],
    rx_crc_en_addr_ch: (),
    _reserved26: [u8; 0x04],
    rx_crc_data_en_wr_data_ch: (),
    _reserved27: [u8; 0x04],
    rx_crc_data_en_addr_ch: (),
    _reserved28: [u8; 0x04],
    rx_ch_arb_weight_ch0: RX_CH_ARB_WEIGHT_CH0,
    rx_arb_weight_opt_dir_ch0: RX_ARB_WEIGHT_OPT_DIR_CH0,
    _reserved30: [u8; 0x20],
    rx_ch_arb_weight_ch1: RX_CH_ARB_WEIGHT_CH1,
    rx_arb_weight_opt_dir_ch1: RX_ARB_WEIGHT_OPT_DIR_CH1,
    _reserved32: [u8; 0x20],
    rx_ch_arb_weight_ch2: RX_CH_ARB_WEIGHT_CH2,
    rx_arb_weight_opt_dir_ch2: RX_ARB_WEIGHT_OPT_DIR_CH2,
    in_link_addr_ch: [IN_LINK_ADDR_CH; 3],
    out_link_addr_ch: [OUT_LINK_ADDR_CH; 3],
    intr_mem_start_addr: INTR_MEM_START_ADDR,
    intr_mem_end_addr: INTR_MEM_END_ADDR,
    arb_timeout_tx: ARB_TIMEOUT_TX,
    arb_timeout_rx: ARB_TIMEOUT_RX,
    weight_en_tx: WEIGHT_EN_TX,
    weight_en_rx: WEIGHT_EN_RX,
    arb_timeout: ARB_TIMEOUT,
    _reserved43: [u8; 0x20],
    weight_en: WEIGHT_EN,
    module_clk_en: MODULE_CLK_EN,
    ahbinf_resp_err_status0: AHBINF_RESP_ERR_STATUS0,
    ahbinf_resp_err_status1: AHBINF_RESP_ERR_STATUS1,
    in_done_des_addr_ch0: IN_DONE_DES_ADDR_CH0,
    out_done_des_addr_ch0: OUT_DONE_DES_ADDR_CH0,
    in_done_des_addr_ch1: IN_DONE_DES_ADDR_CH1,
    out_done_des_addr_ch1: OUT_DONE_DES_ADDR_CH1,
    in_done_des_addr_ch2: IN_DONE_DES_ADDR_CH2,
    out_done_des_addr_ch2: OUT_DONE_DES_ADDR_CH2,
}
impl RegisterBlock {
    #[doc = "0x00..0x30 - Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
    #[inline(always)]
    pub const fn in_int_ch(&self, n: usize) -> &IN_INT_CH {
        &self.in_int_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x30 - Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
    #[inline(always)]
    pub fn in_int_ch_iter(&self) -> impl Iterator<Item = &IN_INT_CH> {
        self.in_int_ch.iter()
    }
    #[doc = "0x30..0x60 - Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
    #[inline(always)]
    pub const fn out_int_ch(&self, n: usize) -> &OUT_INT_CH {
        &self.out_int_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x60 - Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
    #[inline(always)]
    pub fn out_int_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_CH> {
        self.out_int_ch.iter()
    }
    #[doc = "0x60 - only for test"]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x64 - reserved"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0x68 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x70..0x2b0 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PERI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PERI_CH?, OUT_PERI_SEL_CH?"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &CH {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x2b0 - Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PERI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PERI_CH?, OUT_PERI_SEL_CH?"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &CH> {
        self.ch.iter()
    }
    #[doc = "0x2bc..0x2c8 - This register is used to config ch%s crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn out_crc_init_data_ch(&self, n: usize) -> &OUT_CRC_INIT_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(700)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2bc..0x2c8 - This register is used to config ch%s crc initial data(max 32 bit)"]
    #[inline(always)]
    pub fn out_crc_init_data_ch_iter(&self) -> impl Iterator<Item = &OUT_CRC_INIT_DATA_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(700)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2c0..0x2cc - This register is used to confiig tx ch%s crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
    #[inline(always)]
    pub const fn tx_crc_width_ch(&self, n: usize) -> &TX_CRC_WIDTH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(704)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c0..0x2cc - This register is used to confiig tx ch%s crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
    #[inline(always)]
    pub fn tx_crc_width_ch_iter(&self) -> impl Iterator<Item = &TX_CRC_WIDTH_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(704)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2c4..0x2d0 - This register is used to clear ch%s crc result"]
    #[inline(always)]
    pub const fn out_crc_clear_ch(&self, n: usize) -> &OUT_CRC_CLEAR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(708)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c4..0x2d0 - This register is used to clear ch%s crc result"]
    #[inline(always)]
    pub fn out_crc_clear_ch_iter(&self) -> impl Iterator<Item = &OUT_CRC_CLEAR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(708)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2c8..0x2d4 - This register is used to store ch%s crc result"]
    #[inline(always)]
    pub const fn out_crc_final_result_ch(&self, n: usize) -> &OUT_CRC_FINAL_RESULT_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(712)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c8..0x2d4 - This register is used to store ch%s crc result"]
    #[inline(always)]
    pub fn out_crc_final_result_ch_iter(&self) -> impl Iterator<Item = &OUT_CRC_FINAL_RESULT_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(712)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2cc..0x2d8 - This resister is used to config ch%s crc en for every bit"]
    #[inline(always)]
    pub const fn tx_crc_en_wr_data_ch(&self, n: usize) -> &TX_CRC_EN_WR_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(716)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2cc..0x2d8 - This resister is used to config ch%s crc en for every bit"]
    #[inline(always)]
    pub fn tx_crc_en_wr_data_ch_iter(&self) -> impl Iterator<Item = &TX_CRC_EN_WR_DATA_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(716)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2d0..0x2dc - This register is used to config ch%s crc en addr"]
    #[inline(always)]
    pub const fn tx_crc_en_addr_ch(&self, n: usize) -> &TX_CRC_EN_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(720)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2d0..0x2dc - This register is used to config ch%s crc en addr"]
    #[inline(always)]
    pub fn tx_crc_en_addr_ch_iter(&self) -> impl Iterator<Item = &TX_CRC_EN_ADDR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(720)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2d4..0x2e0 - This register is used to config crc data_8bit en"]
    #[inline(always)]
    pub const fn tx_crc_data_en_wr_data_ch(&self, n: usize) -> &TX_CRC_DATA_EN_WR_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(724)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2d4..0x2e0 - This register is used to config crc data_8bit en"]
    #[inline(always)]
    pub fn tx_crc_data_en_wr_data_ch_iter(
        &self,
    ) -> impl Iterator<Item = &TX_CRC_DATA_EN_WR_DATA_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(724)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2d8..0x2e4 - This register is used to config addr of crc data_8bit en"]
    #[inline(always)]
    pub const fn tx_crc_data_en_addr_ch(&self, n: usize) -> &TX_CRC_DATA_EN_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(728)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2d8..0x2e4 - This register is used to config addr of crc data_8bit en"]
    #[inline(always)]
    pub fn tx_crc_data_en_addr_ch_iter(&self) -> impl Iterator<Item = &TX_CRC_DATA_EN_ADDR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(728)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2dc - TX channel 0 arbitration weight configuration register"]
    #[inline(always)]
    pub const fn tx_ch_arb_weight_ch0(&self) -> &TX_CH_ARB_WEIGHT_CH0 {
        &self.tx_ch_arb_weight_ch0
    }
    #[doc = "0x2e0 - TX channel 0 weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn tx_arb_weight_opt_dir_ch0(&self) -> &TX_ARB_WEIGHT_OPT_DIR_CH0 {
        &self.tx_arb_weight_opt_dir_ch0
    }
    #[doc = "0x304 - TX channel 1 arbitration weight configuration register"]
    #[inline(always)]
    pub const fn tx_ch_arb_weight_ch1(&self) -> &TX_CH_ARB_WEIGHT_CH1 {
        &self.tx_ch_arb_weight_ch1
    }
    #[doc = "0x308 - TX channel 1 weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn tx_arb_weight_opt_dir_ch1(&self) -> &TX_ARB_WEIGHT_OPT_DIR_CH1 {
        &self.tx_arb_weight_opt_dir_ch1
    }
    #[doc = "0x32c - TX channel 2 arbitration weight configuration register"]
    #[inline(always)]
    pub const fn tx_ch_arb_weight_ch2(&self) -> &TX_CH_ARB_WEIGHT_CH2 {
        &self.tx_ch_arb_weight_ch2
    }
    #[doc = "0x330 - TX channel 2 weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn tx_arb_weight_opt_dir_ch2(&self) -> &TX_ARB_WEIGHT_OPT_DIR_CH2 {
        &self.tx_arb_weight_opt_dir_ch2
    }
    #[doc = "0x334..0x340 - This register is used to config ch%s crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn in_crc_init_data_ch(&self, n: usize) -> &IN_CRC_INIT_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(820)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x334..0x340 - This register is used to config ch%s crc initial data(max 32 bit)"]
    #[inline(always)]
    pub fn in_crc_init_data_ch_iter(&self) -> impl Iterator<Item = &IN_CRC_INIT_DATA_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(820)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x338..0x344 - This register is used to confiig rx ch%s crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
    #[inline(always)]
    pub const fn rx_crc_width_ch(&self, n: usize) -> &RX_CRC_WIDTH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(824)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x338..0x344 - This register is used to confiig rx ch%s crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
    #[inline(always)]
    pub fn rx_crc_width_ch_iter(&self) -> impl Iterator<Item = &RX_CRC_WIDTH_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(824)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x33c..0x348 - This register is used to clear ch%s crc result"]
    #[inline(always)]
    pub const fn in_crc_clear_ch(&self, n: usize) -> &IN_CRC_CLEAR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(828)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x33c..0x348 - This register is used to clear ch%s crc result"]
    #[inline(always)]
    pub fn in_crc_clear_ch_iter(&self) -> impl Iterator<Item = &IN_CRC_CLEAR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(828)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x340..0x34c - This register is used to store ch%s crc result"]
    #[inline(always)]
    pub const fn in_crc_final_result_ch(&self, n: usize) -> &IN_CRC_FINAL_RESULT_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(832)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x340..0x34c - This register is used to store ch%s crc result"]
    #[inline(always)]
    pub fn in_crc_final_result_ch_iter(&self) -> impl Iterator<Item = &IN_CRC_FINAL_RESULT_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(832)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x344..0x350 - This resister is used to config ch%s crc en for every bit"]
    #[inline(always)]
    pub const fn rx_crc_en_wr_data_ch(&self, n: usize) -> &RX_CRC_EN_WR_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(836)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x344..0x350 - This resister is used to config ch%s crc en for every bit"]
    #[inline(always)]
    pub fn rx_crc_en_wr_data_ch_iter(&self) -> impl Iterator<Item = &RX_CRC_EN_WR_DATA_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(836)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x348..0x354 - This register is used to config ch%s crc en addr"]
    #[inline(always)]
    pub const fn rx_crc_en_addr_ch(&self, n: usize) -> &RX_CRC_EN_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(840)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x348..0x354 - This register is used to config ch%s crc en addr"]
    #[inline(always)]
    pub fn rx_crc_en_addr_ch_iter(&self) -> impl Iterator<Item = &RX_CRC_EN_ADDR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(840)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x34c..0x358 - This register is used to config crc data_8bit en"]
    #[inline(always)]
    pub const fn rx_crc_data_en_wr_data_ch(&self, n: usize) -> &RX_CRC_DATA_EN_WR_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(844)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34c..0x358 - This register is used to config crc data_8bit en"]
    #[inline(always)]
    pub fn rx_crc_data_en_wr_data_ch_iter(
        &self,
    ) -> impl Iterator<Item = &RX_CRC_DATA_EN_WR_DATA_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(844)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x350..0x35c - This register is used to config addr of crc data_8bit en"]
    #[inline(always)]
    pub const fn rx_crc_data_en_addr_ch(&self, n: usize) -> &RX_CRC_DATA_EN_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(848)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x350..0x35c - This register is used to config addr of crc data_8bit en"]
    #[inline(always)]
    pub fn rx_crc_data_en_addr_ch_iter(&self) -> impl Iterator<Item = &RX_CRC_DATA_EN_ADDR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(848)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x354 - RX channel 0 arbitration weight configuration register"]
    #[inline(always)]
    pub const fn rx_ch_arb_weight_ch0(&self) -> &RX_CH_ARB_WEIGHT_CH0 {
        &self.rx_ch_arb_weight_ch0
    }
    #[doc = "0x358 - RX channel 0 weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn rx_arb_weight_opt_dir_ch0(&self) -> &RX_ARB_WEIGHT_OPT_DIR_CH0 {
        &self.rx_arb_weight_opt_dir_ch0
    }
    #[doc = "0x37c - RX channel 1 arbitration weight configuration register"]
    #[inline(always)]
    pub const fn rx_ch_arb_weight_ch1(&self) -> &RX_CH_ARB_WEIGHT_CH1 {
        &self.rx_ch_arb_weight_ch1
    }
    #[doc = "0x380 - RX channel 1 weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn rx_arb_weight_opt_dir_ch1(&self) -> &RX_ARB_WEIGHT_OPT_DIR_CH1 {
        &self.rx_arb_weight_opt_dir_ch1
    }
    #[doc = "0x3a4 - RX channel 2 arbitration weight configuration register"]
    #[inline(always)]
    pub const fn rx_ch_arb_weight_ch2(&self) -> &RX_CH_ARB_WEIGHT_CH2 {
        &self.rx_ch_arb_weight_ch2
    }
    #[doc = "0x3a8 - RX channel 2 weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn rx_arb_weight_opt_dir_ch2(&self) -> &RX_ARB_WEIGHT_OPT_DIR_CH2 {
        &self.rx_arb_weight_opt_dir_ch2
    }
    #[doc = "0x3ac..0x3b8 - Cluster IN_LINK_ADDR_CH%s, containing IN_LINK_ADDR_CH?"]
    #[inline(always)]
    pub const fn in_link_addr_ch(&self, n: usize) -> &IN_LINK_ADDR_CH {
        &self.in_link_addr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3ac..0x3b8 - Cluster IN_LINK_ADDR_CH%s, containing IN_LINK_ADDR_CH?"]
    #[inline(always)]
    pub fn in_link_addr_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_ADDR_CH> {
        self.in_link_addr_ch.iter()
    }
    #[doc = "0x3b8..0x3c4 - Cluster OUT_LINK_ADDR_CH%s, containing OUT_LINK_ADDR_CH?"]
    #[inline(always)]
    pub const fn out_link_addr_ch(&self, n: usize) -> &OUT_LINK_ADDR_CH {
        &self.out_link_addr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3b8..0x3c4 - Cluster OUT_LINK_ADDR_CH%s, containing OUT_LINK_ADDR_CH?"]
    #[inline(always)]
    pub fn out_link_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_LINK_ADDR_CH> {
        self.out_link_addr_ch.iter()
    }
    #[doc = "0x3c4 - Accessible address space start address configuration register"]
    #[inline(always)]
    pub const fn intr_mem_start_addr(&self) -> &INTR_MEM_START_ADDR {
        &self.intr_mem_start_addr
    }
    #[doc = "0x3c8 - Accessible address space end address configuration register"]
    #[inline(always)]
    pub const fn intr_mem_end_addr(&self) -> &INTR_MEM_END_ADDR {
        &self.intr_mem_end_addr
    }
    #[doc = "0x3cc - TX arbitration timeout configuration register"]
    #[inline(always)]
    pub const fn arb_timeout_tx(&self) -> &ARB_TIMEOUT_TX {
        &self.arb_timeout_tx
    }
    #[doc = "0x3d0 - RX arbitration timeout configuration register"]
    #[inline(always)]
    pub const fn arb_timeout_rx(&self) -> &ARB_TIMEOUT_RX {
        &self.arb_timeout_rx
    }
    #[doc = "0x3d4 - TX weight arbitration enable register"]
    #[inline(always)]
    pub const fn weight_en_tx(&self) -> &WEIGHT_EN_TX {
        &self.weight_en_tx
    }
    #[doc = "0x3d8 - RX weight arbitration enable register"]
    #[inline(always)]
    pub const fn weight_en_rx(&self) -> &WEIGHT_EN_RX {
        &self.weight_en_rx
    }
    #[doc = "0x3dc - TX arbitration timeout configuration register"]
    #[inline(always)]
    pub const fn arb_timeout(&self) -> &ARB_TIMEOUT {
        &self.arb_timeout
    }
    #[doc = "0x400 - TX weight arbitration enable register"]
    #[inline(always)]
    pub const fn weight_en(&self) -> &WEIGHT_EN {
        &self.weight_en
    }
    #[doc = "0x404 - Module clock force on register"]
    #[inline(always)]
    pub const fn module_clk_en(&self) -> &MODULE_CLK_EN {
        &self.module_clk_en
    }
    #[doc = "0x408 - AHB response error status 0 register"]
    #[inline(always)]
    pub const fn ahbinf_resp_err_status0(&self) -> &AHBINF_RESP_ERR_STATUS0 {
        &self.ahbinf_resp_err_status0
    }
    #[doc = "0x40c - AHB response error status 1 register"]
    #[inline(always)]
    pub const fn ahbinf_resp_err_status1(&self) -> &AHBINF_RESP_ERR_STATUS1 {
        &self.ahbinf_resp_err_status1
    }
    #[doc = "0x410 - RX_done Inlink descriptor address of RX channel 0"]
    #[inline(always)]
    pub const fn in_done_des_addr_ch0(&self) -> &IN_DONE_DES_ADDR_CH0 {
        &self.in_done_des_addr_ch0
    }
    #[doc = "0x414 - TX done outlink descriptor address of TX channel 0"]
    #[inline(always)]
    pub const fn out_done_des_addr_ch0(&self) -> &OUT_DONE_DES_ADDR_CH0 {
        &self.out_done_des_addr_ch0
    }
    #[doc = "0x418 - RX_done Inlink descriptor address of RX channel 1"]
    #[inline(always)]
    pub const fn in_done_des_addr_ch1(&self) -> &IN_DONE_DES_ADDR_CH1 {
        &self.in_done_des_addr_ch1
    }
    #[doc = "0x41c - TX done outlink descriptor address of TX channel 1"]
    #[inline(always)]
    pub const fn out_done_des_addr_ch1(&self) -> &OUT_DONE_DES_ADDR_CH1 {
        &self.out_done_des_addr_ch1
    }
    #[doc = "0x420 - RX_done Inlink descriptor address of RX channel 2"]
    #[inline(always)]
    pub const fn in_done_des_addr_ch2(&self) -> &IN_DONE_DES_ADDR_CH2 {
        &self.in_done_des_addr_ch2
    }
    #[doc = "0x424 - TX done outlink descriptor address of TX channel 2"]
    #[inline(always)]
    pub const fn out_done_des_addr_ch2(&self) -> &OUT_DONE_DES_ADDR_CH2 {
        &self.out_done_des_addr_ch2
    }
}
#[doc = "Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
pub use self::in_int_ch::IN_INT_CH;
#[doc = r"Cluster"]
#[doc = "Cluster IN_INT_CH%s, containing IN_INT_RAW_CH?, IN_INT_ST_CH?, IN_INT_ENA_CH?, IN_INT_CLR_CH?"]
pub mod in_int_ch;
#[doc = "Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
pub use self::out_int_ch::OUT_INT_CH;
#[doc = r"Cluster"]
#[doc = "Cluster OUT_INT_CH%s, containing OUT_INT_RAW_CH?, OUT_INT_ST_CH?, OUT_INT_ENA_CH?, OUT_INT_CLR_CH?"]
pub mod out_int_ch;
#[doc = "AHB_TEST (rw) register accessor: only for test\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "only for test"]
pub mod ahb_test;
#[doc = "MISC_CONF (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "reserved"]
pub mod misc_conf;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PERI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PERI_CH?, OUT_PERI_SEL_CH?"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing IN_CONF0_CH?, IN_CONF1_CH?, INFIFO_STATUS_CH?, IN_POP_CH?, IN_LINK_CH?, IN_STATE_CH?, IN_SUC_EOF_DES_ADDR_CH?, IN_ERR_EOF_DES_ADDR_CH?, IN_DSCR_CH?, IN_DSCR_BF0_CH?, IN_DSCR_BF1_CH?, IN_PERI_CH?, IN_PERI_SEL_CH?, OUT_CONF0_CH?, OUT_CONF1_CH?, OUTFIFO_STATUS_CH?, OUT_PUSH_CH?, OUT_LINK_CH?, OUT_STATE_CH?, OUT_EOF_DES_ADDR_CH?, OUT_EOF_BFR_DES_ADDR_CH?, OUT_DSCR_CH?, OUT_DSCR_BF0_CH?, OUT_DSCR_BF1_CH?, OUT_PERI_CH?, OUT_PERI_SEL_CH?"]
pub mod ch;
#[doc = "OUT_CRC_INIT_DATA_CH (rw) register accessor: This register is used to config ch%s crc initial data(max 32 bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`out_crc_init_data_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_crc_init_data_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_init_data_ch`] module"]
pub type OUT_CRC_INIT_DATA_CH = crate::Reg<out_crc_init_data_ch::OUT_CRC_INIT_DATA_CH_SPEC>;
#[doc = "This register is used to config ch%s crc initial data(max 32 bit)"]
pub mod out_crc_init_data_ch;
#[doc = "TX_CRC_WIDTH_CH (rw) register accessor: This register is used to confiig tx ch%s crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_width_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_width_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_width_ch`] module"]
pub type TX_CRC_WIDTH_CH = crate::Reg<tx_crc_width_ch::TX_CRC_WIDTH_CH_SPEC>;
#[doc = "This register is used to confiig tx ch%s crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
pub mod tx_crc_width_ch;
#[doc = "OUT_CRC_CLEAR_CH (rw) register accessor: This register is used to clear ch%s crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`out_crc_clear_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_crc_clear_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_clear_ch`] module"]
pub type OUT_CRC_CLEAR_CH = crate::Reg<out_crc_clear_ch::OUT_CRC_CLEAR_CH_SPEC>;
#[doc = "This register is used to clear ch%s crc result"]
pub mod out_crc_clear_ch;
#[doc = "OUT_CRC_FINAL_RESULT_CH (r) register accessor: This register is used to store ch%s crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`out_crc_final_result_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_final_result_ch`] module"]
pub type OUT_CRC_FINAL_RESULT_CH =
    crate::Reg<out_crc_final_result_ch::OUT_CRC_FINAL_RESULT_CH_SPEC>;
#[doc = "This register is used to store ch%s crc result"]
pub mod out_crc_final_result_ch;
#[doc = "TX_CRC_EN_WR_DATA_CH (rw) register accessor: This resister is used to config ch%s crc en for every bit\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_en_wr_data_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_en_wr_data_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_en_wr_data_ch`] module"]
pub type TX_CRC_EN_WR_DATA_CH = crate::Reg<tx_crc_en_wr_data_ch::TX_CRC_EN_WR_DATA_CH_SPEC>;
#[doc = "This resister is used to config ch%s crc en for every bit"]
pub mod tx_crc_en_wr_data_ch;
#[doc = "TX_CRC_EN_ADDR_CH (rw) register accessor: This register is used to config ch%s crc en addr\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_en_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_en_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_en_addr_ch`] module"]
pub type TX_CRC_EN_ADDR_CH = crate::Reg<tx_crc_en_addr_ch::TX_CRC_EN_ADDR_CH_SPEC>;
#[doc = "This register is used to config ch%s crc en addr"]
pub mod tx_crc_en_addr_ch;
#[doc = "TX_CRC_DATA_EN_WR_DATA_CH (rw) register accessor: This register is used to config crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_data_en_wr_data_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_data_en_wr_data_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_data_en_wr_data_ch`] module"]
pub type TX_CRC_DATA_EN_WR_DATA_CH =
    crate::Reg<tx_crc_data_en_wr_data_ch::TX_CRC_DATA_EN_WR_DATA_CH_SPEC>;
#[doc = "This register is used to config crc data_8bit en"]
pub mod tx_crc_data_en_wr_data_ch;
#[doc = "TX_CRC_DATA_EN_ADDR_CH (rw) register accessor: This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_data_en_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_data_en_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_data_en_addr_ch`] module"]
pub type TX_CRC_DATA_EN_ADDR_CH = crate::Reg<tx_crc_data_en_addr_ch::TX_CRC_DATA_EN_ADDR_CH_SPEC>;
#[doc = "This register is used to config addr of crc data_8bit en"]
pub mod tx_crc_data_en_addr_ch;
#[doc = "TX_CH_ARB_WEIGHT_CH0 (rw) register accessor: TX channel 0 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weight_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weight_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ch_arb_weight_ch0`] module"]
pub type TX_CH_ARB_WEIGHT_CH0 = crate::Reg<tx_ch_arb_weight_ch0::TX_CH_ARB_WEIGHT_CH0_SPEC>;
#[doc = "TX channel 0 arbitration weight configuration register"]
pub mod tx_ch_arb_weight_ch0;
#[doc = "TX_ARB_WEIGHT_OPT_DIR_CH0 (rw) register accessor: TX channel 0 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weight_opt_dir_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weight_opt_dir_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_arb_weight_opt_dir_ch0`] module"]
pub type TX_ARB_WEIGHT_OPT_DIR_CH0 =
    crate::Reg<tx_arb_weight_opt_dir_ch0::TX_ARB_WEIGHT_OPT_DIR_CH0_SPEC>;
#[doc = "TX channel 0 weight arbitration optimization enable register"]
pub mod tx_arb_weight_opt_dir_ch0;
#[doc = "TX_CH_ARB_WEIGHT_CH1 (rw) register accessor: TX channel 1 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weight_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weight_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ch_arb_weight_ch1`] module"]
pub type TX_CH_ARB_WEIGHT_CH1 = crate::Reg<tx_ch_arb_weight_ch1::TX_CH_ARB_WEIGHT_CH1_SPEC>;
#[doc = "TX channel 1 arbitration weight configuration register"]
pub mod tx_ch_arb_weight_ch1;
#[doc = "TX_ARB_WEIGHT_OPT_DIR_CH1 (rw) register accessor: TX channel 1 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weight_opt_dir_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weight_opt_dir_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_arb_weight_opt_dir_ch1`] module"]
pub type TX_ARB_WEIGHT_OPT_DIR_CH1 =
    crate::Reg<tx_arb_weight_opt_dir_ch1::TX_ARB_WEIGHT_OPT_DIR_CH1_SPEC>;
#[doc = "TX channel 1 weight arbitration optimization enable register"]
pub mod tx_arb_weight_opt_dir_ch1;
#[doc = "TX_CH_ARB_WEIGHT_CH2 (rw) register accessor: TX channel 2 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weight_ch2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weight_ch2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ch_arb_weight_ch2`] module"]
pub type TX_CH_ARB_WEIGHT_CH2 = crate::Reg<tx_ch_arb_weight_ch2::TX_CH_ARB_WEIGHT_CH2_SPEC>;
#[doc = "TX channel 2 arbitration weight configuration register"]
pub mod tx_ch_arb_weight_ch2;
#[doc = "TX_ARB_WEIGHT_OPT_DIR_CH2 (rw) register accessor: TX channel 2 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weight_opt_dir_ch2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weight_opt_dir_ch2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_arb_weight_opt_dir_ch2`] module"]
pub type TX_ARB_WEIGHT_OPT_DIR_CH2 =
    crate::Reg<tx_arb_weight_opt_dir_ch2::TX_ARB_WEIGHT_OPT_DIR_CH2_SPEC>;
#[doc = "TX channel 2 weight arbitration optimization enable register"]
pub mod tx_arb_weight_opt_dir_ch2;
#[doc = "IN_CRC_INIT_DATA_CH (rw) register accessor: This register is used to config ch%s crc initial data(max 32 bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_init_data_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_crc_init_data_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_init_data_ch`] module"]
pub type IN_CRC_INIT_DATA_CH = crate::Reg<in_crc_init_data_ch::IN_CRC_INIT_DATA_CH_SPEC>;
#[doc = "This register is used to config ch%s crc initial data(max 32 bit)"]
pub mod in_crc_init_data_ch;
#[doc = "RX_CRC_WIDTH_CH (rw) register accessor: This register is used to confiig rx ch%s crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_width_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_width_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_width_ch`] module"]
pub type RX_CRC_WIDTH_CH = crate::Reg<rx_crc_width_ch::RX_CRC_WIDTH_CH_SPEC>;
#[doc = "This register is used to confiig rx ch%s crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32"]
pub mod rx_crc_width_ch;
#[doc = "IN_CRC_CLEAR_CH (rw) register accessor: This register is used to clear ch%s crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_clear_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_crc_clear_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_clear_ch`] module"]
pub type IN_CRC_CLEAR_CH = crate::Reg<in_crc_clear_ch::IN_CRC_CLEAR_CH_SPEC>;
#[doc = "This register is used to clear ch%s crc result"]
pub mod in_crc_clear_ch;
#[doc = "IN_CRC_FINAL_RESULT_CH (r) register accessor: This register is used to store ch%s crc result\n\nYou can [`read`](crate::Reg::read) this register and get [`in_crc_final_result_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_final_result_ch`] module"]
pub type IN_CRC_FINAL_RESULT_CH = crate::Reg<in_crc_final_result_ch::IN_CRC_FINAL_RESULT_CH_SPEC>;
#[doc = "This register is used to store ch%s crc result"]
pub mod in_crc_final_result_ch;
#[doc = "RX_CRC_EN_WR_DATA_CH (rw) register accessor: This resister is used to config ch%s crc en for every bit\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_en_wr_data_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_en_wr_data_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_en_wr_data_ch`] module"]
pub type RX_CRC_EN_WR_DATA_CH = crate::Reg<rx_crc_en_wr_data_ch::RX_CRC_EN_WR_DATA_CH_SPEC>;
#[doc = "This resister is used to config ch%s crc en for every bit"]
pub mod rx_crc_en_wr_data_ch;
#[doc = "RX_CRC_EN_ADDR_CH (rw) register accessor: This register is used to config ch%s crc en addr\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_en_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_en_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_en_addr_ch`] module"]
pub type RX_CRC_EN_ADDR_CH = crate::Reg<rx_crc_en_addr_ch::RX_CRC_EN_ADDR_CH_SPEC>;
#[doc = "This register is used to config ch%s crc en addr"]
pub mod rx_crc_en_addr_ch;
#[doc = "RX_CRC_DATA_EN_WR_DATA_CH (rw) register accessor: This register is used to config crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_data_en_wr_data_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_data_en_wr_data_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_data_en_wr_data_ch`] module"]
pub type RX_CRC_DATA_EN_WR_DATA_CH =
    crate::Reg<rx_crc_data_en_wr_data_ch::RX_CRC_DATA_EN_WR_DATA_CH_SPEC>;
#[doc = "This register is used to config crc data_8bit en"]
pub mod rx_crc_data_en_wr_data_ch;
#[doc = "RX_CRC_DATA_EN_ADDR_CH (rw) register accessor: This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_data_en_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_data_en_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_data_en_addr_ch`] module"]
pub type RX_CRC_DATA_EN_ADDR_CH = crate::Reg<rx_crc_data_en_addr_ch::RX_CRC_DATA_EN_ADDR_CH_SPEC>;
#[doc = "This register is used to config addr of crc data_8bit en"]
pub mod rx_crc_data_en_addr_ch;
#[doc = "RX_CH_ARB_WEIGHT_CH0 (rw) register accessor: RX channel 0 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weight_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weight_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch_arb_weight_ch0`] module"]
pub type RX_CH_ARB_WEIGHT_CH0 = crate::Reg<rx_ch_arb_weight_ch0::RX_CH_ARB_WEIGHT_CH0_SPEC>;
#[doc = "RX channel 0 arbitration weight configuration register"]
pub mod rx_ch_arb_weight_ch0;
#[doc = "RX_ARB_WEIGHT_OPT_DIR_CH0 (rw) register accessor: RX channel 0 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weight_opt_dir_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weight_opt_dir_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_arb_weight_opt_dir_ch0`] module"]
pub type RX_ARB_WEIGHT_OPT_DIR_CH0 =
    crate::Reg<rx_arb_weight_opt_dir_ch0::RX_ARB_WEIGHT_OPT_DIR_CH0_SPEC>;
#[doc = "RX channel 0 weight arbitration optimization enable register"]
pub mod rx_arb_weight_opt_dir_ch0;
#[doc = "RX_CH_ARB_WEIGHT_CH1 (rw) register accessor: RX channel 1 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weight_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weight_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch_arb_weight_ch1`] module"]
pub type RX_CH_ARB_WEIGHT_CH1 = crate::Reg<rx_ch_arb_weight_ch1::RX_CH_ARB_WEIGHT_CH1_SPEC>;
#[doc = "RX channel 1 arbitration weight configuration register"]
pub mod rx_ch_arb_weight_ch1;
#[doc = "RX_ARB_WEIGHT_OPT_DIR_CH1 (rw) register accessor: RX channel 1 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weight_opt_dir_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weight_opt_dir_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_arb_weight_opt_dir_ch1`] module"]
pub type RX_ARB_WEIGHT_OPT_DIR_CH1 =
    crate::Reg<rx_arb_weight_opt_dir_ch1::RX_ARB_WEIGHT_OPT_DIR_CH1_SPEC>;
#[doc = "RX channel 1 weight arbitration optimization enable register"]
pub mod rx_arb_weight_opt_dir_ch1;
#[doc = "RX_CH_ARB_WEIGHT_CH2 (rw) register accessor: RX channel 2 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weight_ch2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weight_ch2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch_arb_weight_ch2`] module"]
pub type RX_CH_ARB_WEIGHT_CH2 = crate::Reg<rx_ch_arb_weight_ch2::RX_CH_ARB_WEIGHT_CH2_SPEC>;
#[doc = "RX channel 2 arbitration weight configuration register"]
pub mod rx_ch_arb_weight_ch2;
#[doc = "RX_ARB_WEIGHT_OPT_DIR_CH2 (rw) register accessor: RX channel 2 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weight_opt_dir_ch2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weight_opt_dir_ch2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_arb_weight_opt_dir_ch2`] module"]
pub type RX_ARB_WEIGHT_OPT_DIR_CH2 =
    crate::Reg<rx_arb_weight_opt_dir_ch2::RX_ARB_WEIGHT_OPT_DIR_CH2_SPEC>;
#[doc = "RX channel 2 weight arbitration optimization enable register"]
pub mod rx_arb_weight_opt_dir_ch2;
#[doc = "Cluster IN_LINK_ADDR_CH%s, containing IN_LINK_ADDR_CH?"]
pub use self::in_link_addr_ch::IN_LINK_ADDR_CH;
#[doc = r"Cluster"]
#[doc = "Cluster IN_LINK_ADDR_CH%s, containing IN_LINK_ADDR_CH?"]
pub mod in_link_addr_ch;
#[doc = "Cluster OUT_LINK_ADDR_CH%s, containing OUT_LINK_ADDR_CH?"]
pub use self::out_link_addr_ch::OUT_LINK_ADDR_CH;
#[doc = r"Cluster"]
#[doc = "Cluster OUT_LINK_ADDR_CH%s, containing OUT_LINK_ADDR_CH?"]
pub mod out_link_addr_ch;
#[doc = "INTR_MEM_START_ADDR (rw) register accessor: Accessible address space start address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_start_addr`] module"]
pub type INTR_MEM_START_ADDR = crate::Reg<intr_mem_start_addr::INTR_MEM_START_ADDR_SPEC>;
#[doc = "Accessible address space start address configuration register"]
pub mod intr_mem_start_addr;
#[doc = "INTR_MEM_END_ADDR (rw) register accessor: Accessible address space end address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_end_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_end_addr`] module"]
pub type INTR_MEM_END_ADDR = crate::Reg<intr_mem_end_addr::INTR_MEM_END_ADDR_SPEC>;
#[doc = "Accessible address space end address configuration register"]
pub mod intr_mem_end_addr;
#[doc = "ARB_TIMEOUT_TX (rw) register accessor: TX arbitration timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout_tx`] module"]
pub type ARB_TIMEOUT_TX = crate::Reg<arb_timeout_tx::ARB_TIMEOUT_TX_SPEC>;
#[doc = "TX arbitration timeout configuration register"]
pub mod arb_timeout_tx;
#[doc = "ARB_TIMEOUT_RX (rw) register accessor: RX arbitration timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout_rx`] module"]
pub type ARB_TIMEOUT_RX = crate::Reg<arb_timeout_rx::ARB_TIMEOUT_RX_SPEC>;
#[doc = "RX arbitration timeout configuration register"]
pub mod arb_timeout_rx;
#[doc = "WEIGHT_EN_TX (rw) register accessor: TX weight arbitration enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en_tx`] module"]
pub type WEIGHT_EN_TX = crate::Reg<weight_en_tx::WEIGHT_EN_TX_SPEC>;
#[doc = "TX weight arbitration enable register"]
pub mod weight_en_tx;
#[doc = "WEIGHT_EN_RX (rw) register accessor: RX weight arbitration enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en_rx`] module"]
pub type WEIGHT_EN_RX = crate::Reg<weight_en_rx::WEIGHT_EN_RX_SPEC>;
#[doc = "RX weight arbitration enable register"]
pub mod weight_en_rx;
#[doc = "ARB_TIMEOUT (rw) register accessor: TX arbitration timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout`] module"]
pub type ARB_TIMEOUT = crate::Reg<arb_timeout::ARB_TIMEOUT_SPEC>;
#[doc = "TX arbitration timeout configuration register"]
pub mod arb_timeout;
#[doc = "WEIGHT_EN (rw) register accessor: TX weight arbitration enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en`] module"]
pub type WEIGHT_EN = crate::Reg<weight_en::WEIGHT_EN_SPEC>;
#[doc = "TX weight arbitration enable register"]
pub mod weight_en;
#[doc = "MODULE_CLK_EN (rw) register accessor: Module clock force on register\n\nYou can [`read`](crate::Reg::read) this register and get [`module_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`module_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@module_clk_en`] module"]
pub type MODULE_CLK_EN = crate::Reg<module_clk_en::MODULE_CLK_EN_SPEC>;
#[doc = "Module clock force on register"]
pub mod module_clk_en;
#[doc = "AHBINF_RESP_ERR_STATUS0 (r) register accessor: AHB response error status 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbinf_resp_err_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbinf_resp_err_status0`] module"]
pub type AHBINF_RESP_ERR_STATUS0 =
    crate::Reg<ahbinf_resp_err_status0::AHBINF_RESP_ERR_STATUS0_SPEC>;
#[doc = "AHB response error status 0 register"]
pub mod ahbinf_resp_err_status0;
#[doc = "AHBINF_RESP_ERR_STATUS1 (r) register accessor: AHB response error status 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbinf_resp_err_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbinf_resp_err_status1`] module"]
pub type AHBINF_RESP_ERR_STATUS1 =
    crate::Reg<ahbinf_resp_err_status1::AHBINF_RESP_ERR_STATUS1_SPEC>;
#[doc = "AHB response error status 1 register"]
pub mod ahbinf_resp_err_status1;
#[doc = "IN_DONE_DES_ADDR_CH0 (r) register accessor: RX_done Inlink descriptor address of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_done_des_addr_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_done_des_addr_ch0`] module"]
pub type IN_DONE_DES_ADDR_CH0 = crate::Reg<in_done_des_addr_ch0::IN_DONE_DES_ADDR_CH0_SPEC>;
#[doc = "RX_done Inlink descriptor address of RX channel 0"]
pub mod in_done_des_addr_ch0;
#[doc = "OUT_DONE_DES_ADDR_CH0 (r) register accessor: TX done outlink descriptor address of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_done_des_addr_ch0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_done_des_addr_ch0`] module"]
pub type OUT_DONE_DES_ADDR_CH0 = crate::Reg<out_done_des_addr_ch0::OUT_DONE_DES_ADDR_CH0_SPEC>;
#[doc = "TX done outlink descriptor address of TX channel 0"]
pub mod out_done_des_addr_ch0;
#[doc = "IN_DONE_DES_ADDR_CH1 (r) register accessor: RX_done Inlink descriptor address of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_done_des_addr_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_done_des_addr_ch1`] module"]
pub type IN_DONE_DES_ADDR_CH1 = crate::Reg<in_done_des_addr_ch1::IN_DONE_DES_ADDR_CH1_SPEC>;
#[doc = "RX_done Inlink descriptor address of RX channel 1"]
pub mod in_done_des_addr_ch1;
#[doc = "OUT_DONE_DES_ADDR_CH1 (r) register accessor: TX done outlink descriptor address of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_done_des_addr_ch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_done_des_addr_ch1`] module"]
pub type OUT_DONE_DES_ADDR_CH1 = crate::Reg<out_done_des_addr_ch1::OUT_DONE_DES_ADDR_CH1_SPEC>;
#[doc = "TX done outlink descriptor address of TX channel 1"]
pub mod out_done_des_addr_ch1;
#[doc = "IN_DONE_DES_ADDR_CH2 (r) register accessor: RX_done Inlink descriptor address of RX channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`in_done_des_addr_ch2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_done_des_addr_ch2`] module"]
pub type IN_DONE_DES_ADDR_CH2 = crate::Reg<in_done_des_addr_ch2::IN_DONE_DES_ADDR_CH2_SPEC>;
#[doc = "RX_done Inlink descriptor address of RX channel 2"]
pub mod in_done_des_addr_ch2;
#[doc = "OUT_DONE_DES_ADDR_CH2 (r) register accessor: TX done outlink descriptor address of TX channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`out_done_des_addr_ch2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_done_des_addr_ch2`] module"]
pub type OUT_DONE_DES_ADDR_CH2 = crate::Reg<out_done_des_addr_ch2::OUT_DONE_DES_ADDR_CH2_SPEC>;
#[doc = "TX done outlink descriptor address of TX channel 2"]
pub mod out_done_des_addr_ch2;
