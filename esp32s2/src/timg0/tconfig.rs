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
#[doc = "Field `USE_XTAL` reader - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type USE_XTAL_R = crate::BitReader;
#[doc = "Field `USE_XTAL` writer - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type USE_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, TCONFIG_SPEC, O>;
#[doc = "Field `ALARM_EN` reader - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type ALARM_EN_R = crate::BitReader;
#[doc = "Field `ALARM_EN` writer - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type ALARM_EN_W<'a, const O: u8> = crate::BitWriter<'a, TCONFIG_SPEC, O>;
#[doc = "Field `LEVEL_INT_EN` reader - When set, an alarm will generate a level type interrupt."]
pub type LEVEL_INT_EN_R = crate::BitReader;
#[doc = "Field `LEVEL_INT_EN` writer - When set, an alarm will generate a level type interrupt."]
pub type LEVEL_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, TCONFIG_SPEC, O>;
#[doc = "Field `EDGE_INT_EN` reader - When set, an alarm will generate an edge type interrupt."]
pub type EDGE_INT_EN_R = crate::BitReader;
#[doc = "Field `EDGE_INT_EN` writer - When set, an alarm will generate an edge type interrupt."]
pub type EDGE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, TCONFIG_SPEC, O>;
#[doc = "Field `DIVIDER` reader - Timer %s clock (T%s_clk) prescaler value."]
pub type DIVIDER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIVIDER` writer - Timer %s clock (T%s_clk) prescaler value."]
pub type DIVIDER_W<'a, const O: u8> = crate::FieldWriter<'a, TCONFIG_SPEC, 16, O, u16, u16>;
#[doc = "Field `AUTORELOAD` reader - When set, timer %s auto-reload at alarm is enabled."]
pub type AUTORELOAD_R = crate::BitReader;
#[doc = "Field `AUTORELOAD` writer - When set, timer %s auto-reload at alarm is enabled."]
pub type AUTORELOAD_W<'a, const O: u8> = crate::BitWriter<'a, TCONFIG_SPEC, O>;
#[doc = "Field `INCREASE` reader - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
pub type INCREASE_R = crate::BitReader;
#[doc = "Field `INCREASE` writer - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
pub type INCREASE_W<'a, const O: u8> = crate::BitWriter<'a, TCONFIG_SPEC, O>;
#[doc = "Field `EN` reader - When set, the timer %s time-base counter is enabled."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - When set, the timer %s time-base counter is enabled."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, TCONFIG_SPEC, O>;
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
    #[doc = "Bit 11 - When set, an alarm will generate a level type interrupt."]
    #[inline(always)]
    pub fn level_int_en(&self) -> LEVEL_INT_EN_R {
        LEVEL_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When set, an alarm will generate an edge type interrupt."]
    #[inline(always)]
    pub fn edge_int_en(&self) -> EDGE_INT_EN_R {
        EDGE_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
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
            .field(
                "level_int_en",
                &format_args!("{}", self.level_int_en().bit()),
            )
            .field("edge_int_en", &format_args!("{}", self.edge_int_en().bit()))
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
    pub fn use_xtal(&mut self) -> USE_XTAL_W<9> {
        USE_XTAL_W::new(self)
    }
    #[doc = "Bit 10 - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_en(&mut self) -> ALARM_EN_W<10> {
        ALARM_EN_W::new(self)
    }
    #[doc = "Bit 11 - When set, an alarm will generate a level type interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn level_int_en(&mut self) -> LEVEL_INT_EN_W<11> {
        LEVEL_INT_EN_W::new(self)
    }
    #[doc = "Bit 12 - When set, an alarm will generate an edge type interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn edge_int_en(&mut self) -> EDGE_INT_EN_W<12> {
        EDGE_INT_EN_W::new(self)
    }
    #[doc = "Bits 13:28 - Timer %s clock (T%s_clk) prescaler value."]
    #[inline(always)]
    #[must_use]
    pub fn divider(&mut self) -> DIVIDER_W<13> {
        DIVIDER_W::new(self)
    }
    #[doc = "Bit 29 - When set, timer %s auto-reload at alarm is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn autoreload(&mut self) -> AUTORELOAD_W<29> {
        AUTORELOAD_W::new(self)
    }
    #[doc = "Bit 30 - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
    #[inline(always)]
    #[must_use]
    pub fn increase(&mut self) -> INCREASE_W<30> {
        INCREASE_W::new(self)
    }
    #[doc = "Bit 31 - When set, the timer %s time-base counter is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T%sCONFIG to value 0x6000_2000"]
impl crate::Resettable for TCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_2000;
}
