#[doc = "Register `SAR_TOUCH_THRES1` reader"]
pub type R = crate::R<SAR_TOUCH_THRES1_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES1` writer"]
pub type W = crate::W<SAR_TOUCH_THRES1_SPEC>;
#[doc = "Field `TOUCH_OUT_TH1` reader - the threshold for touch pad 1"]
pub type TOUCH_OUT_TH1_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUT_TH1` writer - the threshold for touch pad 1"]
pub type TOUCH_OUT_TH1_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_OUT_TH0` reader - the threshold for touch pad 0"]
pub type TOUCH_OUT_TH0_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUT_TH0` writer - the threshold for touch pad 0"]
pub type TOUCH_OUT_TH0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the threshold for touch pad 1"]
    #[inline(always)]
    pub fn touch_out_th1(&self) -> TOUCH_OUT_TH1_R {
        TOUCH_OUT_TH1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 0"]
    #[inline(always)]
    pub fn touch_out_th0(&self) -> TOUCH_OUT_TH0_R {
        TOUCH_OUT_TH0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES1")
            .field(
                "touch_out_th1",
                &format_args!("{}", self.touch_out_th1().bits()),
            )
            .field(
                "touch_out_th0",
                &format_args!("{}", self.touch_out_th0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_THRES1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - the threshold for touch pad 1"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th1(&mut self) -> TOUCH_OUT_TH1_W<SAR_TOUCH_THRES1_SPEC> {
        TOUCH_OUT_TH1_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 0"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th0(&mut self) -> TOUCH_OUT_TH0_W<SAR_TOUCH_THRES1_SPEC> {
        TOUCH_OUT_TH0_W::new(self, 16)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES1_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres1::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres1::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES1 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES1_SPEC {
    const RESET_VALUE: u32 = 0;
}
