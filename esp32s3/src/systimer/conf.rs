///Register `CONF` reader
pub type R = crate::R<CONF_SPEC>;
///Register `CONF` writer
pub type W = crate::W<CONF_SPEC>;
///Field `SYSTIMER_CLK_FO` reader - systimer clock force on
pub type SYSTIMER_CLK_FO_R = crate::BitReader;
///Field `SYSTIMER_CLK_FO` writer - systimer clock force on
pub type SYSTIMER_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TARGET2_WORK_EN` reader - target2 work enable
pub type TARGET2_WORK_EN_R = crate::BitReader;
///Field `TARGET2_WORK_EN` writer - target2 work enable
pub type TARGET2_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TARGET1_WORK_EN` reader - target1 work enable
pub type TARGET1_WORK_EN_R = crate::BitReader;
///Field `TARGET1_WORK_EN` writer - target1 work enable
pub type TARGET1_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TARGET0_WORK_EN` reader - target0 work enable
pub type TARGET0_WORK_EN_R = crate::BitReader;
///Field `TARGET0_WORK_EN` writer - target0 work enable
pub type TARGET0_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_UNIT1_CORE1_STALL_EN` reader - If timer unit1 is stalled when core1 stalled
pub type TIMER_UNIT1_CORE1_STALL_EN_R = crate::BitReader;
///Field `TIMER_UNIT1_CORE1_STALL_EN` writer - If timer unit1 is stalled when core1 stalled
pub type TIMER_UNIT1_CORE1_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_UNIT1_CORE0_STALL_EN` reader - If timer unit1 is stalled when core0 stalled
pub type TIMER_UNIT1_CORE0_STALL_EN_R = crate::BitReader;
///Field `TIMER_UNIT1_CORE0_STALL_EN` writer - If timer unit1 is stalled when core0 stalled
pub type TIMER_UNIT1_CORE0_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_UNIT0_CORE1_STALL_EN` reader - If timer unit0 is stalled when core1 stalled
pub type TIMER_UNIT0_CORE1_STALL_EN_R = crate::BitReader;
///Field `TIMER_UNIT0_CORE1_STALL_EN` writer - If timer unit0 is stalled when core1 stalled
pub type TIMER_UNIT0_CORE1_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_UNIT0_CORE0_STALL_EN` reader - If timer unit0 is stalled when core0 stalled
pub type TIMER_UNIT0_CORE0_STALL_EN_R = crate::BitReader;
///Field `TIMER_UNIT0_CORE0_STALL_EN` writer - If timer unit0 is stalled when core0 stalled
pub type TIMER_UNIT0_CORE0_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_UNIT1_WORK_EN` reader - timer unit1 work enable
pub type TIMER_UNIT1_WORK_EN_R = crate::BitReader;
///Field `TIMER_UNIT1_WORK_EN` writer - timer unit1 work enable
pub type TIMER_UNIT1_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMER_UNIT0_WORK_EN` reader - timer unit0 work enable
pub type TIMER_UNIT0_WORK_EN_R = crate::BitReader;
///Field `TIMER_UNIT0_WORK_EN` writer - timer unit0 work enable
pub type TIMER_UNIT0_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_EN` reader - register file clk gating
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - register file clk gating
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - systimer clock force on
    #[inline(always)]
    pub fn systimer_clk_fo(&self) -> SYSTIMER_CLK_FO_R {
        SYSTIMER_CLK_FO_R::new((self.bits & 1) != 0)
    }
    ///Bit 22 - target2 work enable
    #[inline(always)]
    pub fn target2_work_en(&self) -> TARGET2_WORK_EN_R {
        TARGET2_WORK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - target1 work enable
    #[inline(always)]
    pub fn target1_work_en(&self) -> TARGET1_WORK_EN_R {
        TARGET1_WORK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - target0 work enable
    #[inline(always)]
    pub fn target0_work_en(&self) -> TARGET0_WORK_EN_R {
        TARGET0_WORK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - If timer unit1 is stalled when core1 stalled
    #[inline(always)]
    pub fn timer_unit1_core1_stall_en(&self) -> TIMER_UNIT1_CORE1_STALL_EN_R {
        TIMER_UNIT1_CORE1_STALL_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - If timer unit1 is stalled when core0 stalled
    #[inline(always)]
    pub fn timer_unit1_core0_stall_en(&self) -> TIMER_UNIT1_CORE0_STALL_EN_R {
        TIMER_UNIT1_CORE0_STALL_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - If timer unit0 is stalled when core1 stalled
    #[inline(always)]
    pub fn timer_unit0_core1_stall_en(&self) -> TIMER_UNIT0_CORE1_STALL_EN_R {
        TIMER_UNIT0_CORE1_STALL_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - If timer unit0 is stalled when core0 stalled
    #[inline(always)]
    pub fn timer_unit0_core0_stall_en(&self) -> TIMER_UNIT0_CORE0_STALL_EN_R {
        TIMER_UNIT0_CORE0_STALL_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - timer unit1 work enable
    #[inline(always)]
    pub fn timer_unit1_work_en(&self) -> TIMER_UNIT1_WORK_EN_R {
        TIMER_UNIT1_WORK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - timer unit0 work enable
    #[inline(always)]
    pub fn timer_unit0_work_en(&self) -> TIMER_UNIT0_WORK_EN_R {
        TIMER_UNIT0_WORK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - register file clk gating
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
    ///Bit 0 - systimer clock force on
    #[inline(always)]
    #[must_use]
    pub fn systimer_clk_fo(&mut self) -> SYSTIMER_CLK_FO_W<CONF_SPEC> {
        SYSTIMER_CLK_FO_W::new(self, 0)
    }
    ///Bit 22 - target2 work enable
    #[inline(always)]
    #[must_use]
    pub fn target2_work_en(&mut self) -> TARGET2_WORK_EN_W<CONF_SPEC> {
        TARGET2_WORK_EN_W::new(self, 22)
    }
    ///Bit 23 - target1 work enable
    #[inline(always)]
    #[must_use]
    pub fn target1_work_en(&mut self) -> TARGET1_WORK_EN_W<CONF_SPEC> {
        TARGET1_WORK_EN_W::new(self, 23)
    }
    ///Bit 24 - target0 work enable
    #[inline(always)]
    #[must_use]
    pub fn target0_work_en(&mut self) -> TARGET0_WORK_EN_W<CONF_SPEC> {
        TARGET0_WORK_EN_W::new(self, 24)
    }
    ///Bit 25 - If timer unit1 is stalled when core1 stalled
    #[inline(always)]
    #[must_use]
    pub fn timer_unit1_core1_stall_en(&mut self) -> TIMER_UNIT1_CORE1_STALL_EN_W<CONF_SPEC> {
        TIMER_UNIT1_CORE1_STALL_EN_W::new(self, 25)
    }
    ///Bit 26 - If timer unit1 is stalled when core0 stalled
    #[inline(always)]
    #[must_use]
    pub fn timer_unit1_core0_stall_en(&mut self) -> TIMER_UNIT1_CORE0_STALL_EN_W<CONF_SPEC> {
        TIMER_UNIT1_CORE0_STALL_EN_W::new(self, 26)
    }
    ///Bit 27 - If timer unit0 is stalled when core1 stalled
    #[inline(always)]
    #[must_use]
    pub fn timer_unit0_core1_stall_en(&mut self) -> TIMER_UNIT0_CORE1_STALL_EN_W<CONF_SPEC> {
        TIMER_UNIT0_CORE1_STALL_EN_W::new(self, 27)
    }
    ///Bit 28 - If timer unit0 is stalled when core0 stalled
    #[inline(always)]
    #[must_use]
    pub fn timer_unit0_core0_stall_en(&mut self) -> TIMER_UNIT0_CORE0_STALL_EN_W<CONF_SPEC> {
        TIMER_UNIT0_CORE0_STALL_EN_W::new(self, 28)
    }
    ///Bit 29 - timer unit1 work enable
    #[inline(always)]
    #[must_use]
    pub fn timer_unit1_work_en(&mut self) -> TIMER_UNIT1_WORK_EN_W<CONF_SPEC> {
        TIMER_UNIT1_WORK_EN_W::new(self, 29)
    }
    ///Bit 30 - timer unit0 work enable
    #[inline(always)]
    #[must_use]
    pub fn timer_unit0_work_en(&mut self) -> TIMER_UNIT0_WORK_EN_W<CONF_SPEC> {
        TIMER_UNIT0_WORK_EN_W::new(self, 30)
    }
    ///Bit 31 - register file clk gating
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CONF_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
/**Configure system timer clock

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf::R`](R) reader structure
impl crate::Readable for CONF_SPEC {}
///`write(|w| ..)` method takes [`conf::W`](W) writer structure
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF to value 0x4600_0000
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x4600_0000;
}
