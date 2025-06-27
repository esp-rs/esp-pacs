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
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_START_ADDR_SPEC;
impl crate::RegisterSpec for FIFO_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_start_addr::R`](R) reader structure"]
impl crate::Readable for FIFO_START_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_start_addr::W`](W) writer structure"]
impl crate::Writable for FIFO_START_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO_START_ADDR to value 0"]
impl crate::Resettable for FIFO_START_ADDR_SPEC {}
