#[doc = "Register `TSF_LOAD_HIGH` reader"]
pub type R = crate::R<TSF_LOAD_HIGH_SPEC>;
#[doc = "Register `TSF_LOAD_HIGH` writer"]
pub type W = crate::W<TSF_LOAD_HIGH_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "High word of the value loaded into a TSF counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_load_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_load_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSF_LOAD_HIGH_SPEC;
impl crate::RegisterSpec for TSF_LOAD_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsf_load_high::R`](R) reader structure"]
impl crate::Readable for TSF_LOAD_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsf_load_high::W`](W) writer structure"]
impl crate::Writable for TSF_LOAD_HIGH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSF_LOAD_HIGH to value 0"]
impl crate::Resettable for TSF_LOAD_HIGH_SPEC {}
