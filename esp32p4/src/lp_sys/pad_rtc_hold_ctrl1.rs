#[doc = "Register `PAD_RTC_HOLD_CTRL1` reader"]
pub type R = crate::R<PAD_RTC_HOLD_CTRL1_SPEC>;
#[doc = "Register `PAD_RTC_HOLD_CTRL1` writer"]
pub type W = crate::W<PAD_RTC_HOLD_CTRL1_SPEC>;
#[doc = "Field `PAD_RTC_HOLD_CTRL1` reader - Set 1 to hold pad 32-56 status"]
pub type PAD_RTC_HOLD_CTRL1_R = crate::FieldReader<u32>;
#[doc = "Field `PAD_RTC_HOLD_CTRL1` writer - Set 1 to hold pad 32-56 status"]
pub type PAD_RTC_HOLD_CTRL1_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Set 1 to hold pad 32-56 status"]
    #[inline(always)]
    pub fn pad_rtc_hold_ctrl1(&self) -> PAD_RTC_HOLD_CTRL1_R {
        PAD_RTC_HOLD_CTRL1_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_RTC_HOLD_CTRL1")
            .field("pad_rtc_hold_ctrl1", &self.pad_rtc_hold_ctrl1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:24 - Set 1 to hold pad 32-56 status"]
    #[inline(always)]
    pub fn pad_rtc_hold_ctrl1(&mut self) -> PAD_RTC_HOLD_CTRL1_W<'_, PAD_RTC_HOLD_CTRL1_SPEC> {
        PAD_RTC_HOLD_CTRL1_W::new(self, 0)
    }
}
#[doc = "enable pad hold ctrl\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_rtc_hold_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_rtc_hold_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_RTC_HOLD_CTRL1_SPEC;
impl crate::RegisterSpec for PAD_RTC_HOLD_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_rtc_hold_ctrl1::R`](R) reader structure"]
impl crate::Readable for PAD_RTC_HOLD_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_rtc_hold_ctrl1::W`](W) writer structure"]
impl crate::Writable for PAD_RTC_HOLD_CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_RTC_HOLD_CTRL1 to value 0"]
impl crate::Resettable for PAD_RTC_HOLD_CTRL1_SPEC {}
