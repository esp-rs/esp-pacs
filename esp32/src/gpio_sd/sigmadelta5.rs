#[doc = "Register `SIGMADELTA5` reader"]
pub struct R(crate::R<SIGMADELTA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA5` writer"]
pub struct W(crate::W<SIGMADELTA5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA5_SPEC>;
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
impl From<crate::W<SIGMADELTA5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD5_IN` reader - "]
pub type SD5_IN_R = crate::FieldReader;
#[doc = "Field `SD5_IN` writer - "]
pub type SD5_IN_W<'a, const O: u8> = crate::FieldWriter<'a, SIGMADELTA5_SPEC, 8, O>;
#[doc = "Field `SD5_PRESCALE` reader - "]
pub type SD5_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD5_PRESCALE` writer - "]
pub type SD5_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, SIGMADELTA5_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd5_in(&self) -> SD5_IN_R {
        SD5_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd5_prescale(&self) -> SD5_PRESCALE_R {
        SD5_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA5")
            .field("sd5_in", &format_args!("{}", self.sd5_in().bits()))
            .field(
                "sd5_prescale",
                &format_args!("{}", self.sd5_prescale().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sd5_in(&mut self) -> SD5_IN_W<0> {
        SD5_IN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sd5_prescale(&mut self) -> SD5_PRESCALE_W<8> {
        SD5_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta5](index.html) module"]
pub struct SIGMADELTA5_SPEC;
impl crate::RegisterSpec for SIGMADELTA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta5::R](R) reader structure"]
impl crate::Readable for SIGMADELTA5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta5::W](W) writer structure"]
impl crate::Writable for SIGMADELTA5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA5 to value 0xff00"]
impl crate::Resettable for SIGMADELTA5_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
