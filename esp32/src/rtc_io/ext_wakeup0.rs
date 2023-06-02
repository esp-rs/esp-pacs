#[doc = "Register `EXT_WAKEUP0` reader"]
pub struct R(crate::R<EXT_WAKEUP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_WAKEUP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_WAKEUP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_WAKEUP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_WAKEUP0` writer"]
pub struct W(crate::W<EXT_WAKEUP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_WAKEUP0_SPEC>;
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
impl From<crate::W<EXT_WAKEUP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_WAKEUP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - select the wakeup source Ó0Ó select GPIO0 Ó1Ó select GPIO2 ...Ò17Ó select GPIO17"]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - select the wakeup source Ó0Ó select GPIO0 Ó1Ó select GPIO2 ...Ò17Ó select GPIO17"]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, EXT_WAKEUP0_SPEC, 5, O>;
impl R {
    #[doc = "Bits 27:31 - select the wakeup source Ó0Ó select GPIO0 Ó1Ó select GPIO2 ...Ò17Ó select GPIO17"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP0")
            .field("sel", &format_args!("{}", self.sel().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_WAKEUP0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 27:31 - select the wakeup source Ó0Ó select GPIO0 Ó1Ó select GPIO2 ...Ò17Ó select GPIO17"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<27> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_wakeup0](index.html) module"]
pub struct EXT_WAKEUP0_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_wakeup0::R](R) reader structure"]
impl crate::Readable for EXT_WAKEUP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_wakeup0::W](W) writer structure"]
impl crate::Writable for EXT_WAKEUP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP0 to value 0"]
impl crate::Resettable for EXT_WAKEUP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
