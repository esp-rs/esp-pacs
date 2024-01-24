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
    _reserved4: [u8; 0x24],
    out_int_raw_ch: (),
    _reserved5: [u8; 0x04],
    out_int_st_ch: (),
    _reserved6: [u8; 0x04],
    out_int_ena_ch: (),
    _reserved7: [u8; 0x04],
    out_int_clr_ch: (),
    _reserved8: [u8; 0x24],
    ahb_test: AHB_TEST,
    misc_conf: MISC_CONF,
    date: DATE,
    _reserved11: [u8; 0x04],
    in_conf0_ch: (),
    _reserved12: [u8; 0x04],
    in_conf1_ch: (),
    _reserved13: [u8; 0x04],
    infifo_status_ch: (),
    _reserved14: [u8; 0x04],
    in_pop_ch: (),
    _reserved15: [u8; 0x04],
    in_link_ch: (),
    _reserved16: [u8; 0x04],
    in_state_ch: (),
    _reserved17: [u8; 0x04],
    in_suc_eof_des_addr_ch: (),
    _reserved18: [u8; 0x04],
    in_err_eof_des_addr_ch: (),
    _reserved19: [u8; 0x04],
    in_dscr_ch: (),
    _reserved20: [u8; 0x04],
    in_dscr_bf0_ch: (),
    _reserved21: [u8; 0x04],
    in_dscr_bf1_ch: (),
    _reserved22: [u8; 0x04],
    in_pri_ch: (),
    _reserved23: [u8; 0x04],
    in_peri_sel_ch: (),
    _reserved24: [u8; 0x30],
    out_conf0_ch0: OUT_CONF0_CH0,
    out_conf1_ch: (),
    _reserved26: [u8; 0x04],
    outfifo_status_ch: (),
    _reserved27: [u8; 0x04],
    out_push_ch: (),
    _reserved28: [u8; 0x04],
    out_link_ch: (),
    _reserved29: [u8; 0x04],
    out_state_ch: (),
    _reserved30: [u8; 0x04],
    out_eof_des_addr_ch: (),
    _reserved31: [u8; 0x04],
    out_eof_bfr_des_addr_ch: (),
    _reserved32: [u8; 0x04],
    out_dscr_ch: (),
    _reserved33: [u8; 0x04],
    out_dscr_bf0_ch: (),
    _reserved34: [u8; 0x04],
    out_dscr_bf1_ch: (),
    _reserved35: [u8; 0x04],
    out_pri_ch: (),
    _reserved36: [u8; 0x04],
    out_peri_sel_ch: (),
    _reserved37: [u8; 0x90],
    out_conf0_ch: (),
    _reserved38: [u8; 0x012c],
    out_crc_init_data_ch: (),
    _reserved39: [u8; 0x04],
    tx_crc_width_ch: (),
    _reserved40: [u8; 0x04],
    out_crc_clear_ch: (),
    _reserved41: [u8; 0x04],
    out_crc_final_result_ch: (),
    _reserved42: [u8; 0x04],
    tx_crc_en_wr_data_ch: (),
    _reserved43: [u8; 0x04],
    tx_crc_en_addr_ch: (),
    _reserved44: [u8; 0x04],
    tx_crc_data_en_wr_data_ch: (),
    _reserved45: [u8; 0x04],
    tx_crc_data_en_addr_ch: (),
    _reserved46: [u8; 0x04],
    tx_ch_arb_weigh_ch: (),
    _reserved47: [u8; 0x04],
    tx_arb_weigh_opt_dir_ch: (),
    _reserved48: [u8; 0x54],
    in_crc_init_data_ch: (),
    _reserved49: [u8; 0x04],
    rx_crc_width_ch: (),
    _reserved50: [u8; 0x04],
    in_crc_clear_ch: (),
    _reserved51: [u8; 0x04],
    in_crc_final_result_ch: (),
    _reserved52: [u8; 0x04],
    rx_crc_en_wr_data_ch: (),
    _reserved53: [u8; 0x04],
    rx_crc_en_addr_ch: (),
    _reserved54: [u8; 0x04],
    rx_crc_data_en_wr_data_ch: (),
    _reserved55: [u8; 0x04],
    rx_crc_data_en_addr_ch: (),
    _reserved56: [u8; 0x04],
    rx_ch_arb_weigh_ch: (),
    _reserved57: [u8; 0x04],
    rx_arb_weigh_opt_dir_ch: (),
    _reserved58: [u8; 0x54],
    in_link_addr_ch: [IN_LINK_ADDR_CH; 3],
    out_link_addr_ch: [OUT_LINK_ADDR_CH; 3],
    intr_mem_start_addr: INTR_MEM_START_ADDR,
    intr_mem_end_addr: INTR_MEM_END_ADDR,
    arb_timeout_tx: ARB_TIMEOUT_TX,
    arb_timeout_rx: ARB_TIMEOUT_RX,
    weight_en_tx: WEIGHT_EN_TX,
    weight_en_rx: WEIGHT_EN_RX,
}
impl RegisterBlock {
    #[doc = "0x00..0x0c - Raw status interrupt of channel 0"]
    #[inline(always)]
    pub const fn in_int_raw_ch(&self, n: usize) -> &IN_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(16 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x0c - Raw status interrupt of channel 0"]
    #[inline(always)]
    pub fn in_int_raw_ch_iter(&self) -> impl Iterator<Item = &IN_INT_RAW_CH> {
        (0..3).map(|n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(16 * n).cast() })
    }
    #[doc = "0x04..0x10 - Masked interrupt of channel 0"]
    #[inline(always)]
    pub const fn in_int_st_ch(&self, n: usize) -> &IN_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(4).add(16 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x10 - Masked interrupt of channel 0"]
    #[inline(always)]
    pub fn in_int_st_ch_iter(&self) -> impl Iterator<Item = &IN_INT_ST_CH> {
        (0..3).map(|n| unsafe { &*(self as *const Self).cast::<u8>().add(4).add(16 * n).cast() })
    }
    #[doc = "0x08..0x14 - Interrupt enable bits of channel 0"]
    #[inline(always)]
    pub const fn in_int_ena_ch(&self, n: usize) -> &IN_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(8).add(16 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x14 - Interrupt enable bits of channel 0"]
    #[inline(always)]
    pub fn in_int_ena_ch_iter(&self) -> impl Iterator<Item = &IN_INT_ENA_CH> {
        (0..3).map(|n| unsafe { &*(self as *const Self).cast::<u8>().add(8).add(16 * n).cast() })
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
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x18 - Interrupt clear bits of channel 0"]
    #[inline(always)]
    pub fn in_int_clr_ch_iter(&self) -> impl Iterator<Item = &IN_INT_CLR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(12)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x30..0x3c - Raw status interrupt of channel 0"]
    #[inline(always)]
    pub const fn out_int_raw_ch(&self, n: usize) -> &OUT_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(48)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x3c - Raw status interrupt of channel 0"]
    #[inline(always)]
    pub fn out_int_raw_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_RAW_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(48)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x34..0x40 - Masked interrupt of channel 0"]
    #[inline(always)]
    pub const fn out_int_st_ch(&self, n: usize) -> &OUT_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(52)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x40 - Masked interrupt of channel 0"]
    #[inline(always)]
    pub fn out_int_st_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_ST_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(52)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x38..0x44 - Interrupt enable bits of channel 0"]
    #[inline(always)]
    pub const fn out_int_ena_ch(&self, n: usize) -> &OUT_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(56)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x44 - Interrupt enable bits of channel 0"]
    #[inline(always)]
    pub fn out_int_ena_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_ENA_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(56)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x3c..0x48 - Interrupt clear bits of channel 0"]
    #[inline(always)]
    pub const fn out_int_clr_ch(&self, n: usize) -> &OUT_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(60)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0x48 - Interrupt clear bits of channel 0"]
    #[inline(always)]
    pub fn out_int_clr_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_CLR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(60)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x60 - reserved"]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x64 - MISC register"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0x68 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x70..0x7c - Configure 0 register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_conf0_ch(&self, n: usize) -> &IN_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(112)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x7c - Configure 0 register of Rx channel 0"]
    #[inline(always)]
    pub fn in_conf0_ch_iter(&self) -> impl Iterator<Item = &IN_CONF0_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(112)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x74..0x80 - Configure 1 register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_conf1_ch(&self, n: usize) -> &IN_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(116)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x74..0x80 - Configure 1 register of Rx channel 0"]
    #[inline(always)]
    pub fn in_conf1_ch_iter(&self) -> impl Iterator<Item = &IN_CONF1_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(116)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x78..0x84 - Receive FIFO status of Rx channel 0"]
    #[inline(always)]
    pub const fn infifo_status_ch(&self, n: usize) -> &INFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(120)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x78..0x84 - Receive FIFO status of Rx channel 0"]
    #[inline(always)]
    pub fn infifo_status_ch_iter(&self) -> impl Iterator<Item = &INFIFO_STATUS_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(120)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x7c..0x88 - Pop control register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_pop_ch(&self, n: usize) -> &IN_POP_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(124)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x7c..0x88 - Pop control register of Rx channel 0"]
    #[inline(always)]
    pub fn in_pop_ch_iter(&self) -> impl Iterator<Item = &IN_POP_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(124)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x80..0x8c - Link descriptor configure and control register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_link_ch(&self, n: usize) -> &IN_LINK_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(128)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x8c - Link descriptor configure and control register of Rx channel 0"]
    #[inline(always)]
    pub fn in_link_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(128)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x84..0x90 - Receive status of Rx channel 0"]
    #[inline(always)]
    pub const fn in_state_ch(&self, n: usize) -> &IN_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(132)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x84..0x90 - Receive status of Rx channel 0"]
    #[inline(always)]
    pub fn in_state_ch_iter(&self) -> impl Iterator<Item = &IN_STATE_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(132)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x88..0x94 - Inlink descriptor address when EOF occurs of Rx channel 0"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch(&self, n: usize) -> &IN_SUC_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(136)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88..0x94 - Inlink descriptor address when EOF occurs of Rx channel 0"]
    #[inline(always)]
    pub fn in_suc_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &IN_SUC_EOF_DES_ADDR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(136)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x8c..0x98 - Inlink descriptor address when errors occur of Rx channel 0"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch(&self, n: usize) -> &IN_ERR_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(140)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x8c..0x98 - Inlink descriptor address when errors occur of Rx channel 0"]
    #[inline(always)]
    pub fn in_err_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &IN_ERR_EOF_DES_ADDR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(140)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x90..0x9c - Current inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr_ch(&self, n: usize) -> &IN_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(144)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0x9c - Current inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub fn in_dscr_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(144)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x94..0xa0 - The last inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch(&self, n: usize) -> &IN_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(148)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x94..0xa0 - The last inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub fn in_dscr_bf0_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_BF0_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(148)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x98..0xa4 - The second-to-last inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch(&self, n: usize) -> &IN_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(152)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x98..0xa4 - The second-to-last inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub fn in_dscr_bf1_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_BF1_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(152)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x9c..0xa8 - Priority register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_pri_ch(&self, n: usize) -> &IN_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(156)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c..0xa8 - Priority register of Rx channel 0"]
    #[inline(always)]
    pub fn in_pri_ch_iter(&self) -> impl Iterator<Item = &IN_PRI_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(156)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xa0..0xac - Peripheral selection of Rx channel 0"]
    #[inline(always)]
    pub const fn in_peri_sel_ch(&self, n: usize) -> &IN_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(160)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xac - Peripheral selection of Rx channel 0"]
    #[inline(always)]
    pub fn in_peri_sel_ch_iter(&self) -> impl Iterator<Item = &IN_PERI_SEL_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(160)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xd0 - Configure 0 register of Tx channel 0"]
    #[inline(always)]
    pub const fn out_conf0_ch0(&self) -> &OUT_CONF0_CH0 {
        &self.out_conf0_ch0
    }
    #[doc = "0xd4..0xe0 - Configure 1 register of Tx channel 0"]
    #[inline(always)]
    pub const fn out_conf1_ch(&self, n: usize) -> &OUT_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(212)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd4..0xe0 - Configure 1 register of Tx channel 0"]
    #[inline(always)]
    pub fn out_conf1_ch_iter(&self) -> impl Iterator<Item = &OUT_CONF1_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(212)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xd8..0xe4 - Transmit FIFO status of Tx channel 0"]
    #[inline(always)]
    pub const fn outfifo_status_ch(&self, n: usize) -> &OUTFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(216)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd8..0xe4 - Transmit FIFO status of Tx channel 0"]
    #[inline(always)]
    pub fn outfifo_status_ch_iter(&self) -> impl Iterator<Item = &OUTFIFO_STATUS_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(216)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xdc..0xe8 - Push control register of Rx channel 0"]
    #[inline(always)]
    pub const fn out_push_ch(&self, n: usize) -> &OUT_PUSH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(220)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xdc..0xe8 - Push control register of Rx channel 0"]
    #[inline(always)]
    pub fn out_push_ch_iter(&self) -> impl Iterator<Item = &OUT_PUSH_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(220)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xe0..0xec - Link descriptor configure and control register of Tx channel 0"]
    #[inline(always)]
    pub const fn out_link_ch(&self, n: usize) -> &OUT_LINK_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(224)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe0..0xec - Link descriptor configure and control register of Tx channel 0"]
    #[inline(always)]
    pub fn out_link_ch_iter(&self) -> impl Iterator<Item = &OUT_LINK_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(224)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xe4..0xf0 - Transmit status of Tx channel 0"]
    #[inline(always)]
    pub const fn out_state_ch(&self, n: usize) -> &OUT_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(228)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe4..0xf0 - Transmit status of Tx channel 0"]
    #[inline(always)]
    pub fn out_state_ch_iter(&self) -> impl Iterator<Item = &OUT_STATE_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(228)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xe8..0xf4 - Outlink descriptor address when EOF occurs of Tx channel 0"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch(&self, n: usize) -> &OUT_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(232)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe8..0xf4 - Outlink descriptor address when EOF occurs of Tx channel 0"]
    #[inline(always)]
    pub fn out_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_EOF_DES_ADDR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(232)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xec..0xf8 - The last outlink descriptor address when EOF occurs of Tx channel 0"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch(&self, n: usize) -> &OUT_EOF_BFR_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(236)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xec..0xf8 - The last outlink descriptor address when EOF occurs of Tx channel 0"]
    #[inline(always)]
    pub fn out_eof_bfr_des_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_EOF_BFR_DES_ADDR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(236)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xf0..0xfc - Current inlink descriptor address of Tx channel 0"]
    #[inline(always)]
    pub const fn out_dscr_ch(&self, n: usize) -> &OUT_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(240)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf0..0xfc - Current inlink descriptor address of Tx channel 0"]
    #[inline(always)]
    pub fn out_dscr_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(240)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xf4..0x100 - The last inlink descriptor address of Tx channel 0"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch(&self, n: usize) -> &OUT_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(244)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf4..0x100 - The last inlink descriptor address of Tx channel 0"]
    #[inline(always)]
    pub fn out_dscr_bf0_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_BF0_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(244)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xf8..0x104 - The second-to-last inlink descriptor address of Tx channel 0"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch(&self, n: usize) -> &OUT_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(248)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf8..0x104 - The second-to-last inlink descriptor address of Tx channel 0"]
    #[inline(always)]
    pub fn out_dscr_bf1_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_BF1_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(248)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xfc..0x108 - Priority register of Tx channel 0."]
    #[inline(always)]
    pub const fn out_pri_ch(&self, n: usize) -> &OUT_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(252)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xfc..0x108 - Priority register of Tx channel 0."]
    #[inline(always)]
    pub fn out_pri_ch_iter(&self) -> impl Iterator<Item = &OUT_PRI_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(252)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x100..0x10c - Peripheral selection of Tx channel 0"]
    #[inline(always)]
    pub const fn out_peri_sel_ch(&self, n: usize) -> &OUT_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x10c - Peripheral selection of Tx channel 0"]
    #[inline(always)]
    pub fn out_peri_sel_ch_iter(&self) -> impl Iterator<Item = &OUT_PERI_SEL_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x190..0x198 - Configure 0 register of Tx channel 1"]
    #[inline(always)]
    pub const fn out_conf0_ch(&self, n: usize) -> &OUT_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(400)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x198 - Configure 0 register of Tx channel 1"]
    #[inline(always)]
    pub fn out_conf0_ch_iter(&self) -> impl Iterator<Item = &OUT_CONF0_CH> {
        (0..2).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(400)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x2bc..0x2c8 - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn out_crc_init_data_ch(&self, n: usize) -> &OUT_CRC_INIT_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(700)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2bc..0x2c8 - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub fn out_crc_init_data_ch_iter(&self) -> impl Iterator<Item = &OUT_CRC_INIT_DATA_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(700)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2c0..0x2cc - This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
    #[inline(always)]
    pub const fn tx_crc_width_ch(&self, n: usize) -> &TX_CRC_WIDTH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(704)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c0..0x2cc - This register is used to confiig tx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
    #[inline(always)]
    pub fn tx_crc_width_ch_iter(&self) -> impl Iterator<Item = &TX_CRC_WIDTH_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(704)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2c4..0x2d0 - This register is used to clear ch0 crc result"]
    #[inline(always)]
    pub const fn out_crc_clear_ch(&self, n: usize) -> &OUT_CRC_CLEAR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(708)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c4..0x2d0 - This register is used to clear ch0 crc result"]
    #[inline(always)]
    pub fn out_crc_clear_ch_iter(&self) -> impl Iterator<Item = &OUT_CRC_CLEAR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(708)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2c8..0x2d4 - This register is used to store ch0 crc result"]
    #[inline(always)]
    pub const fn out_crc_final_result_ch(&self, n: usize) -> &OUT_CRC_FINAL_RESULT_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(712)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c8..0x2d4 - This register is used to store ch0 crc result"]
    #[inline(always)]
    pub fn out_crc_final_result_ch_iter(&self) -> impl Iterator<Item = &OUT_CRC_FINAL_RESULT_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(712)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2cc..0x2d8 - This resister is used to config ch0 crc en for every bit"]
    #[inline(always)]
    pub const fn tx_crc_en_wr_data_ch(&self, n: usize) -> &TX_CRC_EN_WR_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(716)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2cc..0x2d8 - This resister is used to config ch0 crc en for every bit"]
    #[inline(always)]
    pub fn tx_crc_en_wr_data_ch_iter(&self) -> impl Iterator<Item = &TX_CRC_EN_WR_DATA_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(716)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2d0..0x2dc - This register is used to config ch0 crc en addr"]
    #[inline(always)]
    pub const fn tx_crc_en_addr_ch(&self, n: usize) -> &TX_CRC_EN_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(720)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2d0..0x2dc - This register is used to config ch0 crc en addr"]
    #[inline(always)]
    pub fn tx_crc_en_addr_ch_iter(&self) -> impl Iterator<Item = &TX_CRC_EN_ADDR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
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
            &*(self as *const Self)
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
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
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
            &*(self as *const Self)
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
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(728)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2dc..0x2e8 - This register is used to config ch0 arbiter weigh"]
    #[inline(always)]
    pub const fn tx_ch_arb_weigh_ch(&self, n: usize) -> &TX_CH_ARB_WEIGH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(732)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2dc..0x2e8 - This register is used to config ch0 arbiter weigh"]
    #[inline(always)]
    pub fn tx_ch_arb_weigh_ch_iter(&self) -> impl Iterator<Item = &TX_CH_ARB_WEIGH_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(732)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2e0..0x2ec - This register is used to config off or on weigh optimization"]
    #[inline(always)]
    pub const fn tx_arb_weigh_opt_dir_ch(&self, n: usize) -> &TX_ARB_WEIGH_OPT_DIR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(736)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2e0..0x2ec - This register is used to config off or on weigh optimization"]
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir_ch_iter(&self) -> impl Iterator<Item = &TX_ARB_WEIGH_OPT_DIR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(736)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x334..0x340 - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub const fn in_crc_init_data_ch(&self, n: usize) -> &IN_CRC_INIT_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(820)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x334..0x340 - This register is used to config ch0 crc initial data(max 32 bit)"]
    #[inline(always)]
    pub fn in_crc_init_data_ch_iter(&self) -> impl Iterator<Item = &IN_CRC_INIT_DATA_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(820)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x338..0x344 - This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
    #[inline(always)]
    pub const fn rx_crc_width_ch(&self, n: usize) -> &RX_CRC_WIDTH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(824)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x338..0x344 - This register is used to confiig rx ch0 crc result width,2'b00 mean crc_width &lt;=8bit,2'b01 8&lt;crc_width&lt;=16 ,2'b10 mean 16&lt;crc_width &lt;=24,2'b11 mean 24&lt;crc_width&lt;=32"]
    #[inline(always)]
    pub fn rx_crc_width_ch_iter(&self) -> impl Iterator<Item = &RX_CRC_WIDTH_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(824)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x33c..0x348 - This register is used to clear ch0 crc result"]
    #[inline(always)]
    pub const fn in_crc_clear_ch(&self, n: usize) -> &IN_CRC_CLEAR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(828)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x33c..0x348 - This register is used to clear ch0 crc result"]
    #[inline(always)]
    pub fn in_crc_clear_ch_iter(&self) -> impl Iterator<Item = &IN_CRC_CLEAR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(828)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x340..0x34c - This register is used to store ch0 crc result"]
    #[inline(always)]
    pub const fn in_crc_final_result_ch(&self, n: usize) -> &IN_CRC_FINAL_RESULT_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(832)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x340..0x34c - This register is used to store ch0 crc result"]
    #[inline(always)]
    pub fn in_crc_final_result_ch_iter(&self) -> impl Iterator<Item = &IN_CRC_FINAL_RESULT_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(832)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x344..0x350 - This resister is used to config ch0 crc en for every bit"]
    #[inline(always)]
    pub const fn rx_crc_en_wr_data_ch(&self, n: usize) -> &RX_CRC_EN_WR_DATA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(836)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x344..0x350 - This resister is used to config ch0 crc en for every bit"]
    #[inline(always)]
    pub fn rx_crc_en_wr_data_ch_iter(&self) -> impl Iterator<Item = &RX_CRC_EN_WR_DATA_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(836)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x348..0x354 - This register is used to config ch0 crc en addr"]
    #[inline(always)]
    pub const fn rx_crc_en_addr_ch(&self, n: usize) -> &RX_CRC_EN_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(840)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x348..0x354 - This register is used to config ch0 crc en addr"]
    #[inline(always)]
    pub fn rx_crc_en_addr_ch_iter(&self) -> impl Iterator<Item = &RX_CRC_EN_ADDR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
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
            &*(self as *const Self)
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
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
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
            &*(self as *const Self)
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
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(848)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x354..0x360 - This register is used to config ch0 arbiter weigh"]
    #[inline(always)]
    pub const fn rx_ch_arb_weigh_ch(&self, n: usize) -> &RX_CH_ARB_WEIGH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(852)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x354..0x360 - This register is used to config ch0 arbiter weigh"]
    #[inline(always)]
    pub fn rx_ch_arb_weigh_ch_iter(&self) -> impl Iterator<Item = &RX_CH_ARB_WEIGH_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(852)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x358..0x364 - This register is used to config off or on weigh optimization"]
    #[inline(always)]
    pub const fn rx_arb_weigh_opt_dir_ch(&self, n: usize) -> &RX_ARB_WEIGH_OPT_DIR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(856)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x358..0x364 - This register is used to config off or on weigh optimization"]
    #[inline(always)]
    pub fn rx_arb_weigh_opt_dir_ch_iter(&self) -> impl Iterator<Item = &RX_ARB_WEIGH_OPT_DIR_CH> {
        (0..3).map(|n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(856)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x3ac..0x3b8 - Link descriptor configure of Rx channel 0"]
    #[inline(always)]
    pub const fn in_link_addr_ch(&self, n: usize) -> &IN_LINK_ADDR_CH {
        &self.in_link_addr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3ac..0x3b8 - Link descriptor configure of Rx channel 0"]
    #[inline(always)]
    pub fn in_link_addr_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_ADDR_CH> {
        self.in_link_addr_ch.iter()
    }
    #[doc = "0x3b8..0x3c4 - Link descriptor configure of Tx channel 0"]
    #[inline(always)]
    pub const fn out_link_addr_ch(&self, n: usize) -> &OUT_LINK_ADDR_CH {
        &self.out_link_addr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3b8..0x3c4 - Link descriptor configure of Tx channel 0"]
    #[inline(always)]
    pub fn out_link_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_LINK_ADDR_CH> {
        self.out_link_addr_ch.iter()
    }
    #[doc = "0x3c4 - The start address of accessible address space."]
    #[inline(always)]
    pub const fn intr_mem_start_addr(&self) -> &INTR_MEM_START_ADDR {
        &self.intr_mem_start_addr
    }
    #[doc = "0x3c8 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub const fn intr_mem_end_addr(&self) -> &INTR_MEM_END_ADDR {
        &self.intr_mem_end_addr
    }
    #[doc = "0x3cc - This retister is used to config arbiter time slice for tx dir"]
    #[inline(always)]
    pub const fn arb_timeout_tx(&self) -> &ARB_TIMEOUT_TX {
        &self.arb_timeout_tx
    }
    #[doc = "0x3d0 - This retister is used to config arbiter time slice for rx dir"]
    #[inline(always)]
    pub const fn arb_timeout_rx(&self) -> &ARB_TIMEOUT_RX {
        &self.arb_timeout_rx
    }
    #[doc = "0x3d4 - This register is used to config arbiter weigh function to on or off for tx dir"]
    #[inline(always)]
    pub const fn weight_en_tx(&self) -> &WEIGHT_EN_TX {
        &self.weight_en_tx
    }
    #[doc = "0x3d8 - This register is used to config arbiter weigh function to on or off for rx dir"]
    #[inline(always)]
    pub const fn weight_en_rx(&self) -> &WEIGHT_EN_RX {
        &self.weight_en_rx
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
#[doc = "OUT_INT_RAW_CH (rw) register accessor: Raw status interrupt of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch`] module"]
pub type OUT_INT_RAW_CH = crate::Reg<out_int_raw_ch::OUT_INT_RAW_CH_SPEC>;
#[doc = "Raw status interrupt of channel 0"]
pub mod out_int_raw_ch;
#[doc = "OUT_INT_ST_CH (r) register accessor: Masked interrupt of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_st_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch`] module"]
pub type OUT_INT_ST_CH = crate::Reg<out_int_st_ch::OUT_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt of channel 0"]
pub mod out_int_st_ch;
#[doc = "OUT_INT_ENA_CH (rw) register accessor: Interrupt enable bits of channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_ena_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_ena_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch`] module"]
pub type OUT_INT_ENA_CH = crate::Reg<out_int_ena_ch::OUT_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of channel 0"]
pub mod out_int_ena_ch;
#[doc = "OUT_INT_CLR_CH (w) register accessor: Interrupt clear bits of channel 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch`] module"]
pub type OUT_INT_CLR_CH = crate::Reg<out_int_clr_ch::OUT_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of channel 0"]
pub mod out_int_clr_ch;
#[doc = "AHB_TEST (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "reserved"]
pub mod ahb_test;
#[doc = "MISC_CONF (rw) register accessor: MISC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "MISC register"]
pub mod misc_conf;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
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
#[doc = "IN_LINK_CH (rw) register accessor: Link descriptor configure and control register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_ch`] module"]
pub type IN_LINK_CH = crate::Reg<in_link_ch::IN_LINK_CH_SPEC>;
#[doc = "Link descriptor configure and control register of Rx channel 0"]
pub mod in_link_ch;
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
#[doc = "OUT_CONF0_CH0 (rw) register accessor: Configure 0 register of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch0`] module"]
pub type OUT_CONF0_CH0 = crate::Reg<out_conf0_ch0::OUT_CONF0_CH0_SPEC>;
#[doc = "Configure 0 register of Tx channel 0"]
pub mod out_conf0_ch0;
#[doc = "OUT_CONF1_CH (rw) register accessor: Configure 1 register of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf1_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf1_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1_ch`] module"]
pub type OUT_CONF1_CH = crate::Reg<out_conf1_ch::OUT_CONF1_CH_SPEC>;
#[doc = "Configure 1 register of Tx channel 0"]
pub mod out_conf1_ch;
#[doc = "OUTFIFO_STATUS_CH (r) register accessor: Transmit FIFO status of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch`] module"]
pub type OUTFIFO_STATUS_CH = crate::Reg<outfifo_status_ch::OUTFIFO_STATUS_CH_SPEC>;
#[doc = "Transmit FIFO status of Tx channel 0"]
pub mod outfifo_status_ch;
#[doc = "OUT_PUSH_CH (rw) register accessor: Push control register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch`] module"]
pub type OUT_PUSH_CH = crate::Reg<out_push_ch::OUT_PUSH_CH_SPEC>;
#[doc = "Push control register of Rx channel 0"]
pub mod out_push_ch;
#[doc = "OUT_LINK_CH (rw) register accessor: Link descriptor configure and control register of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_ch`] module"]
pub type OUT_LINK_CH = crate::Reg<out_link_ch::OUT_LINK_CH_SPEC>;
#[doc = "Link descriptor configure and control register of Tx channel 0"]
pub mod out_link_ch;
#[doc = "OUT_STATE_CH (r) register accessor: Transmit status of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_state_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch`] module"]
pub type OUT_STATE_CH = crate::Reg<out_state_ch::OUT_STATE_CH_SPEC>;
#[doc = "Transmit status of Tx channel 0"]
pub mod out_state_ch;
#[doc = "OUT_EOF_DES_ADDR_CH (r) register accessor: Outlink descriptor address when EOF occurs of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch`] module"]
pub type OUT_EOF_DES_ADDR_CH = crate::Reg<out_eof_des_addr_ch::OUT_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Outlink descriptor address when EOF occurs of Tx channel 0"]
pub mod out_eof_des_addr_ch;
#[doc = "OUT_EOF_BFR_DES_ADDR_CH (r) register accessor: The last outlink descriptor address when EOF occurs of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr_ch`] module"]
pub type OUT_EOF_BFR_DES_ADDR_CH =
    crate::Reg<out_eof_bfr_des_addr_ch::OUT_EOF_BFR_DES_ADDR_CH_SPEC>;
#[doc = "The last outlink descriptor address when EOF occurs of Tx channel 0"]
pub mod out_eof_bfr_des_addr_ch;
#[doc = "OUT_DSCR_CH (r) register accessor: Current inlink descriptor address of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch`] module"]
pub type OUT_DSCR_CH = crate::Reg<out_dscr_ch::OUT_DSCR_CH_SPEC>;
#[doc = "Current inlink descriptor address of Tx channel 0"]
pub mod out_dscr_ch;
#[doc = "OUT_DSCR_BF0_CH (r) register accessor: The last inlink descriptor address of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf0_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch`] module"]
pub type OUT_DSCR_BF0_CH = crate::Reg<out_dscr_bf0_ch::OUT_DSCR_BF0_CH_SPEC>;
#[doc = "The last inlink descriptor address of Tx channel 0"]
pub mod out_dscr_bf0_ch;
#[doc = "OUT_DSCR_BF1_CH (r) register accessor: The second-to-last inlink descriptor address of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_dscr_bf1_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch`] module"]
pub type OUT_DSCR_BF1_CH = crate::Reg<out_dscr_bf1_ch::OUT_DSCR_BF1_CH_SPEC>;
#[doc = "The second-to-last inlink descriptor address of Tx channel 0"]
pub mod out_dscr_bf1_ch;
#[doc = "OUT_PRI_CH (rw) register accessor: Priority register of Tx channel 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_pri_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_pri_ch`] module"]
pub type OUT_PRI_CH = crate::Reg<out_pri_ch::OUT_PRI_CH_SPEC>;
#[doc = "Priority register of Tx channel 0."]
pub mod out_pri_ch;
#[doc = "OUT_PERI_SEL_CH (rw) register accessor: Peripheral selection of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_peri_sel_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_peri_sel_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel_ch`] module"]
pub type OUT_PERI_SEL_CH = crate::Reg<out_peri_sel_ch::OUT_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection of Tx channel 0"]
pub mod out_peri_sel_ch;
#[doc = "OUT_CONF0_CH (rw) register accessor: Configure 0 register of Tx channel 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch`] module"]
pub type OUT_CONF0_CH = crate::Reg<out_conf0_ch::OUT_CONF0_CH_SPEC>;
#[doc = "Configure 0 register of Tx channel 1"]
pub mod out_conf0_ch;
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
#[doc = "TX_CH_ARB_WEIGH_CH (rw) register accessor: This register is used to config ch0 arbiter weigh\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_ch_arb_weigh_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ch_arb_weigh_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ch_arb_weigh_ch`] module"]
pub type TX_CH_ARB_WEIGH_CH = crate::Reg<tx_ch_arb_weigh_ch::TX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "This register is used to config ch0 arbiter weigh"]
pub mod tx_ch_arb_weigh_ch;
#[doc = "TX_ARB_WEIGH_OPT_DIR_CH (rw) register accessor: This register is used to config off or on weigh optimization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_arb_weigh_opt_dir_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_arb_weigh_opt_dir_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_arb_weigh_opt_dir_ch`] module"]
pub type TX_ARB_WEIGH_OPT_DIR_CH =
    crate::Reg<tx_arb_weigh_opt_dir_ch::TX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "This register is used to config off or on weigh optimization"]
pub mod tx_arb_weigh_opt_dir_ch;
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
#[doc = "RX_CH_ARB_WEIGH_CH (rw) register accessor: This register is used to config ch0 arbiter weigh\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_ch_arb_weigh_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ch_arb_weigh_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch_arb_weigh_ch`] module"]
pub type RX_CH_ARB_WEIGH_CH = crate::Reg<rx_ch_arb_weigh_ch::RX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "This register is used to config ch0 arbiter weigh"]
pub mod rx_ch_arb_weigh_ch;
#[doc = "RX_ARB_WEIGH_OPT_DIR_CH (rw) register accessor: This register is used to config off or on weigh optimization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_arb_weigh_opt_dir_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_arb_weigh_opt_dir_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_arb_weigh_opt_dir_ch`] module"]
pub type RX_ARB_WEIGH_OPT_DIR_CH =
    crate::Reg<rx_arb_weigh_opt_dir_ch::RX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "This register is used to config off or on weigh optimization"]
pub mod rx_arb_weigh_opt_dir_ch;
#[doc = "IN_LINK_ADDR_CH (rw) register accessor: Link descriptor configure of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_addr_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_addr_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch`] module"]
pub type IN_LINK_ADDR_CH = crate::Reg<in_link_addr_ch::IN_LINK_ADDR_CH_SPEC>;
#[doc = "Link descriptor configure of Rx channel 0"]
pub mod in_link_addr_ch;
#[doc = "OUT_LINK_ADDR_CH (rw) register accessor: Link descriptor configure of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_addr_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_addr_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch`] module"]
pub type OUT_LINK_ADDR_CH = crate::Reg<out_link_addr_ch::OUT_LINK_ADDR_CH_SPEC>;
#[doc = "Link descriptor configure of Tx channel 0"]
pub mod out_link_addr_ch;
#[doc = "INTR_MEM_START_ADDR (rw) register accessor: The start address of accessible address space.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mem_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_start_addr`] module"]
pub type INTR_MEM_START_ADDR = crate::Reg<intr_mem_start_addr::INTR_MEM_START_ADDR_SPEC>;
#[doc = "The start address of accessible address space."]
pub mod intr_mem_start_addr;
#[doc = "INTR_MEM_END_ADDR (rw) register accessor: The end address of accessible address space. The access address beyond this range would lead to descriptor error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mem_end_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_end_addr`] module"]
pub type INTR_MEM_END_ADDR = crate::Reg<intr_mem_end_addr::INTR_MEM_END_ADDR_SPEC>;
#[doc = "The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub mod intr_mem_end_addr;
#[doc = "ARB_TIMEOUT_TX (rw) register accessor: This retister is used to config arbiter time slice for tx dir\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_timeout_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_timeout_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout_tx`] module"]
pub type ARB_TIMEOUT_TX = crate::Reg<arb_timeout_tx::ARB_TIMEOUT_TX_SPEC>;
#[doc = "This retister is used to config arbiter time slice for tx dir"]
pub mod arb_timeout_tx;
#[doc = "ARB_TIMEOUT_RX (rw) register accessor: This retister is used to config arbiter time slice for rx dir\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arb_timeout_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arb_timeout_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_timeout_rx`] module"]
pub type ARB_TIMEOUT_RX = crate::Reg<arb_timeout_rx::ARB_TIMEOUT_RX_SPEC>;
#[doc = "This retister is used to config arbiter time slice for rx dir"]
pub mod arb_timeout_rx;
#[doc = "WEIGHT_EN_TX (rw) register accessor: This register is used to config arbiter weigh function to on or off for tx dir\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`weight_en_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`weight_en_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en_tx`] module"]
pub type WEIGHT_EN_TX = crate::Reg<weight_en_tx::WEIGHT_EN_TX_SPEC>;
#[doc = "This register is used to config arbiter weigh function to on or off for tx dir"]
pub mod weight_en_tx;
#[doc = "WEIGHT_EN_RX (rw) register accessor: This register is used to config arbiter weigh function to on or off for rx dir\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`weight_en_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`weight_en_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weight_en_rx`] module"]
pub type WEIGHT_EN_RX = crate::Reg<weight_en_rx::WEIGHT_EN_RX_SPEC>;
#[doc = "This register is used to config arbiter weigh function to on or off for rx dir"]
pub mod weight_en_rx;
