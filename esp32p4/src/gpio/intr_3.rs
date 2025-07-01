#[doc = "Register `INTR_3` reader"]
pub type R = crate::R<INTR_3_SPEC>;
#[doc = "Field `INT_3` reader - GPIO interrupt 3 status register for GPIO0-31"]
pub type INT_3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt 3 status register for GPIO0-31"]
    #[inline(always)]
    pub fn int_3(&self) -> INT_3_R {
        INT_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_3")
            .field("int_3", &self.int_3())
            .finish()
    }
}
#[doc = "GPIO interrupt 3 status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_3_SPEC;
impl crate::RegisterSpec for INTR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_3::R`](R) reader structure"]
impl crate::Readable for INTR_3_SPEC {}
#[doc = "`reset()` method sets INTR_3 to value 0"]
impl crate::Resettable for INTR_3_SPEC {}
