#[doc = "Register `T%sCONFIG` reader"]
pub struct R(crate::R<TCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T%sCONFIG` writer"]
pub struct W(crate::W<TCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCONFIG_SPEC>;
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
impl From<crate::W<TCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_USE_XTAL` reader - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type T0_USE_XTAL_R = crate::BitReader<bool>;
#[doc = "Field `T0_USE_XTAL` writer - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type T0_USE_XTAL_W<'a> = crate::BitWriter<'a, u32, TCONFIG_SPEC, bool, 9>;
#[doc = "Field `T0_ALARM_EN` reader - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type T0_ALARM_EN_R = crate::BitReader<bool>;
#[doc = "Field `T0_ALARM_EN` writer - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type T0_ALARM_EN_W<'a> = crate::BitWriter<'a, u32, TCONFIG_SPEC, bool, 10>;
#[doc = "Field `T0_DIVCNT_RST` writer - When set, Timer %s 's clock divider counter will be reset."]
pub type T0_DIVCNT_RST_W<'a> = crate::BitWriter<'a, u32, TCONFIG_SPEC, bool, 12>;
#[doc = "Field `T0_DIVIDER` reader - Timer %s clock (T%s_clk) prescaler value."]
pub type T0_DIVIDER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T0_DIVIDER` writer - Timer %s clock (T%s_clk) prescaler value."]
pub type T0_DIVIDER_W<'a> = crate::FieldWriter<'a, u32, TCONFIG_SPEC, u16, u16, 16, 13>;
#[doc = "Field `T0_AUTORELOAD` reader - When set, timer %s auto-reload at alarm is enabled."]
pub type T0_AUTORELOAD_R = crate::BitReader<bool>;
#[doc = "Field `T0_AUTORELOAD` writer - When set, timer %s auto-reload at alarm is enabled."]
pub type T0_AUTORELOAD_W<'a> = crate::BitWriter<'a, u32, TCONFIG_SPEC, bool, 29>;
#[doc = "Field `T0_INCREASE` reader - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
pub type T0_INCREASE_R = crate::BitReader<bool>;
#[doc = "Field `T0_INCREASE` writer - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
pub type T0_INCREASE_W<'a> = crate::BitWriter<'a, u32, TCONFIG_SPEC, bool, 30>;
#[doc = "Field `T0_EN` reader - When set, the timer %s time-base counter is enabled."]
pub type T0_EN_R = crate::BitReader<bool>;
#[doc = "Field `T0_EN` writer - When set, the timer %s time-base counter is enabled."]
pub type T0_EN_W<'a> = crate::BitWriter<'a, u32, TCONFIG_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    pub fn t0_use_xtal(&self) -> T0_USE_XTAL_R {
        T0_USE_XTAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
    #[inline(always)]
    pub fn t0_alarm_en(&self) -> T0_ALARM_EN_R {
        T0_ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:28 - Timer %s clock (T%s_clk) prescaler value."]
    #[inline(always)]
    pub fn t0_divider(&self) -> T0_DIVIDER_R {
        T0_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - When set, timer %s auto-reload at alarm is enabled."]
    #[inline(always)]
    pub fn t0_autoreload(&self) -> T0_AUTORELOAD_R {
        T0_AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
    #[inline(always)]
    pub fn t0_increase(&self) -> T0_INCREASE_R {
        T0_INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set, the timer %s time-base counter is enabled."]
    #[inline(always)]
    pub fn t0_en(&self) -> T0_EN_R {
        T0_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    pub fn t0_use_xtal(&mut self) -> T0_USE_XTAL_W {
        T0_USE_XTAL_W::new(self)
    }
    #[doc = "Bit 10 - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
    #[inline(always)]
    pub fn t0_alarm_en(&mut self) -> T0_ALARM_EN_W {
        T0_ALARM_EN_W::new(self)
    }
    #[doc = "Bit 12 - When set, Timer %s 's clock divider counter will be reset."]
    #[inline(always)]
    pub fn t0_divcnt_rst(&mut self) -> T0_DIVCNT_RST_W {
        T0_DIVCNT_RST_W::new(self)
    }
    #[doc = "Bits 13:28 - Timer %s clock (T%s_clk) prescaler value."]
    #[inline(always)]
    pub fn t0_divider(&mut self) -> T0_DIVIDER_W {
        T0_DIVIDER_W::new(self)
    }
    #[doc = "Bit 29 - When set, timer %s auto-reload at alarm is enabled."]
    #[inline(always)]
    pub fn t0_autoreload(&mut self) -> T0_AUTORELOAD_W {
        T0_AUTORELOAD_W::new(self)
    }
    #[doc = "Bit 30 - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
    #[inline(always)]
    pub fn t0_increase(&mut self) -> T0_INCREASE_W {
        T0_INCREASE_W::new(self)
    }
    #[doc = "Bit 31 - When set, the timer %s time-base counter is enabled."]
    #[inline(always)]
    pub fn t0_en(&mut self) -> T0_EN_W {
        T0_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer %s configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tconfig](index.html) module"]
pub struct TCONFIG_SPEC;
impl crate::RegisterSpec for TCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tconfig::R](R) reader structure"]
impl crate::Readable for TCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tconfig::W](W) writer structure"]
impl crate::Writable for TCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T%sCONFIG to value 0x6000_2000"]
impl crate::Resettable for TCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6000_2000
    }
}
