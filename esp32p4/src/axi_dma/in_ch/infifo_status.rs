#[doc = "Register `INFIFO_STATUS` reader"]
pub type R = crate::R<INFIFO_STATUS_SPEC>;
#[doc = "Field `INFIFO_L3_FULL` reader - L3 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_L3_FULL_R = crate::BitReader;
#[doc = "Field `INFIFO_L3_EMPTY` reader - L3 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_L3_EMPTY_R = crate::BitReader;
#[doc = "Field `INFIFO_L3_CNT` reader - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0."]
pub type INFIFO_L3_CNT_R = crate::FieldReader;
#[doc = "Field `INFIFO_L3_UDF` reader - L3 Rx FIFO under flow signal for Rx channel 0."]
pub type INFIFO_L3_UDF_R = crate::BitReader;
#[doc = "Field `INFIFO_L3_OVF` reader - L3 Rx FIFO over flow signal for Rx channel 0."]
pub type INFIFO_L3_OVF_R = crate::BitReader;
#[doc = "Field `INFIFO_L1_FULL` reader - L1 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_L1_FULL_R = crate::BitReader;
#[doc = "Field `INFIFO_L1_EMPTY` reader - L1 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_L1_EMPTY_R = crate::BitReader;
#[doc = "Field `INFIFO_L1_UDF` reader - L1 Rx FIFO under flow signal for Rx channel 0."]
pub type INFIFO_L1_UDF_R = crate::BitReader;
#[doc = "Field `INFIFO_L1_OVF` reader - L1 Rx FIFO over flow signal for Rx channel 0."]
pub type INFIFO_L1_OVF_R = crate::BitReader;
#[doc = "Field `INFIFO_L2_FULL` reader - L2 Rx RAM full signal for Rx channel 0."]
pub type INFIFO_L2_FULL_R = crate::BitReader;
#[doc = "Field `INFIFO_L2_EMPTY` reader - L2 Rx RAM empty signal for Rx channel 0."]
pub type INFIFO_L2_EMPTY_R = crate::BitReader;
#[doc = "Field `INFIFO_L2_UDF` reader - L2 Rx FIFO under flow signal for Rx channel 0."]
pub type INFIFO_L2_UDF_R = crate::BitReader;
#[doc = "Field `INFIFO_L2_OVF` reader - L2 Rx FIFO over flow signal for Rx channel 0."]
pub type INFIFO_L2_OVF_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_1B` reader - reserved"]
pub type IN_REMAIN_UNDER_1B_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_2B` reader - reserved"]
pub type IN_REMAIN_UNDER_2B_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_3B` reader - reserved"]
pub type IN_REMAIN_UNDER_3B_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_4B` reader - reserved"]
pub type IN_REMAIN_UNDER_4B_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_5B` reader - reserved"]
pub type IN_REMAIN_UNDER_5B_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_6B` reader - reserved"]
pub type IN_REMAIN_UNDER_6B_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_7B` reader - reserved"]
pub type IN_REMAIN_UNDER_7B_R = crate::BitReader;
#[doc = "Field `IN_REMAIN_UNDER_8B` reader - reserved"]
pub type IN_REMAIN_UNDER_8B_R = crate::BitReader;
#[doc = "Field `IN_BUF_HUNGRY` reader - reserved"]
pub type IN_BUF_HUNGRY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - L3 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_full(&self) -> INFIFO_L3_FULL_R {
        INFIFO_L3_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L3 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_empty(&self) -> INFIFO_L3_EMPTY_R {
        INFIFO_L3_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_cnt(&self) -> INFIFO_L3_CNT_R {
        INFIFO_L3_CNT_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - L3 Rx FIFO under flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_udf(&self) -> INFIFO_L3_UDF_R {
        INFIFO_L3_UDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - L3 Rx FIFO over flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l3_ovf(&self) -> INFIFO_L3_OVF_R {
        INFIFO_L3_OVF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - L1 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_full(&self) -> INFIFO_L1_FULL_R {
        INFIFO_L1_FULL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - L1 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_empty(&self) -> INFIFO_L1_EMPTY_R {
        INFIFO_L1_EMPTY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - L1 Rx FIFO under flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_udf(&self) -> INFIFO_L1_UDF_R {
        INFIFO_L1_UDF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - L1 Rx FIFO over flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l1_ovf(&self) -> INFIFO_L1_OVF_R {
        INFIFO_L1_OVF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - L2 Rx RAM full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_full(&self) -> INFIFO_L2_FULL_R {
        INFIFO_L2_FULL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - L2 Rx RAM empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_empty(&self) -> INFIFO_L2_EMPTY_R {
        INFIFO_L2_EMPTY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L2 Rx FIFO under flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_udf(&self) -> INFIFO_L2_UDF_R {
        INFIFO_L2_UDF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - L2 Rx FIFO over flow signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_l2_ovf(&self) -> INFIFO_L2_OVF_R {
        INFIFO_L2_OVF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_1b(&self) -> IN_REMAIN_UNDER_1B_R {
        IN_REMAIN_UNDER_1B_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_2b(&self) -> IN_REMAIN_UNDER_2B_R {
        IN_REMAIN_UNDER_2B_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_3b(&self) -> IN_REMAIN_UNDER_3B_R {
        IN_REMAIN_UNDER_3B_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_4b(&self) -> IN_REMAIN_UNDER_4B_R {
        IN_REMAIN_UNDER_4B_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_5b(&self) -> IN_REMAIN_UNDER_5B_R {
        IN_REMAIN_UNDER_5B_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_6b(&self) -> IN_REMAIN_UNDER_6B_R {
        IN_REMAIN_UNDER_6B_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_7b(&self) -> IN_REMAIN_UNDER_7B_R {
        IN_REMAIN_UNDER_7B_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_8b(&self) -> IN_REMAIN_UNDER_8B_R {
        IN_REMAIN_UNDER_8B_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn in_buf_hungry(&self) -> IN_BUF_HUNGRY_R {
        IN_BUF_HUNGRY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_STATUS")
            .field("infifo_l3_full", &self.infifo_l3_full().bit())
            .field("infifo_l3_empty", &self.infifo_l3_empty().bit())
            .field("infifo_l3_cnt", &self.infifo_l3_cnt().bits())
            .field("infifo_l3_udf", &self.infifo_l3_udf().bit())
            .field("infifo_l3_ovf", &self.infifo_l3_ovf().bit())
            .field("infifo_l1_full", &self.infifo_l1_full().bit())
            .field("infifo_l1_empty", &self.infifo_l1_empty().bit())
            .field("infifo_l1_udf", &self.infifo_l1_udf().bit())
            .field("infifo_l1_ovf", &self.infifo_l1_ovf().bit())
            .field("infifo_l2_full", &self.infifo_l2_full().bit())
            .field("infifo_l2_empty", &self.infifo_l2_empty().bit())
            .field("infifo_l2_udf", &self.infifo_l2_udf().bit())
            .field("infifo_l2_ovf", &self.infifo_l2_ovf().bit())
            .field("in_remain_under_1b", &self.in_remain_under_1b().bit())
            .field("in_remain_under_2b", &self.in_remain_under_2b().bit())
            .field("in_remain_under_3b", &self.in_remain_under_3b().bit())
            .field("in_remain_under_4b", &self.in_remain_under_4b().bit())
            .field("in_remain_under_5b", &self.in_remain_under_5b().bit())
            .field("in_remain_under_6b", &self.in_remain_under_6b().bit())
            .field("in_remain_under_7b", &self.in_remain_under_7b().bit())
            .field("in_remain_under_8b", &self.in_remain_under_8b().bit())
            .field("in_buf_hungry", &self.in_buf_hungry().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INFIFO_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Receive FIFO status of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFIFO_STATUS_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_status::R`](R) reader structure"]
impl crate::Readable for INFIFO_STATUS_SPEC {}
#[doc = "`reset()` method sets INFIFO_STATUS to value 0x8803"]
impl crate::Resettable for INFIFO_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x8803;
}
