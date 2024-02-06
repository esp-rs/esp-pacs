#[doc = "Register `YUV_FORMAT` reader"]
pub type R = crate::R<YUV_FORMAT_SPEC>;
#[doc = "Register `YUV_FORMAT` writer"]
pub type W = crate::W<YUV_FORMAT_SPEC>;
#[doc = "Field `YUV_MODE` reader - this bit configures the yuv mode. 0: ITU-R BT.601, 1: ITU-R BT.709"]
pub type YUV_MODE_R = crate::BitReader;
#[doc = "Field `YUV_MODE` writer - this bit configures the yuv mode. 0: ITU-R BT.601, 1: ITU-R BT.709"]
pub type YUV_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YUV_RANGE` reader - this bit configures the yuv range. 0: full range, 1: limit range"]
pub type YUV_RANGE_R = crate::BitReader;
#[doc = "Field `YUV_RANGE` writer - this bit configures the yuv range. 0: full range, 1: limit range"]
pub type YUV_RANGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures the yuv mode. 0: ITU-R BT.601, 1: ITU-R BT.709"]
    #[inline(always)]
    pub fn yuv_mode(&self) -> YUV_MODE_R {
        YUV_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit configures the yuv range. 0: full range, 1: limit range"]
    #[inline(always)]
    pub fn yuv_range(&self) -> YUV_RANGE_R {
        YUV_RANGE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("YUV_FORMAT")
            .field("yuv_mode", &format_args!("{}", self.yuv_mode().bit()))
            .field("yuv_range", &format_args!("{}", self.yuv_range().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<YUV_FORMAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures the yuv mode. 0: ITU-R BT.601, 1: ITU-R BT.709"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_mode(&mut self) -> YUV_MODE_W<YUV_FORMAT_SPEC> {
        YUV_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - this bit configures the yuv range. 0: full range, 1: limit range"]
    #[inline(always)]
    #[must_use]
    pub fn yuv_range(&mut self) -> YUV_RANGE_W<YUV_FORMAT_SPEC> {
        YUV_RANGE_W::new(self, 1)
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
#[doc = "yuv format control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`yuv_format::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`yuv_format::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YUV_FORMAT_SPEC;
impl crate::RegisterSpec for YUV_FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`yuv_format::R`](R) reader structure"]
impl crate::Readable for YUV_FORMAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`yuv_format::W`](W) writer structure"]
impl crate::Writable for YUV_FORMAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets YUV_FORMAT to value 0"]
impl crate::Resettable for YUV_FORMAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
