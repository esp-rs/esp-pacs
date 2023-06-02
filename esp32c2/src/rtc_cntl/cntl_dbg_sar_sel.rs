#[doc = "Register `CNTL_DBG_SAR_SEL` reader"]
pub struct R(crate::R<CNTL_DBG_SAR_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTL_DBG_SAR_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTL_DBG_SAR_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTL_DBG_SAR_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTL_DBG_SAR_SEL` writer"]
pub struct W(crate::W<CNTL_DBG_SAR_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTL_DBG_SAR_SEL_SPEC>;
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
impl From<crate::W<CNTL_DBG_SAR_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTL_DBG_SAR_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_DEBUG_SEL` reader - Need add desc"]
pub type SAR_DEBUG_SEL_R = crate::FieldReader;
#[doc = "Field `SAR_DEBUG_SEL` writer - Need add desc"]
pub type SAR_DEBUG_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, CNTL_DBG_SAR_SEL_SPEC, 5, O>;
impl R {
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    pub fn sar_debug_sel(&self) -> SAR_DEBUG_SEL_R {
        SAR_DEBUG_SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL_DBG_SAR_SEL")
            .field(
                "sar_debug_sel",
                &format_args!("{}", self.sar_debug_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CNTL_DBG_SAR_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn sar_debug_sel(&mut self) -> SAR_DEBUG_SEL_W<27> {
        SAR_DEBUG_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl_dbg_sar_sel](index.html) module"]
pub struct CNTL_DBG_SAR_SEL_SPEC;
impl crate::RegisterSpec for CNTL_DBG_SAR_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntl_dbg_sar_sel::R](R) reader structure"]
impl crate::Readable for CNTL_DBG_SAR_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntl_dbg_sar_sel::W](W) writer structure"]
impl crate::Writable for CNTL_DBG_SAR_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTL_DBG_SAR_SEL to value 0"]
impl crate::Resettable for CNTL_DBG_SAR_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
