#[doc = "Register `B_NO_ROI_REGION_QP_OFFSET` reader"]
pub type R = crate::R<B_NO_ROI_REGION_QP_OFFSET_SPEC>;
#[doc = "Register `B_NO_ROI_REGION_QP_OFFSET` writer"]
pub type W = crate::W<B_NO_ROI_REGION_QP_OFFSET_SPEC>;
#[doc = "Field `B_NO_ROI_REGION_QP` reader - Configure H264 no region qp in video B, delta qp."]
pub type B_NO_ROI_REGION_QP_R = crate::FieldReader;
#[doc = "Field `B_NO_ROI_REGION_QP` writer - Configure H264 no region qp in video B, delta qp."]
pub type B_NO_ROI_REGION_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configure H264 no region qp in video B, delta qp."]
    #[inline(always)]
    pub fn b_no_roi_region_qp(&self) -> B_NO_ROI_REGION_QP_R {
        B_NO_ROI_REGION_QP_R::new((self.bits & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_NO_ROI_REGION_QP_OFFSET")
            .field("b_no_roi_region_qp", &self.b_no_roi_region_qp().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<B_NO_ROI_REGION_QP_OFFSET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Configure H264 no region qp in video B, delta qp."]
    #[inline(always)]
    #[must_use]
    pub fn b_no_roi_region_qp(&mut self) -> B_NO_ROI_REGION_QP_W<B_NO_ROI_REGION_QP_OFFSET_SPEC> {
        B_NO_ROI_REGION_QP_W::new(self, 0)
    }
}
#[doc = "Video B H264 no roi region QP register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_no_roi_region_qp_offset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_no_roi_region_qp_offset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B_NO_ROI_REGION_QP_OFFSET_SPEC;
impl crate::RegisterSpec for B_NO_ROI_REGION_QP_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_no_roi_region_qp_offset::R`](R) reader structure"]
impl crate::Readable for B_NO_ROI_REGION_QP_OFFSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`b_no_roi_region_qp_offset::W`](W) writer structure"]
impl crate::Writable for B_NO_ROI_REGION_QP_OFFSET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B_NO_ROI_REGION_QP_OFFSET to value 0"]
impl crate::Resettable for B_NO_ROI_REGION_QP_OFFSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
