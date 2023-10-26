#[doc = "Register `T%sCONFIG` reader"]
pub type R = crate::R<TCONFIG_SPEC>;
#[doc = "Register `T%sCONFIG` writer"]
pub type W = crate::W<TCONFIG_SPEC>;
#[doc = "Field `USE_XTAL` reader - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type USE_XTAL_R = crate::BitReader;
#[doc = "Field `USE_XTAL` writer - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type USE_XTAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALARM_EN` reader - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type ALARM_EN_R = crate::BitReader;
#[doc = "Field `ALARM_EN` writer - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type ALARM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DIVIDER` reader - Timer %s clock (T%s_clk) prescaler value."]
pub type DIVIDER_R = crate::FieldReader<u16>;
#[doc = "Field `DIVIDER` writer - Timer %s clock (T%s_clk) prescaler value."]
pub type DIVIDER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `AUTORELOAD` reader - When set, timer %s auto-reload at alarm is enabled."]
pub type AUTORELOAD_R = crate::BitReader;
#[doc = "Field `AUTORELOAD` writer - When set, timer %s auto-reload at alarm is enabled."]
pub type AUTORELOAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INCREASE` reader - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
pub type INCREASE_R = crate::BitReader;
#[doc = "Field `INCREASE` writer - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
pub type INCREASE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EN` reader - When set, the timer %s time-base counter is enabled."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - When set, the timer %s time-base counter is enabled."]
pub type EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    pub fn use_xtal(&self) -> USE_XTAL_R {
        USE_XTAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
    #[inline(always)]
    pub fn alarm_en(&self) -> ALARM_EN_R {
        ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:28 - Timer %s clock (T%s_clk) prescaler value."]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - When set, timer %s auto-reload at alarm is enabled."]
    #[inline(always)]
    pub fn autoreload(&self) -> AUTORELOAD_R {
        AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
    #[inline(always)]
    pub fn increase(&self) -> INCREASE_R {
        INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set, the timer %s time-base counter is enabled."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCONFIG")
            .field("use_xtal", &format_args!("{}", self.use_xtal().bit()))
            .field("alarm_en", &format_args!("{}", self.alarm_en().bit()))
            .field("divider", &format_args!("{}", self.divider().bits()))
            .field("autoreload", &format_args!("{}", self.autoreload().bit()))
            .field("increase", &format_args!("{}", self.increase().bit()))
            .field("en", &format_args!("{}", self.en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TCONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    #[must_use]
    pub fn use_xtal(&mut self) -> USE_XTAL_W<TCONFIG_SPEC, 9> {
        USE_XTAL_W::new(self)
    }
    #[doc = "Bit 10 - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_en(&mut self) -> ALARM_EN_W<TCONFIG_SPEC, 10> {
        ALARM_EN_W::new(self)
    }
    #[doc = "Bits 13:28 - Timer %s clock (T%s_clk) prescaler value."]
    #[inline(always)]
    #[must_use]
    pub fn divider(&mut self) -> DIVIDER_W<TCONFIG_SPEC, 13> {
        DIVIDER_W::new(self)
    }
    #[doc = "Bit 29 - When set, timer %s auto-reload at alarm is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn autoreload(&mut self) -> AUTORELOAD_W<TCONFIG_SPEC, 29> {
        AUTORELOAD_W::new(self)
    }
    #[doc = "Bit 30 - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
    #[inline(always)]
    #[must_use]
    pub fn increase(&mut self) -> INCREASE_W<TCONFIG_SPEC, 30> {
        INCREASE_W::new(self)
    }
    #[doc = "Bit 31 - When set, the timer %s time-base counter is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<TCONFIG_SPEC, 31> {
        EN_W::new(self)
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
#[doc = "Timer %s configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCONFIG_SPEC;
impl crate::RegisterSpec for TCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tconfig::R`](R) reader structure"]
impl crate::Readable for TCONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tconfig::W`](W) writer structure"]
impl crate::Writable for TCONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T%sCONFIG to value 0x6000_2000"]
impl crate::Resettable for TCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_2000;
}
