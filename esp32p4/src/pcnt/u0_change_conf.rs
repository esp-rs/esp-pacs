#[doc = "Register `U0_CHANGE_CONF` reader"]
pub type R = crate::R<U0_CHANGE_CONF_SPEC>;
#[doc = "Register `U0_CHANGE_CONF` writer"]
pub type W = crate::W<U0_CHANGE_CONF_SPEC>;
#[doc = "Field `CNT_STEP_U0` reader - Configures the step value for unit 0."]
pub type CNT_STEP_U0_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_STEP_U0` writer - Configures the step value for unit 0."]
pub type CNT_STEP_U0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_STEP_LIM_U0` reader - Configures the step limit value for unit 0."]
pub type CNT_STEP_LIM_U0_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_STEP_LIM_U0` writer - Configures the step limit value for unit 0."]
pub type CNT_STEP_LIM_U0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the step value for unit 0."]
    #[inline(always)]
    pub fn cnt_step_u0(&self) -> CNT_STEP_U0_R {
        CNT_STEP_U0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Configures the step limit value for unit 0."]
    #[inline(always)]
    pub fn cnt_step_lim_u0(&self) -> CNT_STEP_LIM_U0_R {
        CNT_STEP_LIM_U0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U0_CHANGE_CONF")
            .field("cnt_step_u0", &self.cnt_step_u0().bits())
            .field("cnt_step_lim_u0", &self.cnt_step_lim_u0().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<U0_CHANGE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the step value for unit 0."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_step_u0(&mut self) -> CNT_STEP_U0_W<U0_CHANGE_CONF_SPEC> {
        CNT_STEP_U0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Configures the step limit value for unit 0."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_step_lim_u0(&mut self) -> CNT_STEP_LIM_U0_W<U0_CHANGE_CONF_SPEC> {
        CNT_STEP_LIM_U0_W::new(self, 16)
    }
}
#[doc = "Configuration register for unit $n's step value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u0_change_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u0_change_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U0_CHANGE_CONF_SPEC;
impl crate::RegisterSpec for U0_CHANGE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u0_change_conf::R`](R) reader structure"]
impl crate::Readable for U0_CHANGE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`u0_change_conf::W`](W) writer structure"]
impl crate::Writable for U0_CHANGE_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets U0_CHANGE_CONF to value 0"]
impl crate::Resettable for U0_CHANGE_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
