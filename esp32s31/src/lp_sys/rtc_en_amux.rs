#[doc = "Register `RTC_EN_AMUX` reader"]
pub type R = crate::R<RTC_EN_AMUX_SPEC>;
#[doc = "Register `RTC_EN_AMUX` writer"]
pub type W = crate::W<RTC_EN_AMUX_SPEC>;
#[doc = "Field `RTC_EN_AMUX` reader - "]
pub type RTC_EN_AMUX_R = crate::FieldReader<u32>;
#[doc = "Field `RTC_EN_AMUX` writer - "]
pub type RTC_EN_AMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn rtc_en_amux(&self) -> RTC_EN_AMUX_R {
        RTC_EN_AMUX_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_EN_AMUX")
            .field("rtc_en_amux", &self.rtc_en_amux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn rtc_en_amux(&mut self) -> RTC_EN_AMUX_W<'_, RTC_EN_AMUX_SPEC> {
        RTC_EN_AMUX_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_en_amux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_en_amux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_EN_AMUX_SPEC;
impl crate::RegisterSpec for RTC_EN_AMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_en_amux::R`](R) reader structure"]
impl crate::Readable for RTC_EN_AMUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_en_amux::W`](W) writer structure"]
impl crate::Writable for RTC_EN_AMUX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_EN_AMUX to value 0"]
impl crate::Resettable for RTC_EN_AMUX_SPEC {}
