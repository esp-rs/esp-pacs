#[doc = "Register `TOUCH_PAD_RTC_EN_AMUX` reader"]
pub type R = crate::R<TOUCH_PAD_RTC_EN_AMUX_SPEC>;
#[doc = "Register `TOUCH_PAD_RTC_EN_AMUX` writer"]
pub type W = crate::W<TOUCH_PAD_RTC_EN_AMUX_SPEC>;
#[doc = "Field `TOUCH_PAD_RTC_EN_AMUX` reader - "]
pub type TOUCH_PAD_RTC_EN_AMUX_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_PAD_RTC_EN_AMUX` writer - "]
pub type TOUCH_PAD_RTC_EN_AMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn touch_pad_rtc_en_amux(&self) -> TOUCH_PAD_RTC_EN_AMUX_R {
        TOUCH_PAD_RTC_EN_AMUX_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_PAD_RTC_EN_AMUX")
            .field("touch_pad_rtc_en_amux", &self.touch_pad_rtc_en_amux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn touch_pad_rtc_en_amux(
        &mut self,
    ) -> TOUCH_PAD_RTC_EN_AMUX_W<'_, TOUCH_PAD_RTC_EN_AMUX_SPEC> {
        TOUCH_PAD_RTC_EN_AMUX_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_pad_rtc_en_amux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_pad_rtc_en_amux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_PAD_RTC_EN_AMUX_SPEC;
impl crate::RegisterSpec for TOUCH_PAD_RTC_EN_AMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_pad_rtc_en_amux::R`](R) reader structure"]
impl crate::Readable for TOUCH_PAD_RTC_EN_AMUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_pad_rtc_en_amux::W`](W) writer structure"]
impl crate::Writable for TOUCH_PAD_RTC_EN_AMUX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_PAD_RTC_EN_AMUX to value 0"]
impl crate::Resettable for TOUCH_PAD_RTC_EN_AMUX_SPEC {}
