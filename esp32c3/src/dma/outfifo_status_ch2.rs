#[doc = "Register `OUTFIFO_STATUS_CH2` reader"]
pub struct R(crate::R<OUTFIFO_STATUS_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTFIFO_STATUS_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTFIFO_STATUS_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTFIFO_STATUS_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTFIFO_FULL_CH2` reader - L1 Tx FIFO full signal for Tx channel 2."]
pub type OUTFIFO_FULL_CH2_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_EMPTY_CH2` reader - L1 Tx FIFO empty signal for Tx channel 2."]
pub type OUTFIFO_EMPTY_CH2_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_CNT_CH2` reader - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 2."]
pub type OUTFIFO_CNT_CH2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_REMAIN_UNDER_1B_CH2` reader - reserved"]
pub type OUT_REMAIN_UNDER_1B_CH2_R = crate::BitReader<bool>;
#[doc = "Field `OUT_REMAIN_UNDER_2B_CH2` reader - reserved"]
pub type OUT_REMAIN_UNDER_2B_CH2_R = crate::BitReader<bool>;
#[doc = "Field `OUT_REMAIN_UNDER_3B_CH2` reader - reserved"]
pub type OUT_REMAIN_UNDER_3B_CH2_R = crate::BitReader<bool>;
#[doc = "Field `OUT_REMAIN_UNDER_4B_CH2` reader - reserved"]
pub type OUT_REMAIN_UNDER_4B_CH2_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - L1 Tx FIFO full signal for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_full_ch2(&self) -> OUTFIFO_FULL_CH2_R {
        OUTFIFO_FULL_CH2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L1 Tx FIFO empty signal for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_empty_ch2(&self) -> OUTFIFO_EMPTY_CH2_R {
        OUTFIFO_EMPTY_CH2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_cnt_ch2(&self) -> OUTFIFO_CNT_CH2_R {
        OUTFIFO_CNT_CH2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_1b_ch2(&self) -> OUT_REMAIN_UNDER_1B_CH2_R {
        OUT_REMAIN_UNDER_1B_CH2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_2b_ch2(&self) -> OUT_REMAIN_UNDER_2B_CH2_R {
        OUT_REMAIN_UNDER_2B_CH2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_3b_ch2(&self) -> OUT_REMAIN_UNDER_3B_CH2_R {
        OUT_REMAIN_UNDER_3B_CH2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_4b_ch2(&self) -> OUT_REMAIN_UNDER_4B_CH2_R {
        OUT_REMAIN_UNDER_4B_CH2_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "DMA_OUTFIFO_STATUS_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outfifo_status_ch2](index.html) module"]
pub struct OUTFIFO_STATUS_CH2_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outfifo_status_ch2::R](R) reader structure"]
impl crate::Readable for OUTFIFO_STATUS_CH2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUTFIFO_STATUS_CH2 to value 0x0780_0002"]
impl crate::Resettable for OUTFIFO_STATUS_CH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0780_0002
    }
}
