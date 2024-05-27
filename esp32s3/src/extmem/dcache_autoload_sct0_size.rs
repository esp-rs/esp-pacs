///Register `DCACHE_AUTOLOAD_SCT0_SIZE` reader
pub type R = crate::R<DCACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
///Register `DCACHE_AUTOLOAD_SCT0_SIZE` writer
pub type W = crate::W<DCACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
///Field `DCACHE_AUTOLOAD_SCT0_SIZE` reader - The bits are used to configure the length of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena.
pub type DCACHE_AUTOLOAD_SCT0_SIZE_R = crate::FieldReader<u32>;
///Field `DCACHE_AUTOLOAD_SCT0_SIZE` writer - The bits are used to configure the length of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena.
pub type DCACHE_AUTOLOAD_SCT0_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    ///Bits 0:26 - The bits are used to configure the length of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena.
    #[inline(always)]
    pub fn dcache_autoload_sct0_size(&self) -> DCACHE_AUTOLOAD_SCT0_SIZE_R {
        DCACHE_AUTOLOAD_SCT0_SIZE_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_AUTOLOAD_SCT0_SIZE")
            .field(
                "dcache_autoload_sct0_size",
                &self.dcache_autoload_sct0_size(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:26 - The bits are used to configure the length of the first section for autoload operation. It should be combined with dcache_autoload_sct0_ena.
    #[inline(always)]
    #[must_use]
    pub fn dcache_autoload_sct0_size(
        &mut self,
    ) -> DCACHE_AUTOLOAD_SCT0_SIZE_W<DCACHE_AUTOLOAD_SCT0_SIZE_SPEC> {
        DCACHE_AUTOLOAD_SCT0_SIZE_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_autoload_sct0_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_autoload_sct0_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_AUTOLOAD_SCT0_SIZE_SPEC;
impl crate::RegisterSpec for DCACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dcache_autoload_sct0_size::R`](R) reader structure
impl crate::Readable for DCACHE_AUTOLOAD_SCT0_SIZE_SPEC {}
///`write(|w| ..)` method takes [`dcache_autoload_sct0_size::W`](W) writer structure
impl crate::Writable for DCACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCACHE_AUTOLOAD_SCT0_SIZE to value 0
impl crate::Resettable for DCACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
