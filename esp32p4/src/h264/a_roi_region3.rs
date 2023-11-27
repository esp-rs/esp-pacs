#[doc = "Register `A_ROI_REGION3` reader"]
pub type R = crate::R<A_ROI_REGION3_SPEC>;
#[doc = "Register `A_ROI_REGION3` writer"]
pub type W = crate::W<A_ROI_REGION3_SPEC>;
#[doc = "Field `X` reader - Configures the horizontal start macroblocks of region 3 in Video A."]
pub type X_R = crate::FieldReader;
#[doc = "Field `X` writer - Configures the horizontal start macroblocks of region 3 in Video A."]
pub type X_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Y` reader - Configures the vertical start macroblocks of region 3 in Video A."]
pub type Y_R = crate::FieldReader;
#[doc = "Field `Y` writer - Configures the vertical start macroblocks of region 3 in Video A."]
pub type Y_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `X_LEN` reader - Configures the number of macroblocks in horizontal direction of the region 3 in video A."]
pub type X_LEN_R = crate::FieldReader;
#[doc = "Field `X_LEN` writer - Configures the number of macroblocks in horizontal direction of the region 3 in video A."]
pub type X_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Y_LEN` reader - Configures the number of macroblocks in vertical direction of the region 3 in video A."]
pub type Y_LEN_R = crate::FieldReader;
#[doc = "Field `Y_LEN` writer - Configures the number of macroblocks in vertical direction of the region 3 in video A."]
pub type Y_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EN` reader - Configures whether or not to open Video A ROI of region 3 .\\\\0:Close ROI\\\\1:Open ROI."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Configures whether or not to open Video A ROI of region 3 .\\\\0:Close ROI\\\\1:Open ROI."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Configures the horizontal start macroblocks of region 3 in Video A."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:13 - Configures the vertical start macroblocks of region 3 in Video A."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    #[doc = "Bits 14:20 - Configures the number of macroblocks in horizontal direction of the region 3 in video A."]
    #[inline(always)]
    pub fn x_len(&self) -> X_LEN_R {
        X_LEN_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bits 21:27 - Configures the number of macroblocks in vertical direction of the region 3 in video A."]
    #[inline(always)]
    pub fn y_len(&self) -> Y_LEN_R {
        Y_LEN_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - Configures whether or not to open Video A ROI of region 3 .\\\\0:Close ROI\\\\1:Open ROI."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A_ROI_REGION3")
            .field("x", &format_args!("{}", self.x().bits()))
            .field("y", &format_args!("{}", self.y().bits()))
            .field("x_len", &format_args!("{}", self.x_len().bits()))
            .field("y_len", &format_args!("{}", self.y_len().bits()))
            .field("en", &format_args!("{}", self.en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<A_ROI_REGION3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Configures the horizontal start macroblocks of region 3 in Video A."]
    #[inline(always)]
    #[must_use]
    pub fn x(&mut self) -> X_W<A_ROI_REGION3_SPEC> {
        X_W::new(self, 0)
    }
    #[doc = "Bits 7:13 - Configures the vertical start macroblocks of region 3 in Video A."]
    #[inline(always)]
    #[must_use]
    pub fn y(&mut self) -> Y_W<A_ROI_REGION3_SPEC> {
        Y_W::new(self, 7)
    }
    #[doc = "Bits 14:20 - Configures the number of macroblocks in horizontal direction of the region 3 in video A."]
    #[inline(always)]
    #[must_use]
    pub fn x_len(&mut self) -> X_LEN_W<A_ROI_REGION3_SPEC> {
        X_LEN_W::new(self, 14)
    }
    #[doc = "Bits 21:27 - Configures the number of macroblocks in vertical direction of the region 3 in video A."]
    #[inline(always)]
    #[must_use]
    pub fn y_len(&mut self) -> Y_LEN_W<A_ROI_REGION3_SPEC> {
        Y_LEN_W::new(self, 21)
    }
    #[doc = "Bit 28 - Configures whether or not to open Video A ROI of region 3 .\\\\0:Close ROI\\\\1:Open ROI."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<A_ROI_REGION3_SPEC> {
        EN_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Video A H264 ROI region3 range configure register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_roi_region3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_roi_region3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A_ROI_REGION3_SPEC;
impl crate::RegisterSpec for A_ROI_REGION3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a_roi_region3::R`](R) reader structure"]
impl crate::Readable for A_ROI_REGION3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`a_roi_region3::W`](W) writer structure"]
impl crate::Writable for A_ROI_REGION3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets A_ROI_REGION3 to value 0"]
impl crate::Resettable for A_ROI_REGION3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
