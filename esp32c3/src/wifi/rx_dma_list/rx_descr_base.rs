#[doc = "Register `RX_DESCR_BASE` reader"]
pub type R = crate::R<RX_DESCR_BASE_SPEC>;
#[doc = "Register `RX_DESCR_BASE` writer"]
pub type W = crate::W<RX_DESCR_BASE_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "base address of the RX DMA list\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_descr_base::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_descr_base::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DESCR_BASE_SPEC;
impl crate::RegisterSpec for RX_DESCR_BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_descr_base::R`](R) reader structure"]
impl crate::Readable for RX_DESCR_BASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_descr_base::W`](W) writer structure"]
impl crate::Writable for RX_DESCR_BASE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_DESCR_BASE to value 0"]
impl crate::Resettable for RX_DESCR_BASE_SPEC {}
