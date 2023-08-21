#[doc = "Register `SAR_TOUCH_THRES2` reader"]
pub type R = crate::R<SAR_TOUCH_THRES2_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES2` writer"]
pub type W = crate::W<SAR_TOUCH_THRES2_SPEC>;
#[doc = "Field `TOUCH_OUT_TH3` reader - the threshold for touch pad 3"]
pub type TOUCH_OUT_TH3_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUT_TH3` writer - the threshold for touch pad 3"]
pub type TOUCH_OUT_TH3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `TOUCH_OUT_TH2` reader - the threshold for touch pad 2"]
pub type TOUCH_OUT_TH2_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUT_TH2` writer - the threshold for touch pad 2"]
pub type TOUCH_OUT_TH2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - the threshold for touch pad 3"]
    #[inline(always)]
    pub fn touch_out_th3(&self) -> TOUCH_OUT_TH3_R {
        TOUCH_OUT_TH3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 2"]
    #[inline(always)]
    pub fn touch_out_th2(&self) -> TOUCH_OUT_TH2_R {
        TOUCH_OUT_TH2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES2")
            .field(
                "touch_out_th3",
                &format_args!("{}", self.touch_out_th3().bits()),
            )
            .field(
                "touch_out_th2",
                &format_args!("{}", self.touch_out_th2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_THRES2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - the threshold for touch pad 3"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th3(&mut self) -> TOUCH_OUT_TH3_W<SAR_TOUCH_THRES2_SPEC, 0> {
        TOUCH_OUT_TH3_W::new(self)
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 2"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th2(&mut self) -> TOUCH_OUT_TH2_W<SAR_TOUCH_THRES2_SPEC, 16> {
        TOUCH_OUT_TH2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES2_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres2::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres2::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES2 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
