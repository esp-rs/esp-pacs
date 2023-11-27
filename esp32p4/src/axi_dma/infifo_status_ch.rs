#[doc = "Register `INFIFO_STATUS_CH%s` reader"]
pub type R = crate::R<INFIFO_STATUS_CH_SPEC>;
#[doc = "Field `INFIFO_L3_FULL_CH` reader - L3 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_L3_FULL_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_L3_EMPTY_CH` reader - L3 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_L3_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_L3_CNT_CH` reader - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0."]
pub type INFIFO_L3_CNT_CH_R = crate::FieldReader;
#[doc = "Field `INFIFO_L3_UDF_CH` reader - L3 Rx FIFO under flow signal for Rx channel 0."]
pub type INFIFO_L3_UDF_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_L3_OVF_CH` reader - L3 Rx FIFO over flow signal for Rx channel 0."]
pub type INFIFO_L3_OVF_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_L1_FULL_CH` reader - L1 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_L1_FULL_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_L1_EMPTY_CH` reader - L1 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_L1_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_L1_UDF_CH` reader - L1 Rx FIFO under flow signal for Rx channel 0."]
pub type INFIFO_L1_UDF_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_L1_OVF_CH` reader - L1 Rx FIFO over flow signal for Rx channel 0."]
pub type INFIFO_L1_OVF_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_L2_FULL_CH` reader - L2 Rx RAM full signal for Rx channel 0."]
pub type INFIFO_L2_FULL_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_L2_EMPTY_CH` reader - L2 Rx RAM empty signal for Rx channel 0."]
pub type INFIFO_L2_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_L2_UDF_CH` reader - L2 Rx FIFO under flow signal for Rx channel 0."]
pub type INFIFO_L2_UDF_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_L2_OVF_CH` reader - L2 Rx FIFO over flow signal for Rx channel 0."]
pub type INFIFO_L2_OVF_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_1B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_1B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_2B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_2B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_3B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_3B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_4B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_4B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_5B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_5B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_6B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_6B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_7B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_7B_CH_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_8B_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_8B_CH_R = crate::BitReader;
#[doc = "Field `IN_BUF_HUNGRY_CH` reader - reserved"]
pub type IN_BUF_HUNGRY_CH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - L3 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_full_ch(&self) -> INFIFO_L3_FULL_CH_R {
        INFIFO_L3_FULL_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L3 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_empty_ch(&self) -> INFIFO_L3_EMPTY_CH_R {
        INFIFO_L3_EMPTY_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_cnt_ch(&self) -> INFIFO_L3_CNT_CH_R {
        INFIFO_L3_CNT_CH_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - L3 Rx FIFO under flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_udf_ch(&self) -> INFIFO_L3_UDF_CH_R {
        INFIFO_L3_UDF_CH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - L3 Rx FIFO over flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_ovf_ch(&self) -> INFIFO_L3_OVF_CH_R {
        INFIFO_L3_OVF_CH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - L1 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_full_ch(&self) -> INFIFO_L1_FULL_CH_R {
        INFIFO_L1_FULL_CH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - L1 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_empty_ch(&self) -> INFIFO_L1_EMPTY_CH_R {
        INFIFO_L1_EMPTY_CH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - L1 Rx FIFO under flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_udf_ch(&self) -> INFIFO_L1_UDF_CH_R {
        INFIFO_L1_UDF_CH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - L1 Rx FIFO over flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_ovf_ch(&self) -> INFIFO_L1_OVF_CH_R {
        INFIFO_L1_OVF_CH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - L2 Rx RAM full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_full_ch(&self) -> INFIFO_L2_FULL_CH_R {
        INFIFO_L2_FULL_CH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - L2 Rx RAM empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_empty_ch(&self) -> INFIFO_L2_EMPTY_CH_R {
        INFIFO_L2_EMPTY_CH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L2 Rx FIFO under flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_udf_ch(&self) -> INFIFO_L2_UDF_CH_R {
        INFIFO_L2_UDF_CH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - L2 Rx FIFO over flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_ovf_ch(&self) -> INFIFO_L2_OVF_CH_R {
        INFIFO_L2_OVF_CH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_1b_ch(&self) -> IN_REMAIN_UNDER_1B_CH_R {
        IN_REMAIN_UNDER_1B_CH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_2b_ch(&self) -> IN_REMAIN_UNDER_2B_CH_R {
        IN_REMAIN_UNDER_2B_CH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_3b_ch(&self) -> IN_REMAIN_UNDER_3B_CH_R {
        IN_REMAIN_UNDER_3B_CH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_4b_ch(&self) -> IN_REMAIN_UNDER_4B_CH_R {
        IN_REMAIN_UNDER_4B_CH_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_5b_ch(&self) -> IN_REMAIN_UNDER_5B_CH_R {
        IN_REMAIN_UNDER_5B_CH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_6b_ch(&self) -> IN_REMAIN_UNDER_6B_CH_R {
        IN_REMAIN_UNDER_6B_CH_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_7b_ch(&self) -> IN_REMAIN_UNDER_7B_CH_R {
        IN_REMAIN_UNDER_7B_CH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_8b_ch(&self) -> IN_REMAIN_UNDER_8B_CH_R {
        IN_REMAIN_UNDER_8B_CH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn in_buf_hungry_ch(&self) -> IN_BUF_HUNGRY_CH_R {
        IN_BUF_HUNGRY_CH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_STATUS_CH")
            .field(
                "infifo_l3_full_ch",
                &format_args!("{}", self.infifo_l3_full_ch().bit()),
            )
            .field(
                "infifo_l3_empty_ch",
                &format_args!("{}", self.infifo_l3_empty_ch().bit()),
            )
            .field(
                "infifo_l3_cnt_ch",
                &format_args!("{}", self.infifo_l3_cnt_ch().bits()),
            )
            .field(
                "infifo_l3_udf_ch",
                &format_args!("{}", self.infifo_l3_udf_ch().bit()),
            )
            .field(
                "infifo_l3_ovf_ch",
                &format_args!("{}", self.infifo_l3_ovf_ch().bit()),
            )
            .field(
                "infifo_l1_full_ch",
                &format_args!("{}", self.infifo_l1_full_ch().bit()),
            )
            .field(
                "infifo_l1_empty_ch",
                &format_args!("{}", self.infifo_l1_empty_ch().bit()),
            )
            .field(
                "infifo_l1_udf_ch",
                &format_args!("{}", self.infifo_l1_udf_ch().bit()),
            )
            .field(
                "infifo_l1_ovf_ch",
                &format_args!("{}", self.infifo_l1_ovf_ch().bit()),
            )
            .field(
                "infifo_l2_full_ch",
                &format_args!("{}", self.infifo_l2_full_ch().bit()),
            )
            .field(
                "infifo_l2_empty_ch",
                &format_args!("{}", self.infifo_l2_empty_ch().bit()),
            )
            .field(
                "infifo_l2_udf_ch",
                &format_args!("{}", self.infifo_l2_udf_ch().bit()),
            )
            .field(
                "infifo_l2_ovf_ch",
                &format_args!("{}", self.infifo_l2_ovf_ch().bit()),
            )
            .field(
                "in_remain_under_1b_ch",
                &format_args!("{}", self.in_remain_under_1b_ch().bit()),
            )
            .field(
                "in_remain_under_2b_ch",
                &format_args!("{}", self.in_remain_under_2b_ch().bit()),
            )
            .field(
                "in_remain_under_3b_ch",
                &format_args!("{}", self.in_remain_under_3b_ch().bit()),
            )
            .field(
                "in_remain_under_4b_ch",
                &format_args!("{}", self.in_remain_under_4b_ch().bit()),
            )
            .field(
                "in_remain_under_5b_ch",
                &format_args!("{}", self.in_remain_under_5b_ch().bit()),
            )
            .field(
                "in_remain_under_6b_ch",
                &format_args!("{}", self.in_remain_under_6b_ch().bit()),
            )
            .field(
                "in_remain_under_7b_ch",
                &format_args!("{}", self.in_remain_under_7b_ch().bit()),
            )
            .field(
                "in_remain_under_8b_ch",
                &format_args!("{}", self.in_remain_under_8b_ch().bit()),
            )
            .field(
                "in_buf_hungry_ch",
                &format_args!("{}", self.in_buf_hungry_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INFIFO_STATUS_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Receive FIFO status of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFIFO_STATUS_CH_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_status_ch::R`](R) reader structure"]
impl crate::Readable for INFIFO_STATUS_CH_SPEC {}
#[doc = "`reset()` method sets INFIFO_STATUS_CH%s to value 0x8803"]
impl crate::Resettable for INFIFO_STATUS_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x8803;
}
