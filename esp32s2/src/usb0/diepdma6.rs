#[doc = "Register `DIEPDMA6` reader"]
pub struct R(crate::R<DIEPDMA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPDMA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPDMA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPDMA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPDMA6` writer"]
pub struct W(crate::W<DIEPDMA6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPDMA6_SPEC>;
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
impl From<crate::W<DIEPDMA6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPDMA6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_DMAADDR6` reader - "]
pub type D_DMAADDR6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `D_DMAADDR6` writer - "]
pub type D_DMAADDR6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIEPDMA6_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmaaddr6(&self) -> D_DMAADDR6_R {
        D_DMAADDR6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmaaddr6(&mut self) -> D_DMAADDR6_W<0> {
        D_DMAADDR6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepdma6](index.html) module"]
pub struct DIEPDMA6_SPEC;
impl crate::RegisterSpec for DIEPDMA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepdma6::R](R) reader structure"]
impl crate::Readable for DIEPDMA6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepdma6::W](W) writer structure"]
impl crate::Writable for DIEPDMA6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPDMA6 to value 0"]
impl crate::Resettable for DIEPDMA6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
