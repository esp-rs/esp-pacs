#[doc = "Register `OUTFIFO_STATUS_CH%s` reader"]
pub struct R(crate::R<OUTFIFO_STATUS_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTFIFO_STATUS_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTFIFO_STATUS_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTFIFO_STATUS_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTFIFO_FULL_CH` reader - L1 Tx FIFO full signal for Tx channel 0."]
pub type OUTFIFO_FULL_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_EMPTY_CH` reader - L1 Tx FIFO empty signal for Tx channel 0."]
pub type OUTFIFO_EMPTY_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_CNT_CH` reader - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0."]
pub type OUTFIFO_CNT_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_REMAIN_UNDER_1B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_1B_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUT_REMAIN_UNDER_2B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_2B_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUT_REMAIN_UNDER_3B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_3B_CH_R = crate::BitReader<bool>;
#[doc = "Field `OUT_REMAIN_UNDER_4B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_4B_CH_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - L1 Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_full_ch(&self) -> OUTFIFO_FULL_CH_R {
        OUTFIFO_FULL_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L1 Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_empty_ch(&self) -> OUTFIFO_EMPTY_CH_R {
        OUTFIFO_EMPTY_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_cnt_ch(&self) -> OUTFIFO_CNT_CH_R {
        OUTFIFO_CNT_CH_R::new(((self.bits >> 2) & 0x3f) as u8)
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
}
#[doc = "Transmit FIFO status of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outfifo_status_ch](index.html) module"]
pub struct OUTFIFO_STATUS_CH_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outfifo_status_ch::R](R) reader structure"]
impl crate::Readable for OUTFIFO_STATUS_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUTFIFO_STATUS_CH%s to value 0x0780_0002"]
impl crate::Resettable for OUTFIFO_STATUS_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0780_0002;
}
