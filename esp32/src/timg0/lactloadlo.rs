#[doc = "Register `LACTLOADLO` reader"]
pub struct R(crate::R<LACTLOADLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LACTLOADLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LACTLOADLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LACTLOADLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LACTLOADLO` writer"]
pub struct W(crate::W<LACTLOADLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LACTLOADLO_SPEC>;
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
impl From<crate::W<LACTLOADLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LACTLOADLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LACT_LOAD_LO` reader - "]
pub type LACT_LOAD_LO_R = crate::FieldReader<u32>;
#[doc = "Field `LACT_LOAD_LO` writer - "]
pub type LACT_LOAD_LO_W<'a, const O: u8> = crate::FieldWriter<'a, LACTLOADLO_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lact_load_lo(&self) -> LACT_LOAD_LO_R {
        LACT_LOAD_LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTLOADLO")
            .field(
                "lact_load_lo",
                &format_args!("{}", self.lact_load_lo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTLOADLO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn lact_load_lo(&mut self) -> LACT_LOAD_LO_W<0> {
        LACT_LOAD_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lactloadlo](index.html) module"]
pub struct LACTLOADLO_SPEC;
impl crate::RegisterSpec for LACTLOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lactloadlo::R](R) reader structure"]
impl crate::Readable for LACTLOADLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lactloadlo::W](W) writer structure"]
impl crate::Writable for LACTLOADLO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LACTLOADLO to value 0"]
impl crate::Resettable for LACTLOADLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
