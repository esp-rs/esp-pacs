///Register `LACTLO` reader
pub type R = crate::R<LACTLO_SPEC>;
///Field `LO` reader - Reserved.
pub type LO_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Reserved.
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTLO").field("lo", &self.lo()).finish()
    }
}
/**LACT low register

You can [`read`](crate::generic::Reg::read) this register and get [`lactlo::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LACTLO_SPEC;
impl crate::RegisterSpec for LACTLO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lactlo::R`](R) reader structure
impl crate::Readable for LACTLO_SPEC {}
///`reset()` method sets LACTLO to value 0
impl crate::Resettable for LACTLO_SPEC {
    const RESET_VALUE: u32 = 0;
}
