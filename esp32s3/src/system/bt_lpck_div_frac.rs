#[doc = "Register `BT_LPCK_DIV_FRAC` reader"]
pub type R = crate::R<BT_LPCK_DIV_FRAC_SPEC>;
#[doc = "Register `BT_LPCK_DIV_FRAC` writer"]
pub type W = crate::W<BT_LPCK_DIV_FRAC_SPEC>;
#[doc = "Field `BT_LPCK_DIV_B` reader - This field is lower power clock frequent division factor b"]
pub type BT_LPCK_DIV_B_R = crate::FieldReader<u16>;
#[doc = "Field `BT_LPCK_DIV_B` writer - This field is lower power clock frequent division factor b"]
pub type BT_LPCK_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BT_LPCK_DIV_A` reader - This field is lower power clock frequent division factor a"]
pub type BT_LPCK_DIV_A_R = crate::FieldReader<u16>;
#[doc = "Field `BT_LPCK_DIV_A` writer - This field is lower power clock frequent division factor a"]
pub type BT_LPCK_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LPCLK_SEL_RTC_SLOW` reader - Set 1 to select rtc-slow clock as rtc low power clock"]
pub type LPCLK_SEL_RTC_SLOW_R = crate::BitReader;
#[doc = "Field `LPCLK_SEL_RTC_SLOW` writer - Set 1 to select rtc-slow clock as rtc low power clock"]
pub type LPCLK_SEL_RTC_SLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCLK_SEL_8M` reader - Set 1 to select 8m clock as rtc low power clock"]
pub type LPCLK_SEL_8M_R = crate::BitReader;
#[doc = "Field `LPCLK_SEL_8M` writer - Set 1 to select 8m clock as rtc low power clock"]
pub type LPCLK_SEL_8M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCLK_SEL_XTAL` reader - Set 1 to select xtal clock as rtc low power clock"]
pub type LPCLK_SEL_XTAL_R = crate::BitReader;
#[doc = "Field `LPCLK_SEL_XTAL` writer - Set 1 to select xtal clock as rtc low power clock"]
pub type LPCLK_SEL_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCLK_SEL_XTAL32K` reader - Set 1 to select xtal32k clock as low power clock"]
pub type LPCLK_SEL_XTAL32K_R = crate::BitReader;
#[doc = "Field `LPCLK_SEL_XTAL32K` writer - Set 1 to select xtal32k clock as low power clock"]
pub type LPCLK_SEL_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCLK_RTC_EN` reader - Set 1 to enable RTC low power clock"]
pub type LPCLK_RTC_EN_R = crate::BitReader;
#[doc = "Field `LPCLK_RTC_EN` writer - Set 1 to enable RTC low power clock"]
pub type LPCLK_RTC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - This field is lower power clock frequent division factor b"]
    #[inline(always)]
    pub fn bt_lpck_div_b(&self) -> BT_LPCK_DIV_B_R {
        BT_LPCK_DIV_B_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - This field is lower power clock frequent division factor a"]
    #[inline(always)]
    pub fn bt_lpck_div_a(&self) -> BT_LPCK_DIV_A_R {
        BT_LPCK_DIV_A_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - Set 1 to select rtc-slow clock as rtc low power clock"]
    #[inline(always)]
    pub fn lpclk_sel_rtc_slow(&self) -> LPCLK_SEL_RTC_SLOW_R {
        LPCLK_SEL_RTC_SLOW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set 1 to select 8m clock as rtc low power clock"]
    #[inline(always)]
    pub fn lpclk_sel_8m(&self) -> LPCLK_SEL_8M_R {
        LPCLK_SEL_8M_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set 1 to select xtal clock as rtc low power clock"]
    #[inline(always)]
    pub fn lpclk_sel_xtal(&self) -> LPCLK_SEL_XTAL_R {
        LPCLK_SEL_XTAL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set 1 to select xtal32k clock as low power clock"]
    #[inline(always)]
    pub fn lpclk_sel_xtal32k(&self) -> LPCLK_SEL_XTAL32K_R {
        LPCLK_SEL_XTAL32K_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set 1 to enable RTC low power clock"]
    #[inline(always)]
    pub fn lpclk_rtc_en(&self) -> LPCLK_RTC_EN_R {
        LPCLK_RTC_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_LPCK_DIV_FRAC")
            .field(
                "bt_lpck_div_b",
                &format_args!("{}", self.bt_lpck_div_b().bits()),
            )
            .field(
                "bt_lpck_div_a",
                &format_args!("{}", self.bt_lpck_div_a().bits()),
            )
            .field(
                "lpclk_sel_rtc_slow",
                &format_args!("{}", self.lpclk_sel_rtc_slow().bit()),
            )
            .field(
                "lpclk_sel_8m",
                &format_args!("{}", self.lpclk_sel_8m().bit()),
            )
            .field(
                "lpclk_sel_xtal",
                &format_args!("{}", self.lpclk_sel_xtal().bit()),
            )
            .field(
                "lpclk_sel_xtal32k",
                &format_args!("{}", self.lpclk_sel_xtal32k().bit()),
            )
            .field(
                "lpclk_rtc_en",
                &format_args!("{}", self.lpclk_rtc_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BT_LPCK_DIV_FRAC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:11 - This field is lower power clock frequent division factor b"]
    #[inline(always)]
    #[must_use]
    pub fn bt_lpck_div_b(&mut self) -> BT_LPCK_DIV_B_W<BT_LPCK_DIV_FRAC_SPEC> {
        BT_LPCK_DIV_B_W::new(self, 0)
    }
    #[doc = "Bits 12:23 - This field is lower power clock frequent division factor a"]
    #[inline(always)]
    #[must_use]
    pub fn bt_lpck_div_a(&mut self) -> BT_LPCK_DIV_A_W<BT_LPCK_DIV_FRAC_SPEC> {
        BT_LPCK_DIV_A_W::new(self, 12)
    }
    #[doc = "Bit 24 - Set 1 to select rtc-slow clock as rtc low power clock"]
    #[inline(always)]
    #[must_use]
    pub fn lpclk_sel_rtc_slow(&mut self) -> LPCLK_SEL_RTC_SLOW_W<BT_LPCK_DIV_FRAC_SPEC> {
        LPCLK_SEL_RTC_SLOW_W::new(self, 24)
    }
    #[doc = "Bit 25 - Set 1 to select 8m clock as rtc low power clock"]
    #[inline(always)]
    #[must_use]
    pub fn lpclk_sel_8m(&mut self) -> LPCLK_SEL_8M_W<BT_LPCK_DIV_FRAC_SPEC> {
        LPCLK_SEL_8M_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set 1 to select xtal clock as rtc low power clock"]
    #[inline(always)]
    #[must_use]
    pub fn lpclk_sel_xtal(&mut self) -> LPCLK_SEL_XTAL_W<BT_LPCK_DIV_FRAC_SPEC> {
        LPCLK_SEL_XTAL_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set 1 to select xtal32k clock as low power clock"]
    #[inline(always)]
    #[must_use]
    pub fn lpclk_sel_xtal32k(&mut self) -> LPCLK_SEL_XTAL32K_W<BT_LPCK_DIV_FRAC_SPEC> {
        LPCLK_SEL_XTAL32K_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set 1 to enable RTC low power clock"]
    #[inline(always)]
    #[must_use]
    pub fn lpclk_rtc_en(&mut self) -> LPCLK_RTC_EN_W<BT_LPCK_DIV_FRAC_SPEC> {
        LPCLK_RTC_EN_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "low power clock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_lpck_div_frac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_lpck_div_frac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BT_LPCK_DIV_FRAC_SPEC;
impl crate::RegisterSpec for BT_LPCK_DIV_FRAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_lpck_div_frac::R`](R) reader structure"]
impl crate::Readable for BT_LPCK_DIV_FRAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bt_lpck_div_frac::W`](W) writer structure"]
impl crate::Writable for BT_LPCK_DIV_FRAC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BT_LPCK_DIV_FRAC to value 0x0200_1001"]
impl crate::Resettable for BT_LPCK_DIV_FRAC_SPEC {
    const RESET_VALUE: u32 = 0x0200_1001;
}
