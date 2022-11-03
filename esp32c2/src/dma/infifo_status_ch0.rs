#[doc = "Register `INFIFO_STATUS_CH0` reader"]
pub struct R(crate::R<INFIFO_STATUS_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFIFO_STATUS_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFIFO_STATUS_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFIFO_STATUS_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INFIFO_FULL` reader - L1 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_EMPTY` reader - L1 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_CNT` reader - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0."]
pub type INFIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN_REMAIN_UNDER_1B` reader - reserved"]
pub type IN_REMAIN_UNDER_1B_R = crate::BitReader<bool>;
#[doc = "Field `IN_REMAIN_UNDER_2B` reader - reserved"]
pub type IN_REMAIN_UNDER_2B_R = crate::BitReader<bool>;
#[doc = "Field `IN_REMAIN_UNDER_3B` reader - reserved"]
pub type IN_REMAIN_UNDER_3B_R = crate::BitReader<bool>;
#[doc = "Field `IN_REMAIN_UNDER_4B` reader - reserved"]
pub type IN_REMAIN_UNDER_4B_R = crate::BitReader<bool>;
#[doc = "Field `IN_BUF_HUNGRY` reader - reserved"]
pub type IN_BUF_HUNGRY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - L1 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_full(&self) -> INFIFO_FULL_R {
        INFIFO_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L1 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_empty(&self) -> INFIFO_EMPTY_R {
        INFIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt(&self) -> INFIFO_CNT_R {
        INFIFO_CNT_R::new(((self.bits >> 2) & 0x3f) as u8)
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
    pub fn in_buf_hungry(&self) -> IN_BUF_HUNGRY_R {
        IN_BUF_HUNGRY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "DMA_INFIFO_STATUS_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [infifo_status_ch0](index.html) module"]
pub struct INFIFO_STATUS_CH0_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [infifo_status_ch0::R](R) reader structure"]
impl crate::Readable for INFIFO_STATUS_CH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INFIFO_STATUS_CH0 to value 0x0780_0003"]
impl crate::Resettable for INFIFO_STATUS_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0780_0003
    }
}
