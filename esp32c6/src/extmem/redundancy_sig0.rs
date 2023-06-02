#[doc = "Register `REDUNDANCY_SIG0` reader"]
pub struct R(crate::R<REDUNDANCY_SIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REDUNDANCY_SIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REDUNDANCY_SIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REDUNDANCY_SIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REDUNDANCY_SIG0` writer"]
pub struct W(crate::W<REDUNDANCY_SIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REDUNDANCY_SIG0_SPEC>;
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
impl From<crate::W<REDUNDANCY_SIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REDUNDANCY_SIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_REDCY_SIG0` reader - Those bits are prepared for ECO."]
pub type CACHE_REDCY_SIG0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CACHE_REDCY_SIG0` writer - Those bits are prepared for ECO."]
pub type CACHE_REDCY_SIG0_W<'a, const O: u8> =
    crate::FieldWriter<'a, REDUNDANCY_SIG0_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are prepared for ECO."]
    #[inline(always)]
    pub fn cache_redcy_sig0(&self) -> CACHE_REDCY_SIG0_R {
        CACHE_REDCY_SIG0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REDUNDANCY_SIG0")
            .field(
                "cache_redcy_sig0",
                &format_args!("{}", self.cache_redcy_sig0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REDUNDANCY_SIG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are prepared for ECO."]
    #[inline(always)]
    #[must_use]
    pub fn cache_redcy_sig0(&mut self) -> CACHE_REDCY_SIG0_W<0> {
        CACHE_REDCY_SIG0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache redundancy signal 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [redundancy_sig0](index.html) module"]
pub struct REDUNDANCY_SIG0_SPEC;
impl crate::RegisterSpec for REDUNDANCY_SIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [redundancy_sig0::R](R) reader structure"]
impl crate::Readable for REDUNDANCY_SIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [redundancy_sig0::W](W) writer structure"]
impl crate::Writable for REDUNDANCY_SIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REDUNDANCY_SIG0 to value 0"]
impl crate::Resettable for REDUNDANCY_SIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
