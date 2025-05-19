#[doc = "Register `ADDR_LOW` reader"]
pub type R = crate::R<ADDR_LOW_SPEC>;
#[doc = "Register `ADDR_LOW` writer"]
pub type W = crate::W<ADDR_LOW_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Lower four octets of the MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_LOW_SPEC;
impl crate::RegisterSpec for ADDR_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_low::R`](R) reader structure"]
impl crate::Readable for ADDR_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr_low::W`](W) writer structure"]
impl crate::Writable for ADDR_LOW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR_LOW to value 0"]
impl crate::Resettable for ADDR_LOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
