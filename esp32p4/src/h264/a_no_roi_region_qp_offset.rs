#[doc = "Register `A_NO_ROI_REGION_QP_OFFSET` reader"]
pub type R = crate::R<A_NO_ROI_REGION_QP_OFFSET_SPEC>;
#[doc = "Register `A_NO_ROI_REGION_QP_OFFSET` writer"]
pub type W = crate::W<A_NO_ROI_REGION_QP_OFFSET_SPEC>;
#[doc = "Field `A_NO_ROI_REGION_QP` reader - Configure H264 no region qp in video A, delta qp."]
pub type A_NO_ROI_REGION_QP_R = crate::FieldReader;
#[doc = "Field `A_NO_ROI_REGION_QP` writer - Configure H264 no region qp in video A, delta qp."]
pub type A_NO_ROI_REGION_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configure H264 no region qp in video A, delta qp."]
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
    #[doc = "Bits 0:6 - Configure H264 no region qp in video A, delta qp."]
    #[inline(always)]
    pub fn a_no_roi_region_qp(&mut self) -> A_NO_ROI_REGION_QP_W<A_NO_ROI_REGION_QP_OFFSET_SPEC> {
        A_NO_ROI_REGION_QP_W::new(self, 0)
    }
}
#[doc = "Video A H264 no roi region QP register.\n\nYou can [`read`](crate::Reg::read) this register and get [`a_no_roi_region_qp_offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a_no_roi_region_qp_offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_NO_ROI_REGION_QP_OFFSET_SPEC;
impl crate::RegisterSpec for A_NO_ROI_REGION_QP_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_no_roi_region_qp_offset::R`](R) reader structure"]
impl crate::Readable for A_NO_ROI_REGION_QP_OFFSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a_no_roi_region_qp_offset::W`](W) writer structure"]
impl crate::Writable for A_NO_ROI_REGION_QP_OFFSET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A_NO_ROI_REGION_QP_OFFSET to value 0"]
impl crate::Resettable for A_NO_ROI_REGION_QP_OFFSET_SPEC {}
