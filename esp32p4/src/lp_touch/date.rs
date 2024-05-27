///Register `DATE` reader
pub type R = crate::R<DATE_SPEC>;
///Register `DATE` writer
pub type W = crate::W<DATE_SPEC>;
///Field `RTC_DATE` reader - need_des
pub type RTC_DATE_R = crate::FieldReader<u32>;
///Field `RTC_DATE` writer - need_des
pub type RTC_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
///Field `RTC_CLK_EN` reader - need_des
pub type RTC_CLK_EN_R = crate::BitReader;
///Field `RTC_CLK_EN` writer - need_des
pub type RTC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:27 - need_des
    #[inline(always)]
    pub fn rtc_date(&self) -> RTC_DATE_R {
        RTC_DATE_R::new(self.bits & 0x0fff_ffff)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn rtc_clk_en(&self) -> RTC_CLK_EN_R {
        RTC_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field("rtc_date", &self.rtc_date())
            .field("rtc_clk_en", &self.rtc_clk_en())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - need_des
    #[inline(always)]
    #[must_use]
    pub fn rtc_date(&mut self) -> RTC_DATE_W<DATE_SPEC> {
        RTC_DATE_W::new(self, 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn rtc_clk_en(&mut self) -> RTC_CLK_EN_W<DATE_SPEC> {
        RTC_CLK_EN_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`date::R`](R) reader structure
impl crate::Readable for DATE_SPEC {}
///`write(|w| ..)` method takes [`date::W`](W) writer structure
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATE to value 0x0023_0314
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0023_0314;
}
