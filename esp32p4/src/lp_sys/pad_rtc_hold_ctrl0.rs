#[doc = "Register `PAD_RTC_HOLD_CTRL0` reader"]
pub type R = crate::R<PAD_RTC_HOLD_CTRL0_SPEC>;
#[doc = "Register `PAD_RTC_HOLD_CTRL0` writer"]
pub type W = crate::W<PAD_RTC_HOLD_CTRL0_SPEC>;
#[doc = "Field `PAD_RTC_HOLD_CTRL0` reader - Set 1 to hold pad 0-31 status"]
pub type PAD_RTC_HOLD_CTRL0_R = crate::FieldReader<u32>;
#[doc = "Field `PAD_RTC_HOLD_CTRL0` writer - Set 1 to hold pad 0-31 status"]
pub type PAD_RTC_HOLD_CTRL0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set 1 to hold pad 0-31 status"]
    #[inline(always)]
    pub fn pad_rtc_hold_ctrl0(&self) -> PAD_RTC_HOLD_CTRL0_R {
        PAD_RTC_HOLD_CTRL0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_RTC_HOLD_CTRL0")
            .field("pad_rtc_hold_ctrl0", &self.pad_rtc_hold_ctrl0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Set 1 to hold pad 0-31 status"]
    #[inline(always)]
    pub fn pad_rtc_hold_ctrl0(&mut self) -> PAD_RTC_HOLD_CTRL0_W<'_, PAD_RTC_HOLD_CTRL0_SPEC> {
        PAD_RTC_HOLD_CTRL0_W::new(self, 0)
    }
}
#[doc = "enable pad hold ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_rtc_hold_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_rtc_hold_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_RTC_HOLD_CTRL0_SPEC;
impl crate::RegisterSpec for PAD_RTC_HOLD_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_rtc_hold_ctrl0::R`](R) reader structure"]
impl crate::Readable for PAD_RTC_HOLD_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_rtc_hold_ctrl0::W`](W) writer structure"]
impl crate::Writable for PAD_RTC_HOLD_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_RTC_HOLD_CTRL0 to value 0"]
impl crate::Resettable for PAD_RTC_HOLD_CTRL0_SPEC {}
