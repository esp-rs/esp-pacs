#[doc = "Register `INTR1_0` reader"]
pub type R = crate::R<INTR1_0_SPEC>;
#[doc = "Field `INT1_0` reader - GPIO interrupt 0 status register for GPIO32-56"]
pub type INT1_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - GPIO interrupt 0 status register for GPIO32-56"]
    #[inline(always)]
    pub fn int1_0(&self) -> INT1_0_R {
        INT1_0_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR1_0")
            .field("int1_0", &format_args!("{}", self.int1_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR1_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "GPIO interrupt 0 status register for GPIO32-56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr1_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR1_0_SPEC;
impl crate::RegisterSpec for INTR1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr1_0::R`](R) reader structure"]
impl crate::Readable for INTR1_0_SPEC {}
#[doc = "`reset()` method sets INTR1_0 to value 0"]
impl crate::Resettable for INTR1_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
