///Register `INTR1_1` reader
pub type R = crate::R<INTR1_1_SPEC>;
///Field `INT1_1` reader - GPIO interrupt 1 status register for GPIO32-56
pub type INT1_1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:24 - GPIO interrupt 1 status register for GPIO32-56
    #[inline(always)]
    pub fn int1_1(&self) -> INT1_1_R {
        INT1_1_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR1_1")
            .field("int1_1", &self.int1_1())
            .finish()
    }
}
/**GPIO interrupt 1 status register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`intr1_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTR1_1_SPEC;
impl crate::RegisterSpec for INTR1_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`intr1_1::R`](R) reader structure
impl crate::Readable for INTR1_1_SPEC {}
///`reset()` method sets INTR1_1 to value 0
impl crate::Resettable for INTR1_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
