#[doc = "Register `UNKNOWN_RX_POLICY%s` reader"]
pub type R = crate::R<UNKNOWN_RX_POLICY_SPEC>;
#[doc = "Register `UNKNOWN_RX_POLICY%s` writer"]
pub type W = crate::W<UNKNOWN_RX_POLICY_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`unknown_rx_policy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unknown_rx_policy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNKNOWN_RX_POLICY_SPEC;
impl crate::RegisterSpec for UNKNOWN_RX_POLICY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unknown_rx_policy::R`](R) reader structure"]
impl crate::Readable for UNKNOWN_RX_POLICY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unknown_rx_policy::W`](W) writer structure"]
impl crate::Writable for UNKNOWN_RX_POLICY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UNKNOWN_RX_POLICY%s to value 0"]
impl crate::Resettable for UNKNOWN_RX_POLICY_SPEC {
    const RESET_VALUE: u32 = 0;
}
