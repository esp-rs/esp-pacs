#[doc = "Register `PWR_INT_CLEAR` reader"]
pub type R = crate::R<PWR_INT_CLEAR_SPEC>;
#[doc = "Register `PWR_INT_CLEAR` writer"]
pub type W = crate::W<PWR_INT_CLEAR_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt status clear for the WIFI_PWR interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_int_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_int_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_INT_CLEAR_SPEC;
impl crate::RegisterSpec for PWR_INT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_int_clear::R`](R) reader structure"]
impl crate::Readable for PWR_INT_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr_int_clear::W`](W) writer structure"]
impl crate::Writable for PWR_INT_CLEAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_INT_CLEAR to value 0"]
impl crate::Resettable for PWR_INT_CLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
