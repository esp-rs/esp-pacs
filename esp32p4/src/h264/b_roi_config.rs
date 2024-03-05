#[doc = "Register `B_ROI_CONFIG` reader"]
pub type R = crate::R<B_ROI_CONFIG_SPEC>;
#[doc = "Register `B_ROI_CONFIG` writer"]
pub type W = crate::W<B_ROI_CONFIG_SPEC>;
#[doc = "Field `B_ROI_EN` reader - Configure whether or not to enable ROI in video B.\\\\0:not enable ROI\\\\1:enable ROI."]
pub type B_ROI_EN_R = crate::BitReader;
#[doc = "Field `B_ROI_EN` writer - Configure whether or not to enable ROI in video B.\\\\0:not enable ROI\\\\1:enable ROI."]
pub type B_ROI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_ROI_MODE` reader - Configure the mode of ROI in video B.\\\\0:fixed qp\\\\1:delta qp."]
pub type B_ROI_MODE_R = crate::BitReader;
#[doc = "Field `B_ROI_MODE` writer - Configure the mode of ROI in video B.\\\\0:fixed qp\\\\1:delta qp."]
pub type B_ROI_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure whether or not to enable ROI in video B.\\\\0:not enable ROI\\\\1:enable ROI."]
    #[inline(always)]
    pub fn b_roi_en(&self) -> B_ROI_EN_R {
        B_ROI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure the mode of ROI in video B.\\\\0:fixed qp\\\\1:delta qp."]
    #[inline(always)]
    pub fn b_roi_mode(&self) -> B_ROI_MODE_R {
        B_ROI_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_ROI_CONFIG")
            .field("b_roi_en", &format_args!("{}", self.b_roi_en().bit()))
            .field("b_roi_mode", &format_args!("{}", self.b_roi_mode().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<B_ROI_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether or not to enable ROI in video B.\\\\0:not enable ROI\\\\1:enable ROI."]
    #[inline(always)]
    #[must_use]
    pub fn b_roi_en(&mut self) -> B_ROI_EN_W<B_ROI_CONFIG_SPEC> {
        B_ROI_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configure the mode of ROI in video B.\\\\0:fixed qp\\\\1:delta qp."]
    #[inline(always)]
    #[must_use]
    pub fn b_roi_mode(&mut self) -> B_ROI_MODE_W<B_ROI_CONFIG_SPEC> {
        B_ROI_MODE_W::new(self, 1)
    }
}
#[doc = "Video B H264 ROI configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b_roi_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b_roi_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B_ROI_CONFIG_SPEC;
impl crate::RegisterSpec for B_ROI_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_roi_config::R`](R) reader structure"]
impl crate::Readable for B_ROI_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`b_roi_config::W`](W) writer structure"]
impl crate::Writable for B_ROI_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B_ROI_CONFIG to value 0"]
impl crate::Resettable for B_ROI_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
