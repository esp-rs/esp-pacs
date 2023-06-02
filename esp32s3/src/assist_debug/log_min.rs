#[doc = "Register `LOG_MIN` reader"]
pub struct R(crate::R<LOG_MIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_MIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_MIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_MIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOG_MIN` writer"]
pub struct W(crate::W<LOG_MIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_MIN_SPEC>;
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
impl From<crate::W<LOG_MIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_MIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOG_MIN` reader - check region min addr"]
pub type LOG_MIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOG_MIN` writer - check region min addr"]
pub type LOG_MIN_W<'a, const O: u8> = crate::FieldWriter<'a, LOG_MIN_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - check region min addr"]
    #[inline(always)]
    pub fn log_min(&self) -> LOG_MIN_R {
        LOG_MIN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_MIN")
            .field("log_min", &format_args!("{}", self.log_min().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_MIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - check region min addr"]
    #[inline(always)]
    #[must_use]
    pub fn log_min(&mut self) -> LOG_MIN_W<0> {
        LOG_MIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "log check region configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_min](index.html) module"]
pub struct LOG_MIN_SPEC;
impl crate::RegisterSpec for LOG_MIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_min::R](R) reader structure"]
impl crate::Readable for LOG_MIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [log_min::W](W) writer structure"]
impl crate::Writable for LOG_MIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOG_MIN to value 0"]
impl crate::Resettable for LOG_MIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
