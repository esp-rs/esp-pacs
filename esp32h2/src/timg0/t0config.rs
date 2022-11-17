#[doc = "Register `T0CONFIG` reader"]
pub struct R(crate::R<T0CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0CONFIG` writer"]
pub struct W(crate::W<T0CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0CONFIG_SPEC>;
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
impl From<crate::W<T0CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_USE_XTAL` reader - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type T_USE_XTAL_R = crate::BitReader<bool>;
#[doc = "Field `T_USE_XTAL` writer - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type T_USE_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
#[doc = "Field `T_ALARM_EN` reader - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type T_ALARM_EN_R = crate::BitReader<bool>;
#[doc = "Field `T_ALARM_EN` writer - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type T_ALARM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
#[doc = "Field `T_DIVCNT_RST` writer - When set, Timer %s 's clock divider counter will be reset."]
pub type T_DIVCNT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
#[doc = "Field `T_DIVIDER` reader - Timer %s clock (T%s_clk) prescaler value."]
pub type T_DIVIDER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_DIVIDER` writer - Timer %s clock (T%s_clk) prescaler value."]
pub type T_DIVIDER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0CONFIG_SPEC, u16, u16, 16, O>;
#[doc = "Field `T_AUTORELOAD` reader - When set, timer %s auto-reload at alarm is enabled."]
pub type T_AUTORELOAD_R = crate::BitReader<bool>;
#[doc = "Field `T_AUTORELOAD` writer - When set, timer %s auto-reload at alarm is enabled."]
pub type T_AUTORELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
#[doc = "Field `T_INCREASE` reader - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
pub type T_INCREASE_R = crate::BitReader<bool>;
#[doc = "Field `T_INCREASE` writer - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
pub type T_INCREASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
#[doc = "Field `T_EN` reader - When set, the timer %s time-base counter is enabled."]
pub type T_EN_R = crate::BitReader<bool>;
#[doc = "Field `T_EN` writer - When set, the timer %s time-base counter is enabled."]
pub type T_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, T0CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    pub fn t_use_xtal(&self) -> T_USE_XTAL_R {
        T_USE_XTAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
    #[inline(always)]
    pub fn t_alarm_en(&self) -> T_ALARM_EN_R {
        T_ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:28 - Timer %s clock (T%s_clk) prescaler value."]
    #[inline(always)]
    pub fn t_divider(&self) -> T_DIVIDER_R {
        T_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - When set, timer %s auto-reload at alarm is enabled."]
    #[inline(always)]
    pub fn t_autoreload(&self) -> T_AUTORELOAD_R {
        T_AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
    #[inline(always)]
    pub fn t_increase(&self) -> T_INCREASE_R {
        T_INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set, the timer %s time-base counter is enabled."]
    #[inline(always)]
    pub fn t_en(&self) -> T_EN_R {
        T_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    #[must_use]
    pub fn t_use_xtal(&mut self) -> T_USE_XTAL_W<9> {
        T_USE_XTAL_W::new(self)
    }
    #[doc = "Bit 10 - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
    #[inline(always)]
    #[must_use]
    pub fn t_alarm_en(&mut self) -> T_ALARM_EN_W<10> {
        T_ALARM_EN_W::new(self)
    }
    #[doc = "Bit 12 - When set, Timer %s 's clock divider counter will be reset."]
    #[inline(always)]
    #[must_use]
    pub fn t_divcnt_rst(&mut self) -> T_DIVCNT_RST_W<12> {
        T_DIVCNT_RST_W::new(self)
    }
    #[doc = "Bits 13:28 - Timer %s clock (T%s_clk) prescaler value."]
    #[inline(always)]
    #[must_use]
    pub fn t_divider(&mut self) -> T_DIVIDER_W<13> {
        T_DIVIDER_W::new(self)
    }
    #[doc = "Bit 29 - When set, timer %s auto-reload at alarm is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn t_autoreload(&mut self) -> T_AUTORELOAD_W<29> {
        T_AUTORELOAD_W::new(self)
    }
    #[doc = "Bit 30 - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
    #[inline(always)]
    #[must_use]
    pub fn t_increase(&mut self) -> T_INCREASE_W<30> {
        T_INCREASE_W::new(self)
    }
    #[doc = "Bit 31 - When set, the timer %s time-base counter is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn t_en(&mut self) -> T_EN_W<31> {
        T_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer %s configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0config](index.html) module"]
pub struct T0CONFIG_SPEC;
impl crate::RegisterSpec for T0CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0config::R](R) reader structure"]
impl crate::Readable for T0CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0config::W](W) writer structure"]
impl crate::Writable for T0CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0CONFIG to value 0x6000_2000"]
impl crate::Resettable for T0CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_2000;
}
