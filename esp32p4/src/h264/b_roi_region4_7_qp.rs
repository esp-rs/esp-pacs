#[doc = "Register `B_ROI_REGION4_7_QP` reader"]
pub type R = crate::R<B_ROI_REGION4_7_QP_SPEC>;
#[doc = "Register `B_ROI_REGION4_7_QP` writer"]
pub type W = crate::W<B_ROI_REGION4_7_QP_SPEC>;
#[doc = "Field `B_ROI_REGION4_QP` reader - Configure H264 ROI region4 qp in video B,fixed qp or delta qp."]
pub type B_ROI_REGION4_QP_R = crate::FieldReader;
#[doc = "Field `B_ROI_REGION4_QP` writer - Configure H264 ROI region4 qp in video B,fixed qp or delta qp."]
pub type B_ROI_REGION4_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `B_ROI_REGION5_QP` reader - Configure H264 ROI region5 qp in video B,fixed qp or delta qp."]
pub type B_ROI_REGION5_QP_R = crate::FieldReader;
#[doc = "Field `B_ROI_REGION5_QP` writer - Configure H264 ROI region5 qp in video B,fixed qp or delta qp."]
pub type B_ROI_REGION5_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `B_ROI_REGION6_QP` reader - Configure H264 ROI region6 qp in video B,fixed qp or delta qp."]
pub type B_ROI_REGION6_QP_R = crate::FieldReader;
#[doc = "Field `B_ROI_REGION6_QP` writer - Configure H264 ROI region6 qp in video B,fixed qp or delta qp."]
pub type B_ROI_REGION6_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `B_ROI_REGION7_QP` reader - Configure H264 ROI region7 qp in video B,fixed qp or delta qp."]
pub type B_ROI_REGION7_QP_R = crate::FieldReader;
#[doc = "Field `B_ROI_REGION7_QP` writer - Configure H264 ROI region7 qp in video B,fixed qp or delta qp."]
pub type B_ROI_REGION7_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configure H264 ROI region4 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region4_qp(&self) -> B_ROI_REGION4_QP_R {
        B_ROI_REGION4_QP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Configure H264 ROI region5 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region5_qp(&self) -> B_ROI_REGION5_QP_R {
        B_ROI_REGION5_QP_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - Configure H264 ROI region6 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region6_qp(&self) -> B_ROI_REGION6_QP_R {
        B_ROI_REGION6_QP_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:27 - Configure H264 ROI region7 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region7_qp(&self) -> B_ROI_REGION7_QP_R {
        B_ROI_REGION7_QP_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_ROI_REGION4_7_QP")
            .field("b_roi_region4_qp", &self.b_roi_region4_qp())
            .field("b_roi_region5_qp", &self.b_roi_region5_qp())
            .field("b_roi_region6_qp", &self.b_roi_region6_qp())
            .field("b_roi_region7_qp", &self.b_roi_region7_qp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Configure H264 ROI region4 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region4_qp(&mut self) -> B_ROI_REGION4_QP_W<B_ROI_REGION4_7_QP_SPEC> {
        B_ROI_REGION4_QP_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - Configure H264 ROI region5 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region5_qp(&mut self) -> B_ROI_REGION5_QP_W<B_ROI_REGION4_7_QP_SPEC> {
        B_ROI_REGION5_QP_W::new(self, 7)
    }
    #[doc = "Bits 14:20 - Configure H264 ROI region6 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region6_qp(&mut self) -> B_ROI_REGION6_QP_W<B_ROI_REGION4_7_QP_SPEC> {
        B_ROI_REGION6_QP_W::new(self, 14)
    }
    #[doc = "Bits 21:27 - Configure H264 ROI region7 qp in video B,fixed qp or delta qp."]
    #[inline(always)]
    pub fn b_roi_region7_qp(&mut self) -> B_ROI_REGION7_QP_W<B_ROI_REGION4_7_QP_SPEC> {
        B_ROI_REGION7_QP_W::new(self, 21)
    }
}
#[doc = "Video B H264 ROI region4, region5,region6,region7 QP register.\n\nYou can [`read`](crate::Reg::read) this register and get [`b_roi_region4_7_qp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_roi_region4_7_qp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B_ROI_REGION4_7_QP_SPEC;
impl crate::RegisterSpec for B_ROI_REGION4_7_QP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_roi_region4_7_qp::R`](R) reader structure"]
impl crate::Readable for B_ROI_REGION4_7_QP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`b_roi_region4_7_qp::W`](W) writer structure"]
impl crate::Writable for B_ROI_REGION4_7_QP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B_ROI_REGION4_7_QP to value 0"]
impl crate::Resettable for B_ROI_REGION4_7_QP_SPEC {
    const RESET_VALUE: u32 = 0;
}
