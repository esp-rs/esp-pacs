#[doc = "Register `BLEAESKEY3` reader"]
pub type R = crate::R<BLEAESKEY3_SPEC>;
#[doc = "Register `BLEAESKEY3` writer"]
pub type W = crate::W<BLEAESKEY3_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "AES key\\[127:96\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`bleaeskey3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleaeskey3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEAESKEY3_SPEC;
impl crate::RegisterSpec for BLEAESKEY3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleaeskey3::R`](R) reader structure"]
impl crate::Readable for BLEAESKEY3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleaeskey3::W`](W) writer structure"]
impl crate::Writable for BLEAESKEY3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEAESKEY3 to value 0"]
impl crate::Resettable for BLEAESKEY3_SPEC {}
