///Register `ICACHE_PRELOCK_SCT_SIZE` reader
pub type R = crate::R<ICACHE_PRELOCK_SCT_SIZE_SPEC>;
///Register `ICACHE_PRELOCK_SCT_SIZE` writer
pub type W = crate::W<ICACHE_PRELOCK_SCT_SIZE_SPEC>;
///Field `ICACHE_PRELOCK_SCT1_SIZE` reader - The bits are used to configure the second length of data locking, which is combined with ICACHE_PRELOCK_SCT1_ADDR_REG
pub type ICACHE_PRELOCK_SCT1_SIZE_R = crate::FieldReader<u16>;
///Field `ICACHE_PRELOCK_SCT1_SIZE` writer - The bits are used to configure the second length of data locking, which is combined with ICACHE_PRELOCK_SCT1_ADDR_REG
pub type ICACHE_PRELOCK_SCT1_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ICACHE_PRELOCK_SCT0_SIZE` reader - The bits are used to configure the first length of data locking, which is combined with ICACHE_PRELOCK_SCT0_ADDR_REG
pub type ICACHE_PRELOCK_SCT0_SIZE_R = crate::FieldReader<u16>;
///Field `ICACHE_PRELOCK_SCT0_SIZE` writer - The bits are used to configure the first length of data locking, which is combined with ICACHE_PRELOCK_SCT0_ADDR_REG
pub type ICACHE_PRELOCK_SCT0_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - The bits are used to configure the second length of data locking, which is combined with ICACHE_PRELOCK_SCT1_ADDR_REG
    #[inline(always)]
    pub fn icache_prelock_sct1_size(&self) -> ICACHE_PRELOCK_SCT1_SIZE_R {
        ICACHE_PRELOCK_SCT1_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - The bits are used to configure the first length of data locking, which is combined with ICACHE_PRELOCK_SCT0_ADDR_REG
    #[inline(always)]
    pub fn icache_prelock_sct0_size(&self) -> ICACHE_PRELOCK_SCT0_SIZE_R {
        ICACHE_PRELOCK_SCT0_SIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_PRELOCK_SCT_SIZE")
            .field("icache_prelock_sct1_size", &self.icache_prelock_sct1_size())
            .field("icache_prelock_sct0_size", &self.icache_prelock_sct0_size())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - The bits are used to configure the second length of data locking, which is combined with ICACHE_PRELOCK_SCT1_ADDR_REG
    #[inline(always)]
    #[must_use]
    pub fn icache_prelock_sct1_size(
        &mut self,
    ) -> ICACHE_PRELOCK_SCT1_SIZE_W<ICACHE_PRELOCK_SCT_SIZE_SPEC> {
        ICACHE_PRELOCK_SCT1_SIZE_W::new(self, 0)
    }
    ///Bits 16:31 - The bits are used to configure the first length of data locking, which is combined with ICACHE_PRELOCK_SCT0_ADDR_REG
    #[inline(always)]
    #[must_use]
    pub fn icache_prelock_sct0_size(
        &mut self,
    ) -> ICACHE_PRELOCK_SCT0_SIZE_W<ICACHE_PRELOCK_SCT_SIZE_SPEC> {
        ICACHE_PRELOCK_SCT0_SIZE_W::new(self, 16)
    }
}
/**This description will be updated in the near future.

You can [`read`](crate::generic::Reg::read) this register and get [`icache_prelock_sct_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icache_prelock_sct_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ICACHE_PRELOCK_SCT_SIZE_SPEC;
impl crate::RegisterSpec for ICACHE_PRELOCK_SCT_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`icache_prelock_sct_size::R`](R) reader structure
impl crate::Readable for ICACHE_PRELOCK_SCT_SIZE_SPEC {}
///`write(|w| ..)` method takes [`icache_prelock_sct_size::W`](W) writer structure
impl crate::Writable for ICACHE_PRELOCK_SCT_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ICACHE_PRELOCK_SCT_SIZE to value 0
impl crate::Resettable for ICACHE_PRELOCK_SCT_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
