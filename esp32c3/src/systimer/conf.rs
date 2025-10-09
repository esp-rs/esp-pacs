#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `SYSTIMER_CLK_FO` reader - systimer clock force on"]
pub type SYSTIMER_CLK_FO_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_FO` writer - systimer clock force on"]
pub type SYSTIMER_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET_WORK_EN(2,1,0)` reader - target%s work enable"]
pub type TARGET_WORK_EN_R = crate::BitReader;
#[doc = "Field `TARGET_WORK_EN(2,1,0)` writer - target%s work enable"]
pub type TARGET_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT_CORE1_STALL_EN(1,0)` reader - If timer unit%s is stalled when core1 stalled"]
pub type TIMER_UNIT_CORE1_STALL_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT_CORE1_STALL_EN(1,0)` writer - If timer unit%s is stalled when core1 stalled"]
pub type TIMER_UNIT_CORE1_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT_CORE0_STALL_EN(1,0)` reader - If timer unit%s is stalled when core0 stalled"]
pub type TIMER_UNIT_CORE0_STALL_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT_CORE0_STALL_EN(1,0)` writer - If timer unit%s is stalled when core0 stalled"]
pub type TIMER_UNIT_CORE0_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UNIT_WORK_EN(1,0)` reader - timer unit%s work enable"]
pub type TIMER_UNIT_WORK_EN_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT_WORK_EN(1,0)` writer - timer unit%s work enable"]
pub type TIMER_UNIT_WORK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - register file clk gating"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - register file clk gating"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - systimer clock force on"]
    #[inline(always)]
    pub fn systimer_clk_fo(&self) -> SYSTIMER_CLK_FO_R {
        SYSTIMER_CLK_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "target(2,1,0) work enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TARGET2_WORK_EN` field.</div>"]
    #[inline(always)]
    pub fn target_work_en(&self, n: u8) -> TARGET_WORK_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TARGET_WORK_EN_R::new(((self.bits >> (n + 22)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "target(2,1,0) work enable"]
    #[inline(always)]
    pub fn target_work_en_iter(&self) -> impl Iterator<Item = TARGET_WORK_EN_R> + '_ {
        (0..3).map(move |n| TARGET_WORK_EN_R::new(((self.bits >> (n + 22)) & 1) != 0))
    }
    #[doc = "Bit 22 - target2 work enable"]
    #[inline(always)]
    pub fn target2_work_en(&self) -> TARGET_WORK_EN_R {
        TARGET_WORK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - target1 work enable"]
    #[inline(always)]
    pub fn target1_work_en(&self) -> TARGET_WORK_EN_R {
        TARGET_WORK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - target0 work enable"]
    #[inline(always)]
    pub fn target0_work_en(&self) -> TARGET_WORK_EN_R {
        TARGET_WORK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "If timer unit(1,0) is stalled when core1 stalled"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER_UNIT1_CORE1_STALL_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_unit_core1_stall_en(&self, n: u8) -> TIMER_UNIT_CORE1_STALL_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TIMER_UNIT_CORE1_STALL_EN_R::new(((self.bits >> (n * 2 + 25)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "If timer unit(1,0) is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit_core1_stall_en_iter(
        &self,
    ) -> impl Iterator<Item = TIMER_UNIT_CORE1_STALL_EN_R> + '_ {
        (0..2)
            .map(move |n| TIMER_UNIT_CORE1_STALL_EN_R::new(((self.bits >> (n * 2 + 25)) & 1) != 0))
    }
    #[doc = "Bit 25 - If timer unit1 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core1_stall_en(&self) -> TIMER_UNIT_CORE1_STALL_EN_R {
        TIMER_UNIT_CORE1_STALL_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - If timer unit0 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core1_stall_en(&self) -> TIMER_UNIT_CORE1_STALL_EN_R {
        TIMER_UNIT_CORE1_STALL_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "If timer unit(1,0) is stalled when core0 stalled"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER_UNIT1_CORE0_STALL_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_unit_core0_stall_en(&self, n: u8) -> TIMER_UNIT_CORE0_STALL_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TIMER_UNIT_CORE0_STALL_EN_R::new(((self.bits >> (n * 2 + 26)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "If timer unit(1,0) is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit_core0_stall_en_iter(
        &self,
    ) -> impl Iterator<Item = TIMER_UNIT_CORE0_STALL_EN_R> + '_ {
        (0..2)
            .map(move |n| TIMER_UNIT_CORE0_STALL_EN_R::new(((self.bits >> (n * 2 + 26)) & 1) != 0))
    }
    #[doc = "Bit 26 - If timer unit1 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core0_stall_en(&self) -> TIMER_UNIT_CORE0_STALL_EN_R {
        TIMER_UNIT_CORE0_STALL_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - If timer unit0 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core0_stall_en(&self) -> TIMER_UNIT_CORE0_STALL_EN_R {
        TIMER_UNIT_CORE0_STALL_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "timer unit(1,0) work enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER_UNIT1_WORK_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_unit_work_en(&self, n: u8) -> TIMER_UNIT_WORK_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TIMER_UNIT_WORK_EN_R::new(((self.bits >> (n + 29)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "timer unit(1,0) work enable"]
    #[inline(always)]
    pub fn timer_unit_work_en_iter(&self) -> impl Iterator<Item = TIMER_UNIT_WORK_EN_R> + '_ {
        (0..2).map(move |n| TIMER_UNIT_WORK_EN_R::new(((self.bits >> (n + 29)) & 1) != 0))
    }
    #[doc = "Bit 29 - timer unit1 work enable"]
    #[inline(always)]
    pub fn timer_unit1_work_en(&self) -> TIMER_UNIT_WORK_EN_R {
        TIMER_UNIT_WORK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - timer unit0 work enable"]
    #[inline(always)]
    pub fn timer_unit0_work_en(&self) -> TIMER_UNIT_WORK_EN_R {
        TIMER_UNIT_WORK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - register file clk gating"]
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
                "timer_unit0_core1_stall_en",
                &self.timer_unit0_core1_stall_en(),
            )
            .field(
                "timer_unit1_core0_stall_en",
                &self.timer_unit1_core0_stall_en(),
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
    pub fn systimer_clk_fo(&mut self) -> SYSTIMER_CLK_FO_W<'_, CONF_SPEC> {
        SYSTIMER_CLK_FO_W::new(self, 0)
    }
    #[doc = "target(2,1,0) work enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TARGET2_WORK_EN` field.</div>"]
    #[inline(always)]
    pub fn target_work_en(&mut self, n: u8) -> TARGET_WORK_EN_W<'_, CONF_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        TARGET_WORK_EN_W::new(self, n + 22)
    }
    #[doc = "Bit 22 - target2 work enable"]
    #[inline(always)]
    pub fn target2_work_en(&mut self) -> TARGET_WORK_EN_W<'_, CONF_SPEC> {
        TARGET_WORK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - target1 work enable"]
    #[inline(always)]
    pub fn target1_work_en(&mut self) -> TARGET_WORK_EN_W<'_, CONF_SPEC> {
        TARGET_WORK_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - target0 work enable"]
    #[inline(always)]
    pub fn target0_work_en(&mut self) -> TARGET_WORK_EN_W<'_, CONF_SPEC> {
        TARGET_WORK_EN_W::new(self, 24)
    }
    #[doc = "If timer unit(1,0) is stalled when core1 stalled"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER_UNIT1_CORE1_STALL_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_unit_core1_stall_en(
        &mut self,
        n: u8,
    ) -> TIMER_UNIT_CORE1_STALL_EN_W<'_, CONF_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TIMER_UNIT_CORE1_STALL_EN_W::new(self, n * 2 + 25)
    }
    #[doc = "Bit 25 - If timer unit1 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core1_stall_en(&mut self) -> TIMER_UNIT_CORE1_STALL_EN_W<'_, CONF_SPEC> {
        TIMER_UNIT_CORE1_STALL_EN_W::new(self, 25)
    }
    #[doc = "Bit 27 - If timer unit0 is stalled when core1 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core1_stall_en(&mut self) -> TIMER_UNIT_CORE1_STALL_EN_W<'_, CONF_SPEC> {
        TIMER_UNIT_CORE1_STALL_EN_W::new(self, 27)
    }
    #[doc = "If timer unit(1,0) is stalled when core0 stalled"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER_UNIT1_CORE0_STALL_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_unit_core0_stall_en(
        &mut self,
        n: u8,
    ) -> TIMER_UNIT_CORE0_STALL_EN_W<'_, CONF_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TIMER_UNIT_CORE0_STALL_EN_W::new(self, n * 2 + 26)
    }
    #[doc = "Bit 26 - If timer unit1 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit1_core0_stall_en(&mut self) -> TIMER_UNIT_CORE0_STALL_EN_W<'_, CONF_SPEC> {
        TIMER_UNIT_CORE0_STALL_EN_W::new(self, 26)
    }
    #[doc = "Bit 28 - If timer unit0 is stalled when core0 stalled"]
    #[inline(always)]
    pub fn timer_unit0_core0_stall_en(&mut self) -> TIMER_UNIT_CORE0_STALL_EN_W<'_, CONF_SPEC> {
        TIMER_UNIT_CORE0_STALL_EN_W::new(self, 28)
    }
    #[doc = "timer unit(1,0) work enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `TIMER_UNIT1_WORK_EN` field.</div>"]
    #[inline(always)]
    pub fn timer_unit_work_en(&mut self, n: u8) -> TIMER_UNIT_WORK_EN_W<'_, CONF_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TIMER_UNIT_WORK_EN_W::new(self, n + 29)
    }
    #[doc = "Bit 29 - timer unit1 work enable"]
    #[inline(always)]
    pub fn timer_unit1_work_en(&mut self) -> TIMER_UNIT_WORK_EN_W<'_, CONF_SPEC> {
        TIMER_UNIT_WORK_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - timer unit0 work enable"]
    #[inline(always)]
    pub fn timer_unit0_work_en(&mut self) -> TIMER_UNIT_WORK_EN_W<'_, CONF_SPEC> {
        TIMER_UNIT_WORK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - register file clk gating"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, CONF_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "SYSTIMER_CONF.\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
