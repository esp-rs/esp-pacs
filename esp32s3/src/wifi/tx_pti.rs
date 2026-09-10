#[doc = "Register `TX_PTI%s` reader"]
pub type R = crate::R<TX_PTI_SPEC>;
#[doc = "Register `TX_PTI%s` writer"]
pub type W = crate::W<TX_PTI_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Coexistence priorities of the TX slot, written by hal_set_tx_pti\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_pti::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pti::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_PTI_SPEC;
impl crate::RegisterSpec for TX_PTI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_pti::R`](R) reader structure"]
impl crate::Readable for TX_PTI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_pti::W`](W) writer structure"]
impl crate::Writable for TX_PTI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_PTI%s to value 0"]
impl crate::Resettable for TX_PTI_SPEC {}
