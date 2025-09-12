#[doc = "Register `PRO_DCACHE_AUTOLOAD_SECTION0_ADDR` reader"]
pub type R = crate::R<PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC>;
#[doc = "Register `PRO_DCACHE_AUTOLOAD_SECTION0_ADDR` writer"]
pub type W = crate::W<PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT0_ADDR` reader - The bits are used to configure the start virtual address of the first section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct0_ena."]
pub type PRO_DCACHE_AUTOLOAD_SCT0_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_DCACHE_AUTOLOAD_SCT0_ADDR` writer - The bits are used to configure the start virtual address of the first section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct0_ena."]
pub type PRO_DCACHE_AUTOLOAD_SCT0_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the first section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct0_ena."]
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
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the first section for conditional pre-load operation. It should be combined with pro_dcache_autoload_sct0_ena."]
    #[inline(always)]
    pub fn pro_dcache_autoload_sct0_addr(
        &mut self,
    ) -> PRO_DCACHE_AUTOLOAD_SCT0_ADDR_W<'_, PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC> {
        PRO_DCACHE_AUTOLOAD_SCT0_ADDR_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_autoload_section0_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_autoload_section0_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_autoload_section0_addr::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dcache_autoload_section0_addr::W`](W) writer structure"]
impl crate::Writable for PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_DCACHE_AUTOLOAD_SECTION0_ADDR to value 0"]
impl crate::Resettable for PRO_DCACHE_AUTOLOAD_SECTION0_ADDR_SPEC {}
