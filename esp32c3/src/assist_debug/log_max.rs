#[doc = "Register `LOG_MAX` reader"]
pub struct R(crate::R<LOG_MAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_MAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_MAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_MAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOG_MAX` writer"]
pub struct W(crate::W<LOG_MAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_MAX_SPEC>;
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
impl From<crate::W<LOG_MAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_MAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOG_MAX` reader - reg_log_max"]
pub type LOG_MAX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOG_MAX` writer - reg_log_max"]
pub type LOG_MAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOG_MAX_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - reg_log_max"]
    #[inline(always)]
    pub fn log_max(&self) -> LOG_MAX_R {
        LOG_MAX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_log_max"]
    #[inline(always)]
    pub fn log_max(&mut self) -> LOG_MAX_W<0> {
        LOG_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST_DEBUG_LOG_MAX_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_max](index.html) module"]
pub struct LOG_MAX_SPEC;
impl crate::RegisterSpec for LOG_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_max::R](R) reader structure"]
impl crate::Readable for LOG_MAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [log_max::W](W) writer structure"]
impl crate::Writable for LOG_MAX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOG_MAX to value 0"]
impl crate::Resettable for LOG_MAX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
