#[doc = "Register `RXDMA_ADDR` reader"]
pub type R = crate::R<RXDMA_ADDR_SPEC>;
#[doc = "Register `RXDMA_ADDR` writer"]
pub type W = crate::W<RXDMA_ADDR_SPEC>;
#[doc = "Field `RXDMA_ADDR` reader - "]
pub type RXDMA_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `RXDMA_ADDR` writer - "]
pub type RXDMA_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rxdma_addr(&self) -> RXDMA_ADDR_R {
        RXDMA_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDMA_ADDR")
            .field("rxdma_addr", &format_args!("{}", self.rxdma_addr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RXDMA_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn rxdma_addr(&mut self) -> RXDMA_ADDR_W<RXDMA_ADDR_SPEC> {
        RXDMA_ADDR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdma_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXDMA_ADDR_SPEC;
impl crate::RegisterSpec for RXDMA_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdma_addr::R`](R) reader structure"]
impl crate::Readable for RXDMA_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxdma_addr::W`](W) writer structure"]
impl crate::Writable for RXDMA_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXDMA_ADDR to value 0"]
impl crate::Resettable for RXDMA_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
