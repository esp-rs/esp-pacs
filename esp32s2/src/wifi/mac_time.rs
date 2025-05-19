#[doc = "Register `MAC_TIME` reader"]
pub type R = crate::R<MAC_TIME_SPEC>;
#[doc = "Register `MAC_TIME` writer"]
pub type W = crate::W<MAC_TIME_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Current value of the MAC timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAC_TIME_SPEC;
impl crate::RegisterSpec for MAC_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac_time::R`](R) reader structure"]
impl crate::Readable for MAC_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mac_time::W`](W) writer structure"]
impl crate::Writable for MAC_TIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAC_TIME to value 0"]
impl crate::Resettable for MAC_TIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
