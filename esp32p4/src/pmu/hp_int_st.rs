#[doc = "Register `HP_INT_ST` reader"]
pub type R = crate::R<HP_INT_ST_SPEC>;
#[doc = "Field `_0P1A_CNT_TARGET0_REACH_0_HP_INT_ST` reader - reg_0p1a_0_counter after xpd reach target0"]
pub type _0P1A_CNT_TARGET0_REACH_0_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `_0P1A_CNT_TARGET1_REACH_0_HP_INT_ST` reader - reg_0p1a_1_counter after xpd reach target1"]
pub type _0P1A_CNT_TARGET1_REACH_0_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `_0P1A_CNT_TARGET0_REACH_1_HP_INT_ST` reader - reg_0p1a_0 counter after xpd reach target0"]
pub type _0P1A_CNT_TARGET0_REACH_1_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `_0P1A_CNT_TARGET1_REACH_1_HP_INT_ST` reader - reg_0p1a_1_counter after xpd reach target1"]
pub type _0P1A_CNT_TARGET1_REACH_1_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `_0P2A_CNT_TARGET0_REACH_0_HP_INT_ST` reader - reg_0p2a_0 counter after xpd reach target0"]
pub type _0P2A_CNT_TARGET0_REACH_0_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `_0P2A_CNT_TARGET1_REACH_0_HP_INT_ST` reader - reg_0p2a_1_counter after xpd reach target1"]
pub type _0P2A_CNT_TARGET1_REACH_0_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `_0P2A_CNT_TARGET0_REACH_1_HP_INT_ST` reader - reg_0p2a_0 counter after xpd reach target0"]
pub type _0P2A_CNT_TARGET0_REACH_1_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `_0P2A_CNT_TARGET1_REACH_1_HP_INT_ST` reader - reg_0p2a_1_counter after xpd reach target1"]
pub type _0P2A_CNT_TARGET1_REACH_1_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `_0P3A_CNT_TARGET0_REACH_0_HP_INT_ST` reader - reg_0p3a_0 counter after xpd reach target0"]
pub type _0P3A_CNT_TARGET0_REACH_0_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `_0P3A_CNT_TARGET1_REACH_0_HP_INT_ST` reader - reg_0p3a_1_counter after xpd reach target1"]
pub type _0P3A_CNT_TARGET1_REACH_0_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `_0P3A_CNT_TARGET0_REACH_1_HP_INT_ST` reader - reg_0p3a_0_counter after xpd reach target0"]
pub type _0P3A_CNT_TARGET0_REACH_1_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `_0P3A_CNT_TARGET1_REACH_1_HP_INT_ST` reader - reg_0p3a_1_counter after xpd reach target1"]
pub type _0P3A_CNT_TARGET1_REACH_1_HP_INT_ST_R = crate::BitReader;
#[doc = "Field `LP_CPU_EXC_INT_ST` reader - need_des"]
pub type LP_CPU_EXC_INT_ST_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE_INT_ST` reader - need_des"]
pub type SDIO_IDLE_INT_ST_R = crate::BitReader;
#[doc = "Field `SW_INT_ST` reader - need_des"]
pub type SW_INT_ST_R = crate::BitReader;
#[doc = "Field `SOC_SLEEP_REJECT_INT_ST` reader - need_des"]
pub type SOC_SLEEP_REJECT_INT_ST_R = crate::BitReader;
#[doc = "Field `SOC_WAKEUP_INT_ST` reader - need_des"]
pub type SOC_WAKEUP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 14 - reg_0p1a_0_counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p1a_cnt_target0_reach_0_hp_int_st(&self) -> _0P1A_CNT_TARGET0_REACH_0_HP_INT_ST_R {
        _0P1A_CNT_TARGET0_REACH_0_HP_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - reg_0p1a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p1a_cnt_target1_reach_0_hp_int_st(&self) -> _0P1A_CNT_TARGET1_REACH_0_HP_INT_ST_R {
        _0P1A_CNT_TARGET1_REACH_0_HP_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - reg_0p1a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p1a_cnt_target0_reach_1_hp_int_st(&self) -> _0P1A_CNT_TARGET0_REACH_1_HP_INT_ST_R {
        _0P1A_CNT_TARGET0_REACH_1_HP_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reg_0p1a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p1a_cnt_target1_reach_1_hp_int_st(&self) -> _0P1A_CNT_TARGET1_REACH_1_HP_INT_ST_R {
        _0P1A_CNT_TARGET1_REACH_1_HP_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - reg_0p2a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p2a_cnt_target0_reach_0_hp_int_st(&self) -> _0P2A_CNT_TARGET0_REACH_0_HP_INT_ST_R {
        _0P2A_CNT_TARGET0_REACH_0_HP_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reg_0p2a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p2a_cnt_target1_reach_0_hp_int_st(&self) -> _0P2A_CNT_TARGET1_REACH_0_HP_INT_ST_R {
        _0P2A_CNT_TARGET1_REACH_0_HP_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - reg_0p2a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p2a_cnt_target0_reach_1_hp_int_st(&self) -> _0P2A_CNT_TARGET0_REACH_1_HP_INT_ST_R {
        _0P2A_CNT_TARGET0_REACH_1_HP_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - reg_0p2a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p2a_cnt_target1_reach_1_hp_int_st(&self) -> _0P2A_CNT_TARGET1_REACH_1_HP_INT_ST_R {
        _0P2A_CNT_TARGET1_REACH_1_HP_INT_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - reg_0p3a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p3a_cnt_target0_reach_0_hp_int_st(&self) -> _0P3A_CNT_TARGET0_REACH_0_HP_INT_ST_R {
        _0P3A_CNT_TARGET0_REACH_0_HP_INT_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - reg_0p3a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p3a_cnt_target1_reach_0_hp_int_st(&self) -> _0P3A_CNT_TARGET1_REACH_0_HP_INT_ST_R {
        _0P3A_CNT_TARGET1_REACH_0_HP_INT_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reg_0p3a_0_counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p3a_cnt_target0_reach_1_hp_int_st(&self) -> _0P3A_CNT_TARGET0_REACH_1_HP_INT_ST_R {
        _0P3A_CNT_TARGET0_REACH_1_HP_INT_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reg_0p3a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p3a_cnt_target1_reach_1_hp_int_st(&self) -> _0P3A_CNT_TARGET1_REACH_1_HP_INT_ST_R {
        _0P3A_CNT_TARGET1_REACH_1_HP_INT_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_exc_int_st(&self) -> LP_CPU_EXC_INT_ST_R {
        LP_CPU_EXC_INT_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sdio_idle_int_st(&self) -> SDIO_IDLE_INT_ST_R {
        SDIO_IDLE_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn sw_int_st(&self) -> SW_INT_ST_R {
        SW_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn soc_sleep_reject_int_st(&self) -> SOC_SLEEP_REJECT_INT_ST_R {
        SOC_SLEEP_REJECT_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_int_st(&self) -> SOC_WAKEUP_INT_ST_R {
        SOC_WAKEUP_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_INT_ST")
            .field(
                "_0p1a_cnt_target0_reach_0_hp_int_st",
                &format_args!("{}", self._0p1a_cnt_target0_reach_0_hp_int_st().bit()),
            )
            .field(
                "_0p1a_cnt_target1_reach_0_hp_int_st",
                &format_args!("{}", self._0p1a_cnt_target1_reach_0_hp_int_st().bit()),
            )
            .field(
                "_0p1a_cnt_target0_reach_1_hp_int_st",
                &format_args!("{}", self._0p1a_cnt_target0_reach_1_hp_int_st().bit()),
            )
            .field(
                "_0p1a_cnt_target1_reach_1_hp_int_st",
                &format_args!("{}", self._0p1a_cnt_target1_reach_1_hp_int_st().bit()),
            )
            .field(
                "_0p2a_cnt_target0_reach_0_hp_int_st",
                &format_args!("{}", self._0p2a_cnt_target0_reach_0_hp_int_st().bit()),
            )
            .field(
                "_0p2a_cnt_target1_reach_0_hp_int_st",
                &format_args!("{}", self._0p2a_cnt_target1_reach_0_hp_int_st().bit()),
            )
            .field(
                "_0p2a_cnt_target0_reach_1_hp_int_st",
                &format_args!("{}", self._0p2a_cnt_target0_reach_1_hp_int_st().bit()),
            )
            .field(
                "_0p2a_cnt_target1_reach_1_hp_int_st",
                &format_args!("{}", self._0p2a_cnt_target1_reach_1_hp_int_st().bit()),
            )
            .field(
                "_0p3a_cnt_target0_reach_0_hp_int_st",
                &format_args!("{}", self._0p3a_cnt_target0_reach_0_hp_int_st().bit()),
            )
            .field(
                "_0p3a_cnt_target1_reach_0_hp_int_st",
                &format_args!("{}", self._0p3a_cnt_target1_reach_0_hp_int_st().bit()),
            )
            .field(
                "_0p3a_cnt_target0_reach_1_hp_int_st",
                &format_args!("{}", self._0p3a_cnt_target0_reach_1_hp_int_st().bit()),
            )
            .field(
                "_0p3a_cnt_target1_reach_1_hp_int_st",
                &format_args!("{}", self._0p3a_cnt_target1_reach_1_hp_int_st().bit()),
            )
            .field(
                "lp_cpu_exc_int_st",
                &format_args!("{}", self.lp_cpu_exc_int_st().bit()),
            )
            .field(
                "sdio_idle_int_st",
                &format_args!("{}", self.sdio_idle_int_st().bit()),
            )
            .field("sw_int_st", &format_args!("{}", self.sw_int_st().bit()))
            .field(
                "soc_sleep_reject_int_st",
                &format_args!("{}", self.soc_sleep_reject_int_st().bit()),
            )
            .field(
                "soc_wakeup_int_st",
                &format_args!("{}", self.soc_wakeup_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_INT_ST_SPEC;
impl crate::RegisterSpec for HP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_int_st::R`](R) reader structure"]
impl crate::Readable for HP_INT_ST_SPEC {}
#[doc = "`reset()` method sets HP_INT_ST to value 0"]
impl crate::Resettable for HP_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
