///Register `LACTHI` reader
pub type R = crate::R<LACTHI_SPEC>;
///Field `HI` reader -
pub type HI_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTHI").field("hi", &self.hi()).finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`lacthi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LACTHI_SPEC;
impl crate::RegisterSpec for LACTHI_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lacthi::R`](R) reader structure
impl crate::Readable for LACTHI_SPEC {}
///`reset()` method sets LACTHI to value 0
impl crate::Resettable for LACTHI_SPEC {
    const RESET_VALUE: u32 = 0;
}
