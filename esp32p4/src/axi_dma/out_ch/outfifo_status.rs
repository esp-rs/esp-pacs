#[doc = "Register `OUTFIFO_STATUS` reader"]
pub type R = crate::R<OUTFIFO_STATUS_SPEC>;
#[doc = "Field `OUTFIFO_L3_FULL` reader - L3 Tx FIFO full signal for Tx channel0."]
pub type OUTFIFO_L3_FULL_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_EMPTY` reader - L3 Tx FIFO empty signal for Tx channel0."]
pub type OUTFIFO_L3_EMPTY_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_CNT` reader - The register stores the byte number of the data in L3 Tx FIFO for Tx channel0."]
pub type OUTFIFO_L3_CNT_R = crate::FieldReader;
#[doc = "Field `OUTFIFO_L3_UDF` reader - L3 Tx FIFO under flow signal for Tx channel0."]
pub type OUTFIFO_L3_UDF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_OVF` reader - L3 Tx FIFO over flow signal for Tx channel0."]
pub type OUTFIFO_L3_OVF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_FULL` reader - L1 Tx FIFO full signal for Tx channel0."]
pub type OUTFIFO_L1_FULL_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_EMPTY` reader - L1 Tx FIFO empty signal for Tx channel0."]
pub type OUTFIFO_L1_EMPTY_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_UDF` reader - L1 Tx FIFO under flow signal for Tx channel0."]
pub type OUTFIFO_L1_UDF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_OVF` reader - L1 Tx FIFO over flow signal for Tx channel0."]
pub type OUTFIFO_L1_OVF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_FULL` reader - L2 Tx RAM full signal for Tx channel0."]
pub type OUTFIFO_L2_FULL_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_EMPTY` reader - L2 Tx RAM empty signal for Tx channel0."]
pub type OUTFIFO_L2_EMPTY_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_UDF` reader - L2 Tx FIFO under flow signal for Tx channel0."]
pub type OUTFIFO_L2_UDF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_OVF` reader - L2 Tx FIFO over flow signal for Tx channel0."]
pub type OUTFIFO_L2_OVF_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_1B` reader - reserved"]
pub type OUT_REMAIN_UNDER_1B_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_2B` reader - reserved"]
pub type OUT_REMAIN_UNDER_2B_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_3B` reader - reserved"]
pub type OUT_REMAIN_UNDER_3B_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_4B` reader - reserved"]
pub type OUT_REMAIN_UNDER_4B_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_5B` reader - reserved"]
pub type OUT_REMAIN_UNDER_5B_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_6B` reader - reserved"]
pub type OUT_REMAIN_UNDER_6B_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_7B` reader - reserved"]
pub type OUT_REMAIN_UNDER_7B_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_8B` reader - reserved"]
pub type OUT_REMAIN_UNDER_8B_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - L3 Tx FIFO full signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_full(&self) -> OUTFIFO_L3_FULL_R {
        OUTFIFO_L3_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L3 Tx FIFO empty signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_empty(&self) -> OUTFIFO_L3_EMPTY_R {
        OUTFIFO_L3_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L3 Tx FIFO for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_cnt(&self) -> OUTFIFO_L3_CNT_R {
        OUTFIFO_L3_CNT_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - L3 Tx FIFO under flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_udf(&self) -> OUTFIFO_L3_UDF_R {
        OUTFIFO_L3_UDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - L3 Tx FIFO over flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l3_ovf(&self) -> OUTFIFO_L3_OVF_R {
        OUTFIFO_L3_OVF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - L1 Tx FIFO full signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_full(&self) -> OUTFIFO_L1_FULL_R {
        OUTFIFO_L1_FULL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - L1 Tx FIFO empty signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_empty(&self) -> OUTFIFO_L1_EMPTY_R {
        OUTFIFO_L1_EMPTY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - L1 Tx FIFO under flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_udf(&self) -> OUTFIFO_L1_UDF_R {
        OUTFIFO_L1_UDF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - L1 Tx FIFO over flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l1_ovf(&self) -> OUTFIFO_L1_OVF_R {
        OUTFIFO_L1_OVF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - L2 Tx RAM full signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_full(&self) -> OUTFIFO_L2_FULL_R {
        OUTFIFO_L2_FULL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - L2 Tx RAM empty signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_empty(&self) -> OUTFIFO_L2_EMPTY_R {
        OUTFIFO_L2_EMPTY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L2 Tx FIFO under flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_udf(&self) -> OUTFIFO_L2_UDF_R {
        OUTFIFO_L2_UDF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - L2 Tx FIFO over flow signal for Tx channel0."]
    #[inline(always)]
    pub fn outfifo_l2_ovf(&self) -> OUTFIFO_L2_OVF_R {
        OUTFIFO_L2_OVF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_1b(&self) -> OUT_REMAIN_UNDER_1B_R {
        OUT_REMAIN_UNDER_1B_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_2b(&self) -> OUT_REMAIN_UNDER_2B_R {
        OUT_REMAIN_UNDER_2B_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_3b(&self) -> OUT_REMAIN_UNDER_3B_R {
        OUT_REMAIN_UNDER_3B_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_4b(&self) -> OUT_REMAIN_UNDER_4B_R {
        OUT_REMAIN_UNDER_4B_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_5b(&self) -> OUT_REMAIN_UNDER_5B_R {
        OUT_REMAIN_UNDER_5B_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_6b(&self) -> OUT_REMAIN_UNDER_6B_R {
        OUT_REMAIN_UNDER_6B_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_7b(&self) -> OUT_REMAIN_UNDER_7B_R {
        OUT_REMAIN_UNDER_7B_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_8b(&self) -> OUT_REMAIN_UNDER_8B_R {
        OUT_REMAIN_UNDER_8B_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS")
            .field("outfifo_l3_full", &self.outfifo_l3_full().bit())
            .field("outfifo_l3_empty", &self.outfifo_l3_empty().bit())
            .field("outfifo_l3_cnt", &self.outfifo_l3_cnt().bits())
            .field("outfifo_l3_udf", &self.outfifo_l3_udf().bit())
            .field("outfifo_l3_ovf", &self.outfifo_l3_ovf().bit())
            .field("outfifo_l1_full", &self.outfifo_l1_full().bit())
            .field("outfifo_l1_empty", &self.outfifo_l1_empty().bit())
            .field("outfifo_l1_udf", &self.outfifo_l1_udf().bit())
            .field("outfifo_l1_ovf", &self.outfifo_l1_ovf().bit())
            .field("outfifo_l2_full", &self.outfifo_l2_full().bit())
            .field("outfifo_l2_empty", &self.outfifo_l2_empty().bit())
            .field("outfifo_l2_udf", &self.outfifo_l2_udf().bit())
            .field("outfifo_l2_ovf", &self.outfifo_l2_ovf().bit())
            .field("out_remain_under_1b", &self.out_remain_under_1b().bit())
            .field("out_remain_under_2b", &self.out_remain_under_2b().bit())
            .field("out_remain_under_3b", &self.out_remain_under_3b().bit())
            .field("out_remain_under_4b", &self.out_remain_under_4b().bit())
            .field("out_remain_under_5b", &self.out_remain_under_5b().bit())
            .field("out_remain_under_6b", &self.out_remain_under_6b().bit())
            .field("out_remain_under_7b", &self.out_remain_under_7b().bit())
            .field("out_remain_under_8b", &self.out_remain_under_8b().bit())
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
