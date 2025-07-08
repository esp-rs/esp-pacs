#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    in_int_raw_ch0: IN_INT_RAW_CH0,
    in_int_st_ch: (),
    _reserved2: [u8; 0x04],
    in_int_ena_ch: (),
    _reserved3: [u8; 0x04],
    in_int_clr_ch: (),
    _reserved4: [u8; 0x04],
    in_int_raw_ch1: IN_INT_RAW_CH1,
    _reserved5: [u8; 0x0c],
    in_int_raw_ch2: IN_INT_RAW_CH2,
    _reserved6: [u8; 0x0c],
    out_int_raw_ch: (),
    _reserved7: [u8; 0x04],
    out_int_st_ch: (),
    _reserved8: [u8; 0x04],
    out_int_ena_ch: (),
    _reserved9: [u8; 0x04],
    out_int_clr_ch: (),
    _reserved10: [u8; 0x24],
    ahb_test: AHB_TEST,
    misc_conf: MISC_CONF,
    date: DATE,
    _reserved13: [u8; 0x04],
    in_conf0_ch: (),
    _reserved14: [u8; 0x04],
    in_conf1_ch: (),
    _reserved15: [u8; 0x04],
    infifo_status_ch: (),
    _reserved16: [u8; 0x04],
    in_pop_ch: (),
    _reserved17: [u8; 0x04],
    in_link_ch: (),
    _reserved18: [u8; 0x04],
    in_state_ch: (),
    _reserved19: [u8; 0x04],
    in_suc_eof_des_addr_ch: (),
    _reserved20: [u8; 0x04],
    in_err_eof_des_addr_ch: (),
    _reserved21: [u8; 0x04],
    in_dscr_ch: (),
    _reserved22: [u8; 0x04],
    in_dscr_bf0_ch: (),
    _reserved23: [u8; 0x04],
    in_dscr_bf1_ch: (),
    _reserved24: [u8; 0x04],
    in_pri_ch: (),
    _reserved25: [u8; 0x04],
    in_peri_sel_ch: (),
    _reserved26: [u8; 0x30],
    out_conf0_ch0: OUT_CONF0_CH0,
    out_conf1_ch: (),
    _reserved28: [u8; 0x04],
    outfifo_status_ch: (),
    _reserved29: [u8; 0x04],
    out_push_ch: (),
    _reserved30: [u8; 0x04],
    out_link_ch: (),
    _reserved31: [u8; 0x04],
    out_state_ch: (),
    _reserved32: [u8; 0x04],
    out_eof_des_addr_ch: (),
    _reserved33: [u8; 0x04],
    out_eof_bfr_des_addr_ch: (),
    _reserved34: [u8; 0x04],
    out_dscr_ch: (),
    _reserved35: [u8; 0x04],
    out_dscr_bf0_ch: (),
    _reserved36: [u8; 0x04],
    out_dscr_bf1_ch: (),
    _reserved37: [u8; 0x04],
    out_pri_ch: (),
    _reserved38: [u8; 0x04],
    out_peri_sel_ch: (),
    _reserved39: [u8; 0x90],
    out_conf0_ch: (),
    _reserved40: [u8; 0x014c],
    tx_ch_arb_weigh_ch: (),
    _reserved41: [u8; 0x04],
    tx_arb_weigh_opt_dir_ch: (),
    _reserved42: [u8; 0x74],
    rx_ch_arb_weigh_ch: (),
    _reserved43: [u8; 0x04],
    rx_arb_weigh_opt_dir_ch: (),
    _reserved44: [u8; 0x54],
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
    #[doc = "0x00 - Raw interrupt status of RX channel 0"]
    #[inline(always)]
    pub const fn in_int_raw_ch0(&self) -> &IN_INT_RAW_CH0 {
        &self.in_int_raw_ch0
    }
    #[doc = "0x04..0x10 - Masked interrupt status of RX channel %s"]
    #[inline(always)]
    pub const fn in_int_st_ch(&self, n: usize) -> &IN_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x10 - Masked interrupt status of RX channel %s"]
    #[inline(always)]
    pub fn in_int_st_ch_iter(&self) -> impl Iterator<Item = &IN_INT_ST_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x08..0x14 - Interrupt enable bits of RX channel %s"]
    #[inline(always)]
    pub const fn in_int_ena_ch(&self, n: usize) -> &IN_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x14 - Interrupt enable bits of RX channel %s"]
    #[inline(always)]
    pub fn in_int_ena_ch_iter(&self) -> impl Iterator<Item = &IN_INT_ENA_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x0c..0x18 - Interrupt clear bits of RX channel %s"]
    #[inline(always)]
    pub const fn in_int_clr_ch(&self, n: usize) -> &IN_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x18 - Interrupt clear bits of RX channel %s"]
    #[inline(always)]
    pub fn in_int_clr_ch_iter(&self) -> impl Iterator<Item = &IN_INT_CLR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x10 - Raw interrupt status interrupt of RX channel 1"]
    #[inline(always)]
    pub const fn in_int_raw_ch1(&self) -> &IN_INT_RAW_CH1 {
        &self.in_int_raw_ch1
    }
    #[doc = "0x20 - Raw interrupt status interrupt of RX channel 2"]
    #[inline(always)]
    pub const fn in_int_raw_ch2(&self) -> &IN_INT_RAW_CH2 {
        &self.in_int_raw_ch2
    }
    #[doc = "0x30..0x3c - Raw interrupt status of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_raw_ch(&self, n: usize) -> &OUT_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(48)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x3c - Raw interrupt status of TX channel %s"]
    #[inline(always)]
    pub fn out_int_raw_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_RAW_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(48)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x34..0x40 - Masked interrupt status of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_st_ch(&self, n: usize) -> &OUT_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(52)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x40 - Masked interrupt status of TX channel %s"]
    #[inline(always)]
    pub fn out_int_st_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_ST_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(52)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x38..0x44 - Interrupt enable bits of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_ena_ch(&self, n: usize) -> &OUT_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(56)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x44 - Interrupt enable bits of TX channel %s"]
    #[inline(always)]
    pub fn out_int_ena_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_ENA_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(56)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x3c..0x48 - Interrupt clear bits of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_clr_ch(&self, n: usize) -> &OUT_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(60)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0x48 - Interrupt clear bits of TX channel %s"]
    #[inline(always)]
    pub fn out_int_clr_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_CLR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
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
    #[doc = "0x64 - Miscellaneous register"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0x68 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x70..0x7c - Configuration register 0 of RX channel %s"]
    #[inline(always)]
    pub const fn in_conf0_ch(&self, n: usize) -> &IN_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x7c - Configuration register 0 of RX channel %s"]
    #[inline(always)]
    pub fn in_conf0_ch_iter(&self) -> impl Iterator<Item = &IN_CONF0_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(112)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x74..0x80 - Configuration register 1 of RX channel %s"]
    #[inline(always)]
    pub const fn in_conf1_ch(&self, n: usize) -> &IN_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(116)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x74..0x80 - Configuration register 1 of RX channel %s"]
    #[inline(always)]
    pub fn in_conf1_ch_iter(&self) -> impl Iterator<Item = &IN_CONF1_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(116)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x78..0x84 - Receive FIFO status of RX channel %s"]
    #[inline(always)]
    pub const fn infifo_status_ch(&self, n: usize) -> &INFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(120)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x78..0x84 - Receive FIFO status of RX channel %s"]
    #[inline(always)]
    pub fn infifo_status_ch_iter(&self) -> impl Iterator<Item = &INFIFO_STATUS_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(120)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x7c..0x88 - Pop control register of RX channel %s"]
    #[inline(always)]
    pub const fn in_pop_ch(&self, n: usize) -> &IN_POP_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(124)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x7c..0x88 - Pop control register of RX channel %s"]
    #[inline(always)]
    pub fn in_pop_ch_iter(&self) -> impl Iterator<Item = &IN_POP_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(124)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x80..0x8c - Linked list descriptor configuration and control register of RX channel %s"]
    #[inline(always)]
    pub const fn in_link_ch(&self, n: usize) -> &IN_LINK_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(128)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x8c - Linked list descriptor configuration and control register of RX channel %s"]
    #[inline(always)]
    pub fn in_link_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(128)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x84..0x90 - Receive status of RX channel %s"]
    #[inline(always)]
    pub const fn in_state_ch(&self, n: usize) -> &IN_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(132)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x84..0x90 - Receive status of RX channel %s"]
    #[inline(always)]
    pub fn in_state_ch_iter(&self) -> impl Iterator<Item = &IN_STATE_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(132)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x88..0x94 - Receive descriptor address when EOF occurs on RX channel %s"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch(&self, n: usize) -> &IN_SUC_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(136)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88..0x94 - Receive descriptor address when EOF occurs on RX channel %s"]
    #[inline(always)]
    pub fn in_suc_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &IN_SUC_EOF_DES_ADDR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(136)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x8c..0x98 - Receive descriptor address when errors occur of RX channel %s"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch(&self, n: usize) -> &IN_ERR_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(140)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x8c..0x98 - Receive descriptor address when errors occur of RX channel %s"]
    #[inline(always)]
    pub fn in_err_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &IN_ERR_EOF_DES_ADDR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(140)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x90..0x9c - Current receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub const fn in_dscr_ch(&self, n: usize) -> &IN_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(144)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x90..0x9c - Current receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub fn in_dscr_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(144)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x94..0xa0 - The last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch(&self, n: usize) -> &IN_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(148)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x94..0xa0 - The last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub fn in_dscr_bf0_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_BF0_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(148)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x98..0xa4 - The second-to-last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch(&self, n: usize) -> &IN_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(152)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x98..0xa4 - The second-to-last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub fn in_dscr_bf1_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_BF1_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(152)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x9c..0xa8 - Priority register of RX channel %s"]
    #[inline(always)]
    pub const fn in_pri_ch(&self, n: usize) -> &IN_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(156)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c..0xa8 - Priority register of RX channel %s"]
    #[inline(always)]
    pub fn in_pri_ch_iter(&self) -> impl Iterator<Item = &IN_PRI_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(156)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xa0..0xac - Peripheral selection register of RX channel %s"]
    #[inline(always)]
    pub const fn in_peri_sel_ch(&self, n: usize) -> &IN_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(160)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xac - Peripheral selection register of RX channel %s"]
    #[inline(always)]
    pub fn in_peri_sel_ch_iter(&self) -> impl Iterator<Item = &IN_PERI_SEL_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(160)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xd0 - Configuration register 0 of TX channel 0"]
    #[inline(always)]
    pub const fn out_conf0_ch0(&self) -> &OUT_CONF0_CH0 {
        &self.out_conf0_ch0
    }
    #[doc = "0xd4..0xe0 - Configuration register 1 of TX channel %s"]
    #[inline(always)]
    pub const fn out_conf1_ch(&self, n: usize) -> &OUT_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(212)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd4..0xe0 - Configuration register 1 of TX channel %s"]
    #[inline(always)]
    pub fn out_conf1_ch_iter(&self) -> impl Iterator<Item = &OUT_CONF1_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(212)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xd8..0xe4 - Transmit FIFO status of TX channel %s"]
    #[inline(always)]
    pub const fn outfifo_status_ch(&self, n: usize) -> &OUTFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(216)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd8..0xe4 - Transmit FIFO status of TX channel %s"]
    #[inline(always)]
    pub fn outfifo_status_ch_iter(&self) -> impl Iterator<Item = &OUTFIFO_STATUS_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(216)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xdc..0xe8 - Push control register of TX channel %s"]
    #[inline(always)]
    pub const fn out_push_ch(&self, n: usize) -> &OUT_PUSH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(220)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xdc..0xe8 - Push control register of TX channel %s"]
    #[inline(always)]
    pub fn out_push_ch_iter(&self) -> impl Iterator<Item = &OUT_PUSH_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(220)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xe0..0xec - Linked list descriptor configuration and control register of TX channel %s"]
    #[inline(always)]
    pub const fn out_link_ch(&self, n: usize) -> &OUT_LINK_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(224)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe0..0xec - Linked list descriptor configuration and control register of TX channel %s"]
    #[inline(always)]
    pub fn out_link_ch_iter(&self) -> impl Iterator<Item = &OUT_LINK_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(224)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xe4..0xf0 - Transmit status of TX channel %s"]
    #[inline(always)]
    pub const fn out_state_ch(&self, n: usize) -> &OUT_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(228)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe4..0xf0 - Transmit status of TX channel %s"]
    #[inline(always)]
    pub fn out_state_ch_iter(&self) -> impl Iterator<Item = &OUT_STATE_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(228)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xe8..0xf4 - Transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch(&self, n: usize) -> &OUT_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(232)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xe8..0xf4 - Transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub fn out_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_EOF_DES_ADDR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(232)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xec..0xf8 - The last transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch(&self, n: usize) -> &OUT_EOF_BFR_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(236)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xec..0xf8 - The last transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub fn out_eof_bfr_des_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_EOF_BFR_DES_ADDR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(236)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xf0..0xfc - Current transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub const fn out_dscr_ch(&self, n: usize) -> &OUT_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(240)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf0..0xfc - Current transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub fn out_dscr_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(240)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xf4..0x100 - The last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch(&self, n: usize) -> &OUT_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(244)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf4..0x100 - The last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub fn out_dscr_bf0_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_BF0_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(244)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xf8..0x104 - The second-to-last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch(&self, n: usize) -> &OUT_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(248)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf8..0x104 - The second-to-last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub fn out_dscr_bf1_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_BF1_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(248)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0xfc..0x108 - Priority register of TX channel %s"]
    #[inline(always)]
    pub const fn out_pri_ch(&self, n: usize) -> &OUT_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(252)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xfc..0x108 - Priority register of TX channel %s"]
    #[inline(always)]
    pub fn out_pri_ch_iter(&self) -> impl Iterator<Item = &OUT_PRI_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(252)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x100..0x10c - Peripheral selection register of TX channel %s"]
    #[inline(always)]
    pub const fn out_peri_sel_ch(&self, n: usize) -> &OUT_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x10c - Peripheral selection register of TX channel %s"]
    #[inline(always)]
    pub fn out_peri_sel_ch_iter(&self) -> impl Iterator<Item = &OUT_PERI_SEL_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x190..0x198 - Configuration register 0 of TX channel %s"]
    #[inline(always)]
    pub const fn out_conf0_ch(&self, n: usize) -> &OUT_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x198 - Configuration register 0 of TX channel %s"]
    #[inline(always)]
    pub fn out_conf0_ch_iter(&self) -> impl Iterator<Item = &OUT_CONF0_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x2dc..0x2e8 - TX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub const fn tx_ch_arb_weigh_ch(&self, n: usize) -> &TX_CH_ARB_WEIGH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(732)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2dc..0x2e8 - TX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub fn tx_ch_arb_weigh_ch_iter(&self) -> impl Iterator<Item = &TX_CH_ARB_WEIGH_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(732)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x2e0..0x2ec - TX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn tx_arb_weigh_opt_dir_ch(&self, n: usize) -> &TX_ARB_WEIGH_OPT_DIR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(736)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2e0..0x2ec - TX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir_ch_iter(&self) -> impl Iterator<Item = &TX_ARB_WEIGH_OPT_DIR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(736)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x354..0x360 - RX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub const fn rx_ch_arb_weigh_ch(&self, n: usize) -> &RX_CH_ARB_WEIGH_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(852)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x354..0x360 - RX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub fn rx_ch_arb_weigh_ch_iter(&self) -> impl Iterator<Item = &RX_CH_ARB_WEIGH_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(852)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x358..0x364 - RX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn rx_arb_weigh_opt_dir_ch(&self, n: usize) -> &RX_ARB_WEIGH_OPT_DIR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(856)
                .add(40 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x358..0x364 - RX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub fn rx_arb_weigh_opt_dir_ch_iter(&self) -> impl Iterator<Item = &RX_ARB_WEIGH_OPT_DIR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(856)
                .add(40 * n)
                .cast()
        })
    }
    #[doc = "0x3ac..0x3b8 - Link list descriptor address configuration of RX channel %s"]
    #[inline(always)]
    pub const fn in_link_addr_ch(&self, n: usize) -> &IN_LINK_ADDR_CH {
        &self.in_link_addr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3ac..0x3b8 - Link list descriptor address configuration of RX channel %s"]
    #[inline(always)]
    pub fn in_link_addr_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_ADDR_CH> {
        self.in_link_addr_ch.iter()
    }
    #[doc = "0x3b8..0x3c4 - Link list descriptor address configuration of TX channel %s"]
    #[inline(always)]
    pub const fn out_link_addr_ch(&self, n: usize) -> &OUT_LINK_ADDR_CH {
        &self.out_link_addr_ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3b8..0x3c4 - Link list descriptor address configuration of TX channel %s"]
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
}
#[doc = "IN_INT_RAW_CH0 (rw) register accessor: Raw interrupt status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_raw_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_raw_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch0`] module"]
pub type IN_INT_RAW_CH0 = crate::Reg<in_int_raw_ch0::IN_INT_RAW_CH0_SPEC>;
#[doc = "Raw interrupt status of RX channel 0"]
pub mod in_int_raw_ch0;
#[doc = "IN_INT_ST_CH (r) register accessor: Masked interrupt status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_st_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch`] module"]
pub type IN_INT_ST_CH = crate::Reg<in_int_st_ch::IN_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt status of RX channel %s"]
pub mod in_int_st_ch;
#[doc = "IN_INT_ENA_CH (rw) register accessor: Interrupt enable bits of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_ena_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_ena_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch`] module"]
pub type IN_INT_ENA_CH = crate::Reg<in_int_ena_ch::IN_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of RX channel %s"]
pub mod in_int_ena_ch;
#[doc = "IN_INT_CLR_CH (w) register accessor: Interrupt clear bits of RX channel %s\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch`] module"]
pub type IN_INT_CLR_CH = crate::Reg<in_int_clr_ch::IN_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of RX channel %s"]
pub mod in_int_clr_ch;
#[doc = "IN_INT_RAW_CH1 (rw) register accessor: Raw interrupt status interrupt of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_raw_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_raw_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch1`] module"]
pub type IN_INT_RAW_CH1 = crate::Reg<in_int_raw_ch1::IN_INT_RAW_CH1_SPEC>;
#[doc = "Raw interrupt status interrupt of RX channel 1"]
pub mod in_int_raw_ch1;
#[doc = "IN_INT_RAW_CH2 (rw) register accessor: Raw interrupt status interrupt of RX channel 2\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_raw_ch2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_raw_ch2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch2`] module"]
pub type IN_INT_RAW_CH2 = crate::Reg<in_int_raw_ch2::IN_INT_RAW_CH2_SPEC>;
#[doc = "Raw interrupt status interrupt of RX channel 2"]
pub mod in_int_raw_ch2;
#[doc = "OUT_INT_RAW_CH (rw) register accessor: Raw interrupt status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_raw_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_raw_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch`] module"]
pub type OUT_INT_RAW_CH = crate::Reg<out_int_raw_ch::OUT_INT_RAW_CH_SPEC>;
#[doc = "Raw interrupt status of TX channel %s"]
pub mod out_int_raw_ch;
#[doc = "OUT_INT_ST_CH (r) register accessor: Masked interrupt status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_st_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch`] module"]
pub type OUT_INT_ST_CH = crate::Reg<out_int_st_ch::OUT_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt status of TX channel %s"]
pub mod out_int_st_ch;
#[doc = "OUT_INT_ENA_CH (rw) register accessor: Interrupt enable bits of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_ena_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_ena_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch`] module"]
pub type OUT_INT_ENA_CH = crate::Reg<out_int_ena_ch::OUT_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of TX channel %s"]
pub mod out_int_ena_ch;
#[doc = "OUT_INT_CLR_CH (w) register accessor: Interrupt clear bits of TX channel %s\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch`] module"]
pub type OUT_INT_CLR_CH = crate::Reg<out_int_clr_ch::OUT_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of TX channel %s"]
pub mod out_int_clr_ch;
#[doc = "AHB_TEST (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "reserved"]
pub mod ahb_test;
#[doc = "MISC_CONF (rw) register accessor: Miscellaneous register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "Miscellaneous register"]
pub mod misc_conf;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "IN_CONF0_CH (rw) register accessor: Configuration register 0 of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf0_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf0_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch`] module"]
pub type IN_CONF0_CH = crate::Reg<in_conf0_ch::IN_CONF0_CH_SPEC>;
#[doc = "Configuration register 0 of RX channel %s"]
pub mod in_conf0_ch;
#[doc = "IN_CONF1_CH (rw) register accessor: Configuration register 1 of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf1_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf1_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1_ch`] module"]
pub type IN_CONF1_CH = crate::Reg<in_conf1_ch::IN_CONF1_CH_SPEC>;
#[doc = "Configuration register 1 of RX channel %s"]
pub mod in_conf1_ch;
#[doc = "INFIFO_STATUS_CH (r) register accessor: Receive FIFO status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch`] module"]
pub type INFIFO_STATUS_CH = crate::Reg<infifo_status_ch::INFIFO_STATUS_CH_SPEC>;
#[doc = "Receive FIFO status of RX channel %s"]
pub mod infifo_status_ch;
#[doc = "IN_POP_CH (rw) register accessor: Pop control register of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pop_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pop_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch`] module"]
pub type IN_POP_CH = crate::Reg<in_pop_ch::IN_POP_CH_SPEC>;
#[doc = "Pop control register of RX channel %s"]
pub mod in_pop_ch;
#[doc = "IN_LINK_CH (rw) register accessor: Linked list descriptor configuration and control register of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_ch`] module"]
pub type IN_LINK_CH = crate::Reg<in_link_ch::IN_LINK_CH_SPEC>;
#[doc = "Linked list descriptor configuration and control register of RX channel %s"]
pub mod in_link_ch;
#[doc = "IN_STATE_CH (r) register accessor: Receive status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_state_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch`] module"]
pub type IN_STATE_CH = crate::Reg<in_state_ch::IN_STATE_CH_SPEC>;
#[doc = "Receive status of RX channel %s"]
pub mod in_state_ch;
#[doc = "IN_SUC_EOF_DES_ADDR_CH (r) register accessor: Receive descriptor address when EOF occurs on RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_suc_eof_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH = crate::Reg<in_suc_eof_des_addr_ch::IN_SUC_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Receive descriptor address when EOF occurs on RX channel %s"]
pub mod in_suc_eof_des_addr_ch;
#[doc = "IN_ERR_EOF_DES_ADDR_CH (r) register accessor: Receive descriptor address when errors occur of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_err_eof_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH = crate::Reg<in_err_eof_des_addr_ch::IN_ERR_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Receive descriptor address when errors occur of RX channel %s"]
pub mod in_err_eof_des_addr_ch;
#[doc = "IN_DSCR_CH (r) register accessor: Current receive descriptor address of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch`] module"]
pub type IN_DSCR_CH = crate::Reg<in_dscr_ch::IN_DSCR_CH_SPEC>;
#[doc = "Current receive descriptor address of RX channel %s"]
pub mod in_dscr_ch;
#[doc = "IN_DSCR_BF0_CH (r) register accessor: The last receive descriptor address of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf0_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch`] module"]
pub type IN_DSCR_BF0_CH = crate::Reg<in_dscr_bf0_ch::IN_DSCR_BF0_CH_SPEC>;
#[doc = "The last receive descriptor address of RX channel %s"]
pub mod in_dscr_bf0_ch;
#[doc = "IN_DSCR_BF1_CH (r) register accessor: The second-to-last receive descriptor address of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf1_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch`] module"]
pub type IN_DSCR_BF1_CH = crate::Reg<in_dscr_bf1_ch::IN_DSCR_BF1_CH_SPEC>;
#[doc = "The second-to-last receive descriptor address of RX channel %s"]
pub mod in_dscr_bf1_ch;
#[doc = "IN_PRI_CH (rw) register accessor: Priority register of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pri_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pri_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pri_ch`] module"]
pub type IN_PRI_CH = crate::Reg<in_pri_ch::IN_PRI_CH_SPEC>;
#[doc = "Priority register of RX channel %s"]
pub mod in_pri_ch;
#[doc = "IN_PERI_SEL_CH (rw) register accessor: Peripheral selection register of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_peri_sel_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_peri_sel_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel_ch`] module"]
pub type IN_PERI_SEL_CH = crate::Reg<in_peri_sel_ch::IN_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection register of RX channel %s"]
pub mod in_peri_sel_ch;
#[doc = "OUT_CONF0_CH0 (rw) register accessor: Configuration register 0 of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf0_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf0_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch0`] module"]
pub type OUT_CONF0_CH0 = crate::Reg<out_conf0_ch0::OUT_CONF0_CH0_SPEC>;
#[doc = "Configuration register 0 of TX channel 0"]
pub mod out_conf0_ch0;
#[doc = "OUT_CONF1_CH (rw) register accessor: Configuration register 1 of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf1_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf1_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1_ch`] module"]
pub type OUT_CONF1_CH = crate::Reg<out_conf1_ch::OUT_CONF1_CH_SPEC>;
#[doc = "Configuration register 1 of TX channel %s"]
pub mod out_conf1_ch;
#[doc = "OUTFIFO_STATUS_CH (r) register accessor: Transmit FIFO status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch`] module"]
pub type OUTFIFO_STATUS_CH = crate::Reg<outfifo_status_ch::OUTFIFO_STATUS_CH_SPEC>;
#[doc = "Transmit FIFO status of TX channel %s"]
pub mod outfifo_status_ch;
#[doc = "OUT_PUSH_CH (rw) register accessor: Push control register of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_push_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_push_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch`] module"]
pub type OUT_PUSH_CH = crate::Reg<out_push_ch::OUT_PUSH_CH_SPEC>;
#[doc = "Push control register of TX channel %s"]
pub mod out_push_ch;
#[doc = "OUT_LINK_CH (rw) register accessor: Linked list descriptor configuration and control register of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_ch`] module"]
pub type OUT_LINK_CH = crate::Reg<out_link_ch::OUT_LINK_CH_SPEC>;
#[doc = "Linked list descriptor configuration and control register of TX channel %s"]
pub mod out_link_ch;
#[doc = "OUT_STATE_CH (r) register accessor: Transmit status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_state_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch`] module"]
pub type OUT_STATE_CH = crate::Reg<out_state_ch::OUT_STATE_CH_SPEC>;
#[doc = "Transmit status of TX channel %s"]
pub mod out_state_ch;
#[doc = "OUT_EOF_DES_ADDR_CH (r) register accessor: Transmit descriptor address when EOF occurs on TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch`] module"]
pub type OUT_EOF_DES_ADDR_CH = crate::Reg<out_eof_des_addr_ch::OUT_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Transmit descriptor address when EOF occurs on TX channel %s"]
pub mod out_eof_des_addr_ch;
#[doc = "OUT_EOF_BFR_DES_ADDR_CH (r) register accessor: The last transmit descriptor address when EOF occurs on TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_bfr_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_bfr_des_addr_ch`] module"]
pub type OUT_EOF_BFR_DES_ADDR_CH =
    crate::Reg<out_eof_bfr_des_addr_ch::OUT_EOF_BFR_DES_ADDR_CH_SPEC>;
#[doc = "The last transmit descriptor address when EOF occurs on TX channel %s"]
pub mod out_eof_bfr_des_addr_ch;
#[doc = "OUT_DSCR_CH (r) register accessor: Current transmit descriptor address of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch`] module"]
pub type OUT_DSCR_CH = crate::Reg<out_dscr_ch::OUT_DSCR_CH_SPEC>;
#[doc = "Current transmit descriptor address of TX channel %s"]
pub mod out_dscr_ch;
#[doc = "OUT_DSCR_BF0_CH (r) register accessor: The last transmit descriptor address of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf0_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch`] module"]
pub type OUT_DSCR_BF0_CH = crate::Reg<out_dscr_bf0_ch::OUT_DSCR_BF0_CH_SPEC>;
#[doc = "The last transmit descriptor address of TX channel %s"]
pub mod out_dscr_bf0_ch;
#[doc = "OUT_DSCR_BF1_CH (r) register accessor: The second-to-last transmit descriptor address of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf1_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch`] module"]
pub type OUT_DSCR_BF1_CH = crate::Reg<out_dscr_bf1_ch::OUT_DSCR_BF1_CH_SPEC>;
#[doc = "The second-to-last transmit descriptor address of TX channel %s"]
pub mod out_dscr_bf1_ch;
#[doc = "OUT_PRI_CH (rw) register accessor: Priority register of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_pri_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_pri_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_pri_ch`] module"]
pub type OUT_PRI_CH = crate::Reg<out_pri_ch::OUT_PRI_CH_SPEC>;
#[doc = "Priority register of TX channel %s"]
pub mod out_pri_ch;
#[doc = "OUT_PERI_SEL_CH (rw) register accessor: Peripheral selection register of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_peri_sel_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_peri_sel_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel_ch`] module"]
pub type OUT_PERI_SEL_CH = crate::Reg<out_peri_sel_ch::OUT_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection register of TX channel %s"]
pub mod out_peri_sel_ch;
#[doc = "OUT_CONF0_CH (rw) register accessor: Configuration register 0 of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf0_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf0_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch`] module"]
pub type OUT_CONF0_CH = crate::Reg<out_conf0_ch::OUT_CONF0_CH_SPEC>;
#[doc = "Configuration register 0 of TX channel %s"]
pub mod out_conf0_ch;
#[doc = "TX_CH_ARB_WEIGH_CH (rw) register accessor: TX channel %s arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weigh_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weigh_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ch_arb_weigh_ch`] module"]
pub type TX_CH_ARB_WEIGH_CH = crate::Reg<tx_ch_arb_weigh_ch::TX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "TX channel %s arbitration weight configuration register"]
pub mod tx_ch_arb_weigh_ch;
#[doc = "TX_ARB_WEIGH_OPT_DIR_CH (rw) register accessor: TX channel %s weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weigh_opt_dir_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weigh_opt_dir_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_arb_weigh_opt_dir_ch`] module"]
pub type TX_ARB_WEIGH_OPT_DIR_CH =
    crate::Reg<tx_arb_weigh_opt_dir_ch::TX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "TX channel %s weight arbitration optimization enable register"]
pub mod tx_arb_weigh_opt_dir_ch;
#[doc = "RX_CH_ARB_WEIGH_CH (rw) register accessor: RX channel %s arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weigh_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weigh_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch_arb_weigh_ch`] module"]
pub type RX_CH_ARB_WEIGH_CH = crate::Reg<rx_ch_arb_weigh_ch::RX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "RX channel %s arbitration weight configuration register"]
pub mod rx_ch_arb_weigh_ch;
#[doc = "RX_ARB_WEIGH_OPT_DIR_CH (rw) register accessor: RX channel %s weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weigh_opt_dir_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weigh_opt_dir_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_arb_weigh_opt_dir_ch`] module"]
pub type RX_ARB_WEIGH_OPT_DIR_CH =
    crate::Reg<rx_arb_weigh_opt_dir_ch::RX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "RX channel %s weight arbitration optimization enable register"]
pub mod rx_arb_weigh_opt_dir_ch;
#[doc = "IN_LINK_ADDR_CH (rw) register accessor: Link list descriptor address configuration of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch`] module"]
pub type IN_LINK_ADDR_CH = crate::Reg<in_link_addr_ch::IN_LINK_ADDR_CH_SPEC>;
#[doc = "Link list descriptor address configuration of RX channel %s"]
pub mod in_link_addr_ch;
#[doc = "OUT_LINK_ADDR_CH (rw) register accessor: Link list descriptor address configuration of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch`] module"]
pub type OUT_LINK_ADDR_CH = crate::Reg<out_link_addr_ch::OUT_LINK_ADDR_CH_SPEC>;
#[doc = "Link list descriptor address configuration of TX channel %s"]
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
