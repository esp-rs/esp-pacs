///Register `A_NO_ROI_REGION_QP_OFFSET` reader
pub type R = crate::R<A_NO_ROI_REGION_QP_OFFSET_SPEC>;
///Register `A_NO_ROI_REGION_QP_OFFSET` writer
pub type W = crate::W<A_NO_ROI_REGION_QP_OFFSET_SPEC>;
///Field `A_NO_ROI_REGION_QP` reader - Configure H264 no region qp in video A, delta qp.
pub type A_NO_ROI_REGION_QP_R = crate::FieldReader;
///Field `A_NO_ROI_REGION_QP` writer - Configure H264 no region qp in video A, delta qp.
pub type A_NO_ROI_REGION_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Configure H264 no region qp in video A, delta qp.
    #[inline(always)]
    pub fn a_no_roi_region_qp(&self) -> A_NO_ROI_REGION_QP_R {
        A_NO_ROI_REGION_QP_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A_NO_ROI_REGION_QP_OFFSET")
            .field("a_no_roi_region_qp", &self.a_no_roi_region_qp())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Configure H264 no region qp in video A, delta qp.
    #[inline(always)]
    #[must_use]
    pub fn a_no_roi_region_qp(&mut self) -> A_NO_ROI_REGION_QP_W<A_NO_ROI_REGION_QP_OFFSET_SPEC> {
        A_NO_ROI_REGION_QP_W::new(self, 0)
    }
}
/**Video A H264 no roi region QP register.

You can [`read`](crate::generic::Reg::read) this register and get [`a_no_roi_region_qp_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_no_roi_region_qp_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct A_NO_ROI_REGION_QP_OFFSET_SPEC;
impl crate::RegisterSpec for A_NO_ROI_REGION_QP_OFFSET_SPEC {
    type Ux = u32;
}
///`read()` method returns [`a_no_roi_region_qp_offset::R`](R) reader structure
impl crate::Readable for A_NO_ROI_REGION_QP_OFFSET_SPEC {}
///`write(|w| ..)` method takes [`a_no_roi_region_qp_offset::W`](W) writer structure
impl crate::Writable for A_NO_ROI_REGION_QP_OFFSET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets A_NO_ROI_REGION_QP_OFFSET to value 0
impl crate::Resettable for A_NO_ROI_REGION_QP_OFFSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
