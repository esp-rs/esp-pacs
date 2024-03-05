#[doc = "Register `IBUS_TO_FLASH_END_VADDR` reader"]
pub type R = crate::R<IBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "Register `IBUS_TO_FLASH_END_VADDR` writer"]
pub type W = crate::W<IBUS_TO_FLASH_END_VADDR_SPEC>;
#[doc = "Field `IBUS_TO_FLASH_END_VADDR` reader - The bits are used to configure the end virtual address of ibus to access flash. The register is used to give constraints to ibus access counter."]
pub type IBUS_TO_FLASH_END_VADDR_R = crate::FieldReader<u32>;
#[doc = "Field `IBUS_TO_FLASH_END_VADDR` writer - The bits are used to configure the end virtual address of ibus to access flash. The register is used to give constraints to ibus access counter."]
pub type IBUS_TO_FLASH_END_VADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the end virtual address of ibus to access flash. The register is used to give constraints to ibus access counter."]
    #[inline(always)]
    pub fn ibus_to_flash_end_vaddr(&self) -> IBUS_TO_FLASH_END_VADDR_R {
        IBUS_TO_FLASH_END_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS_TO_FLASH_END_VADDR")
            .field(
                "ibus_to_flash_end_vaddr",
                &format_args!("{}", self.ibus_to_flash_end_vaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IBUS_TO_FLASH_END_VADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the end virtual address of ibus to access flash. The register is used to give constraints to ibus access counter."]
    #[inline(always)]
    #[must_use]
    pub fn ibus_to_flash_end_vaddr(
        &mut self,
    ) -> IBUS_TO_FLASH_END_VADDR_W<IBUS_TO_FLASH_END_VADDR_SPEC> {
        IBUS_TO_FLASH_END_VADDR_W::new(self, 0)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibus_to_flash_end_vaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibus_to_flash_end_vaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBUS_TO_FLASH_END_VADDR_SPEC;
impl crate::RegisterSpec for IBUS_TO_FLASH_END_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibus_to_flash_end_vaddr::R`](R) reader structure"]
impl crate::Readable for IBUS_TO_FLASH_END_VADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ibus_to_flash_end_vaddr::W`](W) writer structure"]
impl crate::Writable for IBUS_TO_FLASH_END_VADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IBUS_TO_FLASH_END_VADDR to value 0x427f_ffff"]
impl crate::Resettable for IBUS_TO_FLASH_END_VADDR_SPEC {
    const RESET_VALUE: u32 = 0x427f_ffff;
}
