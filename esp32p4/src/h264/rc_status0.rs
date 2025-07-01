#[doc = "Register `RC_STATUS0` reader"]
pub type R = crate::R<RC_STATUS0_SPEC>;
#[doc = "Field `FRAME_MAD_SUM` reader - Represents all MB actual MAD sum value of one frame."]
pub type FRAME_MAD_SUM_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:20 - Represents all MB actual MAD sum value of one frame."]
    #[inline(always)]
    pub fn frame_mad_sum(&self) -> FRAME_MAD_SUM_R {
        FRAME_MAD_SUM_R::new(self.bits & 0x001f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RC_STATUS0")
            .field("frame_mad_sum", &self.frame_mad_sum())
            .finish()
    }
}
#[doc = "Rate control status register0.\n\nYou can [`read`](crate::Reg::read) this register and get [`rc_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC_STATUS0_SPEC;
impl crate::RegisterSpec for RC_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc_status0::R`](R) reader structure"]
impl crate::Readable for RC_STATUS0_SPEC {}
#[doc = "`reset()` method sets RC_STATUS0 to value 0"]
impl crate::Resettable for RC_STATUS0_SPEC {}
