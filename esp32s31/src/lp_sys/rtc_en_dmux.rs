#[doc = "Register `RTC_EN_DMUX` reader"]
pub type R = crate::R<RTC_EN_DMUX_SPEC>;
#[doc = "Register `RTC_EN_DMUX` writer"]
pub type W = crate::W<RTC_EN_DMUX_SPEC>;
#[doc = "Field `RTC_EN_DMUX` reader - "]
pub type RTC_EN_DMUX_R = crate::FieldReader;
#[doc = "Field `RTC_EN_DMUX` writer - "]
pub type RTC_EN_DMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rtc_en_dmux(&self) -> RTC_EN_DMUX_R {
        RTC_EN_DMUX_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_EN_DMUX")
            .field("rtc_en_dmux", &self.rtc_en_dmux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rtc_en_dmux(&mut self) -> RTC_EN_DMUX_W<'_, RTC_EN_DMUX_SPEC> {
        RTC_EN_DMUX_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_en_dmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_en_dmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_EN_DMUX_SPEC;
impl crate::RegisterSpec for RTC_EN_DMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_en_dmux::R`](R) reader structure"]
impl crate::Readable for RTC_EN_DMUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_en_dmux::W`](W) writer structure"]
impl crate::Writable for RTC_EN_DMUX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_EN_DMUX to value 0"]
impl crate::Resettable for RTC_EN_DMUX_SPEC {}
