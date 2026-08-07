#[doc = "Register `CH%s_FADE_CONF` writer"]
pub type W = crate::W<CH_FADE_CONF_SPEC>;
#[doc = "Field `CH_FADE_PAUSE` writer - Configures whether or not to pause duty cycle fading of LEDC ch%s.\\\\0: Invalid. No effect\\\\1: Pause"]
pub type CH_FADE_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_FADE_RESUME` writer - Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\\\0: Invalid. No effect\\\\1: Resume"]
pub type CH_FADE_RESUME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_FADE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 5 - Configures whether or not to pause duty cycle fading of LEDC ch%s.\\\\0: Invalid. No effect\\\\1: Pause"]
    #[inline(always)]
    pub fn ch_fade_pause(&mut self) -> CH_FADE_PAUSE_W<'_, CH_FADE_CONF_SPEC> {
        CH_FADE_PAUSE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or nor to resume duty cycle fading of LEDC ch%s.\\\\0: Invalid. No effect\\\\1: Resume"]
    #[inline(always)]
    pub fn ch_fade_resume(&mut self) -> CH_FADE_RESUME_W<'_, CH_FADE_CONF_SPEC> {
        CH_FADE_RESUME_W::new(self, 6)
    }
}
#[doc = "Ledc ch%s fade config register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_fade_conf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_FADE_CONF_SPEC;
impl crate::RegisterSpec for CH_FADE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch_fade_conf::W`](W) writer structure"]
impl crate::Writable for CH_FADE_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH%s_FADE_CONF to value 0"]
impl crate::Resettable for CH_FADE_CONF_SPEC {}
