#[doc = "Register `INTR1_3` reader"]
pub type R = crate::R<INTR1_3_SPEC>;
#[doc = "Field `INT1_3` reader - GPIO interrupt 3 status register for GPIO32-56"]
pub type INT1_3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO interrupt 3 status register for GPIO32-56"]
    #[inline(always)]
    pub fn int1_3(&self) -> INT1_3_R {
        INT1_3_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR1_3")
            .field("int1_3", &self.int1_3())
            .finish()
    }
}
#[doc = "GPIO interrupt 3 status register for GPIO32-56\n\nYou can [`read`](crate::Reg::read) this register and get [`intr1_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR1_3_SPEC;
impl crate::RegisterSpec for INTR1_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr1_3::R`](R) reader structure"]
impl crate::Readable for INTR1_3_SPEC {}
#[doc = "`reset()` method sets INTR1_3 to value 0"]
impl crate::Resettable for INTR1_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
