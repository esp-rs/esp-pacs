#[doc = "Register `SIGMADELTA6` reader"]
pub struct R(crate::R<SIGMADELTA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA6` writer"]
pub struct W(crate::W<SIGMADELTA6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA6_SPEC>;
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
impl From<crate::W<SIGMADELTA6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD6_IN` reader - "]
pub type SD6_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SD6_IN` writer - "]
pub type SD6_IN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIGMADELTA6_SPEC, u8, u8, 8, O>;
#[doc = "Field `SD6_PRESCALE` reader - "]
pub type SD6_PRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SD6_PRESCALE` writer - "]
pub type SD6_PRESCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGMADELTA6_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd6_in(&self) -> SD6_IN_R {
        SD6_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd6_prescale(&self) -> SD6_PRESCALE_R {
        SD6_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sd6_in(&mut self) -> SD6_IN_W<0> {
        SD6_IN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sd6_prescale(&mut self) -> SD6_PRESCALE_W<8> {
        SD6_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta6](index.html) module"]
pub struct SIGMADELTA6_SPEC;
impl crate::RegisterSpec for SIGMADELTA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta6::R](R) reader structure"]
impl crate::Readable for SIGMADELTA6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta6::W](W) writer structure"]
impl crate::Writable for SIGMADELTA6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA6 to value 0xff00"]
impl crate::Resettable for SIGMADELTA6_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
