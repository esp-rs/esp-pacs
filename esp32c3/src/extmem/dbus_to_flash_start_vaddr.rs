#[doc = "Register `DBUS_TO_FLASH_START_VADDR` reader"]
pub type R = crate::R<DBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "Register `DBUS_TO_FLASH_START_VADDR` writer"]
pub type W = crate::W<DBUS_TO_FLASH_START_VADDR_SPEC>;
#[doc = "Field `DBUS_TO_FLASH_START_VADDR` reader - The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
pub type DBUS_TO_FLASH_START_VADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DBUS_TO_FLASH_START_VADDR` writer - The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
pub type DBUS_TO_FLASH_START_VADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
    #[inline(always)]
    pub fn dbus_to_flash_start_vaddr(&self) -> DBUS_TO_FLASH_START_VADDR_R {
        DBUS_TO_FLASH_START_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBUS_TO_FLASH_START_VADDR")
            .field(
                "dbus_to_flash_start_vaddr",
                &format_args!("{}", self.dbus_to_flash_start_vaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBUS_TO_FLASH_START_VADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
    #[inline(always)]
    #[must_use]
    pub fn dbus_to_flash_start_vaddr(
        &mut self,
    ) -> DBUS_TO_FLASH_START_VADDR_W<DBUS_TO_FLASH_START_VADDR_SPEC> {
        DBUS_TO_FLASH_START_VADDR_W::new(self, 0)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbus_to_flash_start_vaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbus_to_flash_start_vaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBUS_TO_FLASH_START_VADDR_SPEC;
impl crate::RegisterSpec for DBUS_TO_FLASH_START_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbus_to_flash_start_vaddr::R`](R) reader structure"]
impl crate::Readable for DBUS_TO_FLASH_START_VADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbus_to_flash_start_vaddr::W`](W) writer structure"]
impl crate::Writable for DBUS_TO_FLASH_START_VADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBUS_TO_FLASH_START_VADDR to value 0x3c00_0000"]
impl crate::Resettable for DBUS_TO_FLASH_START_VADDR_SPEC {
    const RESET_VALUE: u32 = 0x3c00_0000;
}
