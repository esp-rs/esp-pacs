#[doc = "Register `HW_STAT_PANIC` reader"]
pub type R = crate::R<HW_STAT_PANIC_SPEC>;
#[doc = "Register `HW_STAT_PANIC` writer"]
pub type W = crate::W<HW_STAT_PANIC_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_panic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_panic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_STAT_PANIC_SPEC;
impl crate::RegisterSpec for HW_STAT_PANIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_stat_panic::R`](R) reader structure"]
impl crate::Readable for HW_STAT_PANIC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hw_stat_panic::W`](W) writer structure"]
impl crate::Writable for HW_STAT_PANIC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HW_STAT_PANIC to value 0"]
impl crate::Resettable for HW_STAT_PANIC_SPEC {}
