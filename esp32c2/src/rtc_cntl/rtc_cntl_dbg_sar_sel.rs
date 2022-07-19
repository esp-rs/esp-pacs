#[doc = "Register `RTC_CNTL_DBG_SAR_SEL` reader"]
pub struct R(crate::R<RTC_CNTL_DBG_SAR_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CNTL_DBG_SAR_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CNTL_DBG_SAR_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CNTL_DBG_SAR_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CNTL_DBG_SAR_SEL` writer"]
pub struct W(crate::W<RTC_CNTL_DBG_SAR_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CNTL_DBG_SAR_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RTC_CNTL_DBG_SAR_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CNTL_DBG_SAR_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_DEBUG_SEL` reader - Need add desc"]
pub type SAR_DEBUG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_DEBUG_SEL` writer - Need add desc"]
pub type SAR_DEBUG_SEL_W<'a> =
    crate::FieldWriter<'a, u32, RTC_CNTL_DBG_SAR_SEL_SPEC, u8, u8, 5, 27>;
impl R {
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    pub fn sar_debug_sel(&self) -> SAR_DEBUG_SEL_R {
        SAR_DEBUG_SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    pub fn sar_debug_sel(&mut self) -> SAR_DEBUG_SEL_W {
        SAR_DEBUG_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_dbg_sar_sel](index.html) module"]
pub struct RTC_CNTL_DBG_SAR_SEL_SPEC;
impl crate::RegisterSpec for RTC_CNTL_DBG_SAR_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cntl_dbg_sar_sel::R](R) reader structure"]
impl crate::Readable for RTC_CNTL_DBG_SAR_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dbg_sar_sel::W](W) writer structure"]
impl crate::Writable for RTC_CNTL_DBG_SAR_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CNTL_DBG_SAR_SEL to value 0"]
impl crate::Resettable for RTC_CNTL_DBG_SAR_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
