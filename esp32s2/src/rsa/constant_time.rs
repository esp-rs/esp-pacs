#[doc = "Register `CONSTANT_TIME` reader"]
pub struct R(crate::R<CONSTANT_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONSTANT_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONSTANT_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONSTANT_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONSTANT_TIME` writer"]
pub struct W(crate::W<CONSTANT_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONSTANT_TIME_SPEC>;
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
impl From<crate::W<CONSTANT_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONSTANT_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTANT_TIME` reader - Set this bit to 0 to enable the acceleration option of constant_time for modular exponentiation. Set to 1 to disable the acceleration (by default)."]
pub type CONSTANT_TIME_R = crate::BitReader;
#[doc = "Field `CONSTANT_TIME` writer - Set this bit to 0 to enable the acceleration option of constant_time for modular exponentiation. Set to 1 to disable the acceleration (by default)."]
pub type CONSTANT_TIME_W<'a, const O: u8> = crate::BitWriter<'a, CONSTANT_TIME_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to 0 to enable the acceleration option of constant_time for modular exponentiation. Set to 1 to disable the acceleration (by default)."]
    #[inline(always)]
    pub fn constant_time(&self) -> CONSTANT_TIME_R {
        CONSTANT_TIME_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONSTANT_TIME")
            .field(
                "constant_time",
                &format_args!("{}", self.constant_time().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONSTANT_TIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 0 to enable the acceleration option of constant_time for modular exponentiation. Set to 1 to disable the acceleration (by default)."]
    #[inline(always)]
    #[must_use]
    pub fn constant_time(&mut self) -> CONSTANT_TIME_W<0> {
        CONSTANT_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The constant_time option\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [constant_time](index.html) module"]
pub struct CONSTANT_TIME_SPEC;
impl crate::RegisterSpec for CONSTANT_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [constant_time::R](R) reader structure"]
impl crate::Readable for CONSTANT_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [constant_time::W](W) writer structure"]
impl crate::Writable for CONSTANT_TIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONSTANT_TIME to value 0x01"]
impl crate::Resettable for CONSTANT_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
