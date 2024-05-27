///Register `RC_STATUS2` reader
pub type R = crate::R<RC_STATUS2_SPEC>;
///Field `FRAME_QP_SUM` reader - Represents all MB actual luma QP sum value of one frame.
pub type FRAME_QP_SUM_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:18 - Represents all MB actual luma QP sum value of one frame.
    #[inline(always)]
    pub fn frame_qp_sum(&self) -> FRAME_QP_SUM_R {
        FRAME_QP_SUM_R::new(self.bits & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RC_STATUS2")
            .field("frame_qp_sum", &self.frame_qp_sum())
            .finish()
    }
}
/**Rate control status register2.

You can [`read`](crate::generic::Reg::read) this register and get [`rc_status2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RC_STATUS2_SPEC;
impl crate::RegisterSpec for RC_STATUS2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rc_status2::R`](R) reader structure
impl crate::Readable for RC_STATUS2_SPEC {}
///`reset()` method sets RC_STATUS2 to value 0
impl crate::Resettable for RC_STATUS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
