///Register `INTR_0` reader
pub type R = crate::R<INTR_0_SPEC>;
///Field `INT_0` reader - GPIO interrupt 0 status register for GPIO0-31
pub type INT_0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - GPIO interrupt 0 status register for GPIO0-31
    #[inline(always)]
    pub fn int_0(&self) -> INT_0_R {
        INT_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_0")
            .field("int_0", &self.int_0())
            .finish()
    }
}
/**GPIO interrupt 0 status register for GPIO0-31

You can [`read`](crate::generic::Reg::read) this register and get [`intr_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTR_0_SPEC;
impl crate::RegisterSpec for INTR_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`intr_0::R`](R) reader structure
impl crate::Readable for INTR_0_SPEC {}
///`reset()` method sets INTR_0 to value 0
impl crate::Resettable for INTR_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
