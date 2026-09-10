#[doc = "Register `PWR_INT_ENABLE` reader"]
pub type R = crate::R<PWR_INT_ENABLE_SPEC>;
#[doc = "Register `PWR_INT_ENABLE` writer"]
pub type W = crate::W<PWR_INT_ENABLE_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt enable for the WIFI_PWR interrupt. TBTT of interface n is bit (4 - n), TSF timer n is bit (8 - n).\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_int_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_int_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_INT_ENABLE_SPEC;
impl crate::RegisterSpec for PWR_INT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_int_enable::R`](R) reader structure"]
impl crate::Readable for PWR_INT_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr_int_enable::W`](W) writer structure"]
impl crate::Writable for PWR_INT_ENABLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWR_INT_ENABLE to value 0"]
impl crate::Resettable for PWR_INT_ENABLE_SPEC {}
