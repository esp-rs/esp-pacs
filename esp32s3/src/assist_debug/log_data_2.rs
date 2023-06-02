#[doc = "Register `LOG_DATA_2` reader"]
pub struct R(crate::R<LOG_DATA_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_DATA_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_DATA_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_DATA_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOG_DATA_2` writer"]
pub struct W(crate::W<LOG_DATA_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_DATA_2_SPEC>;
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
impl From<crate::W<LOG_DATA_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_DATA_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOG_DATA_2` reader - check data2"]
pub type LOG_DATA_2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOG_DATA_2` writer - check data2"]
pub type LOG_DATA_2_W<'a, const O: u8> = crate::FieldWriter<'a, LOG_DATA_2_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - check data2"]
    #[inline(always)]
    pub fn log_data_2(&self) -> LOG_DATA_2_R {
        LOG_DATA_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_DATA_2")
            .field("log_data_2", &format_args!("{}", self.log_data_2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_DATA_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - check data2"]
    #[inline(always)]
    #[must_use]
    pub fn log_data_2(&mut self) -> LOG_DATA_2_W<0> {
        LOG_DATA_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "log check data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_data_2](index.html) module"]
pub struct LOG_DATA_2_SPEC;
impl crate::RegisterSpec for LOG_DATA_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_data_2::R](R) reader structure"]
impl crate::Readable for LOG_DATA_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [log_data_2::W](W) writer structure"]
impl crate::Writable for LOG_DATA_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOG_DATA_2 to value 0"]
impl crate::Resettable for LOG_DATA_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
