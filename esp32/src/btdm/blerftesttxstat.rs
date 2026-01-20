#[doc = "Register `BLERFTESTTXSTAT` reader"]
pub type R = crate::R<BLERFTESTTXSTAT_SPEC>;
#[doc = "Register `BLERFTESTTXSTAT` writer"]
pub type W = crate::W<BLERFTESTTXSTAT_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Number of valid packets transmited during Test\n\nYou can [`read`](crate::Reg::read) this register and get [`blerftesttxstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blerftesttxstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLERFTESTTXSTAT_SPEC;
impl crate::RegisterSpec for BLERFTESTTXSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blerftesttxstat::R`](R) reader structure"]
impl crate::Readable for BLERFTESTTXSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blerftesttxstat::W`](W) writer structure"]
impl crate::Writable for BLERFTESTTXSTAT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLERFTESTTXSTAT to value 0"]
impl crate::Resettable for BLERFTESTTXSTAT_SPEC {}
