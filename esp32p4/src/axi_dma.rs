#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    in_int_raw_ch: (),
    _reserved1: [u8; 0x04],
    in_int_st_ch: (),
    _reserved2: [u8; 0x04],
    in_int_ena_ch: (),
    _reserved3: [u8; 0x04],
    in_int_clr_ch: (),
    _reserved4: [u8; 0x04],
    in_conf0_ch: (),
    _reserved5: [u8; 0x04],
    in_conf1_ch: (),
    _reserved6: [u8; 0x04],
    infifo_status_ch: (),
    _reserved7: [u8; 0x04],
    in_pop_ch: (),
    _reserved8: [u8; 0x04],
    in_link1_ch: (),
    _reserved9: [u8; 0x04],
    in_link2_ch: (),
    _reserved10: [u8; 0x04],
    in_state_ch: (),
    _reserved11: [u8; 0x04],
    in_suc_eof_des_addr_ch: (),
    _reserved12: [u8; 0x04],
    in_err_eof_des_addr_ch: (),
    _reserved13: [u8; 0x04],
    in_dscr_ch: (),
    _reserved14: [u8; 0x04],
    in_dscr_bf0_ch: (),
    _reserved15: [u8; 0x04],
    in_dscr_bf1_ch: (),
    _reserved16: [u8; 0x04],
    in_pri_ch: (),
    _reserved17: [u8; 0x04],
    in_peri_sel_ch: (),
    _reserved18: [u8; 0x04],
    in_crc_init_data_ch: (),
    _reserved19: [u8; 0x04],
    rx_crc_width_ch: (),
    _reserved20: [u8; 0x04],
    in_crc_clear_ch: (),
    _reserved21: [u8; 0x04],
    in_crc_final_result_ch: (),
    _reserved22: [u8; 0x04],
    rx_crc_en_wr_data_ch: (),
    _reserved23: [u8; 0x04],
    rx_crc_en_addr_ch: (),
    _reserved24: [u8; 0x04],
    rx_crc_data_en_wr_data_ch: (),
    _reserved25: [u8; 0x04],
    rx_crc_data_en_addr_ch: (),
    _reserved26: [u8; 0xd4],
    out_int_raw_ch: (),
    _reserved27: [u8; 0x04],
    out_int_st_ch: (),
    _reserved28: [u8; 0x04],
    out_int_ena_ch: (),
    _reserved29: [u8; 0x04],
    out_int_clr_ch: (),
    _reserved30: [u8; 0x04],
    out_conf0_ch0: OUT_CONF0_CH0,
    out_conf1_ch: (),
    _reserved32: [u8; 0x04],
    outfifo_status_ch: (),
    _reserved33: [u8; 0x04],
    out_push_ch: (),
    _reserved34: [u8; 0x04],
    out_link1_ch: (),
    _reserved35: [u8; 0x04],
    out_link2_ch: (),
    _reserved36: [u8; 0x04],
    out_state_ch: (),
    _reserved37: [u8; 0x04],
    out_eof_des_addr_ch: (),
    _reserved38: [u8; 0x04],
    out_eof_bfr_des_addr_ch: (),
    _reserved39: [u8; 0x04],
    out_dscr_ch: (),
    _reserved40: [u8; 0x04],
    out_dscr_bf0_ch: (),
    _reserved41: [u8; 0x04],
    out_dscr_bf1_ch: (),
    _reserved42: [u8; 0x04],
    out_pri_ch: (),
    _reserved43: [u8; 0x04],
    out_peri_sel_ch: (),
    _reserved44: [u8; 0x04],
    out_crc_init_data_ch: (),
    _reserved45: [u8; 0x04],
    tx_crc_width_ch: (),
    _reserved46: [u8; 0x04],
    out_crc_clear_ch: (),
    _reserved47: [u8; 0x04],
    out_crc_final_result_ch: (),
    _reserved48: [u8; 0x04],
    tx_crc_en_wr_data_ch: (),
    _reserved49: [u8; 0x04],
    tx_crc_en_addr_ch: (),
    _reserved50: [u8; 0x04],
    tx_crc_data_en_wr_data_ch: (),
    _reserved51: [u8; 0x04],
    tx_crc_data_en_addr_ch: (),
    _reserved52: [u8; 0x14],
    out_conf0_ch1: OUT_CONF0_CH1,
    _reserved53: [u8; 0x64],
    out_conf0_ch2: OUT_CONF0_CH2,
    _reserved54: [u8; 0x54],
    arb_timeout: ARB_TIMEOUT,
    weight_en: WEIGHT_EN,
    in_mem_conf: IN_MEM_CONF,
    intr_mem_start_addr: INTR_MEM_START_ADDR,
    intr_mem_end_addr: INTR_MEM_END_ADDR,
    extr_mem_start_addr: EXTR_MEM_START_ADDR,
    extr_mem_end_addr: EXTR_MEM_END_ADDR,
    in_reset_avail_ch: [IN_RESET_AVAIL_CH; 3],
    out_reset_avail_ch: [OUT_RESET_AVAIL_CH; 3],
    _reserved63: [u8; 0x04],
    misc_conf: MISC_CONF,
    rdn_result: RDN_RESULT,
    rdn_eco_high: RDN_ECO_HIGH,
    rdn_eco_low: RDN_ECO_LOW,
    wresp_cnt: WRESP_CNT,
    rresp_cnt: RRESP_CNT,
    infifo_status1_ch: [INFIFO_STATUS1_CH; 3],
    outfifo_status1_ch: [OUTFIFO_STATUS1_CH; 3],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x0c - Raw status interrupt of channel 0"]
    #[inline(always)]
    pub const fn in_int_raw_ch(&self, n: usize) -> &IN_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x04..0x10 - Masked interrupt of channel 0"]
    #[inline(always)]
    pub const fn in_int_st_ch(&self, n: usize) -> &IN_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x08..0x14 - Interrupt enable bits of channel 0"]
    #[inline(always)]
    pub const fn in_int_ena_ch(&self, n: usize) -> &IN_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(8)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x0c..0x18 - Interrupt clear bits of channel 0"]
    #[inline(always)]
    pub const fn in_int_clr_ch(&self, n: usize) -> &IN_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(12)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x10..0x1c - Configure 0 register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_conf0_ch(&self, n: usize) -> &IN_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x14..0x20 - Configure 1 register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_conf1_ch(&self, n: usize) -> &IN_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(20)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x18..0x24 - Receive FIFO status of Rx channel 0"]
    #[inline(always)]
    pub const fn infifo_status_ch(&self, n: usize) -> &INFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(24)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x1c..0x28 - Pop control register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_pop_ch(&self, n: usize) -> &IN_POP_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(28)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x20..0x2c - Link descriptor configure and control register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_link1_ch(&self, n: usize) -> &IN_LINK1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x24..0x30 - Link descriptor configure and control register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_link2_ch(&self, n: usize) -> &IN_LINK2_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(36)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x28..0x34 - Receive status of Rx channel 0"]
    #[inline(always)]
    pub const fn in_state_ch(&self, n: usize) -> &IN_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(40)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x2c..0x38 - Inlink descriptor address when EOF occurs of Rx channel 0"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch(&self, n: usize) -> &IN_SUC_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(44)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x30..0x3c - Inlink descriptor address when errors occur of Rx channel 0"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch(&self, n: usize) -> &IN_ERR_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(48)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x34..0x40 - Current inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr_ch(&self, n: usize) -> &IN_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(52)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x38..0x44 - The last inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch(&self, n: usize) -> &IN_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(56)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x3c..0x48 - The second-to-last inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch(&self, n: usize) -> &IN_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(60)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x40..0x4c - Priority register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_pri_ch(&self, n: usize) -> &IN_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(64)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x44..0x50 - Peripheral selection of Rx channel 0"]
    #[inline(always)]
    pub const fn in_peri_sel_ch(&self, n: usize) -> &IN_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(68)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x48..0x54 - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn in_crc_init_data_ch(&self, n: usize) -> &IN_CRC_INIT_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(72)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x4c..0x58 - This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
    #[inline(always)]
    pub const fn rx_crc_width_ch(&self, n: usize) -> &RX_CRC_WIDTH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(76)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x50..0x5c - This register is used to clear ch0 crc result"]
    #[inline(always)]
    pub const fn in_crc_clear_ch(&self, n: usize) -> &IN_CRC_CLEAR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(80)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x54..0x60 - This register is used to store ch0 crc result"]
    #[inline(always)]
    pub const fn in_crc_final_result_ch(&self, n: usize) -> &IN_CRC_FINAL_RESULT_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(84)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x58..0x64 - This resister is used to config ch0 crc en for every bit"]
    #[inline(always)]
    pub const fn rx_crc_en_wr_data_ch(&self, n: usize) -> &RX_CRC_EN_WR_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(88)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x5c..0x68 - This register is used to config ch0 crc en addr"]
    #[inline(always)]
    pub const fn rx_crc_en_addr_ch(&self, n: usize) -> &RX_CRC_EN_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(92)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x60..0x6c - This register is used to config crc data_8bit en"]
    #[inline(always)]
    pub const fn rx_crc_data_en_wr_data_ch(&self, n: usize) -> &RX_CRC_DATA_EN_WR_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(96)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x64..0x70 - This register is used to config addr of crc data_8bit en"]
    #[inline(always)]
    pub const fn rx_crc_data_en_addr_ch(&self, n: usize) -> &RX_CRC_DATA_EN_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(100)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x138..0x144 - Raw status interrupt of channel0"]
    #[inline(always)]
    pub const fn out_int_raw_ch(&self, n: usize) -> &OUT_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(312)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x13c..0x148 - Masked interrupt of channel0"]
    #[inline(always)]
    pub const fn out_int_st_ch(&self, n: usize) -> &OUT_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(316)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x140..0x14c - Interrupt enable bits of channel0"]
    #[inline(always)]
    pub const fn out_int_ena_ch(&self, n: usize) -> &OUT_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(320)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x144..0x150 - Interrupt clear bits of channel0"]
    #[inline(always)]
    pub const fn out_int_clr_ch(&self, n: usize) -> &OUT_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(324)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x148 - Configure 0 register of Tx channel0"]
    #[inline(always)]
    pub const fn out_conf0_ch0(&self) -> &OUT_CONF0_CH0 {
        &self.out_conf0_ch0
    }
    #[doc = "0x14c..0x158 - Configure 1 register of Tx channel0"]
    #[inline(always)]
    pub const fn out_conf1_ch(&self, n: usize) -> &OUT_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(332)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x150..0x15c - Transmit FIFO status of Tx channel0"]
    #[inline(always)]
    pub const fn outfifo_status_ch(&self, n: usize) -> &OUTFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(336)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x154..0x160 - Push control register of Tx channel0"]
    #[inline(always)]
    pub const fn out_push_ch(&self, n: usize) -> &OUT_PUSH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(340)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x158..0x164 - Link descriptor configure and control register of Tx channel0"]
    #[inline(always)]
    pub const fn out_link1_ch(&self, n: usize) -> &OUT_LINK1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(344)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x15c..0x168 - Link descriptor configure and control register of Tx channel0"]
    #[inline(always)]
    pub const fn out_link2_ch(&self, n: usize) -> &OUT_LINK2_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(348)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x160..0x16c - Transmit status of Tx channel0"]
    #[inline(always)]
    pub const fn out_state_ch(&self, n: usize) -> &OUT_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(352)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x164..0x170 - Outlink descriptor address when EOF occurs of Tx channel0"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch(&self, n: usize) -> &OUT_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(356)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x168..0x174 - The last outlink descriptor address when EOF occurs of Tx channel0"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch(&self, n: usize) -> &OUT_EOF_BFR_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(360)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x16c..0x178 - Current outlink descriptor address of Tx channel0"]
    #[inline(always)]
    pub const fn out_dscr_ch(&self, n: usize) -> &OUT_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(364)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x170..0x17c - The last outlink descriptor address of Tx channel0"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch(&self, n: usize) -> &OUT_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(368)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x174..0x180 - The second-to-last outlink descriptor address of Tx channel0"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch(&self, n: usize) -> &OUT_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(372)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x178..0x184 - Priority register of Tx channel0."]
    #[inline(always)]
    pub const fn out_pri_ch(&self, n: usize) -> &OUT_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(376)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x17c..0x188 - Peripheral selection of Tx channel0"]
    #[inline(always)]
    pub const fn out_peri_sel_ch(&self, n: usize) -> &OUT_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(380)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x180..0x18c - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn out_crc_init_data_ch(&self, n: usize) -> &OUT_CRC_INIT_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(384)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x184..0x190 - This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
    #[inline(always)]
    pub const fn tx_crc_width_ch(&self, n: usize) -> &TX_CRC_WIDTH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(388)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x188..0x194 - This register is used to clear ch0 crc result"]
    #[inline(always)]
    pub const fn out_crc_clear_ch(&self, n: usize) -> &OUT_CRC_CLEAR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(392)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x18c..0x198 - This register is used to store ch0 crc result"]
    #[inline(always)]
    pub const fn out_crc_final_result_ch(&self, n: usize) -> &OUT_CRC_FINAL_RESULT_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(396)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x190..0x19c - This resister is used to config ch0 crc en for every bit"]
    #[inline(always)]
    pub const fn tx_crc_en_wr_data_ch(&self, n: usize) -> &TX_CRC_EN_WR_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(400)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x194..0x1a0 - This register is used to config ch0 crc en addr"]
    #[inline(always)]
    pub const fn tx_crc_en_addr_ch(&self, n: usize) -> &TX_CRC_EN_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(404)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x198..0x1a4 - This register is used to config crc data_8bit en"]
    #[inline(always)]
    pub const fn tx_crc_data_en_wr_data_ch(&self, n: usize) -> &TX_CRC_DATA_EN_WR_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(408)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x19c..0x1a8 - This register is used to config addr of crc data_8bit en"]
    #[inline(always)]
    pub const fn tx_crc_data_en_addr_ch(&self, n: usize) -> &TX_CRC_DATA_EN_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(412)
                .add(104 * n)
                .cast()
        }
    }
    #[doc = "0x1b0 - Configure 0 register of Tx channel1"]
    #[inline(always)]
    pub const fn out_conf0_ch1(&self) -> &OUT_CONF0_CH1 {
        &self.out_conf0_ch1
    }
    #[doc = "0x218 - Configure 0 register of Tx channel2"]
    #[inline(always)]
    pub const fn out_conf0_ch2(&self) -> &OUT_CONF0_CH2 {
        &self.out_conf0_ch2
    }
    #[doc = "0x270 - This retister is used to config arbiter time slice"]
    #[inline(always)]
    pub const fn arb_timeout(&self) -> &ARB_TIMEOUT {
        &self.arb_timeout
    }
    #[doc = "0x274 - This register is used to config arbiter weight function to on or off"]
    #[inline(always)]
    pub const fn weight_en(&self) -> &WEIGHT_EN {
        &self.weight_en
    }
    #[doc = "0x278 - Mem power configure register of Rx channel"]
    #[inline(always)]
    pub const fn in_mem_conf(&self) -> &IN_MEM_CONF {
        &self.in_mem_conf
    }
    #[doc = "0x27c - The start address of accessible address space."]
    #[inline(always)]
    pub const fn intr_mem_start_addr(&self) -> &INTR_MEM_START_ADDR {
        &self.intr_mem_start_addr
    }
    #[doc = "0x280 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub const fn intr_mem_end_addr(&self) -> &INTR_MEM_END_ADDR {
        &self.intr_mem_end_addr
    }
    #[doc = "0x284 - The start address of accessible address space."]
    #[inline(always)]
    pub const fn extr_mem_start_addr(&self) -> &EXTR_MEM_START_ADDR {
        &self.extr_mem_start_addr
    }
    #[doc = "0x288 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub const fn extr_mem_end_addr(&self) -> &EXTR_MEM_END_ADDR {
        &self.extr_mem_end_addr
    }
    #[doc = "0x28c..0x298 - The rx channel 0 reset valid_flag register."]
    #[inline(always)]
    pub const fn in_reset_avail_ch(&self, n: usize) -> &IN_RESET_AVAIL_CH {
        &self.in_reset_avail_ch[n]
    }
    #[doc = "0x298..0x2a4 - The tx channel 0 reset valid_flag register."]
    #[inline(always)]
    pub const fn out_reset_avail_ch(&self, n: usize) -> &OUT_RESET_AVAIL_CH {
        &self.out_reset_avail_ch[n]
    }
    #[doc = "0x2a8 - MISC register"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0x2ac - reserved"]
    #[inline(always)]
    pub const fn rdn_result(&self) -> &RDN_RESULT {
        &self.rdn_result
    }
    #[doc = "0x2b0 - reserved"]
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> &RDN_ECO_HIGH {
        &self.rdn_eco_high
    }
    #[doc = "0x2b4 - reserved"]
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> &RDN_ECO_LOW {
        &self.rdn_eco_low
    }
    #[doc = "0x2b8 - AXI wr responce cnt register."]
    #[inline(always)]
    pub const fn wresp_cnt(&self) -> &WRESP_CNT {
        &self.wresp_cnt
    }
    #[doc = "0x2bc - AXI wr responce cnt register."]
    #[inline(always)]
    pub const fn rresp_cnt(&self) -> &RRESP_CNT {
        &self.rresp_cnt
    }
    #[doc = "0x2c0..0x2cc - Receive FIFO status of Rx channel 0"]
    #[inline(always)]
    pub const fn infifo_status1_ch(&self, n: usize) -> &INFIFO_STATUS1_CH {
        &self.infifo_status1_ch[n]
    }
    #[doc = "0x2cc..0x2d8 - Receive FIFO status of Tx channel 0"]
    #[inline(always)]
    pub const fn outfifo_status1_ch(&self, n: usize) -> &OUTFIFO_STATUS1_CH {
        &self.outfifo_status1_ch[n]
    }
    #[doc = "0x2d8 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "IN_INT_RAW_CH (rw) register accessor: Raw status interrupt of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_raw_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_raw_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch`] module"]
pub type IN_INT_RAW_CH = crate::Reg<in_int_raw_ch::IN_INT_RAW_CH_SPEC>;
#[doc = "Raw status interrupt of channel 0"]
pub mod in_int_raw_ch;
#[doc = "IN_INT_ST_CH (r) register accessor: Masked interrupt of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_st_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch`] module"]
pub type IN_INT_ST_CH = crate::Reg<in_int_st_ch::IN_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt of channel 0"]
pub mod in_int_st_ch;
#[doc = "IN_INT_ENA_CH (rw) register accessor: Interrupt enable bits of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch`] module"]
pub type IN_INT_ENA_CH = crate::Reg<in_int_ena_ch::IN_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of channel 0"]
pub mod in_int_ena_ch;
#[doc = "IN_INT_CLR_CH (w) register accessor: Interrupt clear bits of channel 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch`] module"]
pub type IN_INT_CLR_CH = crate::Reg<in_int_clr_ch::IN_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of channel 0"]
pub mod in_int_clr_ch;
#[doc = "IN_CONF0_CH (rw) register accessor: Configure 0 register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch`] module"]
pub type IN_CONF0_CH = crate::Reg<in_conf0_ch::IN_CONF0_CH_SPEC>;
#[doc = "Configure 0 register of Rx channel 0"]
pub mod in_conf0_ch;
#[doc = "IN_CONF1_CH (rw) register accessor: Configure 1 register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf1_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf1_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1_ch`] module"]
pub type IN_CONF1_CH = crate::Reg<in_conf1_ch::IN_CONF1_CH_SPEC>;
#[doc = "Configure 1 register of Rx channel 0"]
pub mod in_conf1_ch;
#[doc = "INFIFO_STATUS_CH (r) register accessor: Receive FIFO status of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch`] module"]
pub type INFIFO_STATUS_CH = crate::Reg<infifo_status_ch::INFIFO_STATUS_CH_SPEC>;
#[doc = "Receive FIFO status of Rx channel 0"]
pub mod infifo_status_ch;
#[doc = "IN_POP_CH (rw) register accessor: Pop control register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pop_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pop_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch`] module"]
pub type IN_POP_CH = crate::Reg<in_pop_ch::IN_POP_CH_SPEC>;
#[doc = "Pop control register of Rx channel 0"]
pub mod in_pop_ch;
#[doc = "IN_LINK1_CH (rw) register accessor: Link descriptor configure and control register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link1_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link1_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link1_ch`] module"]
pub type IN_LINK1_CH = crate::Reg<in_link1_ch::IN_LINK1_CH_SPEC>;
#[doc = "Link descriptor configure and control register of Rx channel 0"]
pub mod in_link1_ch;
#[doc = "IN_LINK2_CH (rw) register accessor: Link descriptor configure and control register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link2_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link2_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link2_ch`] module"]
pub type IN_LINK2_CH = crate::Reg<in_link2_ch::IN_LINK2_CH_SPEC>;
#[doc = "Link descriptor configure and control register of Rx channel 0"]
pub mod in_link2_ch;
#[doc = "IN_STATE_CH (r) register accessor: Receive status of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_state_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch`] module"]
pub type IN_STATE_CH = crate::Reg<in_state_ch::IN_STATE_CH_SPEC>;
#[doc = "Receive status of Rx channel 0"]
pub mod in_state_ch;
#[doc = "IN_SUC_EOF_DES_ADDR_CH (r) register accessor: Inlink descriptor address when EOF occurs of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_suc_eof_des_addr_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH = crate::Reg<in_suc_eof_des_addr_ch::IN_SUC_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Inlink descriptor address when EOF occurs of Rx channel 0"]
pub mod in_suc_eof_des_addr_ch;
#[doc = "IN_ERR_EOF_DES_ADDR_CH (r) register accessor: Inlink descriptor address when errors occur of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH = crate::Reg<in_err_eof_des_addr_ch::IN_ERR_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Inlink descriptor address when errors occur of Rx channel 0"]
pub mod in_err_eof_des_addr_ch;
#[doc = "IN_DSCR_CH (r) register accessor: Current inlink descriptor address of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch`] module"]
pub type IN_DSCR_CH = crate::Reg<in_dscr_ch::IN_DSCR_CH_SPEC>;
#[doc = "Current inlink descriptor address of Rx channel 0"]
pub mod in_dscr_ch;
#[doc = "IN_DSCR_BF0_CH (r) register accessor: The last inlink descriptor address of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf0_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch`] module"]
pub type IN_DSCR_BF0_CH = crate::Reg<in_dscr_bf0_ch::IN_DSCR_BF0_CH_SPEC>;
#[doc = "The last inlink descriptor address of Rx channel 0"]
pub mod in_dscr_bf0_ch;
#[doc = "IN_DSCR_BF1_CH (r) register accessor: The second-to-last inlink descriptor address of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_bf1_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch`] module"]
pub type IN_DSCR_BF1_CH = crate::Reg<in_dscr_bf1_ch::IN_DSCR_BF1_CH_SPEC>;
#[doc = "The second-to-last inlink descriptor address of Rx channel 0"]
pub mod in_dscr_bf1_ch;
#[doc = "IN_PRI_CH (rw) register accessor: Priority register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pri_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pri_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pri_ch`] module"]
pub type IN_PRI_CH = crate::Reg<in_pri_ch::IN_PRI_CH_SPEC>;
#[doc = "Priority register of Rx channel 0"]
pub mod in_pri_ch;
#[doc = "IN_PERI_SEL_CH (rw) register accessor: Peripheral selection of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel_ch`] module"]
pub type IN_PERI_SEL_CH = crate::Reg<in_peri_sel_ch::IN_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection of Rx channel 0"]
pub mod in_peri_sel_ch;
#[doc = "IN_CRC_INIT_DATA_CH (rw) register accessor: This register is used to config ch0 crc initial data(max 32 bit)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_crc_init_data_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_crc_init_data_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_init_data_ch`] module"]
pub type IN_CRC_INIT_DATA_CH = crate::Reg<in_crc_init_data_ch::IN_CRC_INIT_DATA_CH_SPEC>;
#[doc = "This register is used to config ch0 crc initial data(max 32 bit)"]
pub mod in_crc_init_data_ch;
#[doc = "RX_CRC_WIDTH_CH (rw) register accessor: This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_width_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_width_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_width_ch`] module"]
pub type RX_CRC_WIDTH_CH = crate::Reg<rx_crc_width_ch::RX_CRC_WIDTH_CH_SPEC>;
#[doc = "This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
pub mod rx_crc_width_ch;
#[doc = "IN_CRC_CLEAR_CH (rw) register accessor: This register is used to clear ch0 crc result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_crc_clear_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_crc_clear_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_clear_ch`] module"]
pub type IN_CRC_CLEAR_CH = crate::Reg<in_crc_clear_ch::IN_CRC_CLEAR_CH_SPEC>;
#[doc = "This register is used to clear ch0 crc result"]
pub mod in_crc_clear_ch;
#[doc = "IN_CRC_FINAL_RESULT_CH (r) register accessor: This register is used to store ch0 crc result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_crc_final_result_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_crc_final_result_ch`] module"]
pub type IN_CRC_FINAL_RESULT_CH = crate::Reg<in_crc_final_result_ch::IN_CRC_FINAL_RESULT_CH_SPEC>;
#[doc = "This register is used to store ch0 crc result"]
pub mod in_crc_final_result_ch;
#[doc = "RX_CRC_EN_WR_DATA_CH (rw) register accessor: This resister is used to config ch0 crc en for every bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_en_wr_data_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_en_wr_data_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_en_wr_data_ch`] module"]
pub type RX_CRC_EN_WR_DATA_CH = crate::Reg<rx_crc_en_wr_data_ch::RX_CRC_EN_WR_DATA_CH_SPEC>;
#[doc = "This resister is used to config ch0 crc en for every bit"]
pub mod rx_crc_en_wr_data_ch;
#[doc = "RX_CRC_EN_ADDR_CH (rw) register accessor: This register is used to config ch0 crc en addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_en_addr_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_en_addr_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_en_addr_ch`] module"]
pub type RX_CRC_EN_ADDR_CH = crate::Reg<rx_crc_en_addr_ch::RX_CRC_EN_ADDR_CH_SPEC>;
#[doc = "This register is used to config ch0 crc en addr"]
pub mod rx_crc_en_addr_ch;
#[doc = "RX_CRC_DATA_EN_WR_DATA_CH (rw) register accessor: This register is used to config crc data_8bit en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_data_en_wr_data_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_data_en_wr_data_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_data_en_wr_data_ch`] module"]
pub type RX_CRC_DATA_EN_WR_DATA_CH =
    crate::Reg<rx_crc_data_en_wr_data_ch::RX_CRC_DATA_EN_WR_DATA_CH_SPEC>;
#[doc = "This register is used to config crc data_8bit en"]
pub mod rx_crc_data_en_wr_data_ch;
#[doc = "RX_CRC_DATA_EN_ADDR_CH (rw) register accessor: This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_data_en_addr_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_data_en_addr_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_crc_data_en_addr_ch`] module"]
pub type RX_CRC_DATA_EN_ADDR_CH = crate::Reg<rx_crc_data_en_addr_ch::RX_CRC_DATA_EN_ADDR_CH_SPEC>;
#[doc = "This register is used to config addr of crc data_8bit en"]
pub mod rx_crc_data_en_addr_ch;
#[doc = "OUT_INT_RAW_CH (rw) register accessor: Raw status interrupt of channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch`] module"]
pub type OUT_INT_RAW_CH = crate::Reg<out_int_raw_ch::OUT_INT_RAW_CH_SPEC>;
#[doc = "Raw status interrupt of channel0"]
pub mod out_int_raw_ch;
#[doc = "OUT_INT_ST_CH (r) register accessor: Masked interrupt of channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_st_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch`] module"]
pub type OUT_INT_ST_CH = crate::Reg<out_int_st_ch::OUT_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt of channel0"]
pub mod out_int_st_ch;
#[doc = "OUT_INT_ENA_CH (rw) register accessor: Interrupt enable bits of channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_ena_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_ena_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch`] module"]
pub type OUT_INT_ENA_CH = crate::Reg<out_int_ena_ch::OUT_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of channel0"]
pub mod out_int_ena_ch;
#[doc = "OUT_INT_CLR_CH (w) register accessor: Interrupt clear bits of channel0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch`] module"]
pub type OUT_INT_CLR_CH = crate::Reg<out_int_clr_ch::OUT_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of channel0"]
pub mod out_int_clr_ch;
#[doc = "OUT_CONF0_CH0 (rw) register accessor: Configure 0 register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch0`] module"]
pub type OUT_CONF0_CH0 = crate::Reg<out_conf0_ch0::OUT_CONF0_CH0_SPEC>;
#[doc = "Configure 0 register of Tx channel0"]
pub mod out_conf0_ch0;
#[doc = "OUT_CONF1_CH (rw) register accessor: Configure 1 register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf1_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf1_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1_ch`] module"]
pub type OUT_CONF1_CH = crate::Reg<out_conf1_ch::OUT_CONF1_CH_SPEC>;
#[doc = "Configure 1 register of Tx channel0"]
pub mod out_conf1_ch;
#[doc = "OUTFIFO_STATUS_CH (r) register accessor: Transmit FIFO status of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch`] module"]
pub type OUTFIFO_STATUS_CH = crate::Reg<outfifo_status_ch::OUTFIFO_STATUS_CH_SPEC>;
#[doc = "Transmit FIFO status of Tx channel0"]
pub mod outfifo_status_ch;
#[doc = "OUT_PUSH_CH (rw) register accessor: Push control register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch`] module"]
pub type OUT_PUSH_CH = crate::Reg<out_push_ch::OUT_PUSH_CH_SPEC>;
#[doc = "Push control register of Tx channel0"]
pub mod out_push_ch;
#[doc = "OUT_LINK1_CH (rw) register accessor: Link descriptor configure and control register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link1_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link1_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link1_ch`] module"]
pub type OUT_LINK1_CH = crate::Reg<out_link1_ch::OUT_LINK1_CH_SPEC>;
#[doc = "Link descriptor configure and control register of Tx channel0"]
pub mod out_link1_ch;
#[doc = "OUT_LINK2_CH (rw) register accessor: Link descriptor configure and control register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link2_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link2_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link2_ch`] module"]
pub type OUT_LINK2_CH = crate::Reg<out_link2_ch::OUT_LINK2_CH_SPEC>;
#[doc = "Link descriptor configure and control register of Tx channel0"]
pub mod out_link2_ch;
#[doc = "OUT_STATE_CH (r) register accessor: Transmit status of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch`] module"]
pub type OUT_STATE_CH = crate::Reg<out_state_ch::OUT_STATE_CH_SPEC>;
#[doc = "Transmit status of Tx channel0"]
pub mod out_state_ch;
#[doc = "OUT_EOF_DES_ADDR_CH (r) register accessor: Outlink descriptor address when EOF occurs of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch`] module"]
pub type OUT_EOF_DES_ADDR_CH = crate::Reg<out_eof_des_addr_ch::OUT_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Outlink descriptor address when EOF occurs of Tx channel0"]
pub mod out_eof_des_addr_ch;
#[doc = "OUT_EOF_BFR_DES_ADDR_CH (r) register accessor: The last outlink descriptor address when EOF occurs of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr_ch`] module"]
pub type OUT_EOF_BFR_DES_ADDR_CH =
    crate::Reg<out_eof_bfr_des_addr_ch::OUT_EOF_BFR_DES_ADDR_CH_SPEC>;
#[doc = "The last outlink descriptor address when EOF occurs of Tx channel0"]
pub mod out_eof_bfr_des_addr_ch;
#[doc = "OUT_DSCR_CH (r) register accessor: Current outlink descriptor address of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch`] module"]
pub type OUT_DSCR_CH = crate::Reg<out_dscr_ch::OUT_DSCR_CH_SPEC>;
#[doc = "Current outlink descriptor address of Tx channel0"]
pub mod out_dscr_ch;
#[doc = "OUT_DSCR_BF0_CH (r) register accessor: The last outlink descriptor address of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch`] module"]
pub type OUT_DSCR_BF0_CH = crate::Reg<out_dscr_bf0_ch::OUT_DSCR_BF0_CH_SPEC>;
#[doc = "The last outlink descriptor address of Tx channel0"]
pub mod out_dscr_bf0_ch;
#[doc = "OUT_DSCR_BF1_CH (r) register accessor: The second-to-last outlink descriptor address of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch`] module"]
pub type OUT_DSCR_BF1_CH = crate::Reg<out_dscr_bf1_ch::OUT_DSCR_BF1_CH_SPEC>;
#[doc = "The second-to-last outlink descriptor address of Tx channel0"]
pub mod out_dscr_bf1_ch;
#[doc = "OUT_PRI_CH (rw) register accessor: Priority register of Tx channel0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_pri_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_pri_ch`] module"]
pub type OUT_PRI_CH = crate::Reg<out_pri_ch::OUT_PRI_CH_SPEC>;
#[doc = "Priority register of Tx channel0."]
pub mod out_pri_ch;
#[doc = "OUT_PERI_SEL_CH (rw) register accessor: Peripheral selection of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_peri_sel_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_peri_sel_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel_ch`] module"]
pub type OUT_PERI_SEL_CH = crate::Reg<out_peri_sel_ch::OUT_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection of Tx channel0"]
pub mod out_peri_sel_ch;
#[doc = "OUT_CRC_INIT_DATA_CH (rw) register accessor: This register is used to config ch0 crc initial data(max 32 bit)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_crc_init_data_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_crc_init_data_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_init_data_ch`] module"]
pub type OUT_CRC_INIT_DATA_CH = crate::Reg<out_crc_init_data_ch::OUT_CRC_INIT_DATA_CH_SPEC>;
#[doc = "This register is used to config ch0 crc initial data(max 32 bit)"]
pub mod out_crc_init_data_ch;
#[doc = "TX_CRC_WIDTH_CH (rw) register accessor: This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_width_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_width_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_width_ch`] module"]
pub type TX_CRC_WIDTH_CH = crate::Reg<tx_crc_width_ch::TX_CRC_WIDTH_CH_SPEC>;
#[doc = "This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
pub mod tx_crc_width_ch;
#[doc = "OUT_CRC_CLEAR_CH (rw) register accessor: This register is used to clear ch0 crc result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_crc_clear_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_crc_clear_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_clear_ch`] module"]
pub type OUT_CRC_CLEAR_CH = crate::Reg<out_crc_clear_ch::OUT_CRC_CLEAR_CH_SPEC>;
#[doc = "This register is used to clear ch0 crc result"]
pub mod out_crc_clear_ch;
#[doc = "OUT_CRC_FINAL_RESULT_CH (r) register accessor: This register is used to store ch0 crc result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_crc_final_result_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_crc_final_result_ch`] module"]
pub type OUT_CRC_FINAL_RESULT_CH =
    crate::Reg<out_crc_final_result_ch::OUT_CRC_FINAL_RESULT_CH_SPEC>;
#[doc = "This register is used to store ch0 crc result"]
pub mod out_crc_final_result_ch;
#[doc = "TX_CRC_EN_WR_DATA_CH (rw) register accessor: This resister is used to config ch0 crc en for every bit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_en_wr_data_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_en_wr_data_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_en_wr_data_ch`] module"]
pub type TX_CRC_EN_WR_DATA_CH = crate::Reg<tx_crc_en_wr_data_ch::TX_CRC_EN_WR_DATA_CH_SPEC>;
#[doc = "This resister is used to config ch0 crc en for every bit"]
pub mod tx_crc_en_wr_data_ch;
#[doc = "TX_CRC_EN_ADDR_CH (rw) register accessor: This register is used to config ch0 crc en addr\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_en_addr_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_en_addr_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_en_addr_ch`] module"]
pub type TX_CRC_EN_ADDR_CH = crate::Reg<tx_crc_en_addr_ch::TX_CRC_EN_ADDR_CH_SPEC>;
#[doc = "This register is used to config ch0 crc en addr"]
pub mod tx_crc_en_addr_ch;
#[doc = "TX_CRC_DATA_EN_WR_DATA_CH (rw) register accessor: This register is used to config crc data_8bit en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_data_en_wr_data_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_data_en_wr_data_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_data_en_wr_data_ch`] module"]
pub type TX_CRC_DATA_EN_WR_DATA_CH =
    crate::Reg<tx_crc_data_en_wr_data_ch::TX_CRC_DATA_EN_WR_DATA_CH_SPEC>;
#[doc = "This register is used to config crc data_8bit en"]
pub mod tx_crc_data_en_wr_data_ch;
#[doc = "TX_CRC_DATA_EN_ADDR_CH (rw) register accessor: This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_data_en_addr_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_data_en_addr_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_crc_data_en_addr_ch`] module"]
pub type TX_CRC_DATA_EN_ADDR_CH = crate::Reg<tx_crc_data_en_addr_ch::TX_CRC_DATA_EN_ADDR_CH_SPEC>;
#[doc = "This register is used to config addr of crc data_8bit en"]
pub mod tx_crc_data_en_addr_ch;
#[doc = "OUT_CONF0_CH1 (rw) register accessor: Configure 0 register of Tx channel1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch1`] module"]
pub type OUT_CONF0_CH1 = crate::Reg<out_conf0_ch1::OUT_CONF0_CH1_SPEC>;
#[doc = "Configure 0 register of Tx channel1"]
pub mod out_conf0_ch1;
#[doc = "OUT_CONF0_CH2 (rw) register accessor: Configure 0 register of Tx channel2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch2`] module"]
pub type OUT_CONF0_CH2 = crate::Reg<out_conf0_ch2::OUT_CONF0_CH2_SPEC>;
#[doc = "Configure 0 register of Tx channel2"]
pub mod out_conf0_ch2;
#[doc = "ARB_TIMEOUT (rw) register accessor: This retister is used to config arbiter time slice\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout`] module"]
pub type ARB_TIMEOUT = crate::Reg<arb_timeout::ARB_TIMEOUT_SPEC>;
#[doc = "This retister is used to config arbiter time slice"]
pub mod arb_timeout;
#[doc = "WEIGHT_EN (rw) register accessor: This register is used to config arbiter weight function to on or off\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`weight_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`weight_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en`] module"]
pub type WEIGHT_EN = crate::Reg<weight_en::WEIGHT_EN_SPEC>;
#[doc = "This register is used to config arbiter weight function to on or off"]
pub mod weight_en;
#[doc = "IN_MEM_CONF (rw) register accessor: Mem power configure register of Rx channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_mem_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_mem_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_mem_conf`] module"]
pub type IN_MEM_CONF = crate::Reg<in_mem_conf::IN_MEM_CONF_SPEC>;
#[doc = "Mem power configure register of Rx channel"]
pub mod in_mem_conf;
#[doc = "INTR_MEM_START_ADDR (rw) register accessor: The start address of accessible address space.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mem_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_start_addr`] module"]
pub type INTR_MEM_START_ADDR = crate::Reg<intr_mem_start_addr::INTR_MEM_START_ADDR_SPEC>;
#[doc = "The start address of accessible address space."]
pub mod intr_mem_start_addr;
#[doc = "INTR_MEM_END_ADDR (rw) register accessor: The end address of accessible address space. The access address beyond this range would lead to descriptor error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mem_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_end_addr`] module"]
pub type INTR_MEM_END_ADDR = crate::Reg<intr_mem_end_addr::INTR_MEM_END_ADDR_SPEC>;
#[doc = "The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub mod intr_mem_end_addr;
#[doc = "EXTR_MEM_START_ADDR (rw) register accessor: The start address of accessible address space.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extr_mem_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extr_mem_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extr_mem_start_addr`] module"]
pub type EXTR_MEM_START_ADDR = crate::Reg<extr_mem_start_addr::EXTR_MEM_START_ADDR_SPEC>;
#[doc = "The start address of accessible address space."]
pub mod extr_mem_start_addr;
#[doc = "EXTR_MEM_END_ADDR (rw) register accessor: The end address of accessible address space. The access address beyond this range would lead to descriptor error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extr_mem_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extr_mem_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extr_mem_end_addr`] module"]
pub type EXTR_MEM_END_ADDR = crate::Reg<extr_mem_end_addr::EXTR_MEM_END_ADDR_SPEC>;
#[doc = "The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub mod extr_mem_end_addr;
#[doc = "IN_RESET_AVAIL_CH (r) register accessor: The rx channel 0 reset valid_flag register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_reset_avail_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_reset_avail_ch`] module"]
pub type IN_RESET_AVAIL_CH = crate::Reg<in_reset_avail_ch::IN_RESET_AVAIL_CH_SPEC>;
#[doc = "The rx channel 0 reset valid_flag register."]
pub mod in_reset_avail_ch;
#[doc = "OUT_RESET_AVAIL_CH (r) register accessor: The tx channel 0 reset valid_flag register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_reset_avail_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_reset_avail_ch`] module"]
pub type OUT_RESET_AVAIL_CH = crate::Reg<out_reset_avail_ch::OUT_RESET_AVAIL_CH_SPEC>;
#[doc = "The tx channel 0 reset valid_flag register."]
pub mod out_reset_avail_ch;
#[doc = "MISC_CONF (rw) register accessor: MISC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "MISC register"]
pub mod misc_conf;
#[doc = "RDN_RESULT (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_result`] module"]
pub type RDN_RESULT = crate::Reg<rdn_result::RDN_RESULT_SPEC>;
#[doc = "reserved"]
pub mod rdn_result;
#[doc = "RDN_ECO_HIGH (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_high::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_high::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_high`] module"]
pub type RDN_ECO_HIGH = crate::Reg<rdn_eco_high::RDN_ECO_HIGH_SPEC>;
#[doc = "reserved"]
pub mod rdn_eco_high;
#[doc = "RDN_ECO_LOW (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco_low::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco_low::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_low`] module"]
pub type RDN_ECO_LOW = crate::Reg<rdn_eco_low::RDN_ECO_LOW_SPEC>;
#[doc = "reserved"]
pub mod rdn_eco_low;
#[doc = "WRESP_CNT (r) register accessor: AXI wr responce cnt register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wresp_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wresp_cnt`] module"]
pub type WRESP_CNT = crate::Reg<wresp_cnt::WRESP_CNT_SPEC>;
#[doc = "AXI wr responce cnt register."]
pub mod wresp_cnt;
#[doc = "RRESP_CNT (r) register accessor: AXI wr responce cnt register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rresp_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rresp_cnt`] module"]
pub type RRESP_CNT = crate::Reg<rresp_cnt::RRESP_CNT_SPEC>;
#[doc = "AXI wr responce cnt register."]
pub mod rresp_cnt;
#[doc = "INFIFO_STATUS1_CH (r) register accessor: Receive FIFO status of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status1_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status1_ch`] module"]
pub type INFIFO_STATUS1_CH = crate::Reg<infifo_status1_ch::INFIFO_STATUS1_CH_SPEC>;
#[doc = "Receive FIFO status of Rx channel 0"]
pub mod infifo_status1_ch;
#[doc = "OUTFIFO_STATUS1_CH (r) register accessor: Receive FIFO status of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status1_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status1_ch`] module"]
pub type OUTFIFO_STATUS1_CH = crate::Reg<outfifo_status1_ch::OUTFIFO_STATUS1_CH_SPEC>;
#[doc = "Receive FIFO status of Tx channel 0"]
pub mod outfifo_status1_ch;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
