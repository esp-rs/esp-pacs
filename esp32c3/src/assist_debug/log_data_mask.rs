#[doc = "Register `LOG_DATA_MASK` reader"]
pub struct R(crate::R<LOG_DATA_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOG_DATA_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOG_DATA_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOG_DATA_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOG_DATA_MASK` writer"]
pub struct W(crate::W<LOG_DATA_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOG_DATA_MASK_SPEC>;
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
impl From<crate::W<LOG_DATA_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOG_DATA_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOG_DATA_SIZE` reader - reg_log_data_size"]
pub type LOG_DATA_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `LOG_DATA_SIZE` writer - reg_log_data_size"]
pub type LOG_DATA_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, LOG_DATA_MASK_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - reg_log_data_size"]
    #[inline(always)]
    pub fn log_data_size(&self) -> LOG_DATA_SIZE_R {
        LOG_DATA_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_DATA_MASK")
            .field(
                "log_data_size",
                &format_args!("{}", self.log_data_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_DATA_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - reg_log_data_size"]
    #[inline(always)]
    #[must_use]
    pub fn log_data_size(&mut self) -> LOG_DATA_SIZE_W<0> {
        LOG_DATA_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASSIST_DEBUG_LOG_DATA_MASK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [log_data_mask](index.html) module"]
pub struct LOG_DATA_MASK_SPEC;
impl crate::RegisterSpec for LOG_DATA_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [log_data_mask::R](R) reader structure"]
impl crate::Readable for LOG_DATA_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [log_data_mask::W](W) writer structure"]
impl crate::Writable for LOG_DATA_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOG_DATA_MASK to value 0"]
impl crate::Resettable for LOG_DATA_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
