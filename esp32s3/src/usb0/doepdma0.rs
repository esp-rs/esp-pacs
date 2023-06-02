#[doc = "Register `DOEPDMA0` reader"]
pub struct R(crate::R<DOEPDMA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPDMA0` writer"]
pub struct W(crate::W<DOEPDMA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMA0_SPEC>;
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
impl From<crate::W<DOEPDMA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR0` reader - "]
pub type DMAADDR0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMAADDR0` writer - "]
pub type DMAADDR0_W<'a, const O: u8> = crate::FieldWriter<'a, DOEPDMA0_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmaaddr0(&self) -> DMAADDR0_R {
        DMAADDR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPDMA0")
            .field("dmaaddr0", &format_args!("{}", self.dmaaddr0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPDMA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr0(&mut self) -> DMAADDR0_W<0> {
        DMAADDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepdma0](index.html) module"]
pub struct DOEPDMA0_SPEC;
impl crate::RegisterSpec for DOEPDMA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepdma0::R](R) reader structure"]
impl crate::Readable for DOEPDMA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepdma0::W](W) writer structure"]
impl crate::Writable for DOEPDMA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPDMA0 to value 0"]
impl crate::Resettable for DOEPDMA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
