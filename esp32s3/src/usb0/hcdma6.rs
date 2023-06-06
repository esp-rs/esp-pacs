#[doc = "Register `HCDMA6` reader"]
pub struct R(crate::R<HCDMA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDMA6` writer"]
pub struct W(crate::W<HCDMA6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDMA6_SPEC>;
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
impl From<crate::W<HCDMA6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDMA6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_DMAADDR6` reader - "]
pub type H_DMAADDR6_R = crate::FieldReader<u32>;
#[doc = "Field `H_DMAADDR6` writer - "]
pub type H_DMAADDR6_W<'a, const O: u8> = crate::FieldWriter<'a, HCDMA6_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn h_dmaaddr6(&self) -> H_DMAADDR6_R {
        H_DMAADDR6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMA6")
            .field("h_dmaaddr6", &format_args!("{}", self.h_dmaaddr6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCDMA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dmaaddr6(&mut self) -> H_DMAADDR6_W<0> {
        H_DMAADDR6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma6](index.html) module"]
pub struct HCDMA6_SPEC;
impl crate::RegisterSpec for HCDMA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdma6::R](R) reader structure"]
impl crate::Readable for HCDMA6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdma6::W](W) writer structure"]
impl crate::Writable for HCDMA6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCDMA6 to value 0"]
impl crate::Resettable for HCDMA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
