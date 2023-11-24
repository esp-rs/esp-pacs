#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    in_conf0_ch: (),
    _reserved1: [u8; 0x04],
    in_conf1_ch: (),
    _reserved2: [u8; 0x04],
    in_int_raw_ch: (),
    _reserved3: [u8; 0x04],
    in_int_st_ch: (),
    _reserved4: [u8; 0x04],
    in_int_ena_ch: (),
    _reserved5: [u8; 0x04],
    in_int_clr_ch: (),
    _reserved6: [u8; 0x04],
    infifo_status_ch: (),
    _reserved7: [u8; 0x04],
    in_pop_ch: (),
    _reserved8: [u8; 0x04],
    in_link_ch: (),
    _reserved9: [u8; 0x04],
    in_state_ch: (),
    _reserved10: [u8; 0x04],
    in_suc_eof_des_addr_ch: (),
    _reserved11: [u8; 0x04],
    in_err_eof_des_addr_ch: (),
    _reserved12: [u8; 0x04],
    in_dscr_ch: (),
    _reserved13: [u8; 0x04],
    in_dscr_bf0_ch: (),
    _reserved14: [u8; 0x04],
    in_dscr_bf1_ch: (),
    _reserved15: [u8; 0x04],
    in_wight_ch: (),
    _reserved16: [u8; 0x08],
    in_pri_ch: (),
    _reserved17: [u8; 0x04],
    in_peri_sel_ch: (),
    _reserved18: [u8; 0x18],
    out_conf0_ch: (),
    _reserved19: [u8; 0x04],
    out_conf1_ch: (),
    _reserved20: [u8; 0x04],
    out_int_raw_ch: (),
    _reserved21: [u8; 0x04],
    out_int_st_ch: (),
    _reserved22: [u8; 0x04],
    out_int_ena_ch: (),
    _reserved23: [u8; 0x04],
    out_int_clr_ch: (),
    _reserved24: [u8; 0x04],
    outfifo_status_ch: (),
    _reserved25: [u8; 0x04],
    out_push_ch: (),
    _reserved26: [u8; 0x04],
    out_link_ch: (),
    _reserved27: [u8; 0x04],
    out_state_ch: (),
    _reserved28: [u8; 0x04],
    out_eof_des_addr_ch: (),
    _reserved29: [u8; 0x04],
    out_eof_bfr_des_addr_ch: (),
    _reserved30: [u8; 0x04],
    out_dscr_ch: (),
    _reserved31: [u8; 0x04],
    out_dscr_bf0_ch: (),
    _reserved32: [u8; 0x04],
    out_dscr_bf1_ch: (),
    _reserved33: [u8; 0x04],
    out_wight_ch: (),
    _reserved34: [u8; 0x08],
    out_pri_ch: (),
    _reserved35: [u8; 0x04],
    out_peri_sel_ch: (),
    _reserved36: [u8; 0x0318],
    ahb_test: AHB_TEST,
    pd_conf: PD_CONF,
    misc_conf: MISC_CONF,
    in_sram_size_ch: (),
    _reserved40: [u8; 0x04],
    out_sram_size_ch: (),
    _reserved41: [u8; 0x24],
    extmem_reject_addr: EXTMEM_REJECT_ADDR,
    extmem_reject_st: EXTMEM_REJECT_ST,
    extmem_reject_int_raw: EXTMEM_REJECT_INT_RAW,
    extmem_reject_int_st: EXTMEM_REJECT_INT_ST,
    extmem_reject_int_ena: EXTMEM_REJECT_INT_ENA,
    extmem_reject_int_clr: EXTMEM_REJECT_INT_CLR,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x14 - Configure 0 register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_conf0_ch(&self, n: usize) -> &IN_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x04..0x18 - Configure 1 register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_conf1_ch(&self, n: usize) -> &IN_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x08..0x1c - Raw status interrupt of Rx channel 0"]
    #[inline(always)]
    pub const fn in_int_raw_ch(&self, n: usize) -> &IN_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(8)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x0c..0x20 - Masked interrupt of Rx channel 0"]
    #[inline(always)]
    pub const fn in_int_st_ch(&self, n: usize) -> &IN_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(12)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x10..0x24 - Interrupt enable bits of Rx channel 0"]
    #[inline(always)]
    pub const fn in_int_ena_ch(&self, n: usize) -> &IN_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x14..0x28 - Interrupt clear bits of Rx channel 0"]
    #[inline(always)]
    pub const fn in_int_clr_ch(&self, n: usize) -> &IN_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(20)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x18..0x2c - Receive FIFO status of Rx channel 0"]
    #[inline(always)]
    pub const fn infifo_status_ch(&self, n: usize) -> &INFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(24)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x1c..0x30 - Pop control register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_pop_ch(&self, n: usize) -> &IN_POP_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(28)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x20..0x34 - Link descriptor configure and control register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_link_ch(&self, n: usize) -> &IN_LINK_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x24..0x38 - Receive status of Rx channel 0"]
    #[inline(always)]
    pub const fn in_state_ch(&self, n: usize) -> &IN_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(36)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x28..0x3c - Inlink descriptor address when EOF occurs of Rx channel 0"]
    #[inline(always)]
    pub const fn in_suc_eof_des_addr_ch(&self, n: usize) -> &IN_SUC_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(40)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x2c..0x40 - Inlink descriptor address when errors occur of Rx channel 0"]
    #[inline(always)]
    pub const fn in_err_eof_des_addr_ch(&self, n: usize) -> &IN_ERR_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(44)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x30..0x44 - Current inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr_ch(&self, n: usize) -> &IN_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(48)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x34..0x48 - The last inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf0_ch(&self, n: usize) -> &IN_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(52)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x38..0x4c - The second-to-last inlink descriptor address of Rx channel 0"]
    #[inline(always)]
    pub const fn in_dscr_bf1_ch(&self, n: usize) -> &IN_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(56)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x3c..0x50 - Weight register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_wight_ch(&self, n: usize) -> &IN_WIGHT_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(60)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x44..0x58 - Priority register of Rx channel 0"]
    #[inline(always)]
    pub const fn in_pri_ch(&self, n: usize) -> &IN_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(68)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x48..0x5c - Peripheral selection of Rx channel 0"]
    #[inline(always)]
    pub const fn in_peri_sel_ch(&self, n: usize) -> &IN_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(72)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x60..0x74 - Configure 0 register of Tx channel 0"]
    #[inline(always)]
    pub const fn out_conf0_ch(&self, n: usize) -> &OUT_CONF0_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(96)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x64..0x78 - Configure 1 register of Tx channel 0"]
    #[inline(always)]
    pub const fn out_conf1_ch(&self, n: usize) -> &OUT_CONF1_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(100)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x68..0x7c - Raw status interrupt of Tx channel 0"]
    #[inline(always)]
    pub const fn out_int_raw_ch(&self, n: usize) -> &OUT_INT_RAW_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(104)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x6c..0x80 - Masked interrupt of Tx channel 0"]
    #[inline(always)]
    pub const fn out_int_st_ch(&self, n: usize) -> &OUT_INT_ST_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(108)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x70..0x84 - Interrupt enable bits of Tx channel 0"]
    #[inline(always)]
    pub const fn out_int_ena_ch(&self, n: usize) -> &OUT_INT_ENA_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(112)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x74..0x88 - Interrupt clear bits of Tx channel 0"]
    #[inline(always)]
    pub const fn out_int_clr_ch(&self, n: usize) -> &OUT_INT_CLR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(116)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x78..0x8c - Transmit FIFO status of Tx channel 0"]
    #[inline(always)]
    pub const fn outfifo_status_ch(&self, n: usize) -> &OUTFIFO_STATUS_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(120)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x7c..0x90 - Push control register of Rx channel 0"]
    #[inline(always)]
    pub const fn out_push_ch(&self, n: usize) -> &OUT_PUSH_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(124)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x80..0x94 - Link descriptor configure and control register of Tx channel 0"]
    #[inline(always)]
    pub const fn out_link_ch(&self, n: usize) -> &OUT_LINK_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(128)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x84..0x98 - Transmit status of Tx channel 0"]
    #[inline(always)]
    pub const fn out_state_ch(&self, n: usize) -> &OUT_STATE_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(132)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x88..0x9c - Outlink descriptor address when EOF occurs of Tx channel 0"]
    #[inline(always)]
    pub const fn out_eof_des_addr_ch(&self, n: usize) -> &OUT_EOF_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(136)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x8c..0xa0 - The last outlink descriptor address when EOF occurs of Tx channel 0"]
    #[inline(always)]
    pub const fn out_eof_bfr_des_addr_ch(&self, n: usize) -> &OUT_EOF_BFR_DES_ADDR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(140)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x90..0xa4 - Current inlink descriptor address of Tx channel 0"]
    #[inline(always)]
    pub const fn out_dscr_ch(&self, n: usize) -> &OUT_DSCR_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(144)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x94..0xa8 - The last inlink descriptor address of Tx channel 0"]
    #[inline(always)]
    pub const fn out_dscr_bf0_ch(&self, n: usize) -> &OUT_DSCR_BF0_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(148)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x98..0xac - The second-to-last inlink descriptor address of Tx channel 0"]
    #[inline(always)]
    pub const fn out_dscr_bf1_ch(&self, n: usize) -> &OUT_DSCR_BF1_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(152)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x9c..0xb0 - Weight register of Rx channel 0"]
    #[inline(always)]
    pub const fn out_wight_ch(&self, n: usize) -> &OUT_WIGHT_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(156)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0xa4..0xb8 - Priority register of Tx channel 0."]
    #[inline(always)]
    pub const fn out_pri_ch(&self, n: usize) -> &OUT_PRI_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(164)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0xa8..0xbc - Peripheral selection of Tx channel 0"]
    #[inline(always)]
    pub const fn out_peri_sel_ch(&self, n: usize) -> &OUT_PERI_SEL_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(168)
                .add(192 * n)
                .cast()
        }
    }
    #[doc = "0x3c0 - reserved"]
    #[inline(always)]
    pub const fn ahb_test(&self) -> &AHB_TEST {
        &self.ahb_test
    }
    #[doc = "0x3c4 - reserved"]
    #[inline(always)]
    pub const fn pd_conf(&self) -> &PD_CONF {
        &self.pd_conf
    }
    #[doc = "0x3c8 - MISC register"]
    #[inline(always)]
    pub const fn misc_conf(&self) -> &MISC_CONF {
        &self.misc_conf
    }
    #[doc = "0x3cc..0x3e0 - Receive L2 FIFO depth of Rx channel 0"]
    #[inline(always)]
    pub const fn in_sram_size_ch(&self, n: usize) -> &IN_SRAM_SIZE_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(972)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "0x3d0..0x3e4 - Transmit L2 FIFO depth of Tx channel 0"]
    #[inline(always)]
    pub const fn out_sram_size_ch(&self, n: usize) -> &OUT_SRAM_SIZE_CH {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(976)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "0x3f4 - Reject address accessing external RAM"]
    #[inline(always)]
    pub const fn extmem_reject_addr(&self) -> &EXTMEM_REJECT_ADDR {
        &self.extmem_reject_addr
    }
    #[doc = "0x3f8 - Reject status accessing external RAM"]
    #[inline(always)]
    pub const fn extmem_reject_st(&self) -> &EXTMEM_REJECT_ST {
        &self.extmem_reject_st
    }
    #[doc = "0x3fc - Raw interrupt status of external RAM permission"]
    #[inline(always)]
    pub const fn extmem_reject_int_raw(&self) -> &EXTMEM_REJECT_INT_RAW {
        &self.extmem_reject_int_raw
    }
    #[doc = "0x400 - Masked interrupt status of external RAM permission"]
    #[inline(always)]
    pub const fn extmem_reject_int_st(&self) -> &EXTMEM_REJECT_INT_ST {
        &self.extmem_reject_int_st
    }
    #[doc = "0x404 - Interrupt enable bits of external RAM permission"]
    #[inline(always)]
    pub const fn extmem_reject_int_ena(&self) -> &EXTMEM_REJECT_INT_ENA {
        &self.extmem_reject_int_ena
    }
    #[doc = "0x408 - Interrupt clear bits of external RAM permission"]
    #[inline(always)]
    pub const fn extmem_reject_int_clr(&self) -> &EXTMEM_REJECT_INT_CLR {
        &self.extmem_reject_int_clr
    }
    #[doc = "0x40c - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "IN_CONF0_CH (rw) register accessor: Configure 0 register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf0_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf0_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf0_ch`] module"]
pub type IN_CONF0_CH = crate::Reg<in_conf0_ch::IN_CONF0_CH_SPEC>;
#[doc = "Configure 0 register of Rx channel 0"]
pub mod in_conf0_ch;
#[doc = "IN_CONF1_CH (rw) register accessor: Configure 1 register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_conf1_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_conf1_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_conf1_ch`] module"]
pub type IN_CONF1_CH = crate::Reg<in_conf1_ch::IN_CONF1_CH_SPEC>;
#[doc = "Configure 1 register of Rx channel 0"]
pub mod in_conf1_ch;
#[doc = "IN_INT_RAW_CH (rw) register accessor: Raw status interrupt of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_raw_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_raw_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_raw_ch`] module"]
pub type IN_INT_RAW_CH = crate::Reg<in_int_raw_ch::IN_INT_RAW_CH_SPEC>;
#[doc = "Raw status interrupt of Rx channel 0"]
pub mod in_int_raw_ch;
#[doc = "IN_INT_ST_CH (r) register accessor: Masked interrupt of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_st_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_st_ch`] module"]
pub type IN_INT_ST_CH = crate::Reg<in_int_st_ch::IN_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt of Rx channel 0"]
pub mod in_int_st_ch;
#[doc = "IN_INT_ENA_CH (rw) register accessor: Interrupt enable bits of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_ena_ch`] module"]
pub type IN_INT_ENA_CH = crate::Reg<in_int_ena_ch::IN_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of Rx channel 0"]
pub mod in_int_ena_ch;
#[doc = "IN_INT_CLR_CH (w) register accessor: Interrupt clear bits of Rx channel 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_int_clr_ch`] module"]
pub type IN_INT_CLR_CH = crate::Reg<in_int_clr_ch::IN_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of Rx channel 0"]
pub mod in_int_clr_ch;
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
#[doc = "IN_WIGHT_CH (rw) register accessor: Weight register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_wight_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_wight_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_wight_ch`] module"]
pub type IN_WIGHT_CH = crate::Reg<in_wight_ch::IN_WIGHT_CH_SPEC>;
#[doc = "Weight register of Rx channel 0"]
pub mod in_wight_ch;
#[doc = "IN_PRI_CH (rw) register accessor: Priority register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_pri_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_pri_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_pri_ch`] module"]
pub type IN_PRI_CH = crate::Reg<in_pri_ch::IN_PRI_CH_SPEC>;
#[doc = "Priority register of Rx channel 0"]
pub mod in_pri_ch;
#[doc = "IN_PERI_SEL_CH (rw) register accessor: Peripheral selection of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_peri_sel_ch`] module"]
pub type IN_PERI_SEL_CH = crate::Reg<in_peri_sel_ch::IN_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection of Rx channel 0"]
pub mod in_peri_sel_ch;
#[doc = "OUT_CONF0_CH (rw) register accessor: Configure 0 register of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf0_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf0_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf0_ch`] module"]
pub type OUT_CONF0_CH = crate::Reg<out_conf0_ch::OUT_CONF0_CH_SPEC>;
#[doc = "Configure 0 register of Tx channel 0"]
pub mod out_conf0_ch;
#[doc = "OUT_CONF1_CH (rw) register accessor: Configure 1 register of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_conf1_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_conf1_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_conf1_ch`] module"]
pub type OUT_CONF1_CH = crate::Reg<out_conf1_ch::OUT_CONF1_CH_SPEC>;
#[doc = "Configure 1 register of Tx channel 0"]
pub mod out_conf1_ch;
#[doc = "OUT_INT_RAW_CH (rw) register accessor: Raw status interrupt of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_raw_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_raw_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_raw_ch`] module"]
pub type OUT_INT_RAW_CH = crate::Reg<out_int_raw_ch::OUT_INT_RAW_CH_SPEC>;
#[doc = "Raw status interrupt of Tx channel 0"]
pub mod out_int_raw_ch;
#[doc = "OUT_INT_ST_CH (r) register accessor: Masked interrupt of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_st_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_st_ch`] module"]
pub type OUT_INT_ST_CH = crate::Reg<out_int_st_ch::OUT_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt of Tx channel 0"]
pub mod out_int_st_ch;
#[doc = "OUT_INT_ENA_CH (rw) register accessor: Interrupt enable bits of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_ena_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_ena_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_ena_ch`] module"]
pub type OUT_INT_ENA_CH = crate::Reg<out_int_ena_ch::OUT_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of Tx channel 0"]
pub mod out_int_ena_ch;
#[doc = "OUT_INT_CLR_CH (w) register accessor: Interrupt clear bits of Tx channel 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_clr_ch::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_int_clr_ch`] module"]
pub type OUT_INT_CLR_CH = crate::Reg<out_int_clr_ch::OUT_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of Tx channel 0"]
pub mod out_int_clr_ch;
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
#[doc = "OUT_WIGHT_CH (rw) register accessor: Weight register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_wight_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_wight_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_wight_ch`] module"]
pub type OUT_WIGHT_CH = crate::Reg<out_wight_ch::OUT_WIGHT_CH_SPEC>;
#[doc = "Weight register of Rx channel 0"]
pub mod out_wight_ch;
#[doc = "OUT_PRI_CH (rw) register accessor: Priority register of Tx channel 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_pri_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_pri_ch`] module"]
pub type OUT_PRI_CH = crate::Reg<out_pri_ch::OUT_PRI_CH_SPEC>;
#[doc = "Priority register of Tx channel 0."]
pub mod out_pri_ch;
#[doc = "OUT_PERI_SEL_CH (rw) register accessor: Peripheral selection of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_peri_sel_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_peri_sel_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_peri_sel_ch`] module"]
pub type OUT_PERI_SEL_CH = crate::Reg<out_peri_sel_ch::OUT_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection of Tx channel 0"]
pub mod out_peri_sel_ch;
#[doc = "AHB_TEST (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_test`] module"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "reserved"]
pub mod ahb_test;
#[doc = "PD_CONF (rw) register accessor: reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd_conf`] module"]
pub type PD_CONF = crate::Reg<pd_conf::PD_CONF_SPEC>;
#[doc = "reserved"]
pub mod pd_conf;
#[doc = "MISC_CONF (rw) register accessor: MISC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_conf`] module"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "MISC register"]
pub mod misc_conf;
#[doc = "IN_SRAM_SIZE_CH (rw) register accessor: Receive L2 FIFO depth of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_sram_size_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_sram_size_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_sram_size_ch`] module"]
pub type IN_SRAM_SIZE_CH = crate::Reg<in_sram_size_ch::IN_SRAM_SIZE_CH_SPEC>;
#[doc = "Receive L2 FIFO depth of Rx channel 0"]
pub mod in_sram_size_ch;
#[doc = "OUT_SRAM_SIZE_CH (rw) register accessor: Transmit L2 FIFO depth of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_sram_size_ch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_sram_size_ch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_sram_size_ch`] module"]
pub type OUT_SRAM_SIZE_CH = crate::Reg<out_sram_size_ch::OUT_SRAM_SIZE_CH_SPEC>;
#[doc = "Transmit L2 FIFO depth of Tx channel 0"]
pub mod out_sram_size_ch;
#[doc = "EXTMEM_REJECT_ADDR (r) register accessor: Reject address accessing external RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extmem_reject_addr`] module"]
pub type EXTMEM_REJECT_ADDR = crate::Reg<extmem_reject_addr::EXTMEM_REJECT_ADDR_SPEC>;
#[doc = "Reject address accessing external RAM"]
pub mod extmem_reject_addr;
#[doc = "EXTMEM_REJECT_ST (r) register accessor: Reject status accessing external RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extmem_reject_st`] module"]
pub type EXTMEM_REJECT_ST = crate::Reg<extmem_reject_st::EXTMEM_REJECT_ST_SPEC>;
#[doc = "Reject status accessing external RAM"]
pub mod extmem_reject_st;
#[doc = "EXTMEM_REJECT_INT_RAW (rw) register accessor: Raw interrupt status of external RAM permission\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extmem_reject_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extmem_reject_int_raw`] module"]
pub type EXTMEM_REJECT_INT_RAW = crate::Reg<extmem_reject_int_raw::EXTMEM_REJECT_INT_RAW_SPEC>;
#[doc = "Raw interrupt status of external RAM permission"]
pub mod extmem_reject_int_raw;
#[doc = "EXTMEM_REJECT_INT_ST (r) register accessor: Masked interrupt status of external RAM permission\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extmem_reject_int_st`] module"]
pub type EXTMEM_REJECT_INT_ST = crate::Reg<extmem_reject_int_st::EXTMEM_REJECT_INT_ST_SPEC>;
#[doc = "Masked interrupt status of external RAM permission"]
pub mod extmem_reject_int_st;
#[doc = "EXTMEM_REJECT_INT_ENA (rw) register accessor: Interrupt enable bits of external RAM permission\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extmem_reject_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extmem_reject_int_ena`] module"]
pub type EXTMEM_REJECT_INT_ENA = crate::Reg<extmem_reject_int_ena::EXTMEM_REJECT_INT_ENA_SPEC>;
#[doc = "Interrupt enable bits of external RAM permission"]
pub mod extmem_reject_int_ena;
#[doc = "EXTMEM_REJECT_INT_CLR (w) register accessor: Interrupt clear bits of external RAM permission\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extmem_reject_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extmem_reject_int_clr`] module"]
pub type EXTMEM_REJECT_INT_CLR = crate::Reg<extmem_reject_int_clr::EXTMEM_REJECT_INT_CLR_SPEC>;
#[doc = "Interrupt clear bits of external RAM permission"]
pub mod extmem_reject_int_clr;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
