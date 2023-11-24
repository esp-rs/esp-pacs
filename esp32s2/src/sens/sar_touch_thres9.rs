#[doc = "Register `SAR_TOUCH_THRES9` reader"]
pub type R = crate::R<SAR_TOUCH_THRES9_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES9` writer"]
pub type W = crate::W<SAR_TOUCH_THRES9_SPEC>;
#[doc = "Field `TOUCH_OUT_TH9` reader - Finger threshold for touch pad 9"]
pub type TOUCH_OUT_TH9_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_OUT_TH9` writer - Finger threshold for touch pad 9"]
pub type TOUCH_OUT_TH9_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 9"]
    #[inline(always)]
    pub fn touch_out_th9(&self) -> TOUCH_OUT_TH9_R {
        TOUCH_OUT_TH9_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES9")
            .field(
                "touch_out_th9",
                &format_args!("{}", self.touch_out_th9().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_THRES9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:21 - Finger threshold for touch pad 9"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th9(&mut self) -> TOUCH_OUT_TH9_W<SAR_TOUCH_THRES9_SPEC> {
        TOUCH_OUT_TH9_W::new(self, 0)
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
#[doc = "Finger threshold for touch pad 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES9_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres9::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres9::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES9 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
