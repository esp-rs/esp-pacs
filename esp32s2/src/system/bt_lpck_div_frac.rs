#[doc = "Register `BT_LPCK_DIV_FRAC` reader"]
pub type R = crate::R<BT_LPCK_DIV_FRAC_SPEC>;
#[doc = "Register `BT_LPCK_DIV_FRAC` writer"]
pub type W = crate::W<BT_LPCK_DIV_FRAC_SPEC>;
#[doc = "Field `LPCLK_SEL_RTC_SLOW` reader - Set this bit to select RTC slow clock as the low power clock."]
pub type LPCLK_SEL_RTC_SLOW_R = crate::BitReader;
#[doc = "Field `LPCLK_SEL_RTC_SLOW` writer - Set this bit to select RTC slow clock as the low power clock."]
pub type LPCLK_SEL_RTC_SLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCLK_SEL_8M` reader - Set this bit to select 8m clock as the low power clock."]
pub type LPCLK_SEL_8M_R = crate::BitReader;
#[doc = "Field `LPCLK_SEL_8M` writer - Set this bit to select 8m clock as the low power clock."]
pub type LPCLK_SEL_8M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCLK_SEL_XTAL` reader - Set this bit to select xtal clock as the low power clock."]
pub type LPCLK_SEL_XTAL_R = crate::BitReader;
#[doc = "Field `LPCLK_SEL_XTAL` writer - Set this bit to select xtal clock as the low power clock."]
pub type LPCLK_SEL_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCLK_SEL_XTAL32K` reader - Set this bit to select xtal32k clock as the low power clock."]
pub type LPCLK_SEL_XTAL32K_R = crate::BitReader;
#[doc = "Field `LPCLK_SEL_XTAL32K` writer - Set this bit to select xtal32k clock as the low power clock."]
pub type LPCLK_SEL_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCLK_RTC_EN` reader - Set this bit to enable the RTC low power clock."]
pub type LPCLK_RTC_EN_R = crate::BitReader;
#[doc = "Field `LPCLK_RTC_EN` writer - Set this bit to enable the RTC low power clock."]
pub type LPCLK_RTC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - Set this bit to select RTC slow clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_rtc_slow(&self) -> LPCLK_SEL_RTC_SLOW_R {
        LPCLK_SEL_RTC_SLOW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set this bit to select 8m clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_8m(&self) -> LPCLK_SEL_8M_R {
        LPCLK_SEL_8M_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to select xtal clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_xtal(&self) -> LPCLK_SEL_XTAL_R {
        LPCLK_SEL_XTAL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to select xtal32k clock as the low power clock."]
    #[inline(always)]
    pub fn lpclk_sel_xtal32k(&self) -> LPCLK_SEL_XTAL32K_R {
        LPCLK_SEL_XTAL32K_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to enable the RTC low power clock."]
    #[inline(always)]
    pub fn lpclk_rtc_en(&self) -> LPCLK_RTC_EN_R {
        LPCLK_RTC_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_LPCK_DIV_FRAC")
            .field("lpclk_sel_rtc_slow", &self.lpclk_sel_rtc_slow())
            .field("lpclk_sel_8m", &self.lpclk_sel_8m())
            .field("lpclk_sel_xtal", &self.lpclk_sel_xtal())
            .field("lpclk_sel_xtal32k", &self.lpclk_sel_xtal32k())
            .field("lpclk_rtc_en", &self.lpclk_rtc_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 24 - Set this bit to select RTC slow clock as the low power clock."]
    #[inline(always)]
    #[must_use]
    pub fn lpclk_sel_rtc_slow(&mut self) -> LPCLK_SEL_RTC_SLOW_W<BT_LPCK_DIV_FRAC_SPEC> {
        LPCLK_SEL_RTC_SLOW_W::new(self, 24)
    }
    #[doc = "Bit 25 - Set this bit to select 8m clock as the low power clock."]
    #[inline(always)]
    #[must_use]
    pub fn lpclk_sel_8m(&mut self) -> LPCLK_SEL_8M_W<BT_LPCK_DIV_FRAC_SPEC> {
        LPCLK_SEL_8M_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to select xtal clock as the low power clock."]
    #[inline(always)]
    #[must_use]
    pub fn lpclk_sel_xtal(&mut self) -> LPCLK_SEL_XTAL_W<BT_LPCK_DIV_FRAC_SPEC> {
        LPCLK_SEL_XTAL_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set this bit to select xtal32k clock as the low power clock."]
    #[inline(always)]
    #[must_use]
    pub fn lpclk_sel_xtal32k(&mut self) -> LPCLK_SEL_XTAL32K_W<BT_LPCK_DIV_FRAC_SPEC> {
        LPCLK_SEL_XTAL32K_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set this bit to enable the RTC low power clock."]
    #[inline(always)]
    #[must_use]
    pub fn lpclk_rtc_en(&mut self) -> LPCLK_RTC_EN_W<BT_LPCK_DIV_FRAC_SPEC> {
        LPCLK_RTC_EN_W::new(self, 28)
    }
}
#[doc = "Divider fraction configuration register for low-power clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_lpck_div_frac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_lpck_div_frac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BT_LPCK_DIV_FRAC_SPEC;
impl crate::RegisterSpec for BT_LPCK_DIV_FRAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_lpck_div_frac::R`](R) reader structure"]
impl crate::Readable for BT_LPCK_DIV_FRAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bt_lpck_div_frac::W`](W) writer structure"]
impl crate::Writable for BT_LPCK_DIV_FRAC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BT_LPCK_DIV_FRAC to value 0x0200_0000"]
impl crate::Resettable for BT_LPCK_DIV_FRAC_SPEC {
    const RESET_VALUE: u32 = 0x0200_0000;
}
