#[doc = "Register `SCL_LOW_PERIOD` reader"]
pub struct R(crate::R<SCL_LOW_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_LOW_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_LOW_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_LOW_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_LOW_PERIOD` writer"]
pub struct W(crate::W<SCL_LOW_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_LOW_PERIOD_SPEC>;
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
impl From<crate::W<SCL_LOW_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_LOW_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCL_LOW_PERIOD` reader - number of cycles that scl == 0"]
pub type SCL_LOW_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `SCL_LOW_PERIOD` writer - number of cycles that scl == 0"]
pub type SCL_LOW_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, SCL_LOW_PERIOD_SPEC, 25, O, u32>;
impl R {
    #[doc = "Bits 0:24 - number of cycles that scl == 0"]
    #[inline(always)]
    pub fn scl_low_period(&self) -> SCL_LOW_PERIOD_R {
        SCL_LOW_PERIOD_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_LOW_PERIOD")
            .field(
                "scl_low_period",
                &format_args!("{}", self.scl_low_period().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_LOW_PERIOD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:24 - number of cycles that scl == 0"]
    #[inline(always)]
    #[must_use]
    pub fn scl_low_period(&mut self) -> SCL_LOW_PERIOD_W<0> {
        SCL_LOW_PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_low_period](index.html) module"]
pub struct SCL_LOW_PERIOD_SPEC;
impl crate::RegisterSpec for SCL_LOW_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_low_period::R](R) reader structure"]
impl crate::Readable for SCL_LOW_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_low_period::W](W) writer structure"]
impl crate::Writable for SCL_LOW_PERIOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL_LOW_PERIOD to value 0"]
impl crate::Resettable for SCL_LOW_PERIOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
