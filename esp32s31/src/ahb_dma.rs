#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    in_int_raw_ch: (),
    _reserved1: [u8; 0x04],
    in_int_st_ch: (),
    _reserved2: [u8; 0x04],
    in_int_ena_ch: (),
    _reserved3: [u8; 0x04],
    in_int_clr_ch: (),
    _reserved4: [u8; 0x44],
    out_int_raw_ch: (),
    _reserved5: [u8; 0x04],
    out_int_st_ch: (),
    _reserved6: [u8; 0x04],
    out_int_ena_ch: (),
    _reserved7: [u8; 0x04],
    out_int_clr_ch: (),
    _reserved8: [u8; 0x44],
    ahb_test: AHB_TEST,
    misc_conf: MISC_CONF,
    date: DATE,
    _reserved11: [u8; 0x54],
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
    in_link_addr_ch: (),
    _reserved17: [u8; 0x04],
    in_state_ch: (),
    _reserved18: [u8; 0x04],
    in_suc_eof_des_addr_ch: (),
    _reserved19: [u8; 0x04],
    in_err_eof_des_addr_ch: (),
    _reserved20: [u8; 0x04],
    in_done_des_addr_ch: (),
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
    _reserved26: [u8; 0x04],
    rx_ch_arb_weigh_ch: (),
    _reserved27: [u8; 0x04],
    rx_arb_weigh_opt_dir_ch: (),
    _reserved28: [u8; 0x40],
    out_conf0_ch0: OUT_CONF0_CH0,
    out_conf1_ch: (),
    _reserved30: [u8; 0x04],
    outfifo_status_ch: (),
    _reserved31: [u8; 0x04],
    out_push_ch: (),
    _reserved32: [u8; 0x04],
    out_link_ch: (),
    _reserved33: [u8; 0x04],
    out_link_addr_ch: (),
    _reserved34: [u8; 0x04],
    out_state_ch: (),
    _reserved35: [u8; 0x04],
    out_eof_des_addr_ch: (),
    _reserved36: [u8; 0x04],
    out_eof_bfr_des_addr_ch: (),
    _reserved37: [u8; 0x04],
    out_done_des_addr_ch: (),
    _reserved38: [u8; 0x04],
    out_dscr_ch: (),
    _reserved39: [u8; 0x04],
    out_dscr_bf0_ch: (),
    _reserved40: [u8; 0x04],
    out_dscr_bf1_ch: (),
    _reserved41: [u8; 0x04],
    out_pri_ch: (),
    _reserved42: [u8; 0x04],
    out_peri_sel_ch: (),
    _reserved43: [u8; 0x04],
    tx_ch_arb_weigh_ch: (),
    _reserved44: [u8; 0x04],
    tx_arb_weigh_opt_dir_ch: (),
    _reserved45: [u8; 0xc0],
    out_conf0_ch: (),
    _reserved46: [u8; 0x0380],
    intr_mem_start_addr: INTR_MEM_START_ADDR,
    intr_mem_end_addr: INTR_MEM_END_ADDR,
    arb_timeout: ARB_TIMEOUT,
    _reserved49: [u8; 0x04],
    weight_en: WEIGHT_EN,
    _reserved50: [u8; 0x04],
    module_clk_en: MODULE_CLK_EN,
    _reserved51: [u8; 0x04],
    ahbinf_resp_err_status0: AHBINF_RESP_ERR_STATUS0,
    ahbinf_resp_err_status1: AHBINF_RESP_ERR_STATUS1,
}
impl RegisterBlock {
    #[doc = "0x00..0x14 - Raw interrupt status of RX channel %s"]
    #[inline(always)]
    pub const fn in_int_raw_ch(&self, n: usize) -> &IN_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x14 - Raw interrupt status of RX channel %s"]
    #[inline(always)]
    pub fn in_int_raw_ch_iter(&self) -> impl Iterator<Item = &IN_INT_RAW_CH> {
        (0..5).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() })
    }
    #[doc = "0x04..0x18 - Masked interrupt status of RX channel %s"]
    #[inline(always)]
    pub const fn in_int_st_ch(&self, n: usize) -> &IN_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x18 - Masked interrupt status of RX channel %s"]
    #[inline(always)]
    pub fn in_int_st_ch_iter(&self) -> impl Iterator<Item = &IN_INT_ST_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x08..0x1c - Interrupt enable bits of RX channel %s"]
    #[inline(always)]
    pub const fn in_int_ena_ch(&self, n: usize) -> &IN_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x1c - Interrupt enable bits of RX channel %s"]
    #[inline(always)]
    pub fn in_int_ena_ch_iter(&self) -> impl Iterator<Item = &IN_INT_ENA_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x0c..0x20 - Interrupt clear bits of RX channel %s"]
    #[inline(always)]
    pub const fn in_int_clr_ch(&self, n: usize) -> &IN_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x20 - Interrupt clear bits of RX channel %s"]
    #[inline(always)]
    pub fn in_int_clr_ch_iter(&self) -> impl Iterator<Item = &IN_INT_CLR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x50..0x64 - Raw interrupt status of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_raw_ch(&self, n: usize) -> &OUT_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(80)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x64 - Raw interrupt status of TX channel %s"]
    #[inline(always)]
    pub fn out_int_raw_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_RAW_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(80)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x54..0x68 - Masked interrupt status of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_st_ch(&self, n: usize) -> &OUT_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x54..0x68 - Masked interrupt status of TX channel %s"]
    #[inline(always)]
    pub fn out_int_st_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_ST_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x58..0x6c - Interrupt enable bits of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_ena_ch(&self, n: usize) -> &OUT_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(88)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x6c - Interrupt enable bits of TX channel %s"]
    #[inline(always)]
    pub fn out_int_ena_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_ENA_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(88)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x5c..0x70 - Interrupt clear bits of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_clr_ch(&self, n: usize) -> &OUT_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(92)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c..0x70 - Interrupt clear bits of TX channel %s"]
    #[inline(always)]
    pub fn out_int_clr_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_CLR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(92)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0xa0 - reserved"]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0xa4 - Miscellaneous register"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0xa8 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x100..0x114 - Configuration register 0 of RX channel %s"]
    #[inline(always)]
    pub const fn in_conf0_ch(&self, n: usize) -> &IN_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x114 - Configuration register 0 of RX channel %s"]
    #[inline(always)]
    pub fn in_conf0_ch_iter(&self) -> impl Iterator<Item = &IN_CONF0_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x104..0x118 - Configuration register 1 of RX channel %s"]
    #[inline(always)]
    pub const fn in_conf1_ch(&self, n: usize) -> &IN_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x118 - Configuration register 1 of RX channel %s"]
    #[inline(always)]
    pub fn in_conf1_ch_iter(&self) -> impl Iterator<Item = &IN_CONF1_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x108..0x11c - Receive FIFO status of RX channel %s"]
    #[inline(always)]
    pub const fn infifo_status_ch(&self, n: usize) -> &INFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(264)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x108..0x11c - Receive FIFO status of RX channel %s"]
    #[inline(always)]
    pub fn infifo_status_ch_iter(&self) -> impl Iterator<Item = &INFIFO_STATUS_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(264)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x10c..0x120 - Pop control register of RX channel %s"]
    #[inline(always)]
    pub const fn in_pop_ch(&self, n: usize) -> &IN_POP_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(268)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10c..0x120 - Pop control register of RX channel %s"]
    #[inline(always)]
    pub fn in_pop_ch_iter(&self) -> impl Iterator<Item = &IN_POP_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(268)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x110..0x124 - Linked list descriptor configuration and control register of RX channel %s"]
    #[inline(always)]
    pub const fn in_link_ch(&self, n: usize) -> &IN_LINK_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(272)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x124 - Linked list descriptor configuration and control register of RX channel %s"]
    #[inline(always)]
    pub fn in_link_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(272)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x114..0x128 - Link list descriptor address configuration of RX channel %s"]
    #[inline(always)]
    pub const fn in_link_addr_ch(&self, n: usize) -> &IN_LINK_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(276)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x114..0x128 - Link list descriptor address configuration of RX channel %s"]
    #[inline(always)]
    pub fn in_link_addr_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_ADDR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(276)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x118..0x12c - Receive status of RX channel %s"]
    #[inline(always)]
    pub const fn in_state_ch(&self, n: usize) -> &IN_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(280)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x118..0x12c - Receive status of RX channel %s"]
    #[inline(always)]
    pub fn in_state_ch_iter(&self) -> impl Iterator<Item = &IN_STATE_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(280)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x11c..0x130 - Receive descriptor address when EOF occurs on RX channel %s"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch(&self, n: usize) -> &IN_SUC_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(284)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x11c..0x130 - Receive descriptor address when EOF occurs on RX channel %s"]
    #[inline(always)]
    pub fn in_suc_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &IN_SUC_EOF_DES_ADDR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(284)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x120..0x134 - Receive descriptor address when errors occur of RX channel %s"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch(&self, n: usize) -> &IN_ERR_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(288)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x134 - Receive descriptor address when errors occur of RX channel %s"]
    #[inline(always)]
    pub fn in_err_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &IN_ERR_EOF_DES_ADDR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(288)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x124..0x138 - RX_done inlink descriptor address of RX channel %s"]
    #[inline(always)]
    pub const fn in_done_des_addr_ch(&self, n: usize) -> &IN_DONE_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(292)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x124..0x138 - RX_done inlink descriptor address of RX channel %s"]
    #[inline(always)]
    pub fn in_done_des_addr_ch_iter(&self) -> impl Iterator<Item = &IN_DONE_DES_ADDR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(292)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x128..0x13c - Current receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub const fn in_dscr_ch(&self, n: usize) -> &IN_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(296)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x128..0x13c - Current receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub fn in_dscr_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(296)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x12c..0x140 - The last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch(&self, n: usize) -> &IN_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(300)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12c..0x140 - The last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub fn in_dscr_bf0_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_BF0_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(300)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x130..0x144 - The second-to-last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch(&self, n: usize) -> &IN_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x144 - The second-to-last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub fn in_dscr_bf1_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_BF1_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x134..0x148 - Priority register of RX channel %s"]
    #[inline(always)]
    pub const fn in_pri_ch(&self, n: usize) -> &IN_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(308)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x134..0x148 - Priority register of RX channel %s"]
    #[inline(always)]
    pub fn in_pri_ch_iter(&self) -> impl Iterator<Item = &IN_PRI_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(308)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x138..0x14c - Peripheral selection register of RX channel %s"]
    #[inline(always)]
    pub const fn in_peri_sel_ch(&self, n: usize) -> &IN_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(312)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x138..0x14c - Peripheral selection register of RX channel %s"]
    #[inline(always)]
    pub fn in_peri_sel_ch_iter(&self) -> impl Iterator<Item = &IN_PERI_SEL_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(312)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x13c..0x150 - RX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub const fn rx_ch_arb_weigh_ch(&self, n: usize) -> &RX_CH_ARB_WEIGH_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(316)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x13c..0x150 - RX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub fn rx_ch_arb_weigh_ch_iter(&self) -> impl Iterator<Item = &RX_CH_ARB_WEIGH_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(316)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x140..0x154 - RX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn rx_arb_weigh_opt_dir_ch(&self, n: usize) -> &RX_ARB_WEIGH_OPT_DIR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(320)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x154 - RX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub fn rx_arb_weigh_opt_dir_ch_iter(&self) -> impl Iterator<Item = &RX_ARB_WEIGH_OPT_DIR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(320)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x180 - Configuration register 0 of TX channel 0"]
    #[inline(always)]
    pub const fn out_conf0_ch0(&self) -> &OUT_CONF0_CH0 {
        &self.out_conf0_ch0
    }
    #[doc = "0x184..0x198 - Configuration register 1 of TX channel %s"]
    #[inline(always)]
    pub const fn out_conf1_ch(&self, n: usize) -> &OUT_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(388)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x184..0x198 - Configuration register 1 of TX channel %s"]
    #[inline(always)]
    pub fn out_conf1_ch_iter(&self) -> impl Iterator<Item = &OUT_CONF1_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(388)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x188..0x19c - Transmit FIFO status of TX channel %s"]
    #[inline(always)]
    pub const fn outfifo_status_ch(&self, n: usize) -> &OUTFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(392)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x188..0x19c - Transmit FIFO status of TX channel %s"]
    #[inline(always)]
    pub fn outfifo_status_ch_iter(&self) -> impl Iterator<Item = &OUTFIFO_STATUS_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(392)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x18c..0x1a0 - Push control register of TX channel %s"]
    #[inline(always)]
    pub const fn out_push_ch(&self, n: usize) -> &OUT_PUSH_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(396)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18c..0x1a0 - Push control register of TX channel %s"]
    #[inline(always)]
    pub fn out_push_ch_iter(&self) -> impl Iterator<Item = &OUT_PUSH_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(396)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x190..0x1a4 - Linked list descriptor configuration and control register of TX channel %s"]
    #[inline(always)]
    pub const fn out_link_ch(&self, n: usize) -> &OUT_LINK_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x1a4 - Linked list descriptor configuration and control register of TX channel %s"]
    #[inline(always)]
    pub fn out_link_ch_iter(&self) -> impl Iterator<Item = &OUT_LINK_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x194..0x1a8 - Link list descriptor address configuration of TX channel %s"]
    #[inline(always)]
    pub const fn out_link_addr_ch(&self, n: usize) -> &OUT_LINK_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(404)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x194..0x1a8 - Link list descriptor address configuration of TX channel %s"]
    #[inline(always)]
    pub fn out_link_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_LINK_ADDR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(404)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x198..0x1ac - Transmit status of TX channel %s"]
    #[inline(always)]
    pub const fn out_state_ch(&self, n: usize) -> &OUT_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(408)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x198..0x1ac - Transmit status of TX channel %s"]
    #[inline(always)]
    pub fn out_state_ch_iter(&self) -> impl Iterator<Item = &OUT_STATE_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(408)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x19c..0x1b0 - Transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch(&self, n: usize) -> &OUT_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(412)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x19c..0x1b0 - Transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub fn out_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_EOF_DES_ADDR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(412)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1a0..0x1b4 - The last transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch(&self, n: usize) -> &OUT_EOF_BFR_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(416)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a0..0x1b4 - The last transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub fn out_eof_bfr_des_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_EOF_BFR_DES_ADDR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(416)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1a4..0x1b8 - TX done outlink descriptor address of TX channel %s"]
    #[inline(always)]
    pub const fn out_done_des_addr_ch(&self, n: usize) -> &OUT_DONE_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(420)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a4..0x1b8 - TX done outlink descriptor address of TX channel %s"]
    #[inline(always)]
    pub fn out_done_des_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_DONE_DES_ADDR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(420)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1a8..0x1bc - Current transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub const fn out_dscr_ch(&self, n: usize) -> &OUT_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(424)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a8..0x1bc - Current transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub fn out_dscr_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(424)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1ac..0x1c0 - The last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch(&self, n: usize) -> &OUT_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(428)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1ac..0x1c0 - The last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub fn out_dscr_bf0_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_BF0_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(428)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1b0..0x1c4 - The second-to-last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch(&self, n: usize) -> &OUT_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(432)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1b0..0x1c4 - The second-to-last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub fn out_dscr_bf1_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_BF1_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(432)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1b4..0x1c8 - Priority register of TX channel %s"]
    #[inline(always)]
    pub const fn out_pri_ch(&self, n: usize) -> &OUT_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(436)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1b4..0x1c8 - Priority register of TX channel %s"]
    #[inline(always)]
    pub fn out_pri_ch_iter(&self) -> impl Iterator<Item = &OUT_PRI_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(436)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1b8..0x1cc - Peripheral selection register of TX channel %s"]
    #[inline(always)]
    pub const fn out_peri_sel_ch(&self, n: usize) -> &OUT_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(440)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1b8..0x1cc - Peripheral selection register of TX channel %s"]
    #[inline(always)]
    pub fn out_peri_sel_ch_iter(&self) -> impl Iterator<Item = &OUT_PERI_SEL_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(440)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1bc..0x1d0 - TX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub const fn tx_ch_arb_weigh_ch(&self, n: usize) -> &TX_CH_ARB_WEIGH_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(444)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1bc..0x1d0 - TX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub fn tx_ch_arb_weigh_ch_iter(&self) -> impl Iterator<Item = &TX_CH_ARB_WEIGH_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(444)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1c0..0x1d4 - TX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn tx_arb_weigh_opt_dir_ch(&self, n: usize) -> &TX_ARB_WEIGH_OPT_DIR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1d4 - TX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir_ch_iter(&self) -> impl Iterator<Item = &TX_ARB_WEIGH_OPT_DIR_CH> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x280..0x290 - Configuration register 0 of TX channel %s"]
    #[inline(always)]
    pub const fn out_conf0_ch(&self, n: usize) -> &OUT_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(640)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x280..0x290 - Configuration register 0 of TX channel %s"]
    #[inline(always)]
    pub fn out_conf0_ch_iter(&self) -> impl Iterator<Item = &OUT_CONF0_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(640)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x600 - Accessible address space start address configuration register"]
    #[inline(always)]
    pub const fn intr_mem_start_addr(&self) -> &INTR_MEM_START_ADDR {
        &self.intr_mem_start_addr
    }
    #[doc = "0x604 - Accessible address space end address configuration register"]
    #[inline(always)]
    pub const fn intr_mem_end_addr(&self) -> &INTR_MEM_END_ADDR {
        &self.intr_mem_end_addr
    }
    #[doc = "0x608 - TX arbitration timeout configuration register"]
    #[inline(always)]
    pub const fn arb_timeout(&self) -> &ARB_TIMEOUT {
        &self.arb_timeout
    }
    #[doc = "0x610 - TX weight arbitration enable register"]
    #[inline(always)]
    pub const fn weight_en(&self) -> &WEIGHT_EN {
        &self.weight_en
    }
    #[doc = "0x618 - Module clock force on register"]
    #[inline(always)]
    pub const fn module_clk_en(&self) -> &MODULE_CLK_EN {
        &self.module_clk_en
    }
    #[doc = "0x620 - AHB response error status 0 register"]
    #[inline(always)]
    pub const fn ahbinf_resp_err_status0(&self) -> &AHBINF_RESP_ERR_STATUS0 {
        &self.ahbinf_resp_err_status0
    }
    #[doc = "0x624 - AHB response error status 1 register"]
    #[inline(always)]
    pub const fn ahbinf_resp_err_status1(&self) -> &AHBINF_RESP_ERR_STATUS1 {
        &self.ahbinf_resp_err_status1
    }
}
#[doc = "IN_INT_RAW_CH (rw) register accessor: Raw interrupt status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_raw_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_raw_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch`] module"]
pub type IN_INT_RAW_CH = crate::Reg<in_int_raw_ch::IN_INT_RAW_CH_SPEC>;
#[doc = "Raw interrupt status of RX channel %s"]
pub mod in_int_raw_ch;
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
#[doc = "IN_LINK_ADDR_CH (rw) register accessor: Link list descriptor address configuration of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch`] module"]
pub type IN_LINK_ADDR_CH = crate::Reg<in_link_addr_ch::IN_LINK_ADDR_CH_SPEC>;
#[doc = "Link list descriptor address configuration of RX channel %s"]
pub mod in_link_addr_ch;
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
#[doc = "IN_DONE_DES_ADDR_CH (r) register accessor: RX_done inlink descriptor address of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_done_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_done_des_addr_ch`] module"]
pub type IN_DONE_DES_ADDR_CH = crate::Reg<in_done_des_addr_ch::IN_DONE_DES_ADDR_CH_SPEC>;
#[doc = "RX_done inlink descriptor address of RX channel %s"]
pub mod in_done_des_addr_ch;
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
#[doc = "RX_CH_ARB_WEIGH_CH (rw) register accessor: RX channel %s arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weigh_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weigh_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ch_arb_weigh_ch`] module"]
pub type RX_CH_ARB_WEIGH_CH = crate::Reg<rx_ch_arb_weigh_ch::RX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "RX channel %s arbitration weight configuration register"]
pub mod rx_ch_arb_weigh_ch;
#[doc = "RX_ARB_WEIGH_OPT_DIR_CH (rw) register accessor: RX channel %s weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weigh_opt_dir_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weigh_opt_dir_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_arb_weigh_opt_dir_ch`] module"]
pub type RX_ARB_WEIGH_OPT_DIR_CH =
    crate::Reg<rx_arb_weigh_opt_dir_ch::RX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "RX channel %s weight arbitration optimization enable register"]
pub mod rx_arb_weigh_opt_dir_ch;
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
#[doc = "OUT_LINK_ADDR_CH (rw) register accessor: Link list descriptor address configuration of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch`] module"]
pub type OUT_LINK_ADDR_CH = crate::Reg<out_link_addr_ch::OUT_LINK_ADDR_CH_SPEC>;
#[doc = "Link list descriptor address configuration of TX channel %s"]
pub mod out_link_addr_ch;
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
#[doc = "OUT_DONE_DES_ADDR_CH (r) register accessor: TX done outlink descriptor address of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_done_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_done_des_addr_ch`] module"]
pub type OUT_DONE_DES_ADDR_CH = crate::Reg<out_done_des_addr_ch::OUT_DONE_DES_ADDR_CH_SPEC>;
#[doc = "TX done outlink descriptor address of TX channel %s"]
pub mod out_done_des_addr_ch;
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
#[doc = "TX_CH_ARB_WEIGH_CH (rw) register accessor: TX channel %s arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ch_arb_weigh_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ch_arb_weigh_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ch_arb_weigh_ch`] module"]
pub type TX_CH_ARB_WEIGH_CH = crate::Reg<tx_ch_arb_weigh_ch::TX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "TX channel %s arbitration weight configuration register"]
pub mod tx_ch_arb_weigh_ch;
#[doc = "TX_ARB_WEIGH_OPT_DIR_CH (rw) register accessor: TX channel %s weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weigh_opt_dir_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weigh_opt_dir_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_arb_weigh_opt_dir_ch`] module"]
pub type TX_ARB_WEIGH_OPT_DIR_CH =
    crate::Reg<tx_arb_weigh_opt_dir_ch::TX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "TX channel %s weight arbitration optimization enable register"]
pub mod tx_arb_weigh_opt_dir_ch;
#[doc = "OUT_CONF0_CH (rw) register accessor: Configuration register 0 of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf0_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf0_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch`] module"]
pub type OUT_CONF0_CH = crate::Reg<out_conf0_ch::OUT_CONF0_CH_SPEC>;
#[doc = "Configuration register 0 of TX channel %s"]
pub mod out_conf0_ch;
#[doc = "INTR_MEM_START_ADDR (rw) register accessor: Accessible address space start address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_start_addr`] module"]
pub type INTR_MEM_START_ADDR = crate::Reg<intr_mem_start_addr::INTR_MEM_START_ADDR_SPEC>;
#[doc = "Accessible address space start address configuration register"]
pub mod intr_mem_start_addr;
#[doc = "INTR_MEM_END_ADDR (rw) register accessor: Accessible address space end address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_end_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_end_addr`] module"]
pub type INTR_MEM_END_ADDR = crate::Reg<intr_mem_end_addr::INTR_MEM_END_ADDR_SPEC>;
#[doc = "Accessible address space end address configuration register"]
pub mod intr_mem_end_addr;
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
