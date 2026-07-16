#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ahb_dma_in_int_raw_ch: (),
    _reserved1: [u8; 0x04],
    ahb_dma_in_int_st_ch: (),
    _reserved2: [u8; 0x04],
    ahb_dma_in_int_ena_ch: (),
    _reserved3: [u8; 0x04],
    ahb_dma_in_int_clr_ch: (),
    _reserved4: [u8; 0x44],
    ahb_dma_out_int_raw_ch: (),
    _reserved5: [u8; 0x04],
    ahb_dma_out_int_st_ch: (),
    _reserved6: [u8; 0x04],
    ahb_dma_out_int_ena_ch: (),
    _reserved7: [u8; 0x04],
    ahb_dma_out_int_clr_ch: (),
    _reserved8: [u8; 0x44],
    ahb_dma_ahb_test: AHB_DMA_AHB_TEST,
    ahb_dma_misc_conf: AHB_DMA_MISC_CONF,
    ahb_dma_date: AHB_DMA_DATE,
    _reserved11: [u8; 0x54],
    ahb_dma_in_conf0_ch: (),
    _reserved12: [u8; 0x04],
    ahb_dma_in_conf1_ch: (),
    _reserved13: [u8; 0x04],
    ahb_dma_infifo_status_ch: (),
    _reserved14: [u8; 0x04],
    ahb_dma_in_pop_ch: (),
    _reserved15: [u8; 0x04],
    ahb_dma_in_link_ch: (),
    _reserved16: [u8; 0x04],
    ahb_dma_in_link_addr_ch: (),
    _reserved17: [u8; 0x04],
    ahb_dma_in_state_ch: (),
    _reserved18: [u8; 0x04],
    ahb_dma_in_suc_eof_des_addr_ch: (),
    _reserved19: [u8; 0x04],
    ahb_dma_in_err_eof_des_addr_ch: (),
    _reserved20: [u8; 0x04],
    ahb_dma_in_done_des_addr_ch: (),
    _reserved21: [u8; 0x04],
    ahb_dma_in_dscr_ch: (),
    _reserved22: [u8; 0x04],
    ahb_dma_in_dscr_bf0_ch: (),
    _reserved23: [u8; 0x04],
    ahb_dma_in_dscr_bf1_ch: (),
    _reserved24: [u8; 0x04],
    ahb_dma_in_pri_ch: (),
    _reserved25: [u8; 0x04],
    ahb_dma_in_peri_sel_ch: (),
    _reserved26: [u8; 0x04],
    ahb_dma_rx_ch_arb_weigh_ch: (),
    _reserved27: [u8; 0x04],
    ahb_dma_rx_arb_weigh_opt_dir_ch: (),
    _reserved28: [u8; 0x40],
    ahb_dma_out_conf0_ch0: AHB_DMA_OUT_CONF0_CH0,
    ahb_dma_out_conf1_ch: (),
    _reserved30: [u8; 0x04],
    ahb_dma_outfifo_status_ch: (),
    _reserved31: [u8; 0x04],
    ahb_dma_out_push_ch: (),
    _reserved32: [u8; 0x04],
    ahb_dma_out_link_ch: (),
    _reserved33: [u8; 0x04],
    ahb_dma_out_link_addr_ch: (),
    _reserved34: [u8; 0x04],
    ahb_dma_out_state_ch: (),
    _reserved35: [u8; 0x04],
    ahb_dma_out_eof_des_addr_ch: (),
    _reserved36: [u8; 0x04],
    ahb_dma_out_eof_bfr_des_addr_ch: (),
    _reserved37: [u8; 0x04],
    ahb_dma_out_done_des_addr_ch: (),
    _reserved38: [u8; 0x04],
    ahb_dma_out_dscr_ch: (),
    _reserved39: [u8; 0x04],
    ahb_dma_out_dscr_bf0_ch: (),
    _reserved40: [u8; 0x04],
    ahb_dma_out_dscr_bf1_ch: (),
    _reserved41: [u8; 0x04],
    ahb_dma_out_pri_ch: (),
    _reserved42: [u8; 0x04],
    ahb_dma_out_peri_sel_ch: (),
    _reserved43: [u8; 0x04],
    ahb_dma_tx_ch_arb_weigh_ch: (),
    _reserved44: [u8; 0x04],
    ahb_dma_tx_arb_weigh_opt_dir_ch: (),
    _reserved45: [u8; 0xc0],
    ahb_dma_out_conf0_ch1: AHB_DMA_OUT_CONF0_CH1,
    _reserved46: [u8; 0x037c],
    ahb_dma_intr_mem_start_addr: AHB_DMA_INTR_MEM_START_ADDR,
    ahb_dma_intr_mem_end_addr: AHB_DMA_INTR_MEM_END_ADDR,
    ahb_dma_arb_timeout: AHB_DMA_ARB_TIMEOUT,
    _reserved49: [u8; 0x04],
    ahb_dma_weight_en: AHB_DMA_WEIGHT_EN,
    _reserved50: [u8; 0x04],
    ahb_dma_module_clk_en: AHB_DMA_MODULE_CLK_EN,
    _reserved51: [u8; 0x04],
    ahb_dma_ahbinf_resp_err_status0: AHB_DMA_AHBINF_RESP_ERR_STATUS0,
    ahb_dma_ahbinf_resp_err_status1: AHB_DMA_AHBINF_RESP_ERR_STATUS1,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Raw interrupt status of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_int_raw_ch(&self, n: usize) -> &AHB_DMA_IN_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - Raw interrupt status of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_int_raw_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_INT_RAW_CH> {
        (0..2).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() })
    }
    #[doc = "0x04..0x0c - Masked interrupt status of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_int_st_ch(&self, n: usize) -> &AHB_DMA_IN_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - Masked interrupt status of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_int_st_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_INT_ST_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x08..0x10 - Interrupt enable bits of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_int_ena_ch(&self, n: usize) -> &AHB_DMA_IN_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x10 - Interrupt enable bits of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_int_ena_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_INT_ENA_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x0c..0x14 - Interrupt clear bits of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_int_clr_ch(&self, n: usize) -> &AHB_DMA_IN_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x14 - Interrupt clear bits of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_int_clr_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_INT_CLR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x50..0x58 - Raw interrupt status of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_int_raw_ch(&self, n: usize) -> &AHB_DMA_OUT_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(80)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x58 - Raw interrupt status of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_int_raw_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_INT_RAW_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(80)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x54..0x5c - Masked interrupt status of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_int_st_ch(&self, n: usize) -> &AHB_DMA_OUT_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x54..0x5c - Masked interrupt status of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_int_st_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_INT_ST_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x58..0x60 - Interrupt enable bits of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_int_ena_ch(&self, n: usize) -> &AHB_DMA_OUT_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(88)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x60 - Interrupt enable bits of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_int_ena_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_INT_ENA_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(88)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x5c..0x64 - Interrupt clear bits of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_int_clr_ch(&self, n: usize) -> &AHB_DMA_OUT_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(92)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c..0x64 - Interrupt clear bits of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_int_clr_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_INT_CLR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(92)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0xa0 - reserved"]
    #[inline(always)]
    pub const fn ahb_dma_ahb_test(&self) -> &AHB_DMA_AHB_TEST {
        &self.ahb_dma_ahb_test
    }
    #[doc = "0xa4 - Miscellaneous register"]
    #[inline(always)]
    pub const fn ahb_dma_misc_conf(&self) -> &AHB_DMA_MISC_CONF {
        &self.ahb_dma_misc_conf
    }
    #[doc = "0xa8 - Version control register"]
    #[inline(always)]
    pub const fn ahb_dma_date(&self) -> &AHB_DMA_DATE {
        &self.ahb_dma_date
    }
    #[doc = "0x100..0x108 - Configuration register 0 of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_conf0_ch(&self, n: usize) -> &AHB_DMA_IN_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x108 - Configuration register 0 of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_conf0_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_CONF0_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x104..0x10c - Configuration register 1 of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_conf1_ch(&self, n: usize) -> &AHB_DMA_IN_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x10c - Configuration register 1 of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_conf1_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_CONF1_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x108..0x110 - Receive FIFO status of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_infifo_status_ch(&self, n: usize) -> &AHB_DMA_INFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(264)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x108..0x110 - Receive FIFO status of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_infifo_status_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_INFIFO_STATUS_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(264)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x10c..0x114 - Pop control register of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_pop_ch(&self, n: usize) -> &AHB_DMA_IN_POP_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(268)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10c..0x114 - Pop control register of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_pop_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_POP_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(268)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x110..0x118 - Linked list descriptor configuration and control register of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_link_ch(&self, n: usize) -> &AHB_DMA_IN_LINK_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(272)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x118 - Linked list descriptor configuration and control register of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_link_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_LINK_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(272)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x114..0x11c - Link list descriptor address configuration of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_link_addr_ch(&self, n: usize) -> &AHB_DMA_IN_LINK_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(276)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x114..0x11c - Link list descriptor address configuration of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_link_addr_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_LINK_ADDR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(276)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x118..0x120 - Receive status of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_state_ch(&self, n: usize) -> &AHB_DMA_IN_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(280)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x118..0x120 - Receive status of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_state_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_STATE_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(280)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x11c..0x124 - Receive descriptor address when EOF occurs on RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_suc_eof_des_addr_ch(
        &self,
        n: usize,
    ) -> &AHB_DMA_IN_SUC_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(284)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x11c..0x124 - Receive descriptor address when EOF occurs on RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_suc_eof_des_addr_ch_iter(
        &self,
    ) -> impl Iterator<Item = &AHB_DMA_IN_SUC_EOF_DES_ADDR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(284)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x120..0x128 - Receive descriptor address when errors occur of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_err_eof_des_addr_ch(
        &self,
        n: usize,
    ) -> &AHB_DMA_IN_ERR_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(288)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x128 - Receive descriptor address when errors occur of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_err_eof_des_addr_ch_iter(
        &self,
    ) -> impl Iterator<Item = &AHB_DMA_IN_ERR_EOF_DES_ADDR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(288)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x124..0x12c - RX_done inlink descriptor address of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_done_des_addr_ch(&self, n: usize) -> &AHB_DMA_IN_DONE_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(292)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x124..0x12c - RX_done inlink descriptor address of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_done_des_addr_ch_iter(
        &self,
    ) -> impl Iterator<Item = &AHB_DMA_IN_DONE_DES_ADDR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(292)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x128..0x130 - Current receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_dscr_ch(&self, n: usize) -> &AHB_DMA_IN_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(296)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x128..0x130 - Current receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_dscr_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_DSCR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(296)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x12c..0x134 - The last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_dscr_bf0_ch(&self, n: usize) -> &AHB_DMA_IN_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(300)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12c..0x134 - The last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_dscr_bf0_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_DSCR_BF0_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(300)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x130..0x138 - The second-to-last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_dscr_bf1_ch(&self, n: usize) -> &AHB_DMA_IN_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x130..0x138 - The second-to-last receive descriptor address of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_dscr_bf1_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_DSCR_BF1_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(304)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x134..0x13c - Priority register of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_pri_ch(&self, n: usize) -> &AHB_DMA_IN_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(308)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x134..0x13c - Priority register of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_pri_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_PRI_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(308)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x138..0x140 - Peripheral selection register of RX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_in_peri_sel_ch(&self, n: usize) -> &AHB_DMA_IN_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(312)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x138..0x140 - Peripheral selection register of RX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_in_peri_sel_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_IN_PERI_SEL_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(312)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x13c..0x144 - RX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub const fn ahb_dma_rx_ch_arb_weigh_ch(&self, n: usize) -> &AHB_DMA_RX_CH_ARB_WEIGH_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(316)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x13c..0x144 - RX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub fn ahb_dma_rx_ch_arb_weigh_ch_iter(
        &self,
    ) -> impl Iterator<Item = &AHB_DMA_RX_CH_ARB_WEIGH_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(316)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x140..0x148 - RX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn ahb_dma_rx_arb_weigh_opt_dir_ch(
        &self,
        n: usize,
    ) -> &AHB_DMA_RX_ARB_WEIGH_OPT_DIR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(320)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x148 - RX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub fn ahb_dma_rx_arb_weigh_opt_dir_ch_iter(
        &self,
    ) -> impl Iterator<Item = &AHB_DMA_RX_ARB_WEIGH_OPT_DIR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(320)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x180 - Configuration register 0 of TX channel 0"]
    #[inline(always)]
    pub const fn ahb_dma_out_conf0_ch0(&self) -> &AHB_DMA_OUT_CONF0_CH0 {
        &self.ahb_dma_out_conf0_ch0
    }
    #[doc = "0x184..0x18c - Configuration register 1 of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_conf1_ch(&self, n: usize) -> &AHB_DMA_OUT_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(388)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x184..0x18c - Configuration register 1 of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_conf1_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_CONF1_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(388)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x188..0x190 - Transmit FIFO status of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_outfifo_status_ch(&self, n: usize) -> &AHB_DMA_OUTFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(392)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x188..0x190 - Transmit FIFO status of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_outfifo_status_ch_iter(
        &self,
    ) -> impl Iterator<Item = &AHB_DMA_OUTFIFO_STATUS_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(392)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x18c..0x194 - Push control register of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_push_ch(&self, n: usize) -> &AHB_DMA_OUT_PUSH_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(396)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18c..0x194 - Push control register of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_push_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_PUSH_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(396)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x190..0x198 - Linked list descriptor configuration and control register of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_link_ch(&self, n: usize) -> &AHB_DMA_OUT_LINK_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x198 - Linked list descriptor configuration and control register of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_link_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_LINK_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(400)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x194..0x19c - Link list descriptor address configuration of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_link_addr_ch(&self, n: usize) -> &AHB_DMA_OUT_LINK_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(404)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x194..0x19c - Link list descriptor address configuration of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_link_addr_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_LINK_ADDR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(404)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x198..0x1a0 - Transmit status of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_state_ch(&self, n: usize) -> &AHB_DMA_OUT_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(408)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x198..0x1a0 - Transmit status of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_state_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_STATE_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(408)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x19c..0x1a4 - Transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_eof_des_addr_ch(&self, n: usize) -> &AHB_DMA_OUT_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(412)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x19c..0x1a4 - Transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_eof_des_addr_ch_iter(
        &self,
    ) -> impl Iterator<Item = &AHB_DMA_OUT_EOF_DES_ADDR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(412)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1a0..0x1a8 - The last transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_eof_bfr_des_addr_ch(
        &self,
        n: usize,
    ) -> &AHB_DMA_OUT_EOF_BFR_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(416)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a0..0x1a8 - The last transmit descriptor address when EOF occurs on TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_eof_bfr_des_addr_ch_iter(
        &self,
    ) -> impl Iterator<Item = &AHB_DMA_OUT_EOF_BFR_DES_ADDR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(416)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1a4..0x1ac - TX done outlink descriptor address of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_done_des_addr_ch(&self, n: usize) -> &AHB_DMA_OUT_DONE_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(420)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a4..0x1ac - TX done outlink descriptor address of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_done_des_addr_ch_iter(
        &self,
    ) -> impl Iterator<Item = &AHB_DMA_OUT_DONE_DES_ADDR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(420)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1a8..0x1b0 - Current transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_dscr_ch(&self, n: usize) -> &AHB_DMA_OUT_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(424)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1a8..0x1b0 - Current transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_dscr_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_DSCR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(424)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1ac..0x1b4 - The last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_dscr_bf0_ch(&self, n: usize) -> &AHB_DMA_OUT_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(428)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1ac..0x1b4 - The last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_dscr_bf0_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_DSCR_BF0_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(428)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1b0..0x1b8 - The second-to-last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_dscr_bf1_ch(&self, n: usize) -> &AHB_DMA_OUT_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(432)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1b0..0x1b8 - The second-to-last transmit descriptor address of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_dscr_bf1_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_DSCR_BF1_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(432)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1b4..0x1bc - Priority register of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_pri_ch(&self, n: usize) -> &AHB_DMA_OUT_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(436)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1b4..0x1bc - Priority register of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_pri_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_PRI_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(436)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1b8..0x1c0 - Peripheral selection register of TX channel %s"]
    #[inline(always)]
    pub const fn ahb_dma_out_peri_sel_ch(&self, n: usize) -> &AHB_DMA_OUT_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(440)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1b8..0x1c0 - Peripheral selection register of TX channel %s"]
    #[inline(always)]
    pub fn ahb_dma_out_peri_sel_ch_iter(&self) -> impl Iterator<Item = &AHB_DMA_OUT_PERI_SEL_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(440)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1bc..0x1c4 - TX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub const fn ahb_dma_tx_ch_arb_weigh_ch(&self, n: usize) -> &AHB_DMA_TX_CH_ARB_WEIGH_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(444)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1bc..0x1c4 - TX channel %s arbitration weight configuration register"]
    #[inline(always)]
    pub fn ahb_dma_tx_ch_arb_weigh_ch_iter(
        &self,
    ) -> impl Iterator<Item = &AHB_DMA_TX_CH_ARB_WEIGH_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(444)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1c0..0x1c8 - TX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub const fn ahb_dma_tx_arb_weigh_opt_dir_ch(
        &self,
        n: usize,
    ) -> &AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1c8 - TX channel %s weight arbitration optimization enable register"]
    #[inline(always)]
    pub fn ahb_dma_tx_arb_weigh_opt_dir_ch_iter(
        &self,
    ) -> impl Iterator<Item = &AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(448)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x280 - Configuration register 0 of TX channel 1"]
    #[inline(always)]
    pub const fn ahb_dma_out_conf0_ch1(&self) -> &AHB_DMA_OUT_CONF0_CH1 {
        &self.ahb_dma_out_conf0_ch1
    }
    #[doc = "0x600 - Accessible address space start address configuration register"]
    #[inline(always)]
    pub const fn ahb_dma_intr_mem_start_addr(&self) -> &AHB_DMA_INTR_MEM_START_ADDR {
        &self.ahb_dma_intr_mem_start_addr
    }
    #[doc = "0x604 - Accessible address space end address configuration register"]
    #[inline(always)]
    pub const fn ahb_dma_intr_mem_end_addr(&self) -> &AHB_DMA_INTR_MEM_END_ADDR {
        &self.ahb_dma_intr_mem_end_addr
    }
    #[doc = "0x608 - TX arbitration timeout configuration register"]
    #[inline(always)]
    pub const fn ahb_dma_arb_timeout(&self) -> &AHB_DMA_ARB_TIMEOUT {
        &self.ahb_dma_arb_timeout
    }
    #[doc = "0x610 - TX weight arbitration enable register"]
    #[inline(always)]
    pub const fn ahb_dma_weight_en(&self) -> &AHB_DMA_WEIGHT_EN {
        &self.ahb_dma_weight_en
    }
    #[doc = "0x618 - Module clock force on register"]
    #[inline(always)]
    pub const fn ahb_dma_module_clk_en(&self) -> &AHB_DMA_MODULE_CLK_EN {
        &self.ahb_dma_module_clk_en
    }
    #[doc = "0x620 - AHB response error status 0 register"]
    #[inline(always)]
    pub const fn ahb_dma_ahbinf_resp_err_status0(&self) -> &AHB_DMA_AHBINF_RESP_ERR_STATUS0 {
        &self.ahb_dma_ahbinf_resp_err_status0
    }
    #[doc = "0x624 - AHB response error status 1 register"]
    #[inline(always)]
    pub const fn ahb_dma_ahbinf_resp_err_status1(&self) -> &AHB_DMA_AHBINF_RESP_ERR_STATUS1 {
        &self.ahb_dma_ahbinf_resp_err_status1
    }
}
#[doc = "AHB_DMA_IN_INT_RAW_CH (rw) register accessor: Raw interrupt status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_int_raw_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_int_raw_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_int_raw_ch`] module"]
pub type AHB_DMA_IN_INT_RAW_CH = crate::Reg<ahb_dma_in_int_raw_ch::AHB_DMA_IN_INT_RAW_CH_SPEC>;
#[doc = "Raw interrupt status of RX channel %s"]
pub mod ahb_dma_in_int_raw_ch;
#[doc = "AHB_DMA_IN_INT_ST_CH (r) register accessor: Masked interrupt status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_int_st_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_int_st_ch`] module"]
pub type AHB_DMA_IN_INT_ST_CH = crate::Reg<ahb_dma_in_int_st_ch::AHB_DMA_IN_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt status of RX channel %s"]
pub mod ahb_dma_in_int_st_ch;
#[doc = "AHB_DMA_IN_INT_ENA_CH (rw) register accessor: Interrupt enable bits of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_int_ena_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_int_ena_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_int_ena_ch`] module"]
pub type AHB_DMA_IN_INT_ENA_CH = crate::Reg<ahb_dma_in_int_ena_ch::AHB_DMA_IN_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of RX channel %s"]
pub mod ahb_dma_in_int_ena_ch;
#[doc = "AHB_DMA_IN_INT_CLR_CH (w) register accessor: Interrupt clear bits of RX channel %s\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_int_clr_ch`] module"]
pub type AHB_DMA_IN_INT_CLR_CH = crate::Reg<ahb_dma_in_int_clr_ch::AHB_DMA_IN_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of RX channel %s"]
pub mod ahb_dma_in_int_clr_ch;
#[doc = "AHB_DMA_OUT_INT_RAW_CH (rw) register accessor: Raw interrupt status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_int_raw_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_int_raw_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_int_raw_ch`] module"]
pub type AHB_DMA_OUT_INT_RAW_CH = crate::Reg<ahb_dma_out_int_raw_ch::AHB_DMA_OUT_INT_RAW_CH_SPEC>;
#[doc = "Raw interrupt status of TX channel %s"]
pub mod ahb_dma_out_int_raw_ch;
#[doc = "AHB_DMA_OUT_INT_ST_CH (r) register accessor: Masked interrupt status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_int_st_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_int_st_ch`] module"]
pub type AHB_DMA_OUT_INT_ST_CH = crate::Reg<ahb_dma_out_int_st_ch::AHB_DMA_OUT_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt status of TX channel %s"]
pub mod ahb_dma_out_int_st_ch;
#[doc = "AHB_DMA_OUT_INT_ENA_CH (rw) register accessor: Interrupt enable bits of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_int_ena_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_int_ena_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_int_ena_ch`] module"]
pub type AHB_DMA_OUT_INT_ENA_CH = crate::Reg<ahb_dma_out_int_ena_ch::AHB_DMA_OUT_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of TX channel %s"]
pub mod ahb_dma_out_int_ena_ch;
#[doc = "AHB_DMA_OUT_INT_CLR_CH (w) register accessor: Interrupt clear bits of TX channel %s\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_int_clr_ch`] module"]
pub type AHB_DMA_OUT_INT_CLR_CH = crate::Reg<ahb_dma_out_int_clr_ch::AHB_DMA_OUT_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of TX channel %s"]
pub mod ahb_dma_out_int_clr_ch;
#[doc = "AHB_DMA_AHB_TEST (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_ahb_test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_ahb_test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_ahb_test`] module"]
pub type AHB_DMA_AHB_TEST = crate::Reg<ahb_dma_ahb_test::AHB_DMA_AHB_TEST_SPEC>;
#[doc = "reserved"]
pub mod ahb_dma_ahb_test;
#[doc = "AHB_DMA_MISC_CONF (rw) register accessor: Miscellaneous register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_misc_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_misc_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_misc_conf`] module"]
pub type AHB_DMA_MISC_CONF = crate::Reg<ahb_dma_misc_conf::AHB_DMA_MISC_CONF_SPEC>;
#[doc = "Miscellaneous register"]
pub mod ahb_dma_misc_conf;
#[doc = "AHB_DMA_DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_date`] module"]
pub type AHB_DMA_DATE = crate::Reg<ahb_dma_date::AHB_DMA_DATE_SPEC>;
#[doc = "Version control register"]
pub mod ahb_dma_date;
#[doc = "AHB_DMA_IN_CONF0_CH (rw) register accessor: Configuration register 0 of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_conf0_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_conf0_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_conf0_ch`] module"]
pub type AHB_DMA_IN_CONF0_CH = crate::Reg<ahb_dma_in_conf0_ch::AHB_DMA_IN_CONF0_CH_SPEC>;
#[doc = "Configuration register 0 of RX channel %s"]
pub mod ahb_dma_in_conf0_ch;
#[doc = "AHB_DMA_IN_CONF1_CH (rw) register accessor: Configuration register 1 of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_conf1_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_conf1_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_conf1_ch`] module"]
pub type AHB_DMA_IN_CONF1_CH = crate::Reg<ahb_dma_in_conf1_ch::AHB_DMA_IN_CONF1_CH_SPEC>;
#[doc = "Configuration register 1 of RX channel %s"]
pub mod ahb_dma_in_conf1_ch;
#[doc = "AHB_DMA_INFIFO_STATUS_CH (r) register accessor: Receive FIFO status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_infifo_status_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_infifo_status_ch`] module"]
pub type AHB_DMA_INFIFO_STATUS_CH =
    crate::Reg<ahb_dma_infifo_status_ch::AHB_DMA_INFIFO_STATUS_CH_SPEC>;
#[doc = "Receive FIFO status of RX channel %s"]
pub mod ahb_dma_infifo_status_ch;
#[doc = "AHB_DMA_IN_POP_CH (rw) register accessor: Pop control register of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_pop_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_pop_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_pop_ch`] module"]
pub type AHB_DMA_IN_POP_CH = crate::Reg<ahb_dma_in_pop_ch::AHB_DMA_IN_POP_CH_SPEC>;
#[doc = "Pop control register of RX channel %s"]
pub mod ahb_dma_in_pop_ch;
#[doc = "AHB_DMA_IN_LINK_CH (rw) register accessor: Linked list descriptor configuration and control register of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_link_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_link_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_link_ch`] module"]
pub type AHB_DMA_IN_LINK_CH = crate::Reg<ahb_dma_in_link_ch::AHB_DMA_IN_LINK_CH_SPEC>;
#[doc = "Linked list descriptor configuration and control register of RX channel %s"]
pub mod ahb_dma_in_link_ch;
#[doc = "AHB_DMA_IN_LINK_ADDR_CH (rw) register accessor: Link list descriptor address configuration of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_link_addr_ch`] module"]
pub type AHB_DMA_IN_LINK_ADDR_CH =
    crate::Reg<ahb_dma_in_link_addr_ch::AHB_DMA_IN_LINK_ADDR_CH_SPEC>;
#[doc = "Link list descriptor address configuration of RX channel %s"]
pub mod ahb_dma_in_link_addr_ch;
#[doc = "AHB_DMA_IN_STATE_CH (r) register accessor: Receive status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_state_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_state_ch`] module"]
pub type AHB_DMA_IN_STATE_CH = crate::Reg<ahb_dma_in_state_ch::AHB_DMA_IN_STATE_CH_SPEC>;
#[doc = "Receive status of RX channel %s"]
pub mod ahb_dma_in_state_ch;
#[doc = "AHB_DMA_IN_SUC_EOF_DES_ADDR_CH (r) register accessor: Receive descriptor address when EOF occurs on RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_suc_eof_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_suc_eof_des_addr_ch`] module"]
pub type AHB_DMA_IN_SUC_EOF_DES_ADDR_CH =
    crate::Reg<ahb_dma_in_suc_eof_des_addr_ch::AHB_DMA_IN_SUC_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Receive descriptor address when EOF occurs on RX channel %s"]
pub mod ahb_dma_in_suc_eof_des_addr_ch;
#[doc = "AHB_DMA_IN_ERR_EOF_DES_ADDR_CH (r) register accessor: Receive descriptor address when errors occur of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_err_eof_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_err_eof_des_addr_ch`] module"]
pub type AHB_DMA_IN_ERR_EOF_DES_ADDR_CH =
    crate::Reg<ahb_dma_in_err_eof_des_addr_ch::AHB_DMA_IN_ERR_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Receive descriptor address when errors occur of RX channel %s"]
pub mod ahb_dma_in_err_eof_des_addr_ch;
#[doc = "AHB_DMA_IN_DONE_DES_ADDR_CH (r) register accessor: RX_done inlink descriptor address of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_done_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_done_des_addr_ch`] module"]
pub type AHB_DMA_IN_DONE_DES_ADDR_CH =
    crate::Reg<ahb_dma_in_done_des_addr_ch::AHB_DMA_IN_DONE_DES_ADDR_CH_SPEC>;
#[doc = "RX_done inlink descriptor address of RX channel %s"]
pub mod ahb_dma_in_done_des_addr_ch;
#[doc = "AHB_DMA_IN_DSCR_CH (r) register accessor: Current receive descriptor address of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_dscr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_dscr_ch`] module"]
pub type AHB_DMA_IN_DSCR_CH = crate::Reg<ahb_dma_in_dscr_ch::AHB_DMA_IN_DSCR_CH_SPEC>;
#[doc = "Current receive descriptor address of RX channel %s"]
pub mod ahb_dma_in_dscr_ch;
#[doc = "AHB_DMA_IN_DSCR_BF0_CH (r) register accessor: The last receive descriptor address of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_dscr_bf0_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_dscr_bf0_ch`] module"]
pub type AHB_DMA_IN_DSCR_BF0_CH = crate::Reg<ahb_dma_in_dscr_bf0_ch::AHB_DMA_IN_DSCR_BF0_CH_SPEC>;
#[doc = "The last receive descriptor address of RX channel %s"]
pub mod ahb_dma_in_dscr_bf0_ch;
#[doc = "AHB_DMA_IN_DSCR_BF1_CH (r) register accessor: The second-to-last receive descriptor address of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_dscr_bf1_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_dscr_bf1_ch`] module"]
pub type AHB_DMA_IN_DSCR_BF1_CH = crate::Reg<ahb_dma_in_dscr_bf1_ch::AHB_DMA_IN_DSCR_BF1_CH_SPEC>;
#[doc = "The second-to-last receive descriptor address of RX channel %s"]
pub mod ahb_dma_in_dscr_bf1_ch;
#[doc = "AHB_DMA_IN_PRI_CH (rw) register accessor: Priority register of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_pri_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_pri_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_pri_ch`] module"]
pub type AHB_DMA_IN_PRI_CH = crate::Reg<ahb_dma_in_pri_ch::AHB_DMA_IN_PRI_CH_SPEC>;
#[doc = "Priority register of RX channel %s"]
pub mod ahb_dma_in_pri_ch;
#[doc = "AHB_DMA_IN_PERI_SEL_CH (rw) register accessor: Peripheral selection register of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_peri_sel_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_peri_sel_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_in_peri_sel_ch`] module"]
pub type AHB_DMA_IN_PERI_SEL_CH = crate::Reg<ahb_dma_in_peri_sel_ch::AHB_DMA_IN_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection register of RX channel %s"]
pub mod ahb_dma_in_peri_sel_ch;
#[doc = "AHB_DMA_RX_CH_ARB_WEIGH_CH (rw) register accessor: RX channel %s arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_rx_ch_arb_weigh_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_rx_ch_arb_weigh_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_rx_ch_arb_weigh_ch`] module"]
pub type AHB_DMA_RX_CH_ARB_WEIGH_CH =
    crate::Reg<ahb_dma_rx_ch_arb_weigh_ch::AHB_DMA_RX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "RX channel %s arbitration weight configuration register"]
pub mod ahb_dma_rx_ch_arb_weigh_ch;
#[doc = "AHB_DMA_RX_ARB_WEIGH_OPT_DIR_CH (rw) register accessor: RX channel %s weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_rx_arb_weigh_opt_dir_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_rx_arb_weigh_opt_dir_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_rx_arb_weigh_opt_dir_ch`] module"]
pub type AHB_DMA_RX_ARB_WEIGH_OPT_DIR_CH =
    crate::Reg<ahb_dma_rx_arb_weigh_opt_dir_ch::AHB_DMA_RX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "RX channel %s weight arbitration optimization enable register"]
pub mod ahb_dma_rx_arb_weigh_opt_dir_ch;
#[doc = "AHB_DMA_OUT_CONF0_CH0 (rw) register accessor: Configuration register 0 of TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_conf0_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_conf0_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_conf0_ch0`] module"]
pub type AHB_DMA_OUT_CONF0_CH0 = crate::Reg<ahb_dma_out_conf0_ch0::AHB_DMA_OUT_CONF0_CH0_SPEC>;
#[doc = "Configuration register 0 of TX channel 0"]
pub mod ahb_dma_out_conf0_ch0;
#[doc = "AHB_DMA_OUT_CONF1_CH (rw) register accessor: Configuration register 1 of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_conf1_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_conf1_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_conf1_ch`] module"]
pub type AHB_DMA_OUT_CONF1_CH = crate::Reg<ahb_dma_out_conf1_ch::AHB_DMA_OUT_CONF1_CH_SPEC>;
#[doc = "Configuration register 1 of TX channel %s"]
pub mod ahb_dma_out_conf1_ch;
#[doc = "AHB_DMA_OUTFIFO_STATUS_CH (r) register accessor: Transmit FIFO status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_outfifo_status_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_outfifo_status_ch`] module"]
pub type AHB_DMA_OUTFIFO_STATUS_CH =
    crate::Reg<ahb_dma_outfifo_status_ch::AHB_DMA_OUTFIFO_STATUS_CH_SPEC>;
#[doc = "Transmit FIFO status of TX channel %s"]
pub mod ahb_dma_outfifo_status_ch;
#[doc = "AHB_DMA_OUT_PUSH_CH (rw) register accessor: Push control register of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_push_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_push_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_push_ch`] module"]
pub type AHB_DMA_OUT_PUSH_CH = crate::Reg<ahb_dma_out_push_ch::AHB_DMA_OUT_PUSH_CH_SPEC>;
#[doc = "Push control register of TX channel %s"]
pub mod ahb_dma_out_push_ch;
#[doc = "AHB_DMA_OUT_LINK_CH (rw) register accessor: Linked list descriptor configuration and control register of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_link_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_link_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_link_ch`] module"]
pub type AHB_DMA_OUT_LINK_CH = crate::Reg<ahb_dma_out_link_ch::AHB_DMA_OUT_LINK_CH_SPEC>;
#[doc = "Linked list descriptor configuration and control register of TX channel %s"]
pub mod ahb_dma_out_link_ch;
#[doc = "AHB_DMA_OUT_LINK_ADDR_CH (rw) register accessor: Link list descriptor address configuration of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_link_addr_ch`] module"]
pub type AHB_DMA_OUT_LINK_ADDR_CH =
    crate::Reg<ahb_dma_out_link_addr_ch::AHB_DMA_OUT_LINK_ADDR_CH_SPEC>;
#[doc = "Link list descriptor address configuration of TX channel %s"]
pub mod ahb_dma_out_link_addr_ch;
#[doc = "AHB_DMA_OUT_STATE_CH (r) register accessor: Transmit status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_state_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_state_ch`] module"]
pub type AHB_DMA_OUT_STATE_CH = crate::Reg<ahb_dma_out_state_ch::AHB_DMA_OUT_STATE_CH_SPEC>;
#[doc = "Transmit status of TX channel %s"]
pub mod ahb_dma_out_state_ch;
#[doc = "AHB_DMA_OUT_EOF_DES_ADDR_CH (r) register accessor: Transmit descriptor address when EOF occurs on TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_eof_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_eof_des_addr_ch`] module"]
pub type AHB_DMA_OUT_EOF_DES_ADDR_CH =
    crate::Reg<ahb_dma_out_eof_des_addr_ch::AHB_DMA_OUT_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Transmit descriptor address when EOF occurs on TX channel %s"]
pub mod ahb_dma_out_eof_des_addr_ch;
#[doc = "AHB_DMA_OUT_EOF_BFR_DES_ADDR_CH (r) register accessor: The last transmit descriptor address when EOF occurs on TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_eof_bfr_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_eof_bfr_des_addr_ch`] module"]
pub type AHB_DMA_OUT_EOF_BFR_DES_ADDR_CH =
    crate::Reg<ahb_dma_out_eof_bfr_des_addr_ch::AHB_DMA_OUT_EOF_BFR_DES_ADDR_CH_SPEC>;
#[doc = "The last transmit descriptor address when EOF occurs on TX channel %s"]
pub mod ahb_dma_out_eof_bfr_des_addr_ch;
#[doc = "AHB_DMA_OUT_DONE_DES_ADDR_CH (r) register accessor: TX done outlink descriptor address of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_done_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_done_des_addr_ch`] module"]
pub type AHB_DMA_OUT_DONE_DES_ADDR_CH =
    crate::Reg<ahb_dma_out_done_des_addr_ch::AHB_DMA_OUT_DONE_DES_ADDR_CH_SPEC>;
#[doc = "TX done outlink descriptor address of TX channel %s"]
pub mod ahb_dma_out_done_des_addr_ch;
#[doc = "AHB_DMA_OUT_DSCR_CH (r) register accessor: Current transmit descriptor address of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_dscr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_dscr_ch`] module"]
pub type AHB_DMA_OUT_DSCR_CH = crate::Reg<ahb_dma_out_dscr_ch::AHB_DMA_OUT_DSCR_CH_SPEC>;
#[doc = "Current transmit descriptor address of TX channel %s"]
pub mod ahb_dma_out_dscr_ch;
#[doc = "AHB_DMA_OUT_DSCR_BF0_CH (r) register accessor: The last transmit descriptor address of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_dscr_bf0_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_dscr_bf0_ch`] module"]
pub type AHB_DMA_OUT_DSCR_BF0_CH =
    crate::Reg<ahb_dma_out_dscr_bf0_ch::AHB_DMA_OUT_DSCR_BF0_CH_SPEC>;
#[doc = "The last transmit descriptor address of TX channel %s"]
pub mod ahb_dma_out_dscr_bf0_ch;
#[doc = "AHB_DMA_OUT_DSCR_BF1_CH (r) register accessor: The second-to-last transmit descriptor address of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_dscr_bf1_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_dscr_bf1_ch`] module"]
pub type AHB_DMA_OUT_DSCR_BF1_CH =
    crate::Reg<ahb_dma_out_dscr_bf1_ch::AHB_DMA_OUT_DSCR_BF1_CH_SPEC>;
#[doc = "The second-to-last transmit descriptor address of TX channel %s"]
pub mod ahb_dma_out_dscr_bf1_ch;
#[doc = "AHB_DMA_OUT_PRI_CH (rw) register accessor: Priority register of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_pri_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_pri_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_pri_ch`] module"]
pub type AHB_DMA_OUT_PRI_CH = crate::Reg<ahb_dma_out_pri_ch::AHB_DMA_OUT_PRI_CH_SPEC>;
#[doc = "Priority register of TX channel %s"]
pub mod ahb_dma_out_pri_ch;
#[doc = "AHB_DMA_OUT_PERI_SEL_CH (rw) register accessor: Peripheral selection register of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_peri_sel_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_peri_sel_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_peri_sel_ch`] module"]
pub type AHB_DMA_OUT_PERI_SEL_CH =
    crate::Reg<ahb_dma_out_peri_sel_ch::AHB_DMA_OUT_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection register of TX channel %s"]
pub mod ahb_dma_out_peri_sel_ch;
#[doc = "AHB_DMA_TX_CH_ARB_WEIGH_CH (rw) register accessor: TX channel %s arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_tx_ch_arb_weigh_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_tx_ch_arb_weigh_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_tx_ch_arb_weigh_ch`] module"]
pub type AHB_DMA_TX_CH_ARB_WEIGH_CH =
    crate::Reg<ahb_dma_tx_ch_arb_weigh_ch::AHB_DMA_TX_CH_ARB_WEIGH_CH_SPEC>;
#[doc = "TX channel %s arbitration weight configuration register"]
pub mod ahb_dma_tx_ch_arb_weigh_ch;
#[doc = "AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH (rw) register accessor: TX channel %s weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_tx_arb_weigh_opt_dir_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_tx_arb_weigh_opt_dir_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_tx_arb_weigh_opt_dir_ch`] module"]
pub type AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH =
    crate::Reg<ahb_dma_tx_arb_weigh_opt_dir_ch::AHB_DMA_TX_ARB_WEIGH_OPT_DIR_CH_SPEC>;
#[doc = "TX channel %s weight arbitration optimization enable register"]
pub mod ahb_dma_tx_arb_weigh_opt_dir_ch;
#[doc = "AHB_DMA_OUT_CONF0_CH1 (rw) register accessor: Configuration register 0 of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_out_conf0_ch1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_out_conf0_ch1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_out_conf0_ch1`] module"]
pub type AHB_DMA_OUT_CONF0_CH1 = crate::Reg<ahb_dma_out_conf0_ch1::AHB_DMA_OUT_CONF0_CH1_SPEC>;
#[doc = "Configuration register 0 of TX channel 1"]
pub mod ahb_dma_out_conf0_ch1;
#[doc = "AHB_DMA_INTR_MEM_START_ADDR (rw) register accessor: Accessible address space start address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_intr_mem_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_intr_mem_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_intr_mem_start_addr`] module"]
pub type AHB_DMA_INTR_MEM_START_ADDR =
    crate::Reg<ahb_dma_intr_mem_start_addr::AHB_DMA_INTR_MEM_START_ADDR_SPEC>;
#[doc = "Accessible address space start address configuration register"]
pub mod ahb_dma_intr_mem_start_addr;
#[doc = "AHB_DMA_INTR_MEM_END_ADDR (rw) register accessor: Accessible address space end address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_intr_mem_end_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_intr_mem_end_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_intr_mem_end_addr`] module"]
pub type AHB_DMA_INTR_MEM_END_ADDR =
    crate::Reg<ahb_dma_intr_mem_end_addr::AHB_DMA_INTR_MEM_END_ADDR_SPEC>;
#[doc = "Accessible address space end address configuration register"]
pub mod ahb_dma_intr_mem_end_addr;
#[doc = "AHB_DMA_ARB_TIMEOUT (rw) register accessor: TX arbitration timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_arb_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_arb_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_arb_timeout`] module"]
pub type AHB_DMA_ARB_TIMEOUT = crate::Reg<ahb_dma_arb_timeout::AHB_DMA_ARB_TIMEOUT_SPEC>;
#[doc = "TX arbitration timeout configuration register"]
pub mod ahb_dma_arb_timeout;
#[doc = "AHB_DMA_WEIGHT_EN (rw) register accessor: TX weight arbitration enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_weight_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_weight_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_weight_en`] module"]
pub type AHB_DMA_WEIGHT_EN = crate::Reg<ahb_dma_weight_en::AHB_DMA_WEIGHT_EN_SPEC>;
#[doc = "TX weight arbitration enable register"]
pub mod ahb_dma_weight_en;
#[doc = "AHB_DMA_MODULE_CLK_EN (rw) register accessor: Module clock force on register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_module_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_module_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_module_clk_en`] module"]
pub type AHB_DMA_MODULE_CLK_EN = crate::Reg<ahb_dma_module_clk_en::AHB_DMA_MODULE_CLK_EN_SPEC>;
#[doc = "Module clock force on register"]
pub mod ahb_dma_module_clk_en;
#[doc = "AHB_DMA_AHBINF_RESP_ERR_STATUS0 (r) register accessor: AHB response error status 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_ahbinf_resp_err_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_ahbinf_resp_err_status0`] module"]
pub type AHB_DMA_AHBINF_RESP_ERR_STATUS0 =
    crate::Reg<ahb_dma_ahbinf_resp_err_status0::AHB_DMA_AHBINF_RESP_ERR_STATUS0_SPEC>;
#[doc = "AHB response error status 0 register"]
pub mod ahb_dma_ahbinf_resp_err_status0;
#[doc = "AHB_DMA_AHBINF_RESP_ERR_STATUS1 (r) register accessor: AHB response error status 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_ahbinf_resp_err_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_dma_ahbinf_resp_err_status1`] module"]
pub type AHB_DMA_AHBINF_RESP_ERR_STATUS1 =
    crate::Reg<ahb_dma_ahbinf_resp_err_status1::AHB_DMA_AHBINF_RESP_ERR_STATUS1_SPEC>;
#[doc = "AHB response error status 1 register"]
pub mod ahb_dma_ahbinf_resp_err_status1;
