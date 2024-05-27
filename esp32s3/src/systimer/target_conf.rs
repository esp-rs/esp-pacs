///Register `TARGET%s_CONF` reader
pub type R = crate::R<TARGET_CONF_SPEC>;
///Register `TARGET%s_CONF` writer
pub type W = crate::W<TARGET_CONF_SPEC>;
///Field `PERIOD` reader - target0 period
pub type PERIOD_R = crate::FieldReader<u32>;
///Field `PERIOD` writer - target0 period
pub type PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
///Field `PERIOD_MODE` reader - Set target0 to period mode
pub type PERIOD_MODE_R = crate::BitReader;
///Field `PERIOD_MODE` writer - Set target0 to period mode
pub type PERIOD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_UNIT_SEL` reader - select which unit to compare
pub type TIMER_UNIT_SEL_R = crate::BitReader;
///Field `TIMER_UNIT_SEL` writer - select which unit to compare
pub type TIMER_UNIT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:25 - target0 period
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(self.bits & 0x03ff_ffff)
    }
    ///Bit 30 - Set target0 to period mode
    #[inline(always)]
    pub fn period_mode(&self) -> PERIOD_MODE_R {
        PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - select which unit to compare
    #[inline(always)]
    pub fn timer_unit_sel(&self) -> TIMER_UNIT_SEL_R {
        TIMER_UNIT_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET_CONF")
            .field("period", &self.period())
            .field("period_mode", &self.period_mode())
            .field("timer_unit_sel", &self.timer_unit_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - target0 period
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<TARGET_CONF_SPEC> {
        PERIOD_W::new(self, 0)
    }
    ///Bit 30 - Set target0 to period mode
    #[inline(always)]
    #[must_use]
    pub fn period_mode(&mut self) -> PERIOD_MODE_W<TARGET_CONF_SPEC> {
        PERIOD_MODE_W::new(self, 30)
    }
    ///Bit 31 - select which unit to compare
    #[inline(always)]
    #[must_use]
    pub fn timer_unit_sel(&mut self) -> TIMER_UNIT_SEL_W<TARGET_CONF_SPEC> {
        TIMER_UNIT_SEL_W::new(self, 31)
    }
}
/**system timer comp%s target mode register

You can [`read`](crate::generic::Reg::read) this register and get [`target_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TARGET_CONF_SPEC;
impl crate::RegisterSpec for TARGET_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`target_conf::R`](R) reader structure
impl crate::Readable for TARGET_CONF_SPEC {}
///`write(|w| ..)` method takes [`target_conf::W`](W) writer structure
impl crate::Writable for TARGET_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TARGET%s_CONF to value 0
impl crate::Resettable for TARGET_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
