#[doc = "Register `BLERADIOCNTL1` reader"]
pub type R = crate::R<BLERADIOCNTL1_SPEC>;
#[doc = "Register `BLERADIOCNTL1` writer"]
pub type W = crate::W<BLERADIOCNTL1_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Radio interface control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bleradiocntl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleradiocntl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLERADIOCNTL1_SPEC;
impl crate::RegisterSpec for BLERADIOCNTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleradiocntl1::R`](R) reader structure"]
impl crate::Readable for BLERADIOCNTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleradiocntl1::W`](W) writer structure"]
impl crate::Writable for BLERADIOCNTL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLERADIOCNTL1 to value 0"]
impl crate::Resettable for BLERADIOCNTL1_SPEC {}
