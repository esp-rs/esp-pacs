///Register `AF_LUM_B` reader
pub type R = crate::R<AF_LUM_B_SPEC>;
///Field `AF_LUMB` reader - this field represents the result of accumulation of pix light of focus window b
pub type AF_LUMB_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:27 - this field represents the result of accumulation of pix light of focus window b
    #[inline(always)]
    pub fn af_lumb(&self) -> AF_LUMB_R {
        AF_LUMB_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_LUM_B")
            .field("af_lumb", &self.af_lumb())
            .finish()
    }
}
/**result of lum of af window b

You can [`read`](crate::generic::Reg::read) this register and get [`af_lum_b::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AF_LUM_B_SPEC;
impl crate::RegisterSpec for AF_LUM_B_SPEC {
    type Ux = u32;
}
///`read()` method returns [`af_lum_b::R`](R) reader structure
impl crate::Readable for AF_LUM_B_SPEC {}
///`reset()` method sets AF_LUM_B to value 0
impl crate::Resettable for AF_LUM_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
