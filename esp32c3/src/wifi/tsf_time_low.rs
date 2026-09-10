#[doc = "Register `TSF_TIME_LOW` reader"]
pub type R = crate::R<TSF_TIME_LOW_SPEC>;
#[doc = "Register `TSF_TIME_LOW` writer"]
pub type W = crate::W<TSF_TIME_LOW_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Low word of the latched TSF counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_time_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_time_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSF_TIME_LOW_SPEC;
impl crate::RegisterSpec for TSF_TIME_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsf_time_low::R`](R) reader structure"]
impl crate::Readable for TSF_TIME_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsf_time_low::W`](W) writer structure"]
impl crate::Writable for TSF_TIME_LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSF_TIME_LOW to value 0"]
impl crate::Resettable for TSF_TIME_LOW_SPEC {}
