#[doc = "Register `TIMEOUT` reader"]
pub struct R(crate::R<TIMEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMEOUT` writer"]
pub struct W(crate::W<TIMEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEOUT_SPEC>;
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
impl From<crate::W<TIMEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUT` reader - Maximum number of FAST_CLK cycles that the transmission can take"]
pub type TIMEOUT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMEOUT` writer - Maximum number of FAST_CLK cycles that the transmission can take"]
pub type TIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, TIMEOUT_SPEC, 20, O, u32, u32>;
impl R {
    #[doc = "Bits 0:19 - Maximum number of FAST_CLK cycles that the transmission can take"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUT")
            .field("timeout", &format_args!("{}", self.timeout().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMEOUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19 - Maximum number of FAST_CLK cycles that the transmission can take"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<0> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout](index.html) module"]
pub struct TIMEOUT_SPEC;
impl crate::RegisterSpec for TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timeout::R](R) reader structure"]
impl crate::Readable for TIMEOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timeout::W](W) writer structure"]
impl crate::Writable for TIMEOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMEOUT to value 0"]
impl crate::Resettable for TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
