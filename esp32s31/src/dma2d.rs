#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    out_conf0_ch: (),
    _reserved1: [u8; 0x04],
    out_int_raw_ch: (),
    _reserved2: [u8; 0x04],
    out_int_ena_ch: (),
    _reserved3: [u8; 0x04],
    out_int_st_ch: (),
    _reserved4: [u8; 0x04],
    out_int_clr_ch: (),
    _reserved5: [u8; 0x04],
    outfifo_status_ch: (),
    _reserved6: [u8; 0x04],
    out_push_ch: (),
    _reserved7: [u8; 0x04],
    out_link_conf_ch: (),
    _reserved8: [u8; 0x04],
    out_link_addr_ch: (),
    _reserved9: [u8; 0x04],
    out_state_ch: (),
    _reserved10: [u8; 0x04],
    out_eof_des_addr_ch: (),
    _reserved11: [u8; 0x04],
    out_dscr_ch: (),
    _reserved12: [u8; 0x04],
    out_dscr_bf0_ch: (),
    _reserved13: [u8; 0x04],
    out_dscr_bf1_ch: (),
    _reserved14: [u8; 0x04],
    out_peri_sel_ch: (),
    _reserved15: [u8; 0x04],
    out_arb_ch: (),
    _reserved16: [u8; 0x04],
    out_ro_status_ch: (),
    _reserved17: [u8; 0x04],
    out_ro_pd_conf_ch0: OUT_RO_PD_CONF_CH0,
    out_color_convert_ch: (),
    _reserved19: [u8; 0x04],
    out_scramble_ch: (),
    _reserved20: [u8; 0x04],
    out_color_param0_ch: (),
    _reserved21: [u8; 0x04],
    out_color_param1_ch: (),
    _reserved22: [u8; 0x04],
    out_color_param2_ch: (),
    _reserved23: [u8; 0x04],
    out_color_param3_ch: (),
    _reserved24: [u8; 0x04],
    out_color_param4_ch: (),
    _reserved25: [u8; 0x04],
    out_color_param5_ch: (),
    _reserved26: [u8; 0x04],
    out_etm_conf_ch: (),
    _reserved27: [u8; 0x04],
    out_dscr_port_blk_ch: (),
    _reserved28: [u8; 0x0494],
    in_conf0_ch: (),
    _reserved29: [u8; 0x04],
    in_int_raw_ch: (),
    _reserved30: [u8; 0x04],
    in_int_ena_ch: (),
    _reserved31: [u8; 0x04],
    in_int_st_ch: (),
    _reserved32: [u8; 0x04],
    in_int_clr_ch: (),
    _reserved33: [u8; 0x04],
    infifo_status_ch: (),
    _reserved34: [u8; 0x04],
    in_pop_ch: (),
    _reserved35: [u8; 0x04],
    in_link_conf_ch: (),
    _reserved36: [u8; 0x04],
    in_link_addr_ch: (),
    _reserved37: [u8; 0x04],
    in_state_ch: (),
    _reserved38: [u8; 0x04],
    in_suc_eof_des_addr_ch: (),
    _reserved39: [u8; 0x04],
    in_err_eof_des_addr_ch: (),
    _reserved40: [u8; 0x04],
    in_dscr_ch: (),
    _reserved41: [u8; 0x04],
    in_dscr_bf0_ch: (),
    _reserved42: [u8; 0x04],
    in_dscr_bf1_ch: (),
    _reserved43: [u8; 0x04],
    in_peri_sel_ch: (),
    _reserved44: [u8; 0x04],
    in_arb_ch: (),
    _reserved45: [u8; 0x04],
    in_ro_status_ch: (),
    _reserved46: [u8; 0x04],
    in_ro_pd_conf_ch0: IN_RO_PD_CONF_CH0,
    in_color_convert_ch0: IN_COLOR_CONVERT_CH0,
    in_scramble_ch0: IN_SCRAMBLE_CH0,
    in_color_param0_ch0: IN_COLOR_PARAM0_CH0,
    in_color_param1_ch0: IN_COLOR_PARAM1_CH0,
    in_color_param2_ch0: IN_COLOR_PARAM2_CH0,
    in_color_param3_ch0: IN_COLOR_PARAM3_CH0,
    in_color_param4_ch0: IN_COLOR_PARAM4_CH0,
    in_color_param5_ch0: IN_COLOR_PARAM5_CH0,
    in_etm_conf_ch: (),
    _reserved56: [u8; 0x0494],
    axi_err: AXI_ERR,
    rst_conf: RST_CONF,
    intr_mem_start_addr: INTR_MEM_START_ADDR,
    intr_mem_end_addr: INTR_MEM_END_ADDR,
    extr_mem_start_addr: EXTR_MEM_START_ADDR,
    extr_mem_end_addr: EXTR_MEM_END_ADDR,
    out_arb_config: OUT_ARB_CONFIG,
    in_arb_config: IN_ARB_CONFIG,
    rdn_result: RDN_RESULT,
    rdn_eco_high: RDN_ECO_HIGH,
    rdn_eco_low: RDN_ECO_LOW,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - Configures the tx direction of channel %s"]
    #[inline(always)]
    pub const fn out_conf0_ch(&self, n: usize) -> &OUT_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - Configures the tx direction of channel %s"]
    #[inline(always)]
    pub fn out_conf0_ch_iter(&self) -> impl Iterator<Item = &OUT_CONF0_CH> {
        (0..4).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(256 * n).cast() })
    }
    #[doc = "0x04..0x14 - Raw interrupt status of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_raw_ch(&self, n: usize) -> &OUT_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x14 - Raw interrupt status of TX channel %s"]
    #[inline(always)]
    pub fn out_int_raw_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_RAW_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x08..0x18 - Interrupt enable bits of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_ena_ch(&self, n: usize) -> &OUT_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x18 - Interrupt enable bits of TX channel %s"]
    #[inline(always)]
    pub fn out_int_ena_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_ENA_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x0c..0x1c - Masked interrupt status of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_st_ch(&self, n: usize) -> &OUT_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x1c - Masked interrupt status of TX channel %s"]
    #[inline(always)]
    pub fn out_int_st_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_ST_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(12)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x10..0x20 - Interrupt clear bits of TX channel %s"]
    #[inline(always)]
    pub const fn out_int_clr_ch(&self, n: usize) -> &OUT_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Interrupt clear bits of TX channel %s"]
    #[inline(always)]
    pub fn out_int_clr_ch_iter(&self) -> impl Iterator<Item = &OUT_INT_CLR_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(16)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x14..0x24 - Represents the status of the tx fifo of channel %s"]
    #[inline(always)]
    pub const fn outfifo_status_ch(&self, n: usize) -> &OUTFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(20)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x24 - Represents the status of the tx fifo of channel %s"]
    #[inline(always)]
    pub fn outfifo_status_ch_iter(&self) -> impl Iterator<Item = &OUTFIFO_STATUS_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(20)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x18..0x28 - Configures the tx fifo of channel %s"]
    #[inline(always)]
    pub const fn out_push_ch(&self, n: usize) -> &OUT_PUSH_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(24)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x28 - Configures the tx fifo of channel %s"]
    #[inline(always)]
    pub fn out_push_ch_iter(&self) -> impl Iterator<Item = &OUT_PUSH_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(24)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x1c..0x2c - Configures the tx descriptor operations of channel %s"]
    #[inline(always)]
    pub const fn out_link_conf_ch(&self, n: usize) -> &OUT_LINK_CONF_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(28)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x2c - Configures the tx descriptor operations of channel %s"]
    #[inline(always)]
    pub fn out_link_conf_ch_iter(&self) -> impl Iterator<Item = &OUT_LINK_CONF_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(28)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x20..0x30 - Configures the tx descriptor address of channel %s"]
    #[inline(always)]
    pub const fn out_link_addr_ch(&self, n: usize) -> &OUT_LINK_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(32)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - Configures the tx descriptor address of channel %s"]
    #[inline(always)]
    pub fn out_link_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_LINK_ADDR_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(32)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x24..0x34 - Represents the working status of the tx descriptor of channel %s"]
    #[inline(always)]
    pub const fn out_state_ch(&self, n: usize) -> &OUT_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24..0x34 - Represents the working status of the tx descriptor of channel %s"]
    #[inline(always)]
    pub fn out_state_ch_iter(&self) -> impl Iterator<Item = &OUT_STATE_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x28..0x38 - Represents the address associated with the outlink descriptor of channel %s"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch(&self, n: usize) -> &OUT_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(40)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28..0x38 - Represents the address associated with the outlink descriptor of channel %s"]
    #[inline(always)]
    pub fn out_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &OUT_EOF_DES_ADDR_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(40)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x2c..0x3c - Represents the address associated with the outlink descriptor of channel %s"]
    #[inline(always)]
    pub const fn out_dscr_ch(&self, n: usize) -> &OUT_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(44)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c..0x3c - Represents the address associated with the outlink descriptor of channel %s"]
    #[inline(always)]
    pub fn out_dscr_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(44)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x30..0x40 - Represents the address associated with the outlink descriptor of channel %s"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch(&self, n: usize) -> &OUT_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(48)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - Represents the address associated with the outlink descriptor of channel %s"]
    #[inline(always)]
    pub fn out_dscr_bf0_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_BF0_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(48)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x34..0x44 - Represents the address associated with the outlink descriptor of channel %s"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch(&self, n: usize) -> &OUT_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(52)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x44 - Represents the address associated with the outlink descriptor of channel %s"]
    #[inline(always)]
    pub fn out_dscr_bf1_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_BF1_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(52)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x38..0x48 - Configures the tx peripheral of channel %s"]
    #[inline(always)]
    pub const fn out_peri_sel_ch(&self, n: usize) -> &OUT_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(56)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x48 - Configures the tx peripheral of channel %s"]
    #[inline(always)]
    pub fn out_peri_sel_ch_iter(&self) -> impl Iterator<Item = &OUT_PERI_SEL_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(56)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x3c..0x4c - Configures the tx arbiter of channel %s"]
    #[inline(always)]
    pub const fn out_arb_ch(&self, n: usize) -> &OUT_ARB_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(60)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0x4c - Configures the tx arbiter of channel %s"]
    #[inline(always)]
    pub fn out_arb_ch_iter(&self) -> impl Iterator<Item = &OUT_ARB_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(60)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x40..0x50 - Represents the status of the tx reorder module of channel %s"]
    #[inline(always)]
    pub const fn out_ro_status_ch(&self, n: usize) -> &OUT_RO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(64)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Represents the status of the tx reorder module of channel %s"]
    #[inline(always)]
    pub fn out_ro_status_ch_iter(&self) -> impl Iterator<Item = &OUT_RO_STATUS_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(64)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x44 - Configures the tx reorder memory of channel 0"]
    #[inline(always)]
    pub const fn out_ro_pd_conf_ch0(&self) -> &OUT_RO_PD_CONF_CH0 {
        &self.out_ro_pd_conf_ch0
    }
    #[doc = "0x48..0x58 - Configures the tx color convert of channel %s"]
    #[inline(always)]
    pub const fn out_color_convert_ch(&self, n: usize) -> &OUT_COLOR_CONVERT_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(72)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x48..0x58 - Configures the tx color convert of channel %s"]
    #[inline(always)]
    pub fn out_color_convert_ch_iter(&self) -> impl Iterator<Item = &OUT_COLOR_CONVERT_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(72)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x4c..0x5c - Configures the tx scramble of channel %s"]
    #[inline(always)]
    pub const fn out_scramble_ch(&self, n: usize) -> &OUT_SCRAMBLE_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(76)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4c..0x5c - Configures the tx scramble of channel %s"]
    #[inline(always)]
    pub fn out_scramble_ch_iter(&self) -> impl Iterator<Item = &OUT_SCRAMBLE_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(76)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x50..0x60 - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub const fn out_color_param0_ch(&self, n: usize) -> &OUT_COLOR_PARAM0_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(80)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x60 - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub fn out_color_param0_ch_iter(&self) -> impl Iterator<Item = &OUT_COLOR_PARAM0_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(80)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x54..0x64 - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub const fn out_color_param1_ch(&self, n: usize) -> &OUT_COLOR_PARAM1_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x54..0x64 - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub fn out_color_param1_ch_iter(&self) -> impl Iterator<Item = &OUT_COLOR_PARAM1_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(84)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x58..0x68 - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub const fn out_color_param2_ch(&self, n: usize) -> &OUT_COLOR_PARAM2_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(88)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x68 - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub fn out_color_param2_ch_iter(&self) -> impl Iterator<Item = &OUT_COLOR_PARAM2_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(88)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x5c..0x6c - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub const fn out_color_param3_ch(&self, n: usize) -> &OUT_COLOR_PARAM3_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(92)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c..0x6c - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub fn out_color_param3_ch_iter(&self) -> impl Iterator<Item = &OUT_COLOR_PARAM3_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(92)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x60..0x70 - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub const fn out_color_param4_ch(&self, n: usize) -> &OUT_COLOR_PARAM4_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(96)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x70 - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub fn out_color_param4_ch_iter(&self) -> impl Iterator<Item = &OUT_COLOR_PARAM4_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(96)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x64..0x74 - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub const fn out_color_param5_ch(&self, n: usize) -> &OUT_COLOR_PARAM5_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(100)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x64..0x74 - Configures the tx color convert parameter of channel %s"]
    #[inline(always)]
    pub fn out_color_param5_ch_iter(&self) -> impl Iterator<Item = &OUT_COLOR_PARAM5_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(100)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x68..0x78 - Configures the tx etm of channel %s"]
    #[inline(always)]
    pub const fn out_etm_conf_ch(&self, n: usize) -> &OUT_ETM_CONF_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(104)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x68..0x78 - Configures the tx etm of channel %s"]
    #[inline(always)]
    pub fn out_etm_conf_ch_iter(&self) -> impl Iterator<Item = &OUT_ETM_CONF_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(104)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x6c..0x7c - Configures the tx block size in dscr port mode"]
    #[inline(always)]
    pub const fn out_dscr_port_blk_ch(&self, n: usize) -> &OUT_DSCR_PORT_BLK_CH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(108)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x6c..0x7c - Configures the tx block size in dscr port mode"]
    #[inline(always)]
    pub fn out_dscr_port_blk_ch_iter(&self) -> impl Iterator<Item = &OUT_DSCR_PORT_BLK_CH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(108)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x500..0x50c - Configures the rx direction of channel %s"]
    #[inline(always)]
    pub const fn in_conf0_ch(&self, n: usize) -> &IN_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1280)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x50c - Configures the rx direction of channel %s"]
    #[inline(always)]
    pub fn in_conf0_ch_iter(&self) -> impl Iterator<Item = &IN_CONF0_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1280)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x504..0x510 - Raw interrupt status of RX channel %s"]
    #[inline(always)]
    pub const fn in_int_raw_ch(&self, n: usize) -> &IN_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1284)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x504..0x510 - Raw interrupt status of RX channel %s"]
    #[inline(always)]
    pub fn in_int_raw_ch_iter(&self) -> impl Iterator<Item = &IN_INT_RAW_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1284)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x508..0x514 - Interrupt enable bits of RX channel %s"]
    #[inline(always)]
    pub const fn in_int_ena_ch(&self, n: usize) -> &IN_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1288)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x508..0x514 - Interrupt enable bits of RX channel %s"]
    #[inline(always)]
    pub fn in_int_ena_ch_iter(&self) -> impl Iterator<Item = &IN_INT_ENA_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1288)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x50c..0x518 - Masked interrupt status of RX channel %s"]
    #[inline(always)]
    pub const fn in_int_st_ch(&self, n: usize) -> &IN_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1292)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50c..0x518 - Masked interrupt status of RX channel %s"]
    #[inline(always)]
    pub fn in_int_st_ch_iter(&self) -> impl Iterator<Item = &IN_INT_ST_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1292)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x510..0x51c - Interrupt clear bits of RX channel %s"]
    #[inline(always)]
    pub const fn in_int_clr_ch(&self, n: usize) -> &IN_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1296)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x510..0x51c - Interrupt clear bits of RX channel %s"]
    #[inline(always)]
    pub fn in_int_clr_ch_iter(&self) -> impl Iterator<Item = &IN_INT_CLR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1296)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x514..0x520 - Represents the status of the rx fifo of channel %s"]
    #[inline(always)]
    pub const fn infifo_status_ch(&self, n: usize) -> &INFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1300)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x514..0x520 - Represents the status of the rx fifo of channel %s"]
    #[inline(always)]
    pub fn infifo_status_ch_iter(&self) -> impl Iterator<Item = &INFIFO_STATUS_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1300)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x518..0x524 - Configures the rx fifo of channel %s"]
    #[inline(always)]
    pub const fn in_pop_ch(&self, n: usize) -> &IN_POP_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1304)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x518..0x524 - Configures the rx fifo of channel %s"]
    #[inline(always)]
    pub fn in_pop_ch_iter(&self) -> impl Iterator<Item = &IN_POP_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1304)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x51c..0x528 - Configures the rx descriptor operations of channel %s"]
    #[inline(always)]
    pub const fn in_link_conf_ch(&self, n: usize) -> &IN_LINK_CONF_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1308)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x51c..0x528 - Configures the rx descriptor operations of channel %s"]
    #[inline(always)]
    pub fn in_link_conf_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_CONF_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1308)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x520..0x52c - Configures the rx descriptor address of channel %s"]
    #[inline(always)]
    pub const fn in_link_addr_ch(&self, n: usize) -> &IN_LINK_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1312)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x520..0x52c - Configures the rx descriptor address of channel %s"]
    #[inline(always)]
    pub fn in_link_addr_ch_iter(&self) -> impl Iterator<Item = &IN_LINK_ADDR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1312)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x524..0x530 - Represents the working status of the rx descriptor of channel %s"]
    #[inline(always)]
    pub const fn in_state_ch(&self, n: usize) -> &IN_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1316)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x524..0x530 - Represents the working status of the rx descriptor of channel %s"]
    #[inline(always)]
    pub fn in_state_ch_iter(&self) -> impl Iterator<Item = &IN_STATE_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1316)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x528..0x534 - Represents the address associated with the inlink descriptor of channel %s"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch(&self, n: usize) -> &IN_SUC_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1320)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x528..0x534 - Represents the address associated with the inlink descriptor of channel %s"]
    #[inline(always)]
    pub fn in_suc_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &IN_SUC_EOF_DES_ADDR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1320)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x52c..0x538 - Represents the address associated with the inlink descriptor of channel %s"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch(&self, n: usize) -> &IN_ERR_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1324)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x52c..0x538 - Represents the address associated with the inlink descriptor of channel %s"]
    #[inline(always)]
    pub fn in_err_eof_des_addr_ch_iter(&self) -> impl Iterator<Item = &IN_ERR_EOF_DES_ADDR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1324)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x530..0x53c - Represents the address associated with the inlink descriptor of channel %s"]
    #[inline(always)]
    pub const fn in_dscr_ch(&self, n: usize) -> &IN_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1328)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x530..0x53c - Represents the address associated with the inlink descriptor of channel %s"]
    #[inline(always)]
    pub fn in_dscr_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1328)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x534..0x540 - Represents the address associated with the inlink descriptor of channel %s"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch(&self, n: usize) -> &IN_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1332)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x534..0x540 - Represents the address associated with the inlink descriptor of channel %s"]
    #[inline(always)]
    pub fn in_dscr_bf0_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_BF0_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1332)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x538..0x544 - Represents the address associated with the inlink descriptor of channel %s"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch(&self, n: usize) -> &IN_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1336)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x538..0x544 - Represents the address associated with the inlink descriptor of channel %s"]
    #[inline(always)]
    pub fn in_dscr_bf1_ch_iter(&self) -> impl Iterator<Item = &IN_DSCR_BF1_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1336)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x53c..0x548 - Configures the rx peripheral of channel %s"]
    #[inline(always)]
    pub const fn in_peri_sel_ch(&self, n: usize) -> &IN_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1340)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x53c..0x548 - Configures the rx peripheral of channel %s"]
    #[inline(always)]
    pub fn in_peri_sel_ch_iter(&self) -> impl Iterator<Item = &IN_PERI_SEL_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1340)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x540..0x54c - Configures the rx arbiter of channel %s"]
    #[inline(always)]
    pub const fn in_arb_ch(&self, n: usize) -> &IN_ARB_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1344)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x540..0x54c - Configures the rx arbiter of channel %s"]
    #[inline(always)]
    pub fn in_arb_ch_iter(&self) -> impl Iterator<Item = &IN_ARB_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1344)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x544..0x550 - Represents the status of the rx reorder module of channel %s"]
    #[inline(always)]
    pub const fn in_ro_status_ch(&self, n: usize) -> &IN_RO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1348)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x544..0x550 - Represents the status of the rx reorder module of channel %s"]
    #[inline(always)]
    pub fn in_ro_status_ch_iter(&self) -> impl Iterator<Item = &IN_RO_STATUS_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1348)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0x548 - Configures the rx reorder memory of channel 0"]
    #[inline(always)]
    pub const fn in_ro_pd_conf_ch0(&self) -> &IN_RO_PD_CONF_CH0 {
        &self.in_ro_pd_conf_ch0
    }
    #[doc = "0x54c - Configures the Rx color convert of channel 0"]
    #[inline(always)]
    pub const fn in_color_convert_ch0(&self) -> &IN_COLOR_CONVERT_CH0 {
        &self.in_color_convert_ch0
    }
    #[doc = "0x550 - Configures the rx scramble of channel 0"]
    #[inline(always)]
    pub const fn in_scramble_ch0(&self) -> &IN_SCRAMBLE_CH0 {
        &self.in_scramble_ch0
    }
    #[doc = "0x554 - Configures the rx color convert parameter of channel 0"]
    #[inline(always)]
    pub const fn in_color_param0_ch0(&self) -> &IN_COLOR_PARAM0_CH0 {
        &self.in_color_param0_ch0
    }
    #[doc = "0x558 - Configures the rx color convert parameter of channel 0"]
    #[inline(always)]
    pub const fn in_color_param1_ch0(&self) -> &IN_COLOR_PARAM1_CH0 {
        &self.in_color_param1_ch0
    }
    #[doc = "0x55c - Configures the rx color convert parameter of channel 0"]
    #[inline(always)]
    pub const fn in_color_param2_ch0(&self) -> &IN_COLOR_PARAM2_CH0 {
        &self.in_color_param2_ch0
    }
    #[doc = "0x560 - Configures the rx color convert parameter of channel 0"]
    #[inline(always)]
    pub const fn in_color_param3_ch0(&self) -> &IN_COLOR_PARAM3_CH0 {
        &self.in_color_param3_ch0
    }
    #[doc = "0x564 - Configures the rx color convert parameter of channel 0"]
    #[inline(always)]
    pub const fn in_color_param4_ch0(&self) -> &IN_COLOR_PARAM4_CH0 {
        &self.in_color_param4_ch0
    }
    #[doc = "0x568 - Configures the rx color convert parameter of channel 0"]
    #[inline(always)]
    pub const fn in_color_param5_ch0(&self) -> &IN_COLOR_PARAM5_CH0 {
        &self.in_color_param5_ch0
    }
    #[doc = "0x56c..0x578 - Configures the rx etm of channel %s"]
    #[inline(always)]
    pub const fn in_etm_conf_ch(&self, n: usize) -> &IN_ETM_CONF_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1388)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x56c..0x578 - Configures the rx etm of channel %s"]
    #[inline(always)]
    pub fn in_etm_conf_ch_iter(&self) -> impl Iterator<Item = &IN_ETM_CONF_CH> {
        (0..3).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1388)
                .add(256 * n)
                .cast()
        })
    }
    #[doc = "0xa00 - Represents the status of th axi bus"]
    #[inline(always)]
    pub const fn axi_err(&self) -> &AXI_ERR {
        &self.axi_err
    }
    #[doc = "0xa04 - Configures the reset of axi"]
    #[inline(always)]
    pub const fn rst_conf(&self) -> &RST_CONF {
        &self.rst_conf
    }
    #[doc = "0xa08 - The start address of accessible address space."]
    #[inline(always)]
    pub const fn intr_mem_start_addr(&self) -> &INTR_MEM_START_ADDR {
        &self.intr_mem_start_addr
    }
    #[doc = "0xa0c - The end address of accessible address space."]
    #[inline(always)]
    pub const fn intr_mem_end_addr(&self) -> &INTR_MEM_END_ADDR {
        &self.intr_mem_end_addr
    }
    #[doc = "0xa10 - The start address of accessible address space."]
    #[inline(always)]
    pub const fn extr_mem_start_addr(&self) -> &EXTR_MEM_START_ADDR {
        &self.extr_mem_start_addr
    }
    #[doc = "0xa14 - The end address of accessible address space."]
    #[inline(always)]
    pub const fn extr_mem_end_addr(&self) -> &EXTR_MEM_END_ADDR {
        &self.extr_mem_end_addr
    }
    #[doc = "0xa18 - Configures the tx arbiter"]
    #[inline(always)]
    pub const fn out_arb_config(&self) -> &OUT_ARB_CONFIG {
        &self.out_arb_config
    }
    #[doc = "0xa1c - Configures the rx arbiter"]
    #[inline(always)]
    pub const fn in_arb_config(&self) -> &IN_ARB_CONFIG {
        &self.in_arb_config
    }
    #[doc = "0xa20 - reserved"]
    #[inline(always)]
    pub const fn rdn_result(&self) -> &RDN_RESULT {
        &self.rdn_result
    }
    #[doc = "0xa24 - reserved"]
    #[inline(always)]
    pub const fn rdn_eco_high(&self) -> &RDN_ECO_HIGH {
        &self.rdn_eco_high
    }
    #[doc = "0xa28 - reserved"]
    #[inline(always)]
    pub const fn rdn_eco_low(&self) -> &RDN_ECO_LOW {
        &self.rdn_eco_low
    }
    #[doc = "0xa2c - register version."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "OUT_CONF0_CH (rw) register accessor: Configures the tx direction of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf0_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf0_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch`] module"]
pub type OUT_CONF0_CH = crate::Reg<out_conf0_ch::OUT_CONF0_CH_SPEC>;
#[doc = "Configures the tx direction of channel %s"]
pub mod out_conf0_ch;
#[doc = "OUT_INT_RAW_CH (rw) register accessor: Raw interrupt status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_raw_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_raw_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch`] module"]
pub type OUT_INT_RAW_CH = crate::Reg<out_int_raw_ch::OUT_INT_RAW_CH_SPEC>;
#[doc = "Raw interrupt status of TX channel %s"]
pub mod out_int_raw_ch;
#[doc = "OUT_INT_ENA_CH (rw) register accessor: Interrupt enable bits of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_ena_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_ena_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch`] module"]
pub type OUT_INT_ENA_CH = crate::Reg<out_int_ena_ch::OUT_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of TX channel %s"]
pub mod out_int_ena_ch;
#[doc = "OUT_INT_ST_CH (r) register accessor: Masked interrupt status of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_int_st_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch`] module"]
pub type OUT_INT_ST_CH = crate::Reg<out_int_st_ch::OUT_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt status of TX channel %s"]
pub mod out_int_st_ch;
#[doc = "OUT_INT_CLR_CH (w) register accessor: Interrupt clear bits of TX channel %s\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch`] module"]
pub type OUT_INT_CLR_CH = crate::Reg<out_int_clr_ch::OUT_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of TX channel %s"]
pub mod out_int_clr_ch;
#[doc = "OUTFIFO_STATUS_CH (r) register accessor: Represents the status of the tx fifo of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outfifo_status_ch`] module"]
pub type OUTFIFO_STATUS_CH = crate::Reg<outfifo_status_ch::OUTFIFO_STATUS_CH_SPEC>;
#[doc = "Represents the status of the tx fifo of channel %s"]
pub mod outfifo_status_ch;
#[doc = "OUT_PUSH_CH (rw) register accessor: Configures the tx fifo of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_push_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_push_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_push_ch`] module"]
pub type OUT_PUSH_CH = crate::Reg<out_push_ch::OUT_PUSH_CH_SPEC>;
#[doc = "Configures the tx fifo of channel %s"]
pub mod out_push_ch;
#[doc = "OUT_LINK_CONF_CH (rw) register accessor: Configures the tx descriptor operations of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_conf_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_conf_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_conf_ch`] module"]
pub type OUT_LINK_CONF_CH = crate::Reg<out_link_conf_ch::OUT_LINK_CONF_CH_SPEC>;
#[doc = "Configures the tx descriptor operations of channel %s"]
pub mod out_link_conf_ch;
#[doc = "OUT_LINK_ADDR_CH (rw) register accessor: Configures the tx descriptor address of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_link_addr_ch`] module"]
pub type OUT_LINK_ADDR_CH = crate::Reg<out_link_addr_ch::OUT_LINK_ADDR_CH_SPEC>;
#[doc = "Configures the tx descriptor address of channel %s"]
pub mod out_link_addr_ch;
#[doc = "OUT_STATE_CH (r) register accessor: Represents the working status of the tx descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_state_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_state_ch`] module"]
pub type OUT_STATE_CH = crate::Reg<out_state_ch::OUT_STATE_CH_SPEC>;
#[doc = "Represents the working status of the tx descriptor of channel %s"]
pub mod out_state_ch;
#[doc = "OUT_EOF_DES_ADDR_CH (r) register accessor: Represents the address associated with the outlink descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_eof_des_addr_ch`] module"]
pub type OUT_EOF_DES_ADDR_CH = crate::Reg<out_eof_des_addr_ch::OUT_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Represents the address associated with the outlink descriptor of channel %s"]
pub mod out_eof_des_addr_ch;
#[doc = "OUT_DSCR_CH (r) register accessor: Represents the address associated with the outlink descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_ch`] module"]
pub type OUT_DSCR_CH = crate::Reg<out_dscr_ch::OUT_DSCR_CH_SPEC>;
#[doc = "Represents the address associated with the outlink descriptor of channel %s"]
pub mod out_dscr_ch;
#[doc = "OUT_DSCR_BF0_CH (r) register accessor: Represents the address associated with the outlink descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf0_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf0_ch`] module"]
pub type OUT_DSCR_BF0_CH = crate::Reg<out_dscr_bf0_ch::OUT_DSCR_BF0_CH_SPEC>;
#[doc = "Represents the address associated with the outlink descriptor of channel %s"]
pub mod out_dscr_bf0_ch;
#[doc = "OUT_DSCR_BF1_CH (r) register accessor: Represents the address associated with the outlink descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_bf1_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_bf1_ch`] module"]
pub type OUT_DSCR_BF1_CH = crate::Reg<out_dscr_bf1_ch::OUT_DSCR_BF1_CH_SPEC>;
#[doc = "Represents the address associated with the outlink descriptor of channel %s"]
pub mod out_dscr_bf1_ch;
#[doc = "OUT_PERI_SEL_CH (rw) register accessor: Configures the tx peripheral of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_peri_sel_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_peri_sel_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel_ch`] module"]
pub type OUT_PERI_SEL_CH = crate::Reg<out_peri_sel_ch::OUT_PERI_SEL_CH_SPEC>;
#[doc = "Configures the tx peripheral of channel %s"]
pub mod out_peri_sel_ch;
#[doc = "OUT_ARB_CH (rw) register accessor: Configures the tx arbiter of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_arb_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_arb_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_arb_ch`] module"]
pub type OUT_ARB_CH = crate::Reg<out_arb_ch::OUT_ARB_CH_SPEC>;
#[doc = "Configures the tx arbiter of channel %s"]
pub mod out_arb_ch;
#[doc = "OUT_RO_STATUS_CH (r) register accessor: Represents the status of the tx reorder module of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_ro_status_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_ro_status_ch`] module"]
pub type OUT_RO_STATUS_CH = crate::Reg<out_ro_status_ch::OUT_RO_STATUS_CH_SPEC>;
#[doc = "Represents the status of the tx reorder module of channel %s"]
pub mod out_ro_status_ch;
#[doc = "OUT_RO_PD_CONF_CH0 (rw) register accessor: Configures the tx reorder memory of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_ro_pd_conf_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_ro_pd_conf_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_ro_pd_conf_ch0`] module"]
pub type OUT_RO_PD_CONF_CH0 = crate::Reg<out_ro_pd_conf_ch0::OUT_RO_PD_CONF_CH0_SPEC>;
#[doc = "Configures the tx reorder memory of channel 0"]
pub mod out_ro_pd_conf_ch0;
#[doc = "OUT_COLOR_CONVERT_CH (rw) register accessor: Configures the tx color convert of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_color_convert_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_color_convert_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_color_convert_ch`] module"]
pub type OUT_COLOR_CONVERT_CH = crate::Reg<out_color_convert_ch::OUT_COLOR_CONVERT_CH_SPEC>;
#[doc = "Configures the tx color convert of channel %s"]
pub mod out_color_convert_ch;
#[doc = "OUT_SCRAMBLE_CH (rw) register accessor: Configures the tx scramble of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_scramble_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_scramble_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_scramble_ch`] module"]
pub type OUT_SCRAMBLE_CH = crate::Reg<out_scramble_ch::OUT_SCRAMBLE_CH_SPEC>;
#[doc = "Configures the tx scramble of channel %s"]
pub mod out_scramble_ch;
#[doc = "OUT_COLOR_PARAM0_CH (rw) register accessor: Configures the tx color convert parameter of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_color_param0_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_color_param0_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_color_param0_ch`] module"]
pub type OUT_COLOR_PARAM0_CH = crate::Reg<out_color_param0_ch::OUT_COLOR_PARAM0_CH_SPEC>;
#[doc = "Configures the tx color convert parameter of channel %s"]
pub mod out_color_param0_ch;
#[doc = "OUT_COLOR_PARAM1_CH (rw) register accessor: Configures the tx color convert parameter of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_color_param1_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_color_param1_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_color_param1_ch`] module"]
pub type OUT_COLOR_PARAM1_CH = crate::Reg<out_color_param1_ch::OUT_COLOR_PARAM1_CH_SPEC>;
#[doc = "Configures the tx color convert parameter of channel %s"]
pub mod out_color_param1_ch;
#[doc = "OUT_COLOR_PARAM2_CH (rw) register accessor: Configures the tx color convert parameter of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_color_param2_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_color_param2_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_color_param2_ch`] module"]
pub type OUT_COLOR_PARAM2_CH = crate::Reg<out_color_param2_ch::OUT_COLOR_PARAM2_CH_SPEC>;
#[doc = "Configures the tx color convert parameter of channel %s"]
pub mod out_color_param2_ch;
#[doc = "OUT_COLOR_PARAM3_CH (rw) register accessor: Configures the tx color convert parameter of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_color_param3_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_color_param3_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_color_param3_ch`] module"]
pub type OUT_COLOR_PARAM3_CH = crate::Reg<out_color_param3_ch::OUT_COLOR_PARAM3_CH_SPEC>;
#[doc = "Configures the tx color convert parameter of channel %s"]
pub mod out_color_param3_ch;
#[doc = "OUT_COLOR_PARAM4_CH (rw) register accessor: Configures the tx color convert parameter of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_color_param4_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_color_param4_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_color_param4_ch`] module"]
pub type OUT_COLOR_PARAM4_CH = crate::Reg<out_color_param4_ch::OUT_COLOR_PARAM4_CH_SPEC>;
#[doc = "Configures the tx color convert parameter of channel %s"]
pub mod out_color_param4_ch;
#[doc = "OUT_COLOR_PARAM5_CH (rw) register accessor: Configures the tx color convert parameter of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_color_param5_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_color_param5_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_color_param5_ch`] module"]
pub type OUT_COLOR_PARAM5_CH = crate::Reg<out_color_param5_ch::OUT_COLOR_PARAM5_CH_SPEC>;
#[doc = "Configures the tx color convert parameter of channel %s"]
pub mod out_color_param5_ch;
#[doc = "OUT_ETM_CONF_CH (rw) register accessor: Configures the tx etm of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_etm_conf_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_etm_conf_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_etm_conf_ch`] module"]
pub type OUT_ETM_CONF_CH = crate::Reg<out_etm_conf_ch::OUT_ETM_CONF_CH_SPEC>;
#[doc = "Configures the tx etm of channel %s"]
pub mod out_etm_conf_ch;
#[doc = "OUT_DSCR_PORT_BLK_CH (rw) register accessor: Configures the tx block size in dscr port mode\n\nYou can [`read`](crate::Reg::read) this register and get [`out_dscr_port_blk_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_dscr_port_blk_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_dscr_port_blk_ch`] module"]
pub type OUT_DSCR_PORT_BLK_CH = crate::Reg<out_dscr_port_blk_ch::OUT_DSCR_PORT_BLK_CH_SPEC>;
#[doc = "Configures the tx block size in dscr port mode"]
pub mod out_dscr_port_blk_ch;
#[doc = "IN_CONF0_CH (rw) register accessor: Configures the rx direction of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf0_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf0_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch`] module"]
pub type IN_CONF0_CH = crate::Reg<in_conf0_ch::IN_CONF0_CH_SPEC>;
#[doc = "Configures the rx direction of channel %s"]
pub mod in_conf0_ch;
#[doc = "IN_INT_RAW_CH (rw) register accessor: Raw interrupt status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_raw_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_raw_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch`] module"]
pub type IN_INT_RAW_CH = crate::Reg<in_int_raw_ch::IN_INT_RAW_CH_SPEC>;
#[doc = "Raw interrupt status of RX channel %s"]
pub mod in_int_raw_ch;
#[doc = "IN_INT_ENA_CH (rw) register accessor: Interrupt enable bits of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_ena_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_ena_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch`] module"]
pub type IN_INT_ENA_CH = crate::Reg<in_int_ena_ch::IN_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of RX channel %s"]
pub mod in_int_ena_ch;
#[doc = "IN_INT_ST_CH (r) register accessor: Masked interrupt status of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_int_st_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch`] module"]
pub type IN_INT_ST_CH = crate::Reg<in_int_st_ch::IN_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt status of RX channel %s"]
pub mod in_int_st_ch;
#[doc = "IN_INT_CLR_CH (w) register accessor: Interrupt clear bits of RX channel %s\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch`] module"]
pub type IN_INT_CLR_CH = crate::Reg<in_int_clr_ch::IN_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of RX channel %s"]
pub mod in_int_clr_ch;
#[doc = "INFIFO_STATUS_CH (r) register accessor: Represents the status of the rx fifo of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infifo_status_ch`] module"]
pub type INFIFO_STATUS_CH = crate::Reg<infifo_status_ch::INFIFO_STATUS_CH_SPEC>;
#[doc = "Represents the status of the rx fifo of channel %s"]
pub mod infifo_status_ch;
#[doc = "IN_POP_CH (rw) register accessor: Configures the rx fifo of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pop_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pop_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pop_ch`] module"]
pub type IN_POP_CH = crate::Reg<in_pop_ch::IN_POP_CH_SPEC>;
#[doc = "Configures the rx fifo of channel %s"]
pub mod in_pop_ch;
#[doc = "IN_LINK_CONF_CH (rw) register accessor: Configures the rx descriptor operations of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_conf_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_conf_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_conf_ch`] module"]
pub type IN_LINK_CONF_CH = crate::Reg<in_link_conf_ch::IN_LINK_CONF_CH_SPEC>;
#[doc = "Configures the rx descriptor operations of channel %s"]
pub mod in_link_conf_ch;
#[doc = "IN_LINK_ADDR_CH (rw) register accessor: Configures the rx descriptor address of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_addr_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_addr_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_link_addr_ch`] module"]
pub type IN_LINK_ADDR_CH = crate::Reg<in_link_addr_ch::IN_LINK_ADDR_CH_SPEC>;
#[doc = "Configures the rx descriptor address of channel %s"]
pub mod in_link_addr_ch;
#[doc = "IN_STATE_CH (r) register accessor: Represents the working status of the rx descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_state_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_state_ch`] module"]
pub type IN_STATE_CH = crate::Reg<in_state_ch::IN_STATE_CH_SPEC>;
#[doc = "Represents the working status of the rx descriptor of channel %s"]
pub mod in_state_ch;
#[doc = "IN_SUC_EOF_DES_ADDR_CH (r) register accessor: Represents the address associated with the inlink descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_suc_eof_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_suc_eof_des_addr_ch`] module"]
pub type IN_SUC_EOF_DES_ADDR_CH = crate::Reg<in_suc_eof_des_addr_ch::IN_SUC_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Represents the address associated with the inlink descriptor of channel %s"]
pub mod in_suc_eof_des_addr_ch;
#[doc = "IN_ERR_EOF_DES_ADDR_CH (r) register accessor: Represents the address associated with the inlink descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_err_eof_des_addr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_err_eof_des_addr_ch`] module"]
pub type IN_ERR_EOF_DES_ADDR_CH = crate::Reg<in_err_eof_des_addr_ch::IN_ERR_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Represents the address associated with the inlink descriptor of channel %s"]
pub mod in_err_eof_des_addr_ch;
#[doc = "IN_DSCR_CH (r) register accessor: Represents the address associated with the inlink descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_ch`] module"]
pub type IN_DSCR_CH = crate::Reg<in_dscr_ch::IN_DSCR_CH_SPEC>;
#[doc = "Represents the address associated with the inlink descriptor of channel %s"]
pub mod in_dscr_ch;
#[doc = "IN_DSCR_BF0_CH (r) register accessor: Represents the address associated with the inlink descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf0_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf0_ch`] module"]
pub type IN_DSCR_BF0_CH = crate::Reg<in_dscr_bf0_ch::IN_DSCR_BF0_CH_SPEC>;
#[doc = "Represents the address associated with the inlink descriptor of channel %s"]
pub mod in_dscr_bf0_ch;
#[doc = "IN_DSCR_BF1_CH (r) register accessor: Represents the address associated with the inlink descriptor of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_dscr_bf1_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_dscr_bf1_ch`] module"]
pub type IN_DSCR_BF1_CH = crate::Reg<in_dscr_bf1_ch::IN_DSCR_BF1_CH_SPEC>;
#[doc = "Represents the address associated with the inlink descriptor of channel %s"]
pub mod in_dscr_bf1_ch;
#[doc = "IN_PERI_SEL_CH (rw) register accessor: Configures the rx peripheral of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_peri_sel_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_peri_sel_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel_ch`] module"]
pub type IN_PERI_SEL_CH = crate::Reg<in_peri_sel_ch::IN_PERI_SEL_CH_SPEC>;
#[doc = "Configures the rx peripheral of channel %s"]
pub mod in_peri_sel_ch;
#[doc = "IN_ARB_CH (rw) register accessor: Configures the rx arbiter of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_arb_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_arb_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_arb_ch`] module"]
pub type IN_ARB_CH = crate::Reg<in_arb_ch::IN_ARB_CH_SPEC>;
#[doc = "Configures the rx arbiter of channel %s"]
pub mod in_arb_ch;
#[doc = "IN_RO_STATUS_CH (r) register accessor: Represents the status of the rx reorder module of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ro_status_ch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_ro_status_ch`] module"]
pub type IN_RO_STATUS_CH = crate::Reg<in_ro_status_ch::IN_RO_STATUS_CH_SPEC>;
#[doc = "Represents the status of the rx reorder module of channel %s"]
pub mod in_ro_status_ch;
#[doc = "IN_RO_PD_CONF_CH0 (rw) register accessor: Configures the rx reorder memory of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_ro_pd_conf_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_ro_pd_conf_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_ro_pd_conf_ch0`] module"]
pub type IN_RO_PD_CONF_CH0 = crate::Reg<in_ro_pd_conf_ch0::IN_RO_PD_CONF_CH0_SPEC>;
#[doc = "Configures the rx reorder memory of channel 0"]
pub mod in_ro_pd_conf_ch0;
#[doc = "IN_COLOR_CONVERT_CH0 (rw) register accessor: Configures the Rx color convert of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_color_convert_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_color_convert_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_color_convert_ch0`] module"]
pub type IN_COLOR_CONVERT_CH0 = crate::Reg<in_color_convert_ch0::IN_COLOR_CONVERT_CH0_SPEC>;
#[doc = "Configures the Rx color convert of channel 0"]
pub mod in_color_convert_ch0;
#[doc = "IN_SCRAMBLE_CH0 (rw) register accessor: Configures the rx scramble of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_scramble_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_scramble_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_scramble_ch0`] module"]
pub type IN_SCRAMBLE_CH0 = crate::Reg<in_scramble_ch0::IN_SCRAMBLE_CH0_SPEC>;
#[doc = "Configures the rx scramble of channel 0"]
pub mod in_scramble_ch0;
#[doc = "IN_COLOR_PARAM0_CH0 (rw) register accessor: Configures the rx color convert parameter of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_color_param0_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_color_param0_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_color_param0_ch0`] module"]
pub type IN_COLOR_PARAM0_CH0 = crate::Reg<in_color_param0_ch0::IN_COLOR_PARAM0_CH0_SPEC>;
#[doc = "Configures the rx color convert parameter of channel 0"]
pub mod in_color_param0_ch0;
#[doc = "IN_COLOR_PARAM1_CH0 (rw) register accessor: Configures the rx color convert parameter of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_color_param1_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_color_param1_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_color_param1_ch0`] module"]
pub type IN_COLOR_PARAM1_CH0 = crate::Reg<in_color_param1_ch0::IN_COLOR_PARAM1_CH0_SPEC>;
#[doc = "Configures the rx color convert parameter of channel 0"]
pub mod in_color_param1_ch0;
#[doc = "IN_COLOR_PARAM2_CH0 (rw) register accessor: Configures the rx color convert parameter of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_color_param2_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_color_param2_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_color_param2_ch0`] module"]
pub type IN_COLOR_PARAM2_CH0 = crate::Reg<in_color_param2_ch0::IN_COLOR_PARAM2_CH0_SPEC>;
#[doc = "Configures the rx color convert parameter of channel 0"]
pub mod in_color_param2_ch0;
#[doc = "IN_COLOR_PARAM3_CH0 (rw) register accessor: Configures the rx color convert parameter of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_color_param3_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_color_param3_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_color_param3_ch0`] module"]
pub type IN_COLOR_PARAM3_CH0 = crate::Reg<in_color_param3_ch0::IN_COLOR_PARAM3_CH0_SPEC>;
#[doc = "Configures the rx color convert parameter of channel 0"]
pub mod in_color_param3_ch0;
#[doc = "IN_COLOR_PARAM4_CH0 (rw) register accessor: Configures the rx color convert parameter of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_color_param4_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_color_param4_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_color_param4_ch0`] module"]
pub type IN_COLOR_PARAM4_CH0 = crate::Reg<in_color_param4_ch0::IN_COLOR_PARAM4_CH0_SPEC>;
#[doc = "Configures the rx color convert parameter of channel 0"]
pub mod in_color_param4_ch0;
#[doc = "IN_COLOR_PARAM5_CH0 (rw) register accessor: Configures the rx color convert parameter of channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_color_param5_ch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_color_param5_ch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_color_param5_ch0`] module"]
pub type IN_COLOR_PARAM5_CH0 = crate::Reg<in_color_param5_ch0::IN_COLOR_PARAM5_CH0_SPEC>;
#[doc = "Configures the rx color convert parameter of channel 0"]
pub mod in_color_param5_ch0;
#[doc = "IN_ETM_CONF_CH (rw) register accessor: Configures the rx etm of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_etm_conf_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_etm_conf_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_etm_conf_ch`] module"]
pub type IN_ETM_CONF_CH = crate::Reg<in_etm_conf_ch::IN_ETM_CONF_CH_SPEC>;
#[doc = "Configures the rx etm of channel %s"]
pub mod in_etm_conf_ch;
#[doc = "AXI_ERR (r) register accessor: Represents the status of th axi bus\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_err::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_err`] module"]
pub type AXI_ERR = crate::Reg<axi_err::AXI_ERR_SPEC>;
#[doc = "Represents the status of th axi bus"]
pub mod axi_err;
#[doc = "RST_CONF (rw) register accessor: Configures the reset of axi\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_conf`] module"]
pub type RST_CONF = crate::Reg<rst_conf::RST_CONF_SPEC>;
#[doc = "Configures the reset of axi"]
pub mod rst_conf;
#[doc = "INTR_MEM_START_ADDR (rw) register accessor: The start address of accessible address space.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_start_addr`] module"]
pub type INTR_MEM_START_ADDR = crate::Reg<intr_mem_start_addr::INTR_MEM_START_ADDR_SPEC>;
#[doc = "The start address of accessible address space."]
pub mod intr_mem_start_addr;
#[doc = "INTR_MEM_END_ADDR (rw) register accessor: The end address of accessible address space.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mem_end_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mem_end_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mem_end_addr`] module"]
pub type INTR_MEM_END_ADDR = crate::Reg<intr_mem_end_addr::INTR_MEM_END_ADDR_SPEC>;
#[doc = "The end address of accessible address space."]
pub mod intr_mem_end_addr;
#[doc = "EXTR_MEM_START_ADDR (rw) register accessor: The start address of accessible address space.\n\nYou can [`read`](crate::Reg::read) this register and get [`extr_mem_start_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extr_mem_start_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extr_mem_start_addr`] module"]
pub type EXTR_MEM_START_ADDR = crate::Reg<extr_mem_start_addr::EXTR_MEM_START_ADDR_SPEC>;
#[doc = "The start address of accessible address space."]
pub mod extr_mem_start_addr;
#[doc = "EXTR_MEM_END_ADDR (rw) register accessor: The end address of accessible address space.\n\nYou can [`read`](crate::Reg::read) this register and get [`extr_mem_end_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extr_mem_end_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extr_mem_end_addr`] module"]
pub type EXTR_MEM_END_ADDR = crate::Reg<extr_mem_end_addr::EXTR_MEM_END_ADDR_SPEC>;
#[doc = "The end address of accessible address space."]
pub mod extr_mem_end_addr;
#[doc = "OUT_ARB_CONFIG (rw) register accessor: Configures the tx arbiter\n\nYou can [`read`](crate::Reg::read) this register and get [`out_arb_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_arb_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_arb_config`] module"]
pub type OUT_ARB_CONFIG = crate::Reg<out_arb_config::OUT_ARB_CONFIG_SPEC>;
#[doc = "Configures the tx arbiter"]
pub mod out_arb_config;
#[doc = "IN_ARB_CONFIG (rw) register accessor: Configures the rx arbiter\n\nYou can [`read`](crate::Reg::read) this register and get [`in_arb_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_arb_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_arb_config`] module"]
pub type IN_ARB_CONFIG = crate::Reg<in_arb_config::IN_ARB_CONFIG_SPEC>;
#[doc = "Configures the rx arbiter"]
pub mod in_arb_config;
#[doc = "RDN_RESULT (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_result`] module"]
pub type RDN_RESULT = crate::Reg<rdn_result::RDN_RESULT_SPEC>;
#[doc = "reserved"]
pub mod rdn_result;
#[doc = "RDN_ECO_HIGH (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_high`] module"]
pub type RDN_ECO_HIGH = crate::Reg<rdn_eco_high::RDN_ECO_HIGH_SPEC>;
#[doc = "reserved"]
pub mod rdn_eco_high;
#[doc = "RDN_ECO_LOW (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco_low`] module"]
pub type RDN_ECO_LOW = crate::Reg<rdn_eco_low::RDN_ECO_LOW_SPEC>;
#[doc = "reserved"]
pub mod rdn_eco_low;
#[doc = "DATE (rw) register accessor: register version.\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "register version."]
pub mod date;
