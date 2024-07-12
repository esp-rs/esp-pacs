#[doc = "Register `SAR_TOUCH_THRES8` reader"]
pub type R = crate::R<SAR_TOUCH_THRES8_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES8` writer"]
pub type W = crate::W<SAR_TOUCH_THRES8_SPEC>;
#[doc = "Field `TOUCH_OUT_TH8` reader - Finger threshold for touch pad 8"]
pub type TOUCH_OUT_TH8_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_OUT_TH8` writer - Finger threshold for touch pad 8"]
pub type TOUCH_OUT_TH8_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 8"]
    #[inline(always)]
    pub fn touch_out_th8(&self) -> TOUCH_OUT_TH8_R {
        TOUCH_OUT_TH8_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES8")
            .field("touch_out_th8", &self.touch_out_th8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 8"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th8(&mut self) -> TOUCH_OUT_TH8_W<SAR_TOUCH_THRES8_SPEC> {
        TOUCH_OUT_TH8_W::new(self, 0)
    }
}
#[doc = "Finger threshold for touch pad 8\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES8_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres8::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres8::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES8 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES8_SPEC {
    const RESET_VALUE: u32 = 0;
}
