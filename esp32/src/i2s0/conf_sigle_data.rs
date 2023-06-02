#[doc = "Register `CONF_SIGLE_DATA` reader"]
pub struct R(crate::R<CONF_SIGLE_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SIGLE_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SIGLE_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SIGLE_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_SIGLE_DATA` writer"]
pub struct W(crate::W<CONF_SIGLE_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SIGLE_DATA_SPEC>;
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
impl From<crate::W<CONF_SIGLE_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SIGLE_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGLE_DATA` reader - "]
pub type SIGLE_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIGLE_DATA` writer - "]
pub type SIGLE_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, CONF_SIGLE_DATA_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sigle_data(&self) -> SIGLE_DATA_R {
        SIGLE_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_SIGLE_DATA")
            .field("sigle_data", &format_args!("{}", self.sigle_data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SIGLE_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sigle_data(&mut self) -> SIGLE_DATA_W<0> {
        SIGLE_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_sigle_data](index.html) module"]
pub struct CONF_SIGLE_DATA_SPEC;
impl crate::RegisterSpec for CONF_SIGLE_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_sigle_data::R](R) reader structure"]
impl crate::Readable for CONF_SIGLE_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_sigle_data::W](W) writer structure"]
impl crate::Writable for CONF_SIGLE_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_SIGLE_DATA to value 0"]
impl crate::Resettable for CONF_SIGLE_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
