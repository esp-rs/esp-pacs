#[doc = "Register `DED_PAD_RTC_HOLD_CTRL` reader"]
pub type R = crate::R<DED_PAD_RTC_HOLD_CTRL_SPEC>;
#[doc = "Register `DED_PAD_RTC_HOLD_CTRL` writer"]
pub type W = crate::W<DED_PAD_RTC_HOLD_CTRL_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DED_PAD_RTC_HOLD_CTRL")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, DED_PAD_RTC_HOLD_CTRL_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_pad_rtc_hold_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_pad_rtc_hold_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DED_PAD_RTC_HOLD_CTRL_SPEC;
impl crate::RegisterSpec for DED_PAD_RTC_HOLD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_pad_rtc_hold_ctrl::R`](R) reader structure"]
impl crate::Readable for DED_PAD_RTC_HOLD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ded_pad_rtc_hold_ctrl::W`](W) writer structure"]
impl crate::Writable for DED_PAD_RTC_HOLD_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DED_PAD_RTC_HOLD_CTRL to value 0"]
impl crate::Resettable for DED_PAD_RTC_HOLD_CTRL_SPEC {}
