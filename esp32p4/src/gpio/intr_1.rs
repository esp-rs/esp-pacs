///Register `INTR_1` reader
pub type R = crate::R<INTR_1_SPEC>;
///Field `INT_1` reader - GPIO interrupt 1 status register for GPIO0-31
pub type INT_1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - GPIO interrupt 1 status register for GPIO0-31
    #[inline(always)]
    pub fn int_1(&self) -> INT_1_R {
        INT_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_1")
            .field("int_1", &self.int_1())
            .finish()
    }
}
/**GPIO interrupt 1 status register for GPIO0-31

You can [`read`](crate::generic::Reg::read) this register and get [`intr_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTR_1_SPEC;
impl crate::RegisterSpec for INTR_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`intr_1::R`](R) reader structure
impl crate::Readable for INTR_1_SPEC {}
///`reset()` method sets INTR_1 to value 0
impl crate::Resettable for INTR_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
