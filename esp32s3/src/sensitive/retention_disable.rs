#[doc = "Register `RETENTION_DISABLE` reader"]
pub struct R(crate::R<RETENTION_DISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_DISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_DISABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_DISABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_DISABLE` writer"]
pub struct W(crate::W<RETENTION_DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_DISABLE_SPEC>;
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
impl From<crate::W<RETENTION_DISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION_DISABLE` reader - Set 1 to disable retention function and lock disable state."]
pub type RETENTION_DISABLE_R = crate::BitReader;
#[doc = "Field `RETENTION_DISABLE` writer - Set 1 to disable retention function and lock disable state."]
pub type RETENTION_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, RETENTION_DISABLE_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to disable retention function and lock disable state."]
    #[inline(always)]
    pub fn retention_disable(&self) -> RETENTION_DISABLE_R {
        RETENTION_DISABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_DISABLE")
            .field(
                "retention_disable",
                &format_args!("{}", self.retention_disable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_DISABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to disable retention function and lock disable state."]
    #[inline(always)]
    #[must_use]
    pub fn retention_disable(&mut self) -> RETENTION_DISABLE_W<0> {
        RETENTION_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Retention configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_disable](index.html) module"]
pub struct RETENTION_DISABLE_SPEC;
impl crate::RegisterSpec for RETENTION_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_disable::R](R) reader structure"]
impl crate::Readable for RETENTION_DISABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_disable::W](W) writer structure"]
impl crate::Writable for RETENTION_DISABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RETENTION_DISABLE to value 0"]
impl crate::Resettable for RETENTION_DISABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
