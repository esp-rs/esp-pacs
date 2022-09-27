#[doc = "Register `OUT_STATE_CH%s` reader"]
pub struct R(crate::R<OUT_STATE_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_STATE_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_STATE_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_STATE_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR_ADDR_CH` reader - This register stores the current outlink descriptor's address."]
pub type OUTLINK_DSCR_ADDR_CH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OUT_DSCR_STATE_CH` reader - reserved"]
pub type OUT_DSCR_STATE_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUT_STATE_CH` reader - reserved"]
pub type OUT_STATE_CH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:17 - This register stores the current outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_dscr_addr_ch(&self) -> OUTLINK_DSCR_ADDR_CH_R {
        OUTLINK_DSCR_ADDR_CH_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn out_dscr_state_ch(&self) -> OUT_DSCR_STATE_CH_R {
        OUT_DSCR_STATE_CH_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - reserved"]
    #[inline(always)]
    pub fn out_state_ch(&self) -> OUT_STATE_CH_R {
        OUT_STATE_CH_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[doc = "Transmit status of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_state_ch](index.html) module"]
pub struct OUT_STATE_CH_SPEC;
impl crate::RegisterSpec for OUT_STATE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_state_ch::R](R) reader structure"]
impl crate::Readable for OUT_STATE_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_STATE_CH%s to value 0"]
impl crate::Resettable for OUT_STATE_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
