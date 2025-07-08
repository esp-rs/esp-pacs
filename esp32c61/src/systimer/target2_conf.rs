#[doc = "Register `TARGET2_CONF` reader"]
pub type R = crate::R<TARGET2_CONF_SPEC>;
#[doc = "Register `TARGET2_CONF` writer"]
pub type W = crate::W<TARGET2_CONF_SPEC>;
#[doc = "Field `TARGET2_PERIOD` reader - Configures COMP2 alarm period."]
pub type TARGET2_PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `TARGET2_PERIOD` writer - Configures COMP2 alarm period."]
pub type TARGET2_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
#[doc = "Field `TARGET2_PERIOD_MODE` reader - Configures Configures the two alarm modes for COMP2. See details in SYSTIMER_TARGET0_PERIOD_MODE."]
pub type TARGET2_PERIOD_MODE_R = crate::BitReader;
#[doc = "Field `TARGET2_PERIOD_MODE` writer - Configures Configures the two alarm modes for COMP2. See details in SYSTIMER_TARGET0_PERIOD_MODE."]
pub type TARGET2_PERIOD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET2_TIMER_UNIT_SEL` reader - Chooses the counter value for comparison with COMP2. See details in SYSTIMER_TARGET0_TIMER_UNIT_SEL."]
pub type TARGET2_TIMER_UNIT_SEL_R = crate::BitReader;
#[doc = "Field `TARGET2_TIMER_UNIT_SEL` writer - Chooses the counter value for comparison with COMP2. See details in SYSTIMER_TARGET0_TIMER_UNIT_SEL."]
pub type TARGET2_TIMER_UNIT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25 - Configures COMP2 alarm period."]
    #[inline(always)]
    pub fn target2_period(&self) -> TARGET2_PERIOD_R {
        TARGET2_PERIOD_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 30 - Configures Configures the two alarm modes for COMP2. See details in SYSTIMER_TARGET0_PERIOD_MODE."]
    #[inline(always)]
    pub fn target2_period_mode(&self) -> TARGET2_PERIOD_MODE_R {
        TARGET2_PERIOD_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Chooses the counter value for comparison with COMP2. See details in SYSTIMER_TARGET0_TIMER_UNIT_SEL."]
    #[inline(always)]
    pub fn target2_timer_unit_sel(&self) -> TARGET2_TIMER_UNIT_SEL_R {
        TARGET2_TIMER_UNIT_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TARGET2_CONF")
            .field("target2_period", &self.target2_period())
            .field("target2_period_mode", &self.target2_period_mode())
            .field("target2_timer_unit_sel", &self.target2_timer_unit_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:25 - Configures COMP2 alarm period."]
    #[inline(always)]
    pub fn target2_period(&mut self) -> TARGET2_PERIOD_W<TARGET2_CONF_SPEC> {
        TARGET2_PERIOD_W::new(self, 0)
    }
    #[doc = "Bit 30 - Configures Configures the two alarm modes for COMP2. See details in SYSTIMER_TARGET0_PERIOD_MODE."]
    #[inline(always)]
    pub fn target2_period_mode(&mut self) -> TARGET2_PERIOD_MODE_W<TARGET2_CONF_SPEC> {
        TARGET2_PERIOD_MODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Chooses the counter value for comparison with COMP2. See details in SYSTIMER_TARGET0_TIMER_UNIT_SEL."]
    #[inline(always)]
    pub fn target2_timer_unit_sel(&mut self) -> TARGET2_TIMER_UNIT_SEL_W<TARGET2_CONF_SPEC> {
        TARGET2_TIMER_UNIT_SEL_W::new(self, 31)
    }
}
#[doc = "Configure COMP2 alarm mode\n\nYou can [`read`](crate::Reg::read) this register and get [`target2_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target2_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARGET2_CONF_SPEC;
impl crate::RegisterSpec for TARGET2_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`target2_conf::R`](R) reader structure"]
impl crate::Readable for TARGET2_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`target2_conf::W`](W) writer structure"]
impl crate::Writable for TARGET2_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TARGET2_CONF to value 0"]
impl crate::Resettable for TARGET2_CONF_SPEC {}
