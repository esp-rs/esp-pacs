#[doc = "Register `INT_STATUS_REG_1` reader"]
pub type R = crate::R<INT_STATUS_REG_1_SPEC>;
#[doc = "Field `INT_STATUS_1` reader - Status register for interrupt sources 32~63 mapping register"]
pub type INT_STATUS_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status register for interrupt sources 32~63 mapping register"]
    #[inline(always)]
    pub fn int_status_1(&self) -> INT_STATUS_1_R {
        INT_STATUS_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS_REG_1")
            .field("int_status_1", &self.int_status_1())
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status_reg_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STATUS_REG_1_SPEC;
impl crate::RegisterSpec for INT_STATUS_REG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status_reg_1::R`](R) reader structure"]
impl crate::Readable for INT_STATUS_REG_1_SPEC {}
#[doc = "`reset()` method sets INT_STATUS_REG_1 to value 0"]
impl crate::Resettable for INT_STATUS_REG_1_SPEC {}
