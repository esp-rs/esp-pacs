#[doc = "Register `CORE_DEBUG_RUNSTALL_CONF` reader"]
pub struct R(crate::R<CORE_DEBUG_RUNSTALL_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_DEBUG_RUNSTALL_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_DEBUG_RUNSTALL_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_DEBUG_RUNSTALL_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_DEBUG_RUNSTALL_CONF` writer"]
pub struct W(crate::W<CORE_DEBUG_RUNSTALL_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_DEBUG_RUNSTALL_CONF_SPEC>;
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
impl From<crate::W<CORE_DEBUG_RUNSTALL_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_DEBUG_RUNSTALL_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_DEBUG_RUNSTALL_ENABLE` reader - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
pub type CORE_DEBUG_RUNSTALL_ENABLE_R = crate::BitReader;
#[doc = "Field `CORE_DEBUG_RUNSTALL_ENABLE` writer - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
pub type CORE_DEBUG_RUNSTALL_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, CORE_DEBUG_RUNSTALL_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
    #[inline(always)]
    pub fn core_debug_runstall_enable(&self) -> CORE_DEBUG_RUNSTALL_ENABLE_R {
        CORE_DEBUG_RUNSTALL_ENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_DEBUG_RUNSTALL_CONF")
            .field(
                "core_debug_runstall_enable",
                &format_args!("{}", self.core_debug_runstall_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_DEBUG_RUNSTALL_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
    #[inline(always)]
    #[must_use]
    pub fn core_debug_runstall_enable(&mut self) -> CORE_DEBUG_RUNSTALL_ENABLE_W<0> {
        CORE_DEBUG_RUNSTALL_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core Debug runstall configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_debug_runstall_conf](index.html) module"]
pub struct CORE_DEBUG_RUNSTALL_CONF_SPEC;
impl crate::RegisterSpec for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_debug_runstall_conf::R](R) reader structure"]
impl crate::Readable for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_debug_runstall_conf::W](W) writer structure"]
impl crate::Writable for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_DEBUG_RUNSTALL_CONF to value 0"]
impl crate::Resettable for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
