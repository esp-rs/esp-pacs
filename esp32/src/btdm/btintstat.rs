#[doc = "Register `BTINTSTAT` reader"]
pub type R = crate::R<BTINTSTAT_SPEC>;
#[doc = "Register `BTINTSTAT` writer"]
pub type W = crate::W<BTINTSTAT_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "BR/EDR interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`btintstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btintstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTINTSTAT_SPEC;
impl crate::RegisterSpec for BTINTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btintstat::R`](R) reader structure"]
impl crate::Readable for BTINTSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btintstat::W`](W) writer structure"]
impl crate::Writable for BTINTSTAT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTINTSTAT to value 0"]
impl crate::Resettable for BTINTSTAT_SPEC {}
