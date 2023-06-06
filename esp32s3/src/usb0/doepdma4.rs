#[doc = "Register `DOEPDMA4` reader"]
pub struct R(crate::R<DOEPDMA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPDMA4` writer"]
pub struct W(crate::W<DOEPDMA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMA4_SPEC>;
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
impl From<crate::W<DOEPDMA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR4` reader - "]
pub type DMAADDR4_R = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR4` writer - "]
pub type DMAADDR4_W<'a, const O: u8> = crate::FieldWriter<'a, DOEPDMA4_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr4(&self) -> DMAADDR4_R {
        DMAADDR4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA4")
            .field("dmaaddr4", &format_args!("{}", self.dmaaddr4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr4(&mut self) -> DMAADDR4_W<0> {
        DMAADDR4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepdma4](index.html) module"]
pub struct DOEPDMA4_SPEC;
impl crate::RegisterSpec for DOEPDMA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepdma4::R](R) reader structure"]
impl crate::Readable for DOEPDMA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepdma4::W](W) writer structure"]
impl crate::Writable for DOEPDMA4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMA4 to value 0"]
impl crate::Resettable for DOEPDMA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
