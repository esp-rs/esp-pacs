#[doc = "Register `DBUS_TO_FLASH_START_VADDR` reader"]
pub struct R(crate::R<DBUS_TO_FLASH_START_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBUS_TO_FLASH_START_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBUS_TO_FLASH_START_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBUS_TO_FLASH_START_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBUS_TO_FLASH_START_VADDR` writer"]
pub struct W(crate::W<DBUS_TO_FLASH_START_VADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBUS_TO_FLASH_START_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DBUS_TO_FLASH_START_VADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBUS_TO_FLASH_START_VADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBUS_TO_FLASH_START_VADDR` reader - The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
pub type DBUS_TO_FLASH_START_VADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DBUS_TO_FLASH_START_VADDR` writer - The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
pub type DBUS_TO_FLASH_START_VADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, DBUS_TO_FLASH_START_VADDR_SPEC, 32, O, u32, u32>;
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of dbus to access flash. The register is used to give constraints to dbus access counter."]
    #[inline(always)]
    #[must_use]
    pub fn dbus_to_flash_start_vaddr(&mut self) -> DBUS_TO_FLASH_START_VADDR_W<0> {
        DBUS_TO_FLASH_START_VADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbus_to_flash_start_vaddr](index.html) module"]
pub struct DBUS_TO_FLASH_START_VADDR_SPEC;
impl crate::RegisterSpec for DBUS_TO_FLASH_START_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbus_to_flash_start_vaddr::R](R) reader structure"]
impl crate::Readable for DBUS_TO_FLASH_START_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbus_to_flash_start_vaddr::W](W) writer structure"]
impl crate::Writable for DBUS_TO_FLASH_START_VADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBUS_TO_FLASH_START_VADDR to value 0x3c00_0000"]
impl crate::Resettable for DBUS_TO_FLASH_START_VADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3c00_0000;
}
