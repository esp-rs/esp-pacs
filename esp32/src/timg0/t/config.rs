///Register `CONFIG` reader
pub type R = crate::R<CONFIG_SPEC>;
///Register `CONFIG` writer
pub type W = crate::W<CONFIG_SPEC>;
///Field `ALARM_EN` reader - When set alarm is enabled
pub type ALARM_EN_R = crate::BitReader;
///Field `ALARM_EN` writer - When set alarm is enabled
pub type ALARM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LEVEL_INT_EN` reader - When set level type interrupt will be generated during alarm
pub type LEVEL_INT_EN_R = crate::BitReader;
///Field `LEVEL_INT_EN` writer - When set level type interrupt will be generated during alarm
pub type LEVEL_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EDGE_INT_EN` reader - When set edge type interrupt will be generated during alarm
pub type EDGE_INT_EN_R = crate::BitReader;
///Field `EDGE_INT_EN` writer - When set edge type interrupt will be generated during alarm
pub type EDGE_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVIDER` reader - Timer 0 clock (T0_clk) prescale value.
pub type DIVIDER_R = crate::FieldReader<u16>;
///Field `DIVIDER` writer - Timer 0 clock (T0_clk) prescale value.
pub type DIVIDER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `AUTORELOAD` reader - When set timer 0 auto-reload at alarming is enabled
pub type AUTORELOAD_R = crate::BitReader;
///Field `AUTORELOAD` writer - When set timer 0 auto-reload at alarming is enabled
pub type AUTORELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCREASE` reader - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement.
pub type INCREASE_R = crate::BitReader;
///Field `INCREASE` writer - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement.
pub type INCREASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN` reader - When set timer 0 time-base counter is enabled
pub type EN_R = crate::BitReader;
///Field `EN` writer - When set timer 0 time-base counter is enabled
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 10 - When set alarm is enabled
    #[inline(always)]
    pub fn alarm_en(&self) -> ALARM_EN_R {
        ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - When set level type interrupt will be generated during alarm
    #[inline(always)]
    pub fn level_int_en(&self) -> LEVEL_INT_EN_R {
        LEVEL_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - When set edge type interrupt will be generated during alarm
    #[inline(always)]
    pub fn edge_int_en(&self) -> EDGE_INT_EN_R {
        EDGE_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:28 - Timer 0 clock (T0_clk) prescale value.
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    ///Bit 29 - When set timer 0 auto-reload at alarming is enabled
    #[inline(always)]
    pub fn autoreload(&self) -> AUTORELOAD_R {
        AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement.
    #[inline(always)]
    pub fn increase(&self) -> INCREASE_R {
        INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - When set timer 0 time-base counter is enabled
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("alarm_en", &self.alarm_en())
            .field("level_int_en", &self.level_int_en())
            .field("edge_int_en", &self.edge_int_en())
            .field("divider", &self.divider())
            .field("autoreload", &self.autoreload())
            .field("increase", &self.increase())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 10 - When set alarm is enabled
    #[inline(always)]
    #[must_use]
    pub fn alarm_en(&mut self) -> ALARM_EN_W<CONFIG_SPEC> {
        ALARM_EN_W::new(self, 10)
    }
    ///Bit 11 - When set level type interrupt will be generated during alarm
    #[inline(always)]
    #[must_use]
    pub fn level_int_en(&mut self) -> LEVEL_INT_EN_W<CONFIG_SPEC> {
        LEVEL_INT_EN_W::new(self, 11)
    }
    ///Bit 12 - When set edge type interrupt will be generated during alarm
    #[inline(always)]
    #[must_use]
    pub fn edge_int_en(&mut self) -> EDGE_INT_EN_W<CONFIG_SPEC> {
        EDGE_INT_EN_W::new(self, 12)
    }
    ///Bits 13:28 - Timer 0 clock (T0_clk) prescale value.
    #[inline(always)]
    #[must_use]
    pub fn divider(&mut self) -> DIVIDER_W<CONFIG_SPEC> {
        DIVIDER_W::new(self, 13)
    }
    ///Bit 29 - When set timer 0 auto-reload at alarming is enabled
    #[inline(always)]
    #[must_use]
    pub fn autoreload(&mut self) -> AUTORELOAD_W<CONFIG_SPEC> {
        AUTORELOAD_W::new(self, 29)
    }
    ///Bit 30 - When set timer 0 time-base counter increment. When cleared timer 0 time-base counter decrement.
    #[inline(always)]
    #[must_use]
    pub fn increase(&mut self) -> INCREASE_W<CONFIG_SPEC> {
        INCREASE_W::new(self, 30)
    }
    ///Bit 31 - When set timer 0 time-base counter is enabled
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CONFIG_SPEC> {
        EN_W::new(self, 31)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`config::R`](R) reader structure
impl crate::Readable for CONFIG_SPEC {}
///`write(|w| ..)` method takes [`config::W`](W) writer structure
impl crate::Writable for CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONFIG to value 0x6000_2000
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x6000_2000;
}
