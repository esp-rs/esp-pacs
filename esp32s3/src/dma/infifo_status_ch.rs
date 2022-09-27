#[doc = "Register `INFIFO_STATUS_CH%s` reader"]
pub struct R(crate::R<INFIFO_STATUS_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFIFO_STATUS_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFIFO_STATUS_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFIFO_STATUS_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INFIFO_FULL_L1_CH` reader - L1 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_FULL_L1_CH_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_EMPTY_L1_CH` reader - L1 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_EMPTY_L1_CH_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_FULL_L2_CH` reader - L2 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_FULL_L2_CH_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_EMPTY_L2_CH` reader - L2 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_EMPTY_L2_CH_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_FULL_L3_CH` reader - L3 Rx FIFO full signal for Rx channel 0."]
pub type INFIFO_FULL_L3_CH_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_EMPTY_L3_CH` reader - L3 Rx FIFO empty signal for Rx channel 0."]
pub type INFIFO_EMPTY_L3_CH_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_CNT_L1_CH` reader - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0."]
pub type INFIFO_CNT_L1_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INFIFO_CNT_L2_CH` reader - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 0."]
pub type INFIFO_CNT_L2_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INFIFO_CNT_L3_CH` reader - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0."]
pub type INFIFO_CNT_L3_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IN_REMAIN_UNDER_1B_L3_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_1B_L3_CH_R = crate::BitReader<bool>;
#[doc = "Field `IN_REMAIN_UNDER_2B_L3_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_2B_L3_CH_R = crate::BitReader<bool>;
#[doc = "Field `IN_REMAIN_UNDER_3B_L3_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_3B_L3_CH_R = crate::BitReader<bool>;
#[doc = "Field `IN_REMAIN_UNDER_4B_L3_CH` reader - reserved"]
pub type IN_REMAIN_UNDER_4B_L3_CH_R = crate::BitReader<bool>;
#[doc = "Field `IN_BUF_HUNGRY_CH` reader - reserved"]
pub type IN_BUF_HUNGRY_CH_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - L1 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l1_ch(&self) -> INFIFO_FULL_L1_CH_R {
        INFIFO_FULL_L1_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L1 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l1_ch(&self) -> INFIFO_EMPTY_L1_CH_R {
        INFIFO_EMPTY_L1_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - L2 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l2_ch(&self) -> INFIFO_FULL_L2_CH_R {
        INFIFO_FULL_L2_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - L2 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l2_ch(&self) -> INFIFO_EMPTY_L2_CH_R {
        INFIFO_EMPTY_L2_CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L3 Rx FIFO full signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l3_ch(&self) -> INFIFO_FULL_L3_CH_R {
        INFIFO_FULL_L3_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L3 Rx FIFO empty signal for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l3_ch(&self) -> INFIFO_EMPTY_L3_CH_R {
        INFIFO_EMPTY_L3_CH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l1_ch(&self) -> INFIFO_CNT_L1_CH_R {
        INFIFO_CNT_L1_CH_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:18 - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l2_ch(&self) -> INFIFO_CNT_L2_CH_R {
        INFIFO_CNT_L2_CH_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bits 19:23 - The register stores the byte number of the data in L3 Rx FIFO for Rx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l3_ch(&self) -> INFIFO_CNT_L3_CH_R {
        INFIFO_CNT_L3_CH_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_1b_l3_ch(&self) -> IN_REMAIN_UNDER_1B_L3_CH_R {
        IN_REMAIN_UNDER_1B_L3_CH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_2b_l3_ch(&self) -> IN_REMAIN_UNDER_2B_L3_CH_R {
        IN_REMAIN_UNDER_2B_L3_CH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_3b_l3_ch(&self) -> IN_REMAIN_UNDER_3B_L3_CH_R {
        IN_REMAIN_UNDER_3B_L3_CH_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn in_remain_under_4b_l3_ch(&self) -> IN_REMAIN_UNDER_4B_L3_CH_R {
        IN_REMAIN_UNDER_4B_L3_CH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn in_buf_hungry_ch(&self) -> IN_BUF_HUNGRY_CH_R {
        IN_BUF_HUNGRY_CH_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Receive FIFO status of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [infifo_status_ch](index.html) module"]
pub struct INFIFO_STATUS_CH_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [infifo_status_ch::R](R) reader structure"]
impl crate::Readable for INFIFO_STATUS_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INFIFO_STATUS_CH%s to value 0x0f00_003a"]
impl crate::Resettable for INFIFO_STATUS_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f00_003a
    }
}
