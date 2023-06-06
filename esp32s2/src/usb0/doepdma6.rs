#[doc = "Register `DOEPDMA6` reader"]
pub struct R(crate::R<DOEPDMA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPDMA6` writer"]
pub struct W(crate::W<DOEPDMA6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMA6_SPEC>;
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
impl From<crate::W<DOEPDMA6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMA6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR6` reader - "]
pub type DMAADDR6_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR6` writer - "]
pub type DMAADDR6_W<'a, const O: u8> = crate::FieldWriter<'a, DOEPDMA6_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr6(&self) -> DMAADDR6_R {
        DMAADDR6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA6")
            .field("dmaaddr6", &format_args!("{}", self.dmaaddr6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr6(&mut self) -> DMAADDR6_W<0> {
        DMAADDR6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepdma6](index.html) module"]
pub struct DOEPDMA6_SPEC;
impl crate::RegisterSpec for DOEPDMA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepdma6::R](R) reader structure"]
impl crate::Readable for DOEPDMA6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepdma6::W](W) writer structure"]
impl crate::Writable for DOEPDMA6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMA6 to value 0"]
impl crate::Resettable for DOEPDMA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
