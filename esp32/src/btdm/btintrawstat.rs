#[doc = "Register `BTINTRAWSTAT` reader"]
pub type R = crate::R<BTINTRAWSTAT_SPEC>;
#[doc = "Register `BTINTRAWSTAT` writer"]
pub type W = crate::W<BTINTRAWSTAT_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "BR/EDR interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`btintrawstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btintrawstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTINTRAWSTAT_SPEC;
impl crate::RegisterSpec for BTINTRAWSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btintrawstat::R`](R) reader structure"]
impl crate::Readable for BTINTRAWSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btintrawstat::W`](W) writer structure"]
impl crate::Writable for BTINTRAWSTAT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTINTRAWSTAT to value 0"]
impl crate::Resettable for BTINTRAWSTAT_SPEC {}
