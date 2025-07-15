#[doc = "Register `STATUS_NEXT` reader"]
pub type R = crate::R<STATUS_NEXT_SPEC>;
#[doc = "Field `STATUS_INTERRUPT_NEXT` reader - Represents the interrupt source status of GPIO0 ~ GPIO7.\\\\ bit0 ~ bit7 are corresponding to GPIO0 ~ 7. Each bit represents:\\\\ 0: Interrupt source status is invalid.\\\\ 1: Interrupt source status is valid.\\\\ The interrupt here can be rising-edge triggered, falling-edge triggered, any edge triggered, or level triggered.\\\\"]
pub type STATUS_INTERRUPT_NEXT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Represents the interrupt source status of GPIO0 ~ GPIO7.\\\\ bit0 ~ bit7 are corresponding to GPIO0 ~ 7. Each bit represents:\\\\ 0: Interrupt source status is invalid.\\\\ 1: Interrupt source status is valid.\\\\ The interrupt here can be rising-edge triggered, falling-edge triggered, any edge triggered, or level triggered.\\\\"]
    #[inline(always)]
    pub fn status_interrupt_next(&self) -> STATUS_INTERRUPT_NEXT_R {
        STATUS_INTERRUPT_NEXT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_NEXT")
            .field("status_interrupt_next", &self.status_interrupt_next())
            .finish()
    }
}
#[doc = "LP GPIO interrupt source register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_NEXT_SPEC;
impl crate::RegisterSpec for STATUS_NEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_next::R`](R) reader structure"]
impl crate::Readable for STATUS_NEXT_SPEC {}
#[doc = "`reset()` method sets STATUS_NEXT to value 0"]
impl crate::Resettable for STATUS_NEXT_SPEC {}
