#[doc = "Register `INTR_2` reader"]
pub type R = crate::R<INTR_2_SPEC>;
#[doc = "Field `INT_2` reader - GPIO interrupt 2 status register for GPIO0-31"]
pub type INT_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO interrupt 2 status register for GPIO0-31"]
    #[inline(always)]
    pub fn int_2(&self) -> INT_2_R {
        INT_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_2")
            .field("int_2", &self.int_2())
            .finish()
    }
}
#[doc = "GPIO interrupt 2 status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_2_SPEC;
impl crate::RegisterSpec for INTR_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_2::R`](R) reader structure"]
impl crate::Readable for INTR_2_SPEC {}
#[doc = "`reset()` method sets INTR_2 to value 0"]
impl crate::Resettable for INTR_2_SPEC {}
