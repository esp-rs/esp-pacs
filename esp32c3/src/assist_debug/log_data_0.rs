#[doc = "Register `LOG_DATA_0` reader"]
pub struct R(crate::R<LOG_DATA_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_DATA_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_DATA_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_DATA_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOG_DATA_0` writer"]
pub struct W(crate::W<LOG_DATA_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_DATA_0_SPEC>;
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
impl From<crate::W<LOG_DATA_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_DATA_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOG_DATA_0` reader - reg_log_data_0"]
pub type LOG_DATA_0_R = crate::FieldReader<u32>;
#[doc = "Field `LOG_DATA_0` writer - reg_log_data_0"]
pub type LOG_DATA_0_W<'a, const O: u8> = crate::FieldWriter<'a, LOG_DATA_0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_log_data_0"]
    #[inline(always)]
    pub fn log_data_0(&self) -> LOG_DATA_0_R {
        LOG_DATA_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_DATA_0")
            .field("log_data_0", &format_args!("{}", self.log_data_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_DATA_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_log_data_0"]
    #[inline(always)]
    #[must_use]
    pub fn log_data_0(&mut self) -> LOG_DATA_0_W<0> {
        LOG_DATA_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST_DEBUG_LOG_DATA_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_data_0](index.html) module"]
pub struct LOG_DATA_0_SPEC;
impl crate::RegisterSpec for LOG_DATA_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_data_0::R](R) reader structure"]
impl crate::Readable for LOG_DATA_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [log_data_0::W](W) writer structure"]
impl crate::Writable for LOG_DATA_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOG_DATA_0 to value 0"]
impl crate::Resettable for LOG_DATA_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
