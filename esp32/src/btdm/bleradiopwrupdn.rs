#[doc = "Register `BLERADIOPWRUPDN` reader"]
pub type R = crate::R<BLERADIOPWRUPDN_SPEC>;
#[doc = "Register `BLERADIOPWRUPDN` writer"]
pub type W = crate::W<BLERADIOPWRUPDN_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RX/TX power up/down phase register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleradiopwrupdn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleradiopwrupdn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLERADIOPWRUPDN_SPEC;
impl crate::RegisterSpec for BLERADIOPWRUPDN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleradiopwrupdn::R`](R) reader structure"]
impl crate::Readable for BLERADIOPWRUPDN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleradiopwrupdn::W`](W) writer structure"]
impl crate::Writable for BLERADIOPWRUPDN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLERADIOPWRUPDN to value 0"]
impl crate::Resettable for BLERADIOPWRUPDN_SPEC {}
