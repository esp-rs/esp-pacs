#[doc = "Register `A_ROI_REGION4_7_QP` reader"]
pub type R = crate::R<A_ROI_REGION4_7_QP_SPEC>;
#[doc = "Register `A_ROI_REGION4_7_QP` writer"]
pub type W = crate::W<A_ROI_REGION4_7_QP_SPEC>;
#[doc = "Field `A_ROI_REGION4_QP` reader - Configure H264 ROI region4 qp in video A,fixed qp or delta qp."]
pub type A_ROI_REGION4_QP_R = crate::FieldReader;
#[doc = "Field `A_ROI_REGION4_QP` writer - Configure H264 ROI region4 qp in video A,fixed qp or delta qp."]
pub type A_ROI_REGION4_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `A_ROI_REGION5_QP` reader - Configure H264 ROI region5 qp in video A,fixed qp or delta qp."]
pub type A_ROI_REGION5_QP_R = crate::FieldReader;
#[doc = "Field `A_ROI_REGION5_QP` writer - Configure H264 ROI region5 qp in video A,fixed qp or delta qp."]
pub type A_ROI_REGION5_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `A_ROI_REGION6_QP` reader - Configure H264 ROI region6 qp in video A,fixed qp or delta qp."]
pub type A_ROI_REGION6_QP_R = crate::FieldReader;
#[doc = "Field `A_ROI_REGION6_QP` writer - Configure H264 ROI region6 qp in video A,fixed qp or delta qp."]
pub type A_ROI_REGION6_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `A_ROI_REGION7_QP` reader - Configure H264 ROI region7 qp in video A,fixed qp or delta qp."]
pub type A_ROI_REGION7_QP_R = crate::FieldReader;
#[doc = "Field `A_ROI_REGION7_QP` writer - Configure H264 ROI region7 qp in video A,fixed qp or delta qp."]
pub type A_ROI_REGION7_QP_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Configure H264 ROI region4 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region4_qp(&self) -> A_ROI_REGION4_QP_R {
        A_ROI_REGION4_QP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Configure H264 ROI region5 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region5_qp(&self) -> A_ROI_REGION5_QP_R {
        A_ROI_REGION5_QP_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - Configure H264 ROI region6 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region6_qp(&self) -> A_ROI_REGION6_QP_R {
        A_ROI_REGION6_QP_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:27 - Configure H264 ROI region7 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region7_qp(&self) -> A_ROI_REGION7_QP_R {
        A_ROI_REGION7_QP_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A_ROI_REGION4_7_QP")
            .field("a_roi_region4_qp", &self.a_roi_region4_qp())
            .field("a_roi_region5_qp", &self.a_roi_region5_qp())
            .field("a_roi_region6_qp", &self.a_roi_region6_qp())
            .field("a_roi_region7_qp", &self.a_roi_region7_qp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Configure H264 ROI region4 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region4_qp(&mut self) -> A_ROI_REGION4_QP_W<'_, A_ROI_REGION4_7_QP_SPEC> {
        A_ROI_REGION4_QP_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - Configure H264 ROI region5 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region5_qp(&mut self) -> A_ROI_REGION5_QP_W<'_, A_ROI_REGION4_7_QP_SPEC> {
        A_ROI_REGION5_QP_W::new(self, 7)
    }
    #[doc = "Bits 14:20 - Configure H264 ROI region6 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region6_qp(&mut self) -> A_ROI_REGION6_QP_W<'_, A_ROI_REGION4_7_QP_SPEC> {
        A_ROI_REGION6_QP_W::new(self, 14)
    }
    #[doc = "Bits 21:27 - Configure H264 ROI region7 qp in video A,fixed qp or delta qp."]
    #[inline(always)]
    pub fn a_roi_region7_qp(&mut self) -> A_ROI_REGION7_QP_W<'_, A_ROI_REGION4_7_QP_SPEC> {
        A_ROI_REGION7_QP_W::new(self, 21)
    }
}
#[doc = "Video A H264 ROI region4, region5,region6,region7 QP register.\n\nYou can [`read`](crate::Reg::read) this register and get [`a_roi_region4_7_qp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a_roi_region4_7_qp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_ROI_REGION4_7_QP_SPEC;
impl crate::RegisterSpec for A_ROI_REGION4_7_QP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_roi_region4_7_qp::R`](R) reader structure"]
impl crate::Readable for A_ROI_REGION4_7_QP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a_roi_region4_7_qp::W`](W) writer structure"]
impl crate::Writable for A_ROI_REGION4_7_QP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets A_ROI_REGION4_7_QP to value 0"]
impl crate::Resettable for A_ROI_REGION4_7_QP_SPEC {}
