#[doc = "Register `ACK_NUM` reader"]
pub type R = crate::R<ACK_NUM_SPEC>;
#[doc = "Register `ACK_NUM` writer"]
pub type W = crate::W<ACK_NUM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ACK_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ack_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACK_NUM_SPEC;
impl crate::RegisterSpec for ACK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ack_num::R`](R) reader structure"]
impl crate::Readable for ACK_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ack_num::W`](W) writer structure"]
impl crate::Writable for ACK_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACK_NUM to value 0"]
impl crate::Resettable for ACK_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
