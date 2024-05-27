///Register `PRO_DCACHE_AUTOLOAD_SECTION0_ADDR` reader
pub type R = crate::R<PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC>;
///Register `PRO_DCACHE_AUTOLOAD_SECTION0_ADDR` writer
pub type W = crate::W<PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC>;
///Field `PRO_DCACHE_AUTOLOAD_SCT0_ADDR` reader - The bits are used to configure the start virtual address of the first section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct0_ena.
pub type PRO_DCACHE_AUTOLOAD_SCT0_ADDR_R = crate::FieldReader<u32>;
///Field `PRO_DCACHE_AUTOLOAD_SCT0_ADDR` writer - The bits are used to configure the start virtual address of the first section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct0_ena.
pub type PRO_DCACHE_AUTOLOAD_SCT0_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The bits are used to configure the start virtual address of the first section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct0_ena.
    #[inline(always)]
    pub fn pro_dcache_autoload_sct0_addr(&self) -> PRO_DCACHE_AUTOLOAD_SCT0_ADDR_R {
        PRO_DCACHE_AUTOLOAD_SCT0_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_AUTOLOAD_SECTION0_ADDR")
            .field(
                "pro_dcache_autoload_sct0_addr",
                &self.pro_dcache_autoload_sct0_addr(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:31 - The bits are used to configure the start virtual address of the first section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct0_ena.
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_autoload_sct0_addr(
        &mut self,
    ) -> PRO_DCACHE_AUTOLOAD_SCT0_ADDR_W<PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC> {
        PRO_DCACHE_AUTOLOAD_SCT0_ADDR_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_autoload_section0_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dcache_autoload_section0_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_dcache_autoload_section0_addr::R`](R) reader structure
impl crate::Readable for PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC {}
///`write(|w| ..)` method takes [`pro_dcache_autoload_section0_addr::W`](W) writer structure
impl crate::Writable for PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_DCACHE_AUTOLOAD_SECTION0_ADDR to value 0
impl crate::Resettable for PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
