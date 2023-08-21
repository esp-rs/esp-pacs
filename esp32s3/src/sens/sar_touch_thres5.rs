#[doc = "Register `SAR_TOUCH_THRES5` reader"]
pub type R = crate::R<SAR_TOUCH_THRES5_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES5` writer"]
pub type W = crate::W<SAR_TOUCH_THRES5_SPEC>;
#[doc = "Field `SAR_TOUCH_OUT_TH5` reader - Finger threshold for touch pad 5"]
pub type SAR_TOUCH_OUT_TH5_R = crate::FieldReader<u32>;
#[doc = "Field `SAR_TOUCH_OUT_TH5` writer - Finger threshold for touch pad 5"]
pub type SAR_TOUCH_OUT_TH5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 22, O, u32>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 5"]
    #[inline(always)]
    pub fn sar_touch_out_th5(&self) -> SAR_TOUCH_OUT_TH5_R {
        SAR_TOUCH_OUT_TH5_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES5")
            .field(
                "sar_touch_out_th5",
                &format_args!("{}", self.sar_touch_out_th5().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_THRES5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 5"]
    #[inline(always)]
    #[must_use]
    pub fn sar_touch_out_th5(&mut self) -> SAR_TOUCH_OUT_TH5_W<SAR_TOUCH_THRES5_SPEC, 0> {
        SAR_TOUCH_OUT_TH5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configure touch thres of touch pad\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES5_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres5::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres5::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES5 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
