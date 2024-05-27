///Register `HI` reader
pub type R = crate::R<HI_SPEC>;
///Field `HI` reader - After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter of timer %s can be read here.
pub type HI_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:21 - After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter of timer %s can be read here.
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HI").field("hi", &self.hi()).finish()
    }
}
/**Timer %s current value, high 22 bits

You can [`read`](crate::generic::Reg::read) this register and get [`hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HI_SPEC;
impl crate::RegisterSpec for HI_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hi::R`](R) reader structure
impl crate::Readable for HI_SPEC {}
///`reset()` method sets HI to value 0
impl crate::Resettable for HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
