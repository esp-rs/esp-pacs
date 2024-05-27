#[doc = "Register `LP_INT_ST` reader"]
pub type R = crate::R<LP_INT_ST_SPEC>;
#[doc = "Field `LP_CPU_SLEEP_REJECT` reader - need_des"]
pub type LP_CPU_SLEEP_REJECT_R = crate::BitReader;
#[doc = "Field `_0P1A_CNT_TARGET0_REACH_0_LP` reader - reg_0p1a_0_counter after xpd reach target0"]
pub type _0P1A_CNT_TARGET0_REACH_0_LP_R = crate::BitReader;
#[doc = "Field `_0P1A_CNT_TARGET1_REACH_0_LP` reader - reg_0p1a_1_counter after xpd reach target1"]
pub type _0P1A_CNT_TARGET1_REACH_0_LP_R = crate::BitReader;
#[doc = "Field `_0P1A_CNT_TARGET0_REACH_1_LP` reader - reg_0p1a_0 counter after xpd reach target0"]
pub type _0P1A_CNT_TARGET0_REACH_1_LP_R = crate::BitReader;
#[doc = "Field `_0P1A_CNT_TARGET1_REACH_1_LP` reader - reg_0p1a_1_counter after xpd reach target1"]
pub type _0P1A_CNT_TARGET1_REACH_1_LP_R = crate::BitReader;
#[doc = "Field `_0P2A_CNT_TARGET0_REACH_0_LP` reader - reg_0p2a_0 counter after xpd reach target0"]
pub type _0P2A_CNT_TARGET0_REACH_0_LP_R = crate::BitReader;
#[doc = "Field `_0P2A_CNT_TARGET1_REACH_0_LP` reader - reg_0p2a_1_counter after xpd reach target1"]
pub type _0P2A_CNT_TARGET1_REACH_0_LP_R = crate::BitReader;
#[doc = "Field `_0P2A_CNT_TARGET0_REACH_1_LP` reader - reg_0p2a_0 counter after xpd reach target0"]
pub type _0P2A_CNT_TARGET0_REACH_1_LP_R = crate::BitReader;
#[doc = "Field `_0P2A_CNT_TARGET1_REACH_1_LP` reader - reg_0p2a_1_counter after xpd reach target1"]
pub type _0P2A_CNT_TARGET1_REACH_1_LP_R = crate::BitReader;
#[doc = "Field `_0P3A_CNT_TARGET0_REACH_0_LP` reader - reg_0p3a_0 counter after xpd reach target0"]
pub type _0P3A_CNT_TARGET0_REACH_0_LP_R = crate::BitReader;
#[doc = "Field `_0P3A_CNT_TARGET1_REACH_0_LP` reader - reg_0p3a_1_counter after xpd reach target1"]
pub type _0P3A_CNT_TARGET1_REACH_0_LP_R = crate::BitReader;
#[doc = "Field `_0P3A_CNT_TARGET0_REACH_1_LP` reader - reg_0p3a_0_counter after xpd reach target0"]
pub type _0P3A_CNT_TARGET0_REACH_1_LP_R = crate::BitReader;
#[doc = "Field `_0P3A_CNT_TARGET1_REACH_1_LP` reader - reg_0p3a_1_counter after xpd reach target1"]
pub type _0P3A_CNT_TARGET1_REACH_1_LP_R = crate::BitReader;
#[doc = "Field `LP_CPU_WAKEUP` reader - need_des"]
pub type LP_CPU_WAKEUP_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_END` reader - need_des"]
pub type SLEEP_SWITCH_ACTIVE_END_R = crate::BitReader;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_END` reader - need_des"]
pub type ACTIVE_SWITCH_SLEEP_END_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_START` reader - need_des"]
pub type SLEEP_SWITCH_ACTIVE_START_R = crate::BitReader;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_START` reader - need_des"]
pub type ACTIVE_SWITCH_SLEEP_START_R = crate::BitReader;
#[doc = "Field `HP_SW_TRIGGER` reader - need_des"]
pub type HP_SW_TRIGGER_R = crate::BitReader;
impl R {
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_sleep_reject(&self) -> LP_CPU_SLEEP_REJECT_R {
        LP_CPU_SLEEP_REJECT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reg_0p1a_0_counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p1a_cnt_target0_reach_0_lp(&self) -> _0P1A_CNT_TARGET0_REACH_0_LP_R {
        _0P1A_CNT_TARGET0_REACH_0_LP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - reg_0p1a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p1a_cnt_target1_reach_0_lp(&self) -> _0P1A_CNT_TARGET1_REACH_0_LP_R {
        _0P1A_CNT_TARGET1_REACH_0_LP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - reg_0p1a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p1a_cnt_target0_reach_1_lp(&self) -> _0P1A_CNT_TARGET0_REACH_1_LP_R {
        _0P1A_CNT_TARGET0_REACH_1_LP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reg_0p1a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p1a_cnt_target1_reach_1_lp(&self) -> _0P1A_CNT_TARGET1_REACH_1_LP_R {
        _0P1A_CNT_TARGET1_REACH_1_LP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - reg_0p2a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p2a_cnt_target0_reach_0_lp(&self) -> _0P2A_CNT_TARGET0_REACH_0_LP_R {
        _0P2A_CNT_TARGET0_REACH_0_LP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reg_0p2a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p2a_cnt_target1_reach_0_lp(&self) -> _0P2A_CNT_TARGET1_REACH_0_LP_R {
        _0P2A_CNT_TARGET1_REACH_0_LP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - reg_0p2a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p2a_cnt_target0_reach_1_lp(&self) -> _0P2A_CNT_TARGET0_REACH_1_LP_R {
        _0P2A_CNT_TARGET0_REACH_1_LP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - reg_0p2a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p2a_cnt_target1_reach_1_lp(&self) -> _0P2A_CNT_TARGET1_REACH_1_LP_R {
        _0P2A_CNT_TARGET1_REACH_1_LP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - reg_0p3a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p3a_cnt_target0_reach_0_lp(&self) -> _0P3A_CNT_TARGET0_REACH_0_LP_R {
        _0P3A_CNT_TARGET0_REACH_0_LP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - reg_0p3a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p3a_cnt_target1_reach_0_lp(&self) -> _0P3A_CNT_TARGET1_REACH_0_LP_R {
        _0P3A_CNT_TARGET1_REACH_0_LP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reg_0p3a_0_counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p3a_cnt_target0_reach_1_lp(&self) -> _0P3A_CNT_TARGET0_REACH_1_LP_R {
        _0P3A_CNT_TARGET0_REACH_1_LP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reg_0p3a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p3a_cnt_target1_reach_1_lp(&self) -> _0P3A_CNT_TARGET1_REACH_1_LP_R {
        _0P3A_CNT_TARGET1_REACH_1_LP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup(&self) -> LP_CPU_WAKEUP_R {
        LP_CPU_WAKEUP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_end(&self) -> SLEEP_SWITCH_ACTIVE_END_R {
        SLEEP_SWITCH_ACTIVE_END_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_end(&self) -> ACTIVE_SWITCH_SLEEP_END_R {
        ACTIVE_SWITCH_SLEEP_END_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_start(&self) -> SLEEP_SWITCH_ACTIVE_START_R {
        SLEEP_SWITCH_ACTIVE_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_start(&self) -> ACTIVE_SWITCH_SLEEP_START_R {
        ACTIVE_SWITCH_SLEEP_START_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_sw_trigger(&self) -> HP_SW_TRIGGER_R {
        HP_SW_TRIGGER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ST")
            .field("lp_cpu_sleep_reject", &self.lp_cpu_sleep_reject())
            .field(
                "_0p1a_cnt_target0_reach_0_lp",
                &self._0p1a_cnt_target0_reach_0_lp(),
            )
            .field(
                "_0p1a_cnt_target1_reach_0_lp",
                &self._0p1a_cnt_target1_reach_0_lp(),
            )
            .field(
                "_0p1a_cnt_target0_reach_1_lp",
                &self._0p1a_cnt_target0_reach_1_lp(),
            )
            .field(
                "_0p1a_cnt_target1_reach_1_lp",
                &self._0p1a_cnt_target1_reach_1_lp(),
            )
            .field(
                "_0p2a_cnt_target0_reach_0_lp",
                &self._0p2a_cnt_target0_reach_0_lp(),
            )
            .field(
                "_0p2a_cnt_target1_reach_0_lp",
                &self._0p2a_cnt_target1_reach_0_lp(),
            )
            .field(
                "_0p2a_cnt_target0_reach_1_lp",
                &self._0p2a_cnt_target0_reach_1_lp(),
            )
            .field(
                "_0p2a_cnt_target1_reach_1_lp",
                &self._0p2a_cnt_target1_reach_1_lp(),
            )
            .field(
                "_0p3a_cnt_target0_reach_0_lp",
                &self._0p3a_cnt_target0_reach_0_lp(),
            )
            .field(
                "_0p3a_cnt_target1_reach_0_lp",
                &self._0p3a_cnt_target1_reach_0_lp(),
            )
            .field(
                "_0p3a_cnt_target0_reach_1_lp",
                &self._0p3a_cnt_target0_reach_1_lp(),
            )
            .field(
                "_0p3a_cnt_target1_reach_1_lp",
                &self._0p3a_cnt_target1_reach_1_lp(),
            )
            .field("lp_cpu_wakeup", &self.lp_cpu_wakeup())
            .field("sleep_switch_active_end", &self.sleep_switch_active_end())
            .field("active_switch_sleep_end", &self.active_switch_sleep_end())
            .field(
                "sleep_switch_active_start",
                &self.sleep_switch_active_start(),
            )
            .field(
                "active_switch_sleep_start",
                &self.active_switch_sleep_start(),
            )
            .field("hp_sw_trigger", &self.hp_sw_trigger())
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_ST_SPEC;
impl crate::RegisterSpec for LP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_st::R`](R) reader structure"]
impl crate::Readable for LP_INT_ST_SPEC {}
#[doc = "`reset()` method sets LP_INT_ST to value 0"]
impl crate::Resettable for LP_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
