#[doc = "Register `BLEDIAGSTAT` reader"]
pub type R = crate::R<BLEDIAGSTAT_SPEC>;
#[doc = "Register `BLEDIAGSTAT` writer"]
pub type W = crate::W<BLEDIAGSTAT_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Diagnostics status register\n\nYou can [`read`](crate::Reg::read) this register and get [`blediagstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blediagstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEDIAGSTAT_SPEC;
impl crate::RegisterSpec for BLEDIAGSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blediagstat::R`](R) reader structure"]
impl crate::Readable for BLEDIAGSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blediagstat::W`](W) writer structure"]
impl crate::Writable for BLEDIAGSTAT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEDIAGSTAT to value 0"]
impl crate::Resettable for BLEDIAGSTAT_SPEC {}
