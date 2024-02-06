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
    _reserved24: [u8; 0x34],
    out_conf1_ch: (),
    _reserved25: [u8; 0x04],
    outfifo_status_ch: (),
    _reserved26: [u8; 0x04],
    out_push_ch: (),
    _reserved27: [u8; 0x04],
    out_link_ch: (),
    _reserved28: [u8; 0x04],
    out_state_ch: (),
    _reserved29: [u8; 0x04],
    out_eof_des_addr_ch: (),
    _reserved30: [u8; 0x04],
    out_eof_bfr_des_addr_ch: (),
    _reserved31: [u8; 0x04],
    out_dscr_ch: (),
    _reserved32: [u8; 0x04],
    out_dscr_bf0_ch: (),
    _reserved33: [u8; 0x04],
    out_dscr_bf1_ch: (),
    _reserved34: [u8; 0x04],
    out_pri_ch: (),
    _reserved35: [u8; 0x04],
    out_peri_sel_ch: (),
    _reserved36: [u8; 0x90],
    out_conf0_ch: (),
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
        (0..3)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(16 * n).cast() })
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
        (0..3)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(4).add(16 * n).cast() })
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
        (0..3)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(8).add(16 * n).cast() })
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(160)
                .add(192 * n)
                .cast()
        })
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
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
        (0..3).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(192 * n)
                .cast()
        })
    }
    #[doc = "0x190..0x19c - Configure 0 register of Tx channel 1"]
    #[inline(always)]
    pub const fn out_conf0_ch(&self, n: usize) -> &OUT_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(400)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x190..0x19c - Configure 0 register of Tx channel 1"]
    #[inline(always)]
    pub fn out_conf0_ch_iter(&self) -> impl Iterator<Item = &OUT_CONF0_CH> {
        (0..3).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(400)
                .add(192 * n)
                .cast()
        })
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
