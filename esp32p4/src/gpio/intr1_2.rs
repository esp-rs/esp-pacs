#[doc = "Register `INTR1_2` reader"]
pub type R = crate::R<INTR1_2_SPEC>;
#[doc = "Field `INT1_2` reader - GPIO interrupt 2 status register for GPIO32-56"]
pub type INT1_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO interrupt 2 status register for GPIO32-56"]
    #[inline(always)]
    pub fn int1_2(&self) -> INT1_2_R {
        INT1_2_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR1_2")
            .field("int1_2", &self.int1_2())
            .finish()
    }
}
#[doc = "GPIO interrupt 2 status register for GPIO32-56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr1_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR1_2_SPEC;
impl crate::RegisterSpec for INTR1_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr1_2::R`](R) reader structure"]
impl crate::Readable for INTR1_2_SPEC {}
#[doc = "`reset()` method sets INTR1_2 to value 0"]
impl crate::Resettable for INTR1_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
