#[doc = "Register `B_ROI_REGION6` reader"]
pub type R = crate::R<B_ROI_REGION6_SPEC>;
#[doc = "Register `B_ROI_REGION6` writer"]
pub type W = crate::W<B_ROI_REGION6_SPEC>;
#[doc = "Field `X` reader - Configures the horizontial start macroblocks of region 6 video B."]
pub type X_R = crate::FieldReader;
#[doc = "Field `X` writer - Configures the horizontial start macroblocks of region 6 video B."]
pub type X_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Y` reader - Configures the vertical start macroblocks of region 6 in video B."]
pub type Y_R = crate::FieldReader;
#[doc = "Field `Y` writer - Configures the vertical start macroblocks of region 6 in video B."]
pub type Y_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `X_LEN` reader - Configures the number of macroblocks in horizontal direction of the region 6 in video B."]
pub type X_LEN_R = crate::FieldReader;
#[doc = "Field `X_LEN` writer - Configures the number of macroblocks in horizontal direction of the region 6 in video B."]
pub type X_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Y_LEN` reader - Configures the number of macroblocks in vertical direction of the region 6 in video B."]
pub type Y_LEN_R = crate::FieldReader;
#[doc = "Field `Y_LEN` writer - Configures the number of macroblocks in vertical direction of the region 6 in video B."]
pub type Y_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EN` reader - Configures whether or not to open Video B ROI of region 6 .\\\\0:Close ROI\\\\1:Open ROI."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Configures whether or not to open Video B ROI of region 6 .\\\\0:Close ROI\\\\1:Open ROI."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Configures the horizontial start macroblocks of region 6 video B."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Configures the vertical start macroblocks of region 6 in video B."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - Configures the number of macroblocks in horizontal direction of the region 6 in video B."]
    #[inline(always)]
    pub fn x_len(&self) -> X_LEN_R {
        X_LEN_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:27 - Configures the number of macroblocks in vertical direction of the region 6 in video B."]
    #[inline(always)]
    pub fn y_len(&self) -> Y_LEN_R {
        Y_LEN_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - Configures whether or not to open Video B ROI of region 6 .\\\\0:Close ROI\\\\1:Open ROI."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("B_ROI_REGION6")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("x_len", &self.x_len())
            .field("y_len", &self.y_len())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures the horizontial start macroblocks of region 6 video B."]
    #[inline(always)]
    pub fn x(&mut self) -> X_W<'_, B_ROI_REGION6_SPEC> {
        X_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - Configures the vertical start macroblocks of region 6 in video B."]
    #[inline(always)]
    pub fn y(&mut self) -> Y_W<'_, B_ROI_REGION6_SPEC> {
        Y_W::new(self, 7)
    }
    #[doc = "Bits 14:20 - Configures the number of macroblocks in horizontal direction of the region 6 in video B."]
    #[inline(always)]
    pub fn x_len(&mut self) -> X_LEN_W<'_, B_ROI_REGION6_SPEC> {
        X_LEN_W::new(self, 14)
    }
    #[doc = "Bits 21:27 - Configures the number of macroblocks in vertical direction of the region 6 in video B."]
    #[inline(always)]
    pub fn y_len(&mut self) -> Y_LEN_W<'_, B_ROI_REGION6_SPEC> {
        Y_LEN_W::new(self, 21)
    }
    #[doc = "Bit 28 - Configures whether or not to open Video B ROI of region 6 .\\\\0:Close ROI\\\\1:Open ROI."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, B_ROI_REGION6_SPEC> {
        EN_W::new(self, 28)
    }
}
#[doc = "Video B H264 ROI region6 range configure register.\n\nYou can [`read`](crate::Reg::read) this register and get [`b_roi_region6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b_roi_region6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B_ROI_REGION6_SPEC;
impl crate::RegisterSpec for B_ROI_REGION6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b_roi_region6::R`](R) reader structure"]
impl crate::Readable for B_ROI_REGION6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`b_roi_region6::W`](W) writer structure"]
impl crate::Writable for B_ROI_REGION6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets B_ROI_REGION6 to value 0"]
impl crate::Resettable for B_ROI_REGION6_SPEC {}
