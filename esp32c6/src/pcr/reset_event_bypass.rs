#[doc = "Register `RESET_EVENT_BYPASS` reader"]
pub struct R(crate::R<RESET_EVENT_BYPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_EVENT_BYPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_EVENT_BYPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_EVENT_BYPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET_EVENT_BYPASS` writer"]
pub struct W(crate::W<RESET_EVENT_BYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_EVENT_BYPASS_SPEC>;
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
impl From<crate::W<RESET_EVENT_BYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_EVENT_BYPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APM` reader - This field is used to control reset event relationship for tee_reg/apm_reg/hp_system_reg. 1: tee_reg/apm_reg/hp_system_reg will only be reset by power-reset. some reset event will be bypass. 0: tee_reg/apm_reg/hp_system_reg will not only be reset by power-reset, but also some reset event."]
pub type APM_R = crate::BitReader;
#[doc = "Field `APM` writer - This field is used to control reset event relationship for tee_reg/apm_reg/hp_system_reg. 1: tee_reg/apm_reg/hp_system_reg will only be reset by power-reset. some reset event will be bypass. 0: tee_reg/apm_reg/hp_system_reg will not only be reset by power-reset, but also some reset event."]
pub type APM_W<'a, const O: u8> = crate::BitWriter<'a, RESET_EVENT_BYPASS_SPEC, O>;
#[doc = "Field `RESET_EVENT_BYPASS` reader - This field is used to control reset event relationship for system-bus. 1: system bus (including arbiter/router) will only be reset by power-reset. some reset event will be bypass. 0: system bus (including arbiter/router) will not only be reset by power-reset, but also some reset event."]
pub type RESET_EVENT_BYPASS_R = crate::BitReader;
#[doc = "Field `RESET_EVENT_BYPASS` writer - This field is used to control reset event relationship for system-bus. 1: system bus (including arbiter/router) will only be reset by power-reset. some reset event will be bypass. 0: system bus (including arbiter/router) will not only be reset by power-reset, but also some reset event."]
pub type RESET_EVENT_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, RESET_EVENT_BYPASS_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This field is used to control reset event relationship for tee_reg/apm_reg/hp_system_reg. 1: tee_reg/apm_reg/hp_system_reg will only be reset by power-reset. some reset event will be bypass. 0: tee_reg/apm_reg/hp_system_reg will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn apm(&self) -> APM_R {
        APM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This field is used to control reset event relationship for system-bus. 1: system bus (including arbiter/router) will only be reset by power-reset. some reset event will be bypass. 0: system bus (including arbiter/router) will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn reset_event_bypass(&self) -> RESET_EVENT_BYPASS_R {
        RESET_EVENT_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_EVENT_BYPASS")
            .field("apm", &format_args!("{}", self.apm().bit()))
            .field(
                "reset_event_bypass",
                &format_args!("{}", self.reset_event_bypass().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RESET_EVENT_BYPASS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This field is used to control reset event relationship for tee_reg/apm_reg/hp_system_reg. 1: tee_reg/apm_reg/hp_system_reg will only be reset by power-reset. some reset event will be bypass. 0: tee_reg/apm_reg/hp_system_reg will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    #[must_use]
    pub fn apm(&mut self) -> APM_W<0> {
        APM_W::new(self)
    }
    #[doc = "Bit 1 - This field is used to control reset event relationship for system-bus. 1: system bus (including arbiter/router) will only be reset by power-reset. some reset event will be bypass. 0: system bus (including arbiter/router) will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    #[must_use]
    pub fn reset_event_bypass(&mut self) -> RESET_EVENT_BYPASS_W<1> {
        RESET_EVENT_BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reset event bypass backdoor configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset_event_bypass](index.html) module"]
pub struct RESET_EVENT_BYPASS_SPEC;
impl crate::RegisterSpec for RESET_EVENT_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset_event_bypass::R](R) reader structure"]
impl crate::Readable for RESET_EVENT_BYPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset_event_bypass::W](W) writer structure"]
impl crate::Writable for RESET_EVENT_BYPASS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESET_EVENT_BYPASS to value 0x02"]
impl crate::Resettable for RESET_EVENT_BYPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
