#[doc = "Register `RETENTION_CONF` reader"]
pub struct R(crate::R<RETENTION_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_CONF` writer"]
pub struct W(crate::W<RETENTION_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_CONF_SPEC>;
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
impl From<crate::W<RETENTION_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION_DISABLE` reader - Set this bit as 1 to disable retention function. Not disable by default."]
pub type RETENTION_DISABLE_R = crate::BitReader;
#[doc = "Field `RETENTION_DISABLE` writer - Set this bit as 1 to disable retention function. Not disable by default."]
pub type RETENTION_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, RETENTION_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this bit as 1 to disable retention function. Not disable by default."]
    #[inline(always)]
    pub fn retention_disable(&self) -> RETENTION_DISABLE_R {
        RETENTION_DISABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CONF")
            .field(
                "retention_disable",
                &format_args!("{}", self.retention_disable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit as 1 to disable retention function. Not disable by default."]
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
#[doc = "Retention configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_conf](index.html) module"]
pub struct RETENTION_CONF_SPEC;
impl crate::RegisterSpec for RETENTION_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_conf::R](R) reader structure"]
impl crate::Readable for RETENTION_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_conf::W](W) writer structure"]
impl crate::Writable for RETENTION_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RETENTION_CONF to value 0"]
impl crate::Resettable for RETENTION_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
