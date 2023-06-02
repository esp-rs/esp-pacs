#[doc = "Register `SIGMADELTA7` reader"]
pub struct R(crate::R<SIGMADELTA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA7` writer"]
pub struct W(crate::W<SIGMADELTA7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA7_SPEC>;
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
impl From<crate::W<SIGMADELTA7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD7_IN` reader - "]
pub type SD7_IN_R = crate::FieldReader;
#[doc = "Field `SD7_IN` writer - "]
pub type SD7_IN_W<'a, const O: u8> = crate::FieldWriter<'a, SIGMADELTA7_SPEC, 8, O>;
#[doc = "Field `SD7_PRESCALE` reader - "]
pub type SD7_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD7_PRESCALE` writer - "]
pub type SD7_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, SIGMADELTA7_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd7_in(&self) -> SD7_IN_R {
        SD7_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd7_prescale(&self) -> SD7_PRESCALE_R {
        SD7_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA7")
            .field("sd7_in", &format_args!("{}", self.sd7_in().bits()))
            .field(
                "sd7_prescale",
                &format_args!("{}", self.sd7_prescale().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sd7_in(&mut self) -> SD7_IN_W<0> {
        SD7_IN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sd7_prescale(&mut self) -> SD7_PRESCALE_W<8> {
        SD7_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta7](index.html) module"]
pub struct SIGMADELTA7_SPEC;
impl crate::RegisterSpec for SIGMADELTA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta7::R](R) reader structure"]
impl crate::Readable for SIGMADELTA7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta7::W](W) writer structure"]
impl crate::Writable for SIGMADELTA7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA7 to value 0xff00"]
impl crate::Resettable for SIGMADELTA7_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
