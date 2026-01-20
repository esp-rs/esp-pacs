#[doc = "Register `BLERFTESTRXSTAT` reader"]
pub type R = crate::R<BLERFTESTRXSTAT_SPEC>;
#[doc = "Register `BLERFTESTRXSTAT` writer"]
pub type W = crate::W<BLERFTESTRXSTAT_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Number of valid packets received during Test\n\nYou can [`read`](crate::Reg::read) this register and get [`blerftestrxstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerftestrxstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLERFTESTRXSTAT_SPEC;
impl crate::RegisterSpec for BLERFTESTRXSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blerftestrxstat::R`](R) reader structure"]
impl crate::Readable for BLERFTESTRXSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blerftestrxstat::W`](W) writer structure"]
impl crate::Writable for BLERFTESTRXSTAT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLERFTESTRXSTAT to value 0"]
impl crate::Resettable for BLERFTESTRXSTAT_SPEC {}
