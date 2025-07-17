#[doc = "Register `SAR_TOUCH_THRES%s` reader"]
pub type R = crate::R<SAR_TOUCH_THRES_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES%s` writer"]
pub type W = crate::W<SAR_TOUCH_THRES_SPEC>;
#[doc = "Field `THRESHOLD` reader - Finger threshold for touch pad 1"]
pub type THRESHOLD_R = crate::FieldReader<u32>;
#[doc = "Field `THRESHOLD` writer - Finger threshold for touch pad 1"]
pub type THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 1"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES")
            .field("threshold", &self.threshold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 1"]
    #[inline(always)]
    pub fn threshold(&mut self) -> THRESHOLD_W<SAR_TOUCH_THRES_SPEC> {
        THRESHOLD_W::new(self, 0)
    }
}
#[doc = "Finger threshold for touch pad %s\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_thres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_thres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES%s to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES_SPEC {}
