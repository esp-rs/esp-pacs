#[doc = "Register `RESET_EVENT_BYPASS` reader"]
pub type R = crate::R<RESET_EVENT_BYPASS_SPEC>;
#[doc = "Register `RESET_EVENT_BYPASS` writer"]
pub type W = crate::W<RESET_EVENT_BYPASS_SPEC>;
#[doc = "Field `APM` reader - This field is used to control reset event relationship for tee_reg/apm_reg/hp_system_reg. 1: tee_reg/apm_reg/hp_system_reg will only be reset by power-reset. some reset event will be bypass. 0: tee_reg/apm_reg/hp_system_reg will not only be reset by power-reset, but also some reset event."]
pub type APM_R = crate::BitReader;
#[doc = "Field `APM` writer - This field is used to control reset event relationship for tee_reg/apm_reg/hp_system_reg. 1: tee_reg/apm_reg/hp_system_reg will only be reset by power-reset. some reset event will be bypass. 0: tee_reg/apm_reg/hp_system_reg will not only be reset by power-reset, but also some reset event."]
pub type APM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_EVENT_BYPASS` reader - This field is used to control reset event relationship for system-bus. 1: system bus (including arbiter/router) will only be reset by power-reset. some reset event will be bypass. 0: system bus (including arbiter/router) will not only be reset by power-reset, but also some reset event."]
pub type RESET_EVENT_BYPASS_R = crate::BitReader;
#[doc = "Field `RESET_EVENT_BYPASS` writer - This field is used to control reset event relationship for system-bus. 1: system bus (including arbiter/router) will only be reset by power-reset. some reset event will be bypass. 0: system bus (including arbiter/router) will not only be reset by power-reset, but also some reset event."]
pub type RESET_EVENT_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This field is used to control reset event relationship for tee_reg/apm_reg/hp_system_reg. 1: tee_reg/apm_reg/hp_system_reg will only be reset by power-reset. some reset event will be bypass. 0: tee_reg/apm_reg/hp_system_reg will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    #[must_use]
    pub fn apm(&mut self) -> APM_W<RESET_EVENT_BYPASS_SPEC> {
        APM_W::new(self, 0)
    }
    #[doc = "Bit 1 - This field is used to control reset event relationship for system-bus. 1: system bus (including arbiter/router) will only be reset by power-reset. some reset event will be bypass. 0: system bus (including arbiter/router) will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    #[must_use]
    pub fn reset_event_bypass(&mut self) -> RESET_EVENT_BYPASS_W<RESET_EVENT_BYPASS_SPEC> {
        RESET_EVENT_BYPASS_W::new(self, 1)
    }
}
#[doc = "reset event bypass backdoor configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_event_bypass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_event_bypass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_EVENT_BYPASS_SPEC;
impl crate::RegisterSpec for RESET_EVENT_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_event_bypass::R`](R) reader structure"]
impl crate::Readable for RESET_EVENT_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_event_bypass::W`](W) writer structure"]
impl crate::Writable for RESET_EVENT_BYPASS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET_EVENT_BYPASS to value 0x02"]
impl crate::Resettable for RESET_EVENT_BYPASS_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
