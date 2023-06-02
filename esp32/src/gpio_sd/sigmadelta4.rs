#[doc = "Register `SIGMADELTA4` reader"]
pub struct R(crate::R<SIGMADELTA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA4` writer"]
pub struct W(crate::W<SIGMADELTA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA4_SPEC>;
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
impl From<crate::W<SIGMADELTA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD4_IN` reader - "]
pub type SD4_IN_R = crate::FieldReader;
#[doc = "Field `SD4_IN` writer - "]
pub type SD4_IN_W<'a, const O: u8> = crate::FieldWriter<'a, SIGMADELTA4_SPEC, 8, O>;
#[doc = "Field `SD4_PRESCALE` reader - "]
pub type SD4_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD4_PRESCALE` writer - "]
pub type SD4_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, SIGMADELTA4_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd4_in(&self) -> SD4_IN_R {
        SD4_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd4_prescale(&self) -> SD4_PRESCALE_R {
        SD4_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA4")
            .field("sd4_in", &format_args!("{}", self.sd4_in().bits()))
            .field(
                "sd4_prescale",
                &format_args!("{}", self.sd4_prescale().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sd4_in(&mut self) -> SD4_IN_W<0> {
        SD4_IN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sd4_prescale(&mut self) -> SD4_PRESCALE_W<8> {
        SD4_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta4](index.html) module"]
pub struct SIGMADELTA4_SPEC;
impl crate::RegisterSpec for SIGMADELTA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta4::R](R) reader structure"]
impl crate::Readable for SIGMADELTA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta4::W](W) writer structure"]
impl crate::Writable for SIGMADELTA4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA4 to value 0xff00"]
impl crate::Resettable for SIGMADELTA4_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
