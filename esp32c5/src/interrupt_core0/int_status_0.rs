#[doc = "Register `INT_STATUS_0` reader"]
pub type R = crate::R<INT_STATUS_0_SPEC>;
#[doc = "Field `INT_STATUS_0` reader - Represents the status of the interrupt sources numbered from .Each bit corresponds to one interrupt source 0:The corresponding interrupt source triggered an interrupt 1:No interrupt triggered"]
pub type INT_STATUS_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the status of the interrupt sources numbered from .Each bit corresponds to one interrupt source 0:The corresponding interrupt source triggered an interrupt 1:No interrupt triggered"]
    #[inline(always)]
    pub fn int_status_0(&self) -> INT_STATUS_0_R {
        INT_STATUS_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS_0")
            .field("int_status_0", &self.int_status_0())
            .finish()
    }
}
#[doc = "Status register for interrupt sources 0 ~ 31\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STATUS_0_SPEC;
impl crate::RegisterSpec for INT_STATUS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status_0::R`](R) reader structure"]
impl crate::Readable for INT_STATUS_0_SPEC {}
#[doc = "`reset()` method sets INT_STATUS_0 to value 0"]
impl crate::Resettable for INT_STATUS_0_SPEC {}
