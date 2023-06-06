#[doc = "Register `DIEPDMA2` reader"]
pub struct R(crate::R<DIEPDMA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPDMA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPDMA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPDMA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPDMA2` writer"]
pub struct W(crate::W<DIEPDMA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPDMA2_SPEC>;
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
impl From<crate::W<DIEPDMA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPDMA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_DMAADDR2` reader - "]
pub type D_DMAADDR2_R = crate::FieldReader<u32>;
#[doc = "Field `D_DMAADDR2` writer - "]
pub type D_DMAADDR2_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPDMA2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmaaddr2(&self) -> D_DMAADDR2_R {
        D_DMAADDR2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMA2")
            .field("d_dmaaddr2", &format_args!("{}", self.d_dmaaddr2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn d_dmaaddr2(&mut self) -> D_DMAADDR2_W<0> {
        D_DMAADDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdma2](index.html) module"]
pub struct DIEPDMA2_SPEC;
impl crate::RegisterSpec for DIEPDMA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepdma2::R](R) reader structure"]
impl crate::Readable for DIEPDMA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepdma2::W](W) writer structure"]
impl crate::Writable for DIEPDMA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPDMA2 to value 0"]
impl crate::Resettable for DIEPDMA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
