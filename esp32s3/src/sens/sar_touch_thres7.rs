#[doc = "Register `SAR_TOUCH_THRES7` reader"]
pub type R = crate::R<SAR_TOUCH_THRES7_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES7` writer"]
pub type W = crate::W<SAR_TOUCH_THRES7_SPEC>;
#[doc = "Field `SAR_TOUCH_OUT_TH7` reader - Finger threshold for touch pad 7"]
pub type SAR_TOUCH_OUT_TH7_R = crate::FieldReader<u32>;
#[doc = "Field `SAR_TOUCH_OUT_TH7` writer - Finger threshold for touch pad 7"]
pub type SAR_TOUCH_OUT_TH7_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 7"]
    #[inline(always)]
    pub fn sar_touch_out_th7(&self) -> SAR_TOUCH_OUT_TH7_R {
        SAR_TOUCH_OUT_TH7_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES7")
            .field("sar_touch_out_th7", &self.sar_touch_out_th7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 7"]
    #[inline(always)]
    pub fn sar_touch_out_th7(&mut self) -> SAR_TOUCH_OUT_TH7_W<SAR_TOUCH_THRES7_SPEC> {
        SAR_TOUCH_OUT_TH7_W::new(self, 0)
    }
}
#[doc = "configure touch thres of touch pad\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES7_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres7::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres7::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES7 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES7_SPEC {}
