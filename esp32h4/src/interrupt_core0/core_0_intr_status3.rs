#[doc = "Register `CORE_0_INTR_STATUS3` reader"]
pub type R = crate::R<CORE_0_INTR_STATUS3_SPEC>;
#[doc = "Field `INT_STATUS_3` reader - Represents the status of the interrupt sources within interrupt-index-range 96 ~ 97. Each bit corresponds to one interrupt source 0:The corresponding interrupt source triggered an interrupt 1:No interrupt triggered"]
pub type INT_STATUS_3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents the status of the interrupt sources within interrupt-index-range 96 ~ 97. Each bit corresponds to one interrupt source 0:The corresponding interrupt source triggered an interrupt 1:No interrupt triggered"]
    #[inline(always)]
    pub fn int_status_3(&self) -> INT_STATUS_3_R {
        INT_STATUS_3_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_INTR_STATUS3")
            .field("int_status_3", &self.int_status_3())
            .finish()
    }
}
#[doc = "Status register for interrupt sources 96 ~ 97\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_status3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_INTR_STATUS3_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_STATUS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_intr_status3::R`](R) reader structure"]
impl crate::Readable for CORE_0_INTR_STATUS3_SPEC {}
#[doc = "`reset()` method sets CORE_0_INTR_STATUS3 to value 0"]
impl crate::Resettable for CORE_0_INTR_STATUS3_SPEC {}
