#[doc = "Register `LC_STATE0` reader"]
pub struct R(crate::R<LC_STATE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LC_STATE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LC_STATE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LC_STATE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR_ADDR` reader - I2S DMA out descriptor address."]
pub type OUTLINK_DSCR_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OUT_DSCR_STATE` reader - I2S DMA out descriptor state."]
pub type OUT_DSCR_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_STATE` reader - I2S DMA out data state."]
pub type OUT_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTFIFO_CNT` reader - The remains of I2S DMA outfifo data."]
pub type OUTFIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_FULL` reader - I2S DMA outfifo is full."]
pub type OUT_FULL_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EMPTY` reader - I2S DMA outfifo is empty."]
pub type OUT_EMPTY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:17 - I2S DMA out descriptor address."]
    #[inline(always)]
    pub fn outlink_dscr_addr(&self) -> OUTLINK_DSCR_ADDR_R {
        OUTLINK_DSCR_ADDR_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - I2S DMA out descriptor state."]
    #[inline(always)]
    pub fn out_dscr_state(&self) -> OUT_DSCR_STATE_R {
        OUT_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - I2S DMA out data state."]
    #[inline(always)]
    pub fn out_state(&self) -> OUT_STATE_R {
        OUT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:29 - The remains of I2S DMA outfifo data."]
    #[inline(always)]
    pub fn outfifo_cnt(&self) -> OUTFIFO_CNT_R {
        OUTFIFO_CNT_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - I2S DMA outfifo is full."]
    #[inline(always)]
    pub fn out_full(&self) -> OUT_FULL_R {
        OUT_FULL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I2S DMA outfifo is empty."]
    #[inline(always)]
    pub fn out_empty(&self) -> OUT_EMPTY_R {
        OUT_EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "I2S DMA TX status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lc_state0](index.html) module"]
pub struct LC_STATE0_SPEC;
impl crate::RegisterSpec for LC_STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lc_state0::R](R) reader structure"]
impl crate::Readable for LC_STATE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LC_STATE0 to value 0"]
impl crate::Resettable for LC_STATE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
