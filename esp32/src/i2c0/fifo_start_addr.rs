#[doc = "Register `FIFO_START_ADDR` reader"]
pub type R = crate::R<FIFO_START_ADDR_SPEC>;
#[doc = "Register `FIFO_START_ADDR` writer"]
pub type W = crate::W<FIFO_START_ADDR_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_start_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_start_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_START_ADDR_SPEC;
impl crate::RegisterSpec for FIFO_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_start_addr::R`](R) reader structure"]
impl crate::Readable for FIFO_START_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_start_addr::W`](W) writer structure"]
impl crate::Writable for FIFO_START_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_START_ADDR to value 0"]
impl crate::Resettable for FIFO_START_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
