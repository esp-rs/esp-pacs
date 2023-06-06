#[doc = "Register `LACTLOADHI` reader"]
pub struct R(crate::R<LACTLOADHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LACTLOADHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LACTLOADHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LACTLOADHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LACTLOADHI` writer"]
pub struct W(crate::W<LACTLOADHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LACTLOADHI_SPEC>;
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
impl From<crate::W<LACTLOADHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LACTLOADHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LACT_LOAD_HI` reader - "]
pub type LACT_LOAD_HI_R = crate::FieldReader<u32>;
#[doc = "Field `LACT_LOAD_HI` writer - "]
pub type LACT_LOAD_HI_W<'a, const O: u8> = crate::FieldWriter<'a, LACTLOADHI_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lact_load_hi(&self) -> LACT_LOAD_HI_R {
        LACT_LOAD_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTLOADHI")
            .field(
                "lact_load_hi",
                &format_args!("{}", self.lact_load_hi().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTLOADHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn lact_load_hi(&mut self) -> LACT_LOAD_HI_W<0> {
        LACT_LOAD_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lactloadhi](index.html) module"]
pub struct LACTLOADHI_SPEC;
impl crate::RegisterSpec for LACTLOADHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lactloadhi::R](R) reader structure"]
impl crate::Readable for LACTLOADHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lactloadhi::W](W) writer structure"]
impl crate::Writable for LACTLOADHI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LACTLOADHI to value 0"]
impl crate::Resettable for LACTLOADHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
