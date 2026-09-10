#[doc = "Register `TSF_TIMER_TARGET%s` reader"]
pub type R = crate::R<TSF_TIMER_TARGET_SPEC>;
#[doc = "Register `TSF_TIMER_TARGET%s` writer"]
pub type W = crate::W<TSF_TIMER_TARGET_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Target of a TSF timer, written by tsf_hal_set_timer_target\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_timer_target::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_timer_target::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSF_TIMER_TARGET_SPEC;
impl crate::RegisterSpec for TSF_TIMER_TARGET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsf_timer_target::R`](R) reader structure"]
impl crate::Readable for TSF_TIMER_TARGET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsf_timer_target::W`](W) writer structure"]
impl crate::Writable for TSF_TIMER_TARGET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSF_TIMER_TARGET%s to value 0"]
impl crate::Resettable for TSF_TIMER_TARGET_SPEC {}
