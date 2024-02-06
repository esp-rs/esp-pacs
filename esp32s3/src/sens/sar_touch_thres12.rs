#[doc = "Register `SAR_TOUCH_THRES12` reader"]
pub type R = crate::R<SAR_TOUCH_THRES12_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES12` writer"]
pub type W = crate::W<SAR_TOUCH_THRES12_SPEC>;
#[doc = "Field `SAR_TOUCH_OUT_TH12` reader - Finger threshold for touch pad 12"]
pub type SAR_TOUCH_OUT_TH12_R = crate::FieldReader<u32>;
#[doc = "Field `SAR_TOUCH_OUT_TH12` writer - Finger threshold for touch pad 12"]
pub type SAR_TOUCH_OUT_TH12_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 12"]
    #[inline(always)]
    pub fn sar_touch_out_th12(&self) -> SAR_TOUCH_OUT_TH12_R {
        SAR_TOUCH_OUT_TH12_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES12")
            .field(
                "sar_touch_out_th12",
                &format_args!("{}", self.sar_touch_out_th12().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_THRES12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 12"]
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_out_th12(&mut self) -> SAR_TOUCH_OUT_TH12_W<SAR_TOUCH_THRES12_SPEC> {
        SAR_TOUCH_OUT_TH12_W::new(self, 0)
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
#[doc = "configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES12_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres12::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres12::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES12_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES12 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES12_SPEC {
    const RESET_VALUE: u32 = 0;
}
