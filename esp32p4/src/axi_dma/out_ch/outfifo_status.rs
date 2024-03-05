#[doc = "Register `OUTFIFO_STATUS` reader"]
pub type R = crate::R<OUTFIFO_STATUS_SPEC>;
#[doc = "Field `OUTFIFO_L3_FULL_CH` reader - L3 Tx FIFO full signal for Tx channel0."]
pub type OUTFIFO_L3_FULL_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_EMPTY_CH` reader - L3 Tx FIFO empty signal for Tx channel0."]
pub type OUTFIFO_L3_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_CNT_CH` reader - The register stores the byte number of the data in L3 Tx FIFO for Tx channel0."]
pub type OUTFIFO_L3_CNT_CH_R = crate::FieldReader;
#[doc = "Field `OUTFIFO_L3_UDF_CH` reader - L3 Tx FIFO under flow signal for Tx channel0."]
pub type OUTFIFO_L3_UDF_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_OVF_CH` reader - L3 Tx FIFO over flow signal for Tx channel0."]
pub type OUTFIFO_L3_OVF_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_FULL_CH` reader - L1 Tx FIFO full signal for Tx channel0."]
pub type OUTFIFO_L1_FULL_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_EMPTY_CH` reader - L1 Tx FIFO empty signal for Tx channel0."]
pub type OUTFIFO_L1_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_UDF_CH` reader - L1 Tx FIFO under flow signal for Tx channel0."]
pub type OUTFIFO_L1_UDF_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_OVF_CH` reader - L1 Tx FIFO over flow signal for Tx channel0."]
pub type OUTFIFO_L1_OVF_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_FULL_CH` reader - L2 Tx RAM full signal for Tx channel0."]
pub type OUTFIFO_L2_FULL_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_EMPTY_CH` reader - L2 Tx RAM empty signal for Tx channel0."]
pub type OUTFIFO_L2_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_UDF_CH` reader - L2 Tx FIFO under flow signal for Tx channel0."]
pub type OUTFIFO_L2_UDF_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_OVF_CH` reader - L2 Tx FIFO over flow signal for Tx channel0."]
pub type OUTFIFO_L2_OVF_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_1B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_1B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_2B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_2B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_3B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_3B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_4B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_4B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_5B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_5B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_6B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_6B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_7B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_7B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_8B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_8B_CH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - L3 Tx FIFO full signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_full_ch(&self) -> OUTFIFO_L3_FULL_CH_R {
        OUTFIFO_L3_FULL_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L3 Tx FIFO empty signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_empty_ch(&self) -> OUTFIFO_L3_EMPTY_CH_R {
        OUTFIFO_L3_EMPTY_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L3 Tx FIFO for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_cnt_ch(&self) -> OUTFIFO_L3_CNT_CH_R {
        OUTFIFO_L3_CNT_CH_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - L3 Tx FIFO under flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_udf_ch(&self) -> OUTFIFO_L3_UDF_CH_R {
        OUTFIFO_L3_UDF_CH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - L3 Tx FIFO over flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_ovf_ch(&self) -> OUTFIFO_L3_OVF_CH_R {
        OUTFIFO_L3_OVF_CH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - L1 Tx FIFO full signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_full_ch(&self) -> OUTFIFO_L1_FULL_CH_R {
        OUTFIFO_L1_FULL_CH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - L1 Tx FIFO empty signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_empty_ch(&self) -> OUTFIFO_L1_EMPTY_CH_R {
        OUTFIFO_L1_EMPTY_CH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - L1 Tx FIFO under flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_udf_ch(&self) -> OUTFIFO_L1_UDF_CH_R {
        OUTFIFO_L1_UDF_CH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - L1 Tx FIFO over flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_ovf_ch(&self) -> OUTFIFO_L1_OVF_CH_R {
        OUTFIFO_L1_OVF_CH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - L2 Tx RAM full signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_full_ch(&self) -> OUTFIFO_L2_FULL_CH_R {
        OUTFIFO_L2_FULL_CH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - L2 Tx RAM empty signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_empty_ch(&self) -> OUTFIFO_L2_EMPTY_CH_R {
        OUTFIFO_L2_EMPTY_CH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L2 Tx FIFO under flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_udf_ch(&self) -> OUTFIFO_L2_UDF_CH_R {
        OUTFIFO_L2_UDF_CH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - L2 Tx FIFO over flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_ovf_ch(&self) -> OUTFIFO_L2_OVF_CH_R {
        OUTFIFO_L2_OVF_CH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_1b_ch(&self) -> OUT_REMAIN_UNDER_1B_CH_R {
        OUT_REMAIN_UNDER_1B_CH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_2b_ch(&self) -> OUT_REMAIN_UNDER_2B_CH_R {
        OUT_REMAIN_UNDER_2B_CH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_3b_ch(&self) -> OUT_REMAIN_UNDER_3B_CH_R {
        OUT_REMAIN_UNDER_3B_CH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_4b_ch(&self) -> OUT_REMAIN_UNDER_4B_CH_R {
        OUT_REMAIN_UNDER_4B_CH_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_5b_ch(&self) -> OUT_REMAIN_UNDER_5B_CH_R {
        OUT_REMAIN_UNDER_5B_CH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_6b_ch(&self) -> OUT_REMAIN_UNDER_6B_CH_R {
        OUT_REMAIN_UNDER_6B_CH_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_7b_ch(&self) -> OUT_REMAIN_UNDER_7B_CH_R {
        OUT_REMAIN_UNDER_7B_CH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_8b_ch(&self) -> OUT_REMAIN_UNDER_8B_CH_R {
        OUT_REMAIN_UNDER_8B_CH_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS")
            .field(
                "outfifo_l3_full_ch",
                &format_args!("{}", self.outfifo_l3_full_ch().bit()),
            )
            .field(
                "outfifo_l3_empty_ch",
                &format_args!("{}", self.outfifo_l3_empty_ch().bit()),
            )
            .field(
                "outfifo_l3_cnt_ch",
                &format_args!("{}", self.outfifo_l3_cnt_ch().bits()),
            )
            .field(
                "outfifo_l3_udf_ch",
                &format_args!("{}", self.outfifo_l3_udf_ch().bit()),
            )
            .field(
                "outfifo_l3_ovf_ch",
                &format_args!("{}", self.outfifo_l3_ovf_ch().bit()),
            )
            .field(
                "outfifo_l1_full_ch",
                &format_args!("{}", self.outfifo_l1_full_ch().bit()),
            )
            .field(
                "outfifo_l1_empty_ch",
                &format_args!("{}", self.outfifo_l1_empty_ch().bit()),
            )
            .field(
                "outfifo_l1_udf_ch",
                &format_args!("{}", self.outfifo_l1_udf_ch().bit()),
            )
            .field(
                "outfifo_l1_ovf_ch",
                &format_args!("{}", self.outfifo_l1_ovf_ch().bit()),
            )
            .field(
                "outfifo_l2_full_ch",
                &format_args!("{}", self.outfifo_l2_full_ch().bit()),
            )
            .field(
                "outfifo_l2_empty_ch",
                &format_args!("{}", self.outfifo_l2_empty_ch().bit()),
            )
            .field(
                "outfifo_l2_udf_ch",
                &format_args!("{}", self.outfifo_l2_udf_ch().bit()),
            )
            .field(
                "outfifo_l2_ovf_ch",
                &format_args!("{}", self.outfifo_l2_ovf_ch().bit()),
            )
            .field(
                "out_remain_under_1b_ch",
                &format_args!("{}", self.out_remain_under_1b_ch().bit()),
            )
            .field(
                "out_remain_under_2b_ch",
                &format_args!("{}", self.out_remain_under_2b_ch().bit()),
            )
            .field(
                "out_remain_under_3b_ch",
                &format_args!("{}", self.out_remain_under_3b_ch().bit()),
            )
            .field(
                "out_remain_under_4b_ch",
                &format_args!("{}", self.out_remain_under_4b_ch().bit()),
            )
            .field(
                "out_remain_under_5b_ch",
                &format_args!("{}", self.out_remain_under_5b_ch().bit()),
            )
            .field(
                "out_remain_under_6b_ch",
                &format_args!("{}", self.out_remain_under_6b_ch().bit()),
            )
            .field(
                "out_remain_under_7b_ch",
                &format_args!("{}", self.out_remain_under_7b_ch().bit()),
            )
            .field(
                "out_remain_under_8b_ch",
                &format_args!("{}", self.out_remain_under_8b_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUTFIFO_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Transmit FIFO status of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTFIFO_STATUS_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_status::R`](R) reader structure"]
impl crate::Readable for OUTFIFO_STATUS_SPEC {}
#[doc = "`reset()` method sets OUTFIFO_STATUS to value 0x7f80_8802"]
impl crate::Resettable for OUTFIFO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x7f80_8802;
}
