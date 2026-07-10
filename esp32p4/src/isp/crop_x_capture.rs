#[doc = "Register `CROP_X_CAPTURE` reader"]
pub type R = crate::R<CROP_X_CAPTURE_SPEC>;
#[doc = "Register `CROP_X_CAPTURE` writer"]
pub type W = crate::W<CROP_X_CAPTURE_SPEC>;
#[doc = "Field `CROP_X_START` reader - isp_crop capture col start index"]
pub type CROP_X_START_R = crate::FieldReader<u16>;
#[doc = "Field `CROP_X_START` writer - isp_crop capture col start index"]
pub type CROP_X_START_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CROP_X_END` reader - isp_crop capture col end index"]
pub type CROP_X_END_R = crate::FieldReader<u16>;
#[doc = "Field `CROP_X_END` writer - isp_crop capture col end index"]
pub type CROP_X_END_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - isp_crop capture col start index"]
    #[inline(always)]
    pub fn crop_x_start(&self) -> CROP_X_START_R {
        CROP_X_START_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - isp_crop capture col end index"]
    #[inline(always)]
    pub fn crop_x_end(&self) -> CROP_X_END_R {
        CROP_X_END_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CROP_X_CAPTURE")
            .field("crop_x_start", &self.crop_x_start())
            .field("crop_x_end", &self.crop_x_end())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - isp_crop capture col start index"]
    #[inline(always)]
    pub fn crop_x_start(&mut self) -> CROP_X_START_W<'_, CROP_X_CAPTURE_SPEC> {
        CROP_X_START_W::new(self, 0)
    }
    #[doc = "Bits 12:23 - isp_crop capture col end index"]
    #[inline(always)]
    pub fn crop_x_end(&mut self) -> CROP_X_END_W<'_, CROP_X_CAPTURE_SPEC> {
        CROP_X_END_W::new(self, 12)
    }
}
#[doc = "isp_crop col capture range register\n\nYou can [`read`](crate::Reg::read) this register and get [`crop_x_capture::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crop_x_capture::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CROP_X_CAPTURE_SPEC;
impl crate::RegisterSpec for CROP_X_CAPTURE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crop_x_capture::R`](R) reader structure"]
impl crate::Readable for CROP_X_CAPTURE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crop_x_capture::W`](W) writer structure"]
impl crate::Writable for CROP_X_CAPTURE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CROP_X_CAPTURE to value 0"]
impl crate::Resettable for CROP_X_CAPTURE_SPEC {}
