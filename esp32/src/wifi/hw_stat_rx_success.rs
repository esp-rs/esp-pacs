#[doc = "Register `HW_STAT_RX_SUCCESS` reader"]
pub type R = crate::R<HW_STAT_RX_SUCCESS_SPEC>;
#[doc = "Register `HW_STAT_RX_SUCCESS` writer"]
pub type W = crate::W<HW_STAT_RX_SUCCESS_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_stat_rx_success::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_stat_rx_success::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_STAT_RX_SUCCESS_SPEC;
impl crate::RegisterSpec for HW_STAT_RX_SUCCESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_stat_rx_success::R`](R) reader structure"]
impl crate::Readable for HW_STAT_RX_SUCCESS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hw_stat_rx_success::W`](W) writer structure"]
impl crate::Writable for HW_STAT_RX_SUCCESS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HW_STAT_RX_SUCCESS to value 0"]
impl crate::Resettable for HW_STAT_RX_SUCCESS_SPEC {}
