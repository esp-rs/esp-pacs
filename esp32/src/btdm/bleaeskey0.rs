#[doc = "Register `BLEAESKEY0` reader"]
pub type R = crate::R<BLEAESKEY0_SPEC>;
#[doc = "Register `BLEAESKEY0` writer"]
pub type W = crate::W<BLEAESKEY0_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AES key\\[31:0\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`bleaeskey0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleaeskey0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEAESKEY0_SPEC;
impl crate::RegisterSpec for BLEAESKEY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleaeskey0::R`](R) reader structure"]
impl crate::Readable for BLEAESKEY0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleaeskey0::W`](W) writer structure"]
impl crate::Writable for BLEAESKEY0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEAESKEY0 to value 0"]
impl crate::Resettable for BLEAESKEY0_SPEC {}
