#[doc = "Register `A_ROI_CONFIG` reader"]
pub type R = crate::R<A_ROI_CONFIG_SPEC>;
#[doc = "Register `A_ROI_CONFIG` writer"]
pub type W = crate::W<A_ROI_CONFIG_SPEC>;
#[doc = "Field `A_ROI_EN` reader - Configure whether or not to enable ROI in video A.\\\\0:not enable ROI\\\\1:enable ROI."]
pub type A_ROI_EN_R = crate::BitReader;
#[doc = "Field `A_ROI_EN` writer - Configure whether or not to enable ROI in video A.\\\\0:not enable ROI\\\\1:enable ROI."]
pub type A_ROI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_ROI_MODE` reader - Configure the mode of ROI in video A.\\\\0:fixed qp\\\\1:delta qp."]
pub type A_ROI_MODE_R = crate::BitReader;
#[doc = "Field `A_ROI_MODE` writer - Configure the mode of ROI in video A.\\\\0:fixed qp\\\\1:delta qp."]
pub type A_ROI_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure whether or not to enable ROI in video A.\\\\0:not enable ROI\\\\1:enable ROI."]
    #[inline(always)]
    pub fn a_roi_en(&self) -> A_ROI_EN_R {
        A_ROI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure the mode of ROI in video A.\\\\0:fixed qp\\\\1:delta qp."]
    #[inline(always)]
    pub fn a_roi_mode(&self) -> A_ROI_MODE_R {
        A_ROI_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A_ROI_CONFIG")
            .field("a_roi_en", &self.a_roi_en())
            .field("a_roi_mode", &self.a_roi_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether or not to enable ROI in video A.\\\\0:not enable ROI\\\\1:enable ROI."]
    #[inline(always)]
    #[must_use]
    pub fn a_roi_en(&mut self) -> A_ROI_EN_W<A_ROI_CONFIG_SPEC> {
        A_ROI_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configure the mode of ROI in video A.\\\\0:fixed qp\\\\1:delta qp."]
    #[inline(always)]
    #[must_use]
    pub fn a_roi_mode(&mut self) -> A_ROI_MODE_W<A_ROI_CONFIG_SPEC> {
        A_ROI_MODE_W::new(self, 1)
    }
}
#[doc = "Video A H264 ROI configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_ROI_CONFIG_SPEC;
impl crate::RegisterSpec for A_ROI_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_roi_config::R`](R) reader structure"]
impl crate::Readable for A_ROI_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a_roi_config::W`](W) writer structure"]
impl crate::Writable for A_ROI_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A_ROI_CONFIG to value 0"]
impl crate::Resettable for A_ROI_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
