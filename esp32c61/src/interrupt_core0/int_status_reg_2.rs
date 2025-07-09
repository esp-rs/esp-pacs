#[doc = "Register `INT_STATUS_REG_2` reader"]
pub type R = crate::R<INT_STATUS_REG_2_SPEC>;
#[doc = "Field `INT_STATUS_2` reader - Represents the status of the interrupt sources within interrupt-index-range 64 ~ 65. Each bit corresponds to one interrupt source 0:The corresponding interrupt source triggered an interrupt 1:No interrupt triggered"]
pub type INT_STATUS_2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Represents the status of the interrupt sources within interrupt-index-range 64 ~ 65. Each bit corresponds to one interrupt source 0:The corresponding interrupt source triggered an interrupt 1:No interrupt triggered"]
    #[inline(always)]
    pub fn int_status_2(&self) -> INT_STATUS_2_R {
        INT_STATUS_2_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS_REG_2")
            .field("int_status_2", &self.int_status_2())
            .finish()
    }
}
#[doc = "Status register for interrupt sources 64 ~ 65\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status_reg_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STATUS_REG_2_SPEC;
impl crate::RegisterSpec for INT_STATUS_REG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status_reg_2::R`](R) reader structure"]
impl crate::Readable for INT_STATUS_REG_2_SPEC {}
#[doc = "`reset()` method sets INT_STATUS_REG_2 to value 0"]
impl crate::Resettable for INT_STATUS_REG_2_SPEC {}
