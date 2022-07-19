#[doc = "Register `BT_LPCK_DIV_FRAC` reader"]
pub struct R(crate::R<BT_LPCK_DIV_FRAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BT_LPCK_DIV_FRAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BT_LPCK_DIV_FRAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BT_LPCK_DIV_FRAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BT_LPCK_DIV_FRAC` writer"]
pub struct W(crate::W<BT_LPCK_DIV_FRAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BT_LPCK_DIV_FRAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BT_LPCK_DIV_FRAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BT_LPCK_DIV_FRAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BT_LPCK_DIV_B` reader - This field is lower power clock frequent division factor b"]
pub type BT_LPCK_DIV_B_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BT_LPCK_DIV_B` writer - This field is lower power clock frequent division factor b"]
pub type BT_LPCK_DIV_B_W<'a> = crate::FieldWriter<'a, u32, BT_LPCK_DIV_FRAC_SPEC, u16, u16, 12, 0>;
#[doc = "Field `BT_LPCK_DIV_A` reader - This field is lower power clock frequent division factor a"]
pub type BT_LPCK_DIV_A_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BT_LPCK_DIV_A` writer - This field is lower power clock frequent division factor a"]
pub type BT_LPCK_DIV_A_W<'a> = crate::FieldWriter<'a, u32, BT_LPCK_DIV_FRAC_SPEC, u16, u16, 12, 12>;
#[doc = "Field `LPCLK_SEL_RTC_SLOW` reader - Set 1 to select rtc-slow clock as rtc low power clock"]
pub type LPCLK_SEL_RTC_SLOW_R = crate::BitReader<bool>;
#[doc = "Field `LPCLK_SEL_RTC_SLOW` writer - Set 1 to select rtc-slow clock as rtc low power clock"]
pub type LPCLK_SEL_RTC_SLOW_W<'a> = crate::BitWriter<'a, u32, BT_LPCK_DIV_FRAC_SPEC, bool, 24>;
#[doc = "Field `LPCLK_SEL_8M` reader - Set 1 to select 8m clock as rtc low power clock"]
pub type LPCLK_SEL_8M_R = crate::BitReader<bool>;
#[doc = "Field `LPCLK_SEL_8M` writer - Set 1 to select 8m clock as rtc low power clock"]
pub type LPCLK_SEL_8M_W<'a> = crate::BitWriter<'a, u32, BT_LPCK_DIV_FRAC_SPEC, bool, 25>;
#[doc = "Field `LPCLK_SEL_XTAL` reader - Set 1 to select xtal clock as rtc low power clock"]
pub type LPCLK_SEL_XTAL_R = crate::BitReader<bool>;
#[doc = "Field `LPCLK_SEL_XTAL` writer - Set 1 to select xtal clock as rtc low power clock"]
pub type LPCLK_SEL_XTAL_W<'a> = crate::BitWriter<'a, u32, BT_LPCK_DIV_FRAC_SPEC, bool, 26>;
#[doc = "Field `LPCLK_SEL_XTAL32K` reader - Set 1 to select xtal32k clock as low power clock"]
pub type LPCLK_SEL_XTAL32K_R = crate::BitReader<bool>;
#[doc = "Field `LPCLK_SEL_XTAL32K` writer - Set 1 to select xtal32k clock as low power clock"]
pub type LPCLK_SEL_XTAL32K_W<'a> = crate::BitWriter<'a, u32, BT_LPCK_DIV_FRAC_SPEC, bool, 27>;
#[doc = "Field `LPCLK_RTC_EN` reader - Set 1 to enable RTC low power clock"]
pub type LPCLK_RTC_EN_R = crate::BitReader<bool>;
#[doc = "Field `LPCLK_RTC_EN` writer - Set 1 to enable RTC low power clock"]
pub type LPCLK_RTC_EN_W<'a> = crate::BitWriter<'a, u32, BT_LPCK_DIV_FRAC_SPEC, bool, 28>;
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
impl W {
    #[doc = "Bits 0:11 - This field is lower power clock frequent division factor b"]
    #[inline(always)]
    pub fn bt_lpck_div_b(&mut self) -> BT_LPCK_DIV_B_W {
        BT_LPCK_DIV_B_W::new(self)
    }
    #[doc = "Bits 12:23 - This field is lower power clock frequent division factor a"]
    #[inline(always)]
    pub fn bt_lpck_div_a(&mut self) -> BT_LPCK_DIV_A_W {
        BT_LPCK_DIV_A_W::new(self)
    }
    #[doc = "Bit 24 - Set 1 to select rtc-slow clock as rtc low power clock"]
    #[inline(always)]
    pub fn lpclk_sel_rtc_slow(&mut self) -> LPCLK_SEL_RTC_SLOW_W {
        LPCLK_SEL_RTC_SLOW_W::new(self)
    }
    #[doc = "Bit 25 - Set 1 to select 8m clock as rtc low power clock"]
    #[inline(always)]
    pub fn lpclk_sel_8m(&mut self) -> LPCLK_SEL_8M_W {
        LPCLK_SEL_8M_W::new(self)
    }
    #[doc = "Bit 26 - Set 1 to select xtal clock as rtc low power clock"]
    #[inline(always)]
    pub fn lpclk_sel_xtal(&mut self) -> LPCLK_SEL_XTAL_W {
        LPCLK_SEL_XTAL_W::new(self)
    }
    #[doc = "Bit 27 - Set 1 to select xtal32k clock as low power clock"]
    #[inline(always)]
    pub fn lpclk_sel_xtal32k(&mut self) -> LPCLK_SEL_XTAL32K_W {
        LPCLK_SEL_XTAL32K_W::new(self)
    }
    #[doc = "Bit 28 - Set 1 to enable RTC low power clock"]
    #[inline(always)]
    pub fn lpclk_rtc_en(&mut self) -> LPCLK_RTC_EN_W {
        LPCLK_RTC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "low power clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bt_lpck_div_frac](index.html) module"]
pub struct BT_LPCK_DIV_FRAC_SPEC;
impl crate::RegisterSpec for BT_LPCK_DIV_FRAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bt_lpck_div_frac::R](R) reader structure"]
impl crate::Readable for BT_LPCK_DIV_FRAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bt_lpck_div_frac::W](W) writer structure"]
impl crate::Writable for BT_LPCK_DIV_FRAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BT_LPCK_DIV_FRAC to value 0x0200_1001"]
impl crate::Resettable for BT_LPCK_DIV_FRAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_1001
    }
}
