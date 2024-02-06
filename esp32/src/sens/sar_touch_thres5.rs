#[doc = "Register `SAR_TOUCH_THRES5` reader"]
pub type R = crate::R<SAR_TOUCH_THRES5_SPEC>;
#[doc = "Register `SAR_TOUCH_THRES5` writer"]
pub type W = crate::W<SAR_TOUCH_THRES5_SPEC>;
#[doc = "Field `TOUCH_OUT_TH9` reader - the threshold for touch pad 9"]
pub type TOUCH_OUT_TH9_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUT_TH9` writer - the threshold for touch pad 9"]
pub type TOUCH_OUT_TH9_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUCH_OUT_TH8` reader - the threshold for touch pad 8"]
pub type TOUCH_OUT_TH8_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUT_TH8` writer - the threshold for touch pad 8"]
pub type TOUCH_OUT_TH8_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - the threshold for touch pad 9"]
    #[inline(always)]
    pub fn touch_out_th9(&self) -> TOUCH_OUT_TH9_R {
        TOUCH_OUT_TH9_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 8"]
    #[inline(always)]
    pub fn touch_out_th8(&self) -> TOUCH_OUT_TH8_R {
        TOUCH_OUT_TH8_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_THRES5")
            .field(
                "touch_out_th9",
                &format_args!("{}", self.touch_out_th9().bits()),
            )
            .field(
                "touch_out_th8",
                &format_args!("{}", self.touch_out_th8().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_THRES5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - the threshold for touch pad 9"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th9(&mut self) -> TOUCH_OUT_TH9_W<SAR_TOUCH_THRES5_SPEC> {
        TOUCH_OUT_TH9_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - the threshold for touch pad 8"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_th8(&mut self) -> TOUCH_OUT_TH8_W<SAR_TOUCH_THRES5_SPEC> {
        TOUCH_OUT_TH8_W::new(self, 16)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_thres5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_touch_thres5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_THRES5_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_THRES5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_thres5::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_THRES5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_thres5::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_THRES5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_THRES5 to value 0"]
impl crate::Resettable for SAR_TOUCH_THRES5_SPEC {
    const RESET_VALUE: u32 = 0;
}
