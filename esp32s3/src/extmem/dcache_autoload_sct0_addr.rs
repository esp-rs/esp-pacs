///Register `DCACHE_AUTOLOAD_SCT0_ADDR` reader
pub type R = crate::R<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
///Register `DCACHE_AUTOLOAD_SCT0_ADDR` writer
pub type W = crate::W<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC>;
///Field `DCACHE_AUTOLOAD_SCT0_ADDR` reader - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena.
pub type DCACHE_AUTOLOAD_SCT0_ADDR_R = crate::FieldReader<u32>;
///Field `DCACHE_AUTOLOAD_SCT0_ADDR` writer - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena.
pub type DCACHE_AUTOLOAD_SCT0_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena.
    #[inline(always)]
    pub fn dcache_autoload_sct0_addr(&self) -> DCACHE_AUTOLOAD_SCT0_ADDR_R {
        DCACHE_AUTOLOAD_SCT0_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_AUTOLOAD_SCT0_ADDR")
            .field(
                "dcache_autoload_sct0_addr",
                &self.dcache_autoload_sct0_addr(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:31 - The bits are used to configure the start virtual address of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena.
    #[inline(always)]
    #[must_use]
    pub fn dcache_autoload_sct0_addr(
        &mut self,
    ) -> DCACHE_AUTOLOAD_SCT0_ADDR_W<DCACHE_AUTOLOAD_SCT0_ADDR_SPEC> {
        DCACHE_AUTOLOAD_SCT0_ADDR_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_autoload_sct0_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_autoload_sct0_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_AUTOLOAD_SCT0_ADDR_SPEC;
impl crate::RegisterSpec for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dcache_autoload_sct0_addr::R`](R) reader structure
impl crate::Readable for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {}
///`write(|w| ..)` method takes [`dcache_autoload_sct0_addr::W`](W) writer structure
impl crate::Writable for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCACHE_AUTOLOAD_SCT0_ADDR to value 0
impl crate::Resettable for DCACHE_AUTOLOAD_SCT0_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
