#[doc = "Register `BLEERRORTYPESTAT` reader"]
pub type R = crate::R<BLEERRORTYPESTAT_SPEC>;
#[doc = "Register `BLEERRORTYPESTAT` writer"]
pub type W = crate::W<BLEERRORTYPESTAT_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Error type status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bleerrortypestat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleerrortypestat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEERRORTYPESTAT_SPEC;
impl crate::RegisterSpec for BLEERRORTYPESTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleerrortypestat::R`](R) reader structure"]
impl crate::Readable for BLEERRORTYPESTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleerrortypestat::W`](W) writer structure"]
impl crate::Writable for BLEERRORTYPESTAT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEERRORTYPESTAT to value 0"]
impl crate::Resettable for BLEERRORTYPESTAT_SPEC {}
