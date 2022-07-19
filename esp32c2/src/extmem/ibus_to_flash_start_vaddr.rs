#[doc = "Register `IBUS_TO_FLASH_START_VADDR` reader"]
pub struct R(crate::R<IBUS_TO_FLASH_START_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBUS_TO_FLASH_START_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBUS_TO_FLASH_START_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBUS_TO_FLASH_START_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBUS_TO_FLASH_START_VADDR` writer"]
pub struct W(crate::W<IBUS_TO_FLASH_START_VADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBUS_TO_FLASH_START_VADDR_SPEC>;
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
impl From<crate::W<IBUS_TO_FLASH_START_VADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBUS_TO_FLASH_START_VADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBUS_TO_FLASH_START_VADDR` reader - The bits are used to configure the start virtual address of ibus to access flash. The register is used to give constraints to ibus access counter."]
pub type IBUS_TO_FLASH_START_VADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IBUS_TO_FLASH_START_VADDR` writer - The bits are used to configure the start virtual address of ibus to access flash. The register is used to give constraints to ibus access counter."]
pub type IBUS_TO_FLASH_START_VADDR_W<'a> =
    crate::FieldWriter<'a, u32, IBUS_TO_FLASH_START_VADDR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of ibus to access flash. The register is used to give constraints to ibus access counter."]
    #[inline(always)]
    pub fn ibus_to_flash_start_vaddr(&self) -> IBUS_TO_FLASH_START_VADDR_R {
        IBUS_TO_FLASH_START_VADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of ibus to access flash. The register is used to give constraints to ibus access counter."]
    #[inline(always)]
    pub fn ibus_to_flash_start_vaddr(&mut self) -> IBUS_TO_FLASH_START_VADDR_W {
        IBUS_TO_FLASH_START_VADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibus_to_flash_start_vaddr](index.html) module"]
pub struct IBUS_TO_FLASH_START_VADDR_SPEC;
impl crate::RegisterSpec for IBUS_TO_FLASH_START_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibus_to_flash_start_vaddr::R](R) reader structure"]
impl crate::Readable for IBUS_TO_FLASH_START_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibus_to_flash_start_vaddr::W](W) writer structure"]
impl crate::Writable for IBUS_TO_FLASH_START_VADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IBUS_TO_FLASH_START_VADDR to value 0x4200_0000"]
impl crate::Resettable for IBUS_TO_FLASH_START_VADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4200_0000
    }
}
