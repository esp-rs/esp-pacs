///Register `RXDMA_ADDR` reader
pub type R = crate::R<RXDMA_ADDR_SPEC>;
///Register `RXDMA_ADDR` writer
pub type W = crate::W<RXDMA_ADDR_SPEC>;
///Field `RXDMA_ADDR` reader -
pub type RXDMA_ADDR_R = crate::FieldReader<u32>;
///Field `RXDMA_ADDR` writer -
pub type RXDMA_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn rxdma_addr(&self) -> RXDMA_ADDR_R {
        RXDMA_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXDMA_ADDR")
            .field("rxdma_addr", &self.rxdma_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn rxdma_addr(&mut self) -> RXDMA_ADDR_W<RXDMA_ADDR_SPEC> {
        RXDMA_ADDR_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`rxdma_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RXDMA_ADDR_SPEC;
impl crate::RegisterSpec for RXDMA_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rxdma_addr::R`](R) reader structure
impl crate::Readable for RXDMA_ADDR_SPEC {}
///`write(|w| ..)` method takes [`rxdma_addr::W`](W) writer structure
impl crate::Writable for RXDMA_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RXDMA_ADDR to value 0
impl crate::Resettable for RXDMA_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
