#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `RTC_DATE` reader - need_des"]
pub type RTC_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `RTC_DATE` writer - need_des"]
pub type RTC_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `RTC_CLK_EN` reader - need_des"]
pub type RTC_CLK_EN_R = crate::BitReader;
#[doc = "Field `RTC_CLK_EN` writer - need_des"]
pub type RTC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27 - need_des"]
    #[inline(always)]
    pub fn rtc_date(&self) -> RTC_DATE_R {
        RTC_DATE_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn rtc_clk_en(&self) -> RTC_CLK_EN_R {
        RTC_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("rtc_date", &self.rtc_date().bits())
            .field("rtc_clk_en", &self.rtc_clk_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_date(&mut self) -> RTC_DATE_W<DATE_SPEC> {
        RTC_DATE_W::new(self, 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_clk_en(&mut self) -> RTC_CLK_EN_W<DATE_SPEC> {
        RTC_CLK_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATE to value 0x0023_0314"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0023_0314;
}
