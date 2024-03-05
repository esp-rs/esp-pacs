#[doc = "Register `TARGET0_CONF` reader"]
pub type R = crate::R<TARGET0_CONF_SPEC>;
#[doc = "Register `TARGET0_CONF` writer"]
pub type W = crate::W<TARGET0_CONF_SPEC>;
#[doc = "Field `TARGET0_PERIOD` reader - target0 period"]
pub type TARGET0_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `TARGET0_PERIOD` writer - target0 period"]
pub type TARGET0_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `TARGET0_PERIOD_MODE` reader - Set target0 to period mode"]
pub type TARGET0_PERIOD_MODE_R = crate::BitReader;
#[doc = "Field `TARGET0_PERIOD_MODE` writer - Set target0 to period mode"]
pub type TARGET0_PERIOD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET0_TIMER_UNIT_SEL` reader - select which unit to compare"]
pub type TARGET0_TIMER_UNIT_SEL_R = crate::BitReader;
#[doc = "Field `TARGET0_TIMER_UNIT_SEL` writer - select which unit to compare"]
pub type TARGET0_TIMER_UNIT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25 - target0 period"]
    #[inline(always)]
    pub fn target0_period(&self) -> TARGET0_PERIOD_R {
        TARGET0_PERIOD_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 30 - Set target0 to period mode"]
    #[inline(always)]
    pub fn target0_period_mode(&self) -> TARGET0_PERIOD_MODE_R {
        TARGET0_PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    pub fn target0_timer_unit_sel(&self) -> TARGET0_TIMER_UNIT_SEL_R {
        TARGET0_TIMER_UNIT_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET0_CONF")
            .field(
                "target0_period",
                &format_args!("{}", self.target0_period().bits()),
            )
            .field(
                "target0_period_mode",
                &format_args!("{}", self.target0_period_mode().bit()),
            )
            .field(
                "target0_timer_unit_sel",
                &format_args!("{}", self.target0_timer_unit_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TARGET0_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:25 - target0 period"]
    #[inline(always)]
    #[must_use]
    pub fn target0_period(&mut self) -> TARGET0_PERIOD_W<TARGET0_CONF_SPEC> {
        TARGET0_PERIOD_W::new(self, 0)
    }
    #[doc = "Bit 30 - Set target0 to period mode"]
    #[inline(always)]
    #[must_use]
    pub fn target0_period_mode(&mut self) -> TARGET0_PERIOD_MODE_W<TARGET0_CONF_SPEC> {
        TARGET0_PERIOD_MODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - select which unit to compare"]
    #[inline(always)]
    #[must_use]
    pub fn target0_timer_unit_sel(&mut self) -> TARGET0_TIMER_UNIT_SEL_W<TARGET0_CONF_SPEC> {
        TARGET0_TIMER_UNIT_SEL_W::new(self, 31)
    }
}
#[doc = "SYSTIMER_TARGET0_CONF.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target0_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target0_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET0_CONF_SPEC;
impl crate::RegisterSpec for TARGET0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target0_conf::R`](R) reader structure"]
impl crate::Readable for TARGET0_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target0_conf::W`](W) writer structure"]
impl crate::Writable for TARGET0_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARGET0_CONF to value 0"]
impl crate::Resettable for TARGET0_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
