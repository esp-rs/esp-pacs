#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `SYSTIMER_CLK_FO` reader - systimer clock force on"]
pub type SYSTIMER_CLK_FO_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_FO` writer - systimer clock force on"]
pub type SYSTIMER_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_EN` reader - Configures whether or not to enable generation of ETM events.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type ETM_EN_R = crate::BitReader;
#[doc = "Field `ETM_EN` writer - Configures whether or not to enable generation of ETM events.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type ETM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET2_WORK_EN` reader - Configures whether or not to enable COMP2.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type TARGET2_WORK_EN_R = crate::BitReader;
#[doc = "Field `TARGET2_WORK_EN` writer - Configures whether or not to enable COMP2.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type TARGET2_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET1_WORK_EN` reader - Configures whether or not to enable COMP1. See details in SYSTIMER_TARGET2_WORK_EN."]
pub type TARGET1_WORK_EN_R = crate::BitReader;
#[doc = "Field `TARGET1_WORK_EN` writer - Configures whether or not to enable COMP1. See details in SYSTIMER_TARGET2_WORK_EN."]
pub type TARGET1_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET0_WORK_EN` reader - Configures whether or not to enable COMP0. See details in SYSTIMER_TARGET2_WORK_EN."]
pub type TARGET0_WORK_EN_R = crate::BitReader;
#[doc = "Field `TARGET0_WORK_EN` writer - Configures whether or not to enable COMP0. See details in SYSTIMER_TARGET2_WORK_EN."]
pub type TARGET0_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT1_CORE1_STALL_EN` reader - Configures whether or not UNIT1 is stalled when CORE1 is stalled. \\\\ 0: UNIT1 is not stalled. \\\\ 1: UNIT1 is stalled.\\\\"]
pub type TIMER_UNIT1_CORE1_STALL_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_CORE1_STALL_EN` writer - Configures whether or not UNIT1 is stalled when CORE1 is stalled. \\\\ 0: UNIT1 is not stalled. \\\\ 1: UNIT1 is stalled.\\\\"]
pub type TIMER_UNIT1_CORE1_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT1_CORE0_STALL_EN` reader - Configures whether or not UNIT1 is stalled when CORE0 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
pub type TIMER_UNIT1_CORE0_STALL_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_CORE0_STALL_EN` writer - Configures whether or not UNIT1 is stalled when CORE0 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
pub type TIMER_UNIT1_CORE0_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT0_CORE1_STALL_EN` reader - Configures whether or not UNIT0 is stalled when CORE1 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
pub type TIMER_UNIT0_CORE1_STALL_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT0_CORE1_STALL_EN` writer - Configures whether or not UNIT0 is stalled when CORE1 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
pub type TIMER_UNIT0_CORE1_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT0_CORE0_STALL_EN` reader - Configures whether or not UNIT0 is stalled when CORE0 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
pub type TIMER_UNIT0_CORE0_STALL_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT0_CORE0_STALL_EN` writer - Configures whether or not UNIT0 is stalled when CORE0 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
pub type TIMER_UNIT0_CORE0_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT1_WORK_EN` reader - Configures whether or not to enable UNIT1. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type TIMER_UNIT1_WORK_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT1_WORK_EN` writer - Configures whether or not to enable UNIT1. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type TIMER_UNIT1_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT0_WORK_EN` reader - Configures whether or not to enable UNIT0. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type TIMER_UNIT0_WORK_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT0_WORK_EN` writer - Configures whether or not to enable UNIT0. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type TIMER_UNIT0_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Configures register clock gating. \\\\ 0: Only enable needed clock for register read or write operations. \\\\ 1: Register clock is always enabled for read and write operations. \\\\"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Configures register clock gating. \\\\ 0: Only enable needed clock for register read or write operations. \\\\ 1: Register clock is always enabled for read and write operations. \\\\"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - systimer clock force on"]
    #[inline(always)]
    pub fn systimer_clk_fo(&self) -> SYSTIMER_CLK_FO_R {
        SYSTIMER_CLK_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable generation of ETM events.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_en(&self) -> ETM_EN_R {
        ETM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 22 - Configures whether or not to enable COMP2.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn target2_work_en(&self) -> TARGET2_WORK_EN_R {
        TARGET2_WORK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Configures whether or not to enable COMP1. See details in SYSTIMER_TARGET2_WORK_EN."]
    #[inline(always)]
    pub fn target1_work_en(&self) -> TARGET1_WORK_EN_R {
        TARGET1_WORK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Configures whether or not to enable COMP0. See details in SYSTIMER_TARGET2_WORK_EN."]
    #[inline(always)]
    pub fn target0_work_en(&self) -> TARGET0_WORK_EN_R {
        TARGET0_WORK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Configures whether or not UNIT1 is stalled when CORE1 is stalled. \\\\ 0: UNIT1 is not stalled. \\\\ 1: UNIT1 is stalled.\\\\"]
    #[inline(always)]
    pub fn timer_unit1_core1_stall_en(&self) -> TIMER_UNIT1_CORE1_STALL_EN_R {
        TIMER_UNIT1_CORE1_STALL_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Configures whether or not UNIT1 is stalled when CORE0 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
    #[inline(always)]
    pub fn timer_unit1_core0_stall_en(&self) -> TIMER_UNIT1_CORE0_STALL_EN_R {
        TIMER_UNIT1_CORE0_STALL_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Configures whether or not UNIT0 is stalled when CORE1 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
    #[inline(always)]
    pub fn timer_unit0_core1_stall_en(&self) -> TIMER_UNIT0_CORE1_STALL_EN_R {
        TIMER_UNIT0_CORE1_STALL_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Configures whether or not UNIT0 is stalled when CORE0 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
    #[inline(always)]
    pub fn timer_unit0_core0_stall_en(&self) -> TIMER_UNIT0_CORE0_STALL_EN_R {
        TIMER_UNIT0_CORE0_STALL_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Configures whether or not to enable UNIT1. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn timer_unit1_work_en(&self) -> TIMER_UNIT1_WORK_EN_R {
        TIMER_UNIT1_WORK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Configures whether or not to enable UNIT0. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn timer_unit0_work_en(&self) -> TIMER_UNIT0_WORK_EN_R {
        TIMER_UNIT0_WORK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configures register clock gating. \\\\ 0: Only enable needed clock for register read or write operations. \\\\ 1: Register clock is always enabled for read and write operations. \\\\"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("systimer_clk_fo", &self.systimer_clk_fo())
            .field("etm_en", &self.etm_en())
            .field("target2_work_en", &self.target2_work_en())
            .field("target1_work_en", &self.target1_work_en())
            .field("target0_work_en", &self.target0_work_en())
            .field(
                "timer_unit1_core1_stall_en",
                &self.timer_unit1_core1_stall_en(),
            )
            .field(
                "timer_unit1_core0_stall_en",
                &self.timer_unit1_core0_stall_en(),
            )
            .field(
                "timer_unit0_core1_stall_en",
                &self.timer_unit0_core1_stall_en(),
            )
            .field(
                "timer_unit0_core0_stall_en",
                &self.timer_unit0_core0_stall_en(),
            )
            .field("timer_unit1_work_en", &self.timer_unit1_work_en())
            .field("timer_unit0_work_en", &self.timer_unit0_work_en())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - systimer clock force on"]
    #[inline(always)]
    pub fn systimer_clk_fo(&mut self) -> SYSTIMER_CLK_FO_W<CONF_SPEC> {
        SYSTIMER_CLK_FO_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable generation of ETM events.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn etm_en(&mut self) -> ETM_EN_W<CONF_SPEC> {
        ETM_EN_W::new(self, 1)
    }
    #[doc = "Bit 22 - Configures whether or not to enable COMP2.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn target2_work_en(&mut self) -> TARGET2_WORK_EN_W<CONF_SPEC> {
        TARGET2_WORK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to enable COMP1. See details in SYSTIMER_TARGET2_WORK_EN."]
    #[inline(always)]
    pub fn target1_work_en(&mut self) -> TARGET1_WORK_EN_W<CONF_SPEC> {
        TARGET1_WORK_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to enable COMP0. See details in SYSTIMER_TARGET2_WORK_EN."]
    #[inline(always)]
    pub fn target0_work_en(&mut self) -> TARGET0_WORK_EN_W<CONF_SPEC> {
        TARGET0_WORK_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not UNIT1 is stalled when CORE1 is stalled. \\\\ 0: UNIT1 is not stalled. \\\\ 1: UNIT1 is stalled.\\\\"]
    #[inline(always)]
    pub fn timer_unit1_core1_stall_en(&mut self) -> TIMER_UNIT1_CORE1_STALL_EN_W<CONF_SPEC> {
        TIMER_UNIT1_CORE1_STALL_EN_W::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not UNIT1 is stalled when CORE0 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
    #[inline(always)]
    pub fn timer_unit1_core0_stall_en(&mut self) -> TIMER_UNIT1_CORE0_STALL_EN_W<CONF_SPEC> {
        TIMER_UNIT1_CORE0_STALL_EN_W::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not UNIT0 is stalled when CORE1 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
    #[inline(always)]
    pub fn timer_unit0_core1_stall_en(&mut self) -> TIMER_UNIT0_CORE1_STALL_EN_W<CONF_SPEC> {
        TIMER_UNIT0_CORE1_STALL_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not UNIT0 is stalled when CORE0 is stalled. See details in SYSTIMER_TIMER_UNIT1_CORE1_STALL_EN."]
    #[inline(always)]
    pub fn timer_unit0_core0_stall_en(&mut self) -> TIMER_UNIT0_CORE0_STALL_EN_W<CONF_SPEC> {
        TIMER_UNIT0_CORE0_STALL_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to enable UNIT1. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn timer_unit1_work_en(&mut self) -> TIMER_UNIT1_WORK_EN_W<CONF_SPEC> {
        TIMER_UNIT1_WORK_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to enable UNIT0. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn timer_unit0_work_en(&mut self) -> TIMER_UNIT0_WORK_EN_W<CONF_SPEC> {
        TIMER_UNIT0_WORK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures register clock gating. \\\\ 0: Only enable needed clock for register read or write operations. \\\\ 1: Register clock is always enabled for read and write operations. \\\\"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "Configure system timer clock\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0x4600_0000"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x4600_0000;
}
