///Register `LP_INT_CLR` writer
pub type W = crate::W<LP_INT_CLR_SPEC>;
///Field `LP_CPU_SLEEP_REJECT_LP` writer - need_des
pub type LP_CPU_SLEEP_REJECT_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P1A_CNT_TARGET0_REACH_0_LP` writer - reg_0p1a_0_counter after xpd reach target0
pub type _0P1A_CNT_TARGET0_REACH_0_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P1A_CNT_TARGET1_REACH_0_LP` writer - reg_0p1a_1_counter after xpd reach target1
pub type _0P1A_CNT_TARGET1_REACH_0_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P1A_CNT_TARGET0_REACH_1_LP` writer - reg_0p1a_0 counter after xpd reach target0
pub type _0P1A_CNT_TARGET0_REACH_1_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P1A_CNT_TARGET1_REACH_1_LP` writer - reg_0p1a_1_counter after xpd reach target1
pub type _0P1A_CNT_TARGET1_REACH_1_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P2A_CNT_TARGET0_REACH_0_LP` writer - reg_0p2a_0 counter after xpd reach target0
pub type _0P2A_CNT_TARGET0_REACH_0_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P2A_CNT_TARGET1_REACH_0_LP` writer - reg_0p2a_1_counter after xpd reach target1
pub type _0P2A_CNT_TARGET1_REACH_0_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P2A_CNT_TARGET0_REACH_1_LP` writer - reg_0p2a_0 counter after xpd reach target0
pub type _0P2A_CNT_TARGET0_REACH_1_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P2A_CNT_TARGET1_REACH_1_LP` writer - reg_0p2a_1_counter after xpd reach target1
pub type _0P2A_CNT_TARGET1_REACH_1_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P3A_CNT_TARGET0_REACH_0_LP` writer - reg_0p3a_0 counter after xpd reach target0
pub type _0P3A_CNT_TARGET0_REACH_0_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P3A_CNT_TARGET1_REACH_0_LP` writer - reg_0p3a_1_counter after xpd reach target1
pub type _0P3A_CNT_TARGET1_REACH_0_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P3A_CNT_TARGET0_REACH_1_LP` writer - reg_0p3a_0_counter after xpd reach target0
pub type _0P3A_CNT_TARGET0_REACH_1_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `_0P3A_CNT_TARGET1_REACH_1_LP` writer - reg_0p3a_1_counter after xpd reach target1
pub type _0P3A_CNT_TARGET1_REACH_1_LP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `LP_CPU_WAKEUP` writer - need_des
pub type LP_CPU_WAKEUP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SLEEP_SWITCH_ACTIVE_END` writer - need_des
pub type SLEEP_SWITCH_ACTIVE_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ACTIVE_SWITCH_SLEEP_END` writer - need_des
pub type ACTIVE_SWITCH_SLEEP_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SLEEP_SWITCH_ACTIVE_START` writer - need_des
pub type SLEEP_SWITCH_ACTIVE_START_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ACTIVE_SWITCH_SLEEP_START` writer - need_des
pub type ACTIVE_SWITCH_SLEEP_START_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `HP_SW_TRIGGER` writer - need_des
pub type HP_SW_TRIGGER_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 13 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_sleep_reject_lp(&mut self) -> LP_CPU_SLEEP_REJECT_LP_W<LP_INT_CLR_SPEC> {
        LP_CPU_SLEEP_REJECT_LP_W::new(self, 13)
    }
    ///Bit 14 - reg_0p1a_0_counter after xpd reach target0
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_cnt_target0_reach_0_lp(
        &mut self,
    ) -> _0P1A_CNT_TARGET0_REACH_0_LP_W<LP_INT_CLR_SPEC> {
        _0P1A_CNT_TARGET0_REACH_0_LP_W::new(self, 14)
    }
    ///Bit 15 - reg_0p1a_1_counter after xpd reach target1
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_cnt_target1_reach_0_lp(
        &mut self,
    ) -> _0P1A_CNT_TARGET1_REACH_0_LP_W<LP_INT_CLR_SPEC> {
        _0P1A_CNT_TARGET1_REACH_0_LP_W::new(self, 15)
    }
    ///Bit 16 - reg_0p1a_0 counter after xpd reach target0
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_cnt_target0_reach_1_lp(
        &mut self,
    ) -> _0P1A_CNT_TARGET0_REACH_1_LP_W<LP_INT_CLR_SPEC> {
        _0P1A_CNT_TARGET0_REACH_1_LP_W::new(self, 16)
    }
    ///Bit 17 - reg_0p1a_1_counter after xpd reach target1
    #[inline(always)]
    #[must_use]
    pub fn _0p1a_cnt_target1_reach_1_lp(
        &mut self,
    ) -> _0P1A_CNT_TARGET1_REACH_1_LP_W<LP_INT_CLR_SPEC> {
        _0P1A_CNT_TARGET1_REACH_1_LP_W::new(self, 17)
    }
    ///Bit 18 - reg_0p2a_0 counter after xpd reach target0
    #[inline(always)]
    #[must_use]
    pub fn _0p2a_cnt_target0_reach_0_lp(
        &mut self,
    ) -> _0P2A_CNT_TARGET0_REACH_0_LP_W<LP_INT_CLR_SPEC> {
        _0P2A_CNT_TARGET0_REACH_0_LP_W::new(self, 18)
    }
    ///Bit 19 - reg_0p2a_1_counter after xpd reach target1
    #[inline(always)]
    #[must_use]
    pub fn _0p2a_cnt_target1_reach_0_lp(
        &mut self,
    ) -> _0P2A_CNT_TARGET1_REACH_0_LP_W<LP_INT_CLR_SPEC> {
        _0P2A_CNT_TARGET1_REACH_0_LP_W::new(self, 19)
    }
    ///Bit 20 - reg_0p2a_0 counter after xpd reach target0
    #[inline(always)]
    #[must_use]
    pub fn _0p2a_cnt_target0_reach_1_lp(
        &mut self,
    ) -> _0P2A_CNT_TARGET0_REACH_1_LP_W<LP_INT_CLR_SPEC> {
        _0P2A_CNT_TARGET0_REACH_1_LP_W::new(self, 20)
    }
    ///Bit 21 - reg_0p2a_1_counter after xpd reach target1
    #[inline(always)]
    #[must_use]
    pub fn _0p2a_cnt_target1_reach_1_lp(
        &mut self,
    ) -> _0P2A_CNT_TARGET1_REACH_1_LP_W<LP_INT_CLR_SPEC> {
        _0P2A_CNT_TARGET1_REACH_1_LP_W::new(self, 21)
    }
    ///Bit 22 - reg_0p3a_0 counter after xpd reach target0
    #[inline(always)]
    #[must_use]
    pub fn _0p3a_cnt_target0_reach_0_lp(
        &mut self,
    ) -> _0P3A_CNT_TARGET0_REACH_0_LP_W<LP_INT_CLR_SPEC> {
        _0P3A_CNT_TARGET0_REACH_0_LP_W::new(self, 22)
    }
    ///Bit 23 - reg_0p3a_1_counter after xpd reach target1
    #[inline(always)]
    #[must_use]
    pub fn _0p3a_cnt_target1_reach_0_lp(
        &mut self,
    ) -> _0P3A_CNT_TARGET1_REACH_0_LP_W<LP_INT_CLR_SPEC> {
        _0P3A_CNT_TARGET1_REACH_0_LP_W::new(self, 23)
    }
    ///Bit 24 - reg_0p3a_0_counter after xpd reach target0
    #[inline(always)]
    #[must_use]
    pub fn _0p3a_cnt_target0_reach_1_lp(
        &mut self,
    ) -> _0P3A_CNT_TARGET0_REACH_1_LP_W<LP_INT_CLR_SPEC> {
        _0P3A_CNT_TARGET0_REACH_1_LP_W::new(self, 24)
    }
    ///Bit 25 - reg_0p3a_1_counter after xpd reach target1
    #[inline(always)]
    #[must_use]
    pub fn _0p3a_cnt_target1_reach_1_lp(
        &mut self,
    ) -> _0P3A_CNT_TARGET1_REACH_1_LP_W<LP_INT_CLR_SPEC> {
        _0P3A_CNT_TARGET1_REACH_1_LP_W::new(self, 25)
    }
    ///Bit 26 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_wakeup(&mut self) -> LP_CPU_WAKEUP_W<LP_INT_CLR_SPEC> {
        LP_CPU_WAKEUP_W::new(self, 26)
    }
    ///Bit 27 - need_des
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_active_end(&mut self) -> SLEEP_SWITCH_ACTIVE_END_W<LP_INT_CLR_SPEC> {
        SLEEP_SWITCH_ACTIVE_END_W::new(self, 27)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn active_switch_sleep_end(&mut self) -> ACTIVE_SWITCH_SLEEP_END_W<LP_INT_CLR_SPEC> {
        ACTIVE_SWITCH_SLEEP_END_W::new(self, 28)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_active_start(&mut self) -> SLEEP_SWITCH_ACTIVE_START_W<LP_INT_CLR_SPEC> {
        SLEEP_SWITCH_ACTIVE_START_W::new(self, 29)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn active_switch_sleep_start(&mut self) -> ACTIVE_SWITCH_SLEEP_START_W<LP_INT_CLR_SPEC> {
        ACTIVE_SWITCH_SLEEP_START_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sw_trigger(&mut self) -> HP_SW_TRIGGER_W<LP_INT_CLR_SPEC> {
        HP_SW_TRIGGER_W::new(self, 31)
    }
}
/**need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_INT_CLR_SPEC;
impl crate::RegisterSpec for LP_INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lp_int_clr::W`](W) writer structure
impl crate::Writable for LP_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_e000;
}
///`reset()` method sets LP_INT_CLR to value 0
impl crate::Resettable for LP_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
