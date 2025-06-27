#[doc = "Register `WIFI_INT_STATUS` reader"]
pub type R = crate::R<WIFI_INT_STATUS_SPEC>;
#[doc = "Register `WIFI_INT_STATUS` writer"]
pub type W = crate::W<WIFI_INT_STATUS_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt status of WIFI peripheral\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_INT_STATUS_SPEC;
impl crate::RegisterSpec for WIFI_INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_int_status::R`](R) reader structure"]
impl crate::Readable for WIFI_INT_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_int_status::W`](W) writer structure"]
impl crate::Writable for WIFI_INT_STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WIFI_INT_STATUS to value 0"]
impl crate::Resettable for WIFI_INT_STATUS_SPEC {}
