#[doc = "Register `TARGET0_CONF` reader"]
pub type R = crate::R<TARGET0_CONF_SPEC>;
#[doc = "Register `TARGET0_CONF` writer"]
pub type W = crate::W<TARGET0_CONF_SPEC>;
#[doc = "Field `TARGET0_PERIOD` reader - Configures COMP0 alarm period."]
pub type TARGET0_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `TARGET0_PERIOD` writer - Configures COMP0 alarm period."]
pub type TARGET0_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `TARGET0_PERIOD_MODE` reader - Selects the two alarm modes for COMP0. \\\\ 0: Target mode\\\\ 1: Period mode\\\\"]
pub type TARGET0_PERIOD_MODE_R = crate::BitReader;
#[doc = "Field `TARGET0_PERIOD_MODE` writer - Selects the two alarm modes for COMP0. \\\\ 0: Target mode\\\\ 1: Period mode\\\\"]
pub type TARGET0_PERIOD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET0_TIMER_UNIT_SEL` reader - Chooses the counter value for comparison with COMP0.\\\\ 0: Use the count value from UNIT$0\\\\ 1: Use the count value from UNIT$1\\\\"]
pub type TARGET0_TIMER_UNIT_SEL_R = crate::BitReader;
#[doc = "Field `TARGET0_TIMER_UNIT_SEL` writer - Chooses the counter value for comparison with COMP0.\\\\ 0: Use the count value from UNIT$0\\\\ 1: Use the count value from UNIT$1\\\\"]
pub type TARGET0_TIMER_UNIT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25 - Configures COMP0 alarm period."]
    #[inline(always)]
    pub fn target0_period(&self) -> TARGET0_PERIOD_R {
        TARGET0_PERIOD_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 30 - Selects the two alarm modes for COMP0. \\\\ 0: Target mode\\\\ 1: Period mode\\\\"]
    #[inline(always)]
    pub fn target0_period_mode(&self) -> TARGET0_PERIOD_MODE_R {
        TARGET0_PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Chooses the counter value for comparison with COMP0.\\\\ 0: Use the count value from UNIT$0\\\\ 1: Use the count value from UNIT$1\\\\"]
    #[inline(always)]
    pub fn target0_timer_unit_sel(&self) -> TARGET0_TIMER_UNIT_SEL_R {
        TARGET0_TIMER_UNIT_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET0_CONF")
            .field("target0_period", &self.target0_period())
            .field("target0_period_mode", &self.target0_period_mode())
            .field("target0_timer_unit_sel", &self.target0_timer_unit_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:25 - Configures COMP0 alarm period."]
    #[inline(always)]
    pub fn target0_period(&mut self) -> TARGET0_PERIOD_W<TARGET0_CONF_SPEC> {
        TARGET0_PERIOD_W::new(self, 0)
    }
    #[doc = "Bit 30 - Selects the two alarm modes for COMP0. \\\\ 0: Target mode\\\\ 1: Period mode\\\\"]
    #[inline(always)]
    pub fn target0_period_mode(&mut self) -> TARGET0_PERIOD_MODE_W<TARGET0_CONF_SPEC> {
        TARGET0_PERIOD_MODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Chooses the counter value for comparison with COMP0.\\\\ 0: Use the count value from UNIT$0\\\\ 1: Use the count value from UNIT$1\\\\"]
    #[inline(always)]
    pub fn target0_timer_unit_sel(&mut self) -> TARGET0_TIMER_UNIT_SEL_W<TARGET0_CONF_SPEC> {
        TARGET0_TIMER_UNIT_SEL_W::new(self, 31)
    }
}
#[doc = "Configure COMP0 alarm mode\n\nYou can [`read`](crate::Reg::read) this register and get [`target0_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target0_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET0_CONF_SPEC;
impl crate::RegisterSpec for TARGET0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target0_conf::R`](R) reader structure"]
impl crate::Readable for TARGET0_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target0_conf::W`](W) writer structure"]
impl crate::Writable for TARGET0_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TARGET0_CONF to value 0"]
impl crate::Resettable for TARGET0_CONF_SPEC {}
