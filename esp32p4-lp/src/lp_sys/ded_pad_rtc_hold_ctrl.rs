#[doc = "Register `DED_PAD_RTC_HOLD_CTRL` reader"]
pub type R = crate::R<DED_PAD_RTC_HOLD_CTRL_SPEC>;
#[doc = "Register `DED_PAD_RTC_HOLD_CTRL` writer"]
pub type W = crate::W<DED_PAD_RTC_HOLD_CTRL_SPEC>;
#[doc = "Field `DED_PAD_RTC_HOLD_CTRL` reader - Set bit0-5 to hold flash pad status. Set bit6-25 to hold psram pad status."]
pub type DED_PAD_RTC_HOLD_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `DED_PAD_RTC_HOLD_CTRL` writer - Set bit0-5 to hold flash pad status. Set bit6-25 to hold psram pad status."]
pub type DED_PAD_RTC_HOLD_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - Set bit0-5 to hold flash pad status. Set bit6-25 to hold psram pad status."]
    #[inline(always)]
    pub fn ded_pad_rtc_hold_ctrl(&self) -> DED_PAD_RTC_HOLD_CTRL_R {
        DED_PAD_RTC_HOLD_CTRL_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DED_PAD_RTC_HOLD_CTRL")
            .field("ded_pad_rtc_hold_ctrl", &self.ded_pad_rtc_hold_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:25 - Set bit0-5 to hold flash pad status. Set bit6-25 to hold psram pad status."]
    #[inline(always)]
    pub fn ded_pad_rtc_hold_ctrl(
        &mut self,
    ) -> DED_PAD_RTC_HOLD_CTRL_W<'_, DED_PAD_RTC_HOLD_CTRL_SPEC> {
        DED_PAD_RTC_HOLD_CTRL_W::new(self, 0)
    }
}
#[doc = "enable pad hold ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_pad_rtc_hold_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_pad_rtc_hold_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
