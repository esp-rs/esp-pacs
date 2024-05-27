///Register `L1_ICACHE1_PRELOAD_SIZE` reader
pub type R = crate::R<L1_ICACHE1_PRELOAD_SIZE_SPEC>;
///Register `L1_ICACHE1_PRELOAD_SIZE` writer
pub type W = crate::W<L1_ICACHE1_PRELOAD_SIZE_SPEC>;
///Field `L1_ICACHE1_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_ADDR_REG
pub type L1_ICACHE1_PRELOAD_SIZE_R = crate::FieldReader<u16>;
///Field `L1_ICACHE1_PRELOAD_SIZE` writer - Those bits are used to configure the size of the first section of prelock on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_ADDR_REG
pub type L1_ICACHE1_PRELOAD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_ADDR_REG
    #[inline(always)]
    pub fn l1_icache1_preload_size(&self) -> L1_ICACHE1_PRELOAD_SIZE_R {
        L1_ICACHE1_PRELOAD_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE1_PRELOAD_SIZE")
            .field("l1_icache1_preload_size", &self.l1_icache1_preload_size())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-ICache1, which should be used together with L1_ICACHE1_PRELOAD_ADDR_REG
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_preload_size(
        &mut self,
    ) -> L1_ICACHE1_PRELOAD_SIZE_W<L1_ICACHE1_PRELOAD_SIZE_SPEC> {
        L1_ICACHE1_PRELOAD_SIZE_W::new(self, 0)
    }
}
/**L1 instruction Cache 1 preload size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_preload_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_preload_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_ICACHE1_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for L1_ICACHE1_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_icache1_preload_size::R`](R) reader structure
impl crate::Readable for L1_ICACHE1_PRELOAD_SIZE_SPEC {}
///`write(|w| ..)` method takes [`l1_icache1_preload_size::W`](W) writer structure
impl crate::Writable for L1_ICACHE1_PRELOAD_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_ICACHE1_PRELOAD_SIZE to value 0
impl crate::Resettable for L1_ICACHE1_PRELOAD_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
