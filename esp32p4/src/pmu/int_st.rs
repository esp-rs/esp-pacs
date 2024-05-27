///Register `INT_ST` reader
pub type R = crate::R<INT_ST_SPEC>;
///Field `_0P1A_CNT_TARGET0_REACH_0_HP` reader - reg_0p1a_0_counter after xpd reach target0
pub type _0P1A_CNT_TARGET0_REACH_0_HP_R = crate::BitReader;
///Field `_0P1A_CNT_TARGET1_REACH_0_HP` reader - reg_0p1a_1_counter after xpd reach target1
pub type _0P1A_CNT_TARGET1_REACH_0_HP_R = crate::BitReader;
///Field `_0P1A_CNT_TARGET0_REACH_1_HP` reader - reg_0p1a_0 counter after xpd reach target0
pub type _0P1A_CNT_TARGET0_REACH_1_HP_R = crate::BitReader;
///Field `_0P1A_CNT_TARGET1_REACH_1_HP` reader - reg_0p1a_1_counter after xpd reach target1
pub type _0P1A_CNT_TARGET1_REACH_1_HP_R = crate::BitReader;
///Field `_0P2A_CNT_TARGET0_REACH_0_HP` reader - reg_0p2a_0 counter after xpd reach target0
pub type _0P2A_CNT_TARGET0_REACH_0_HP_R = crate::BitReader;
///Field `_0P2A_CNT_TARGET1_REACH_0_HP` reader - reg_0p2a_1_counter after xpd reach target1
pub type _0P2A_CNT_TARGET1_REACH_0_HP_R = crate::BitReader;
///Field `_0P2A_CNT_TARGET0_REACH_1_HP` reader - reg_0p2a_0 counter after xpd reach target0
pub type _0P2A_CNT_TARGET0_REACH_1_HP_R = crate::BitReader;
///Field `_0P2A_CNT_TARGET1_REACH_1_HP` reader - reg_0p2a_1_counter after xpd reach target1
pub type _0P2A_CNT_TARGET1_REACH_1_HP_R = crate::BitReader;
///Field `_0P3A_CNT_TARGET0_REACH_0_HP` reader - reg_0p3a_0 counter after xpd reach target0
pub type _0P3A_CNT_TARGET0_REACH_0_HP_R = crate::BitReader;
///Field `_0P3A_CNT_TARGET1_REACH_0_HP` reader - reg_0p3a_1_counter after xpd reach target1
pub type _0P3A_CNT_TARGET1_REACH_0_HP_R = crate::BitReader;
///Field `_0P3A_CNT_TARGET0_REACH_1_HP` reader - reg_0p3a_0_counter after xpd reach target0
pub type _0P3A_CNT_TARGET0_REACH_1_HP_R = crate::BitReader;
///Field `_0P3A_CNT_TARGET1_REACH_1_HP` reader - reg_0p3a_1_counter after xpd reach target1
pub type _0P3A_CNT_TARGET1_REACH_1_HP_R = crate::BitReader;
///Field `LP_CPU_EXC` reader - need_des
pub type LP_CPU_EXC_R = crate::BitReader;
///Field `SDIO_IDLE` reader - need_des
pub type SDIO_IDLE_R = crate::BitReader;
///Field `SW` reader - need_des
pub type SW_R = crate::BitReader;
///Field `SOC_SLEEP_REJECT` reader - need_des
pub type SOC_SLEEP_REJECT_R = crate::BitReader;
///Field `SOC_WAKEUP` reader - need_des
pub type SOC_WAKEUP_R = crate::BitReader;
impl R {
    ///Bit 14 - reg_0p1a_0_counter after xpd reach target0
    #[inline(always)]
    pub fn _0p1a_cnt_target0_reach_0_hp(&self) -> _0P1A_CNT_TARGET0_REACH_0_HP_R {
        _0P1A_CNT_TARGET0_REACH_0_HP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - reg_0p1a_1_counter after xpd reach target1
    #[inline(always)]
    pub fn _0p1a_cnt_target1_reach_0_hp(&self) -> _0P1A_CNT_TARGET1_REACH_0_HP_R {
        _0P1A_CNT_TARGET1_REACH_0_HP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - reg_0p1a_0 counter after xpd reach target0
    #[inline(always)]
    pub fn _0p1a_cnt_target0_reach_1_hp(&self) -> _0P1A_CNT_TARGET0_REACH_1_HP_R {
        _0P1A_CNT_TARGET0_REACH_1_HP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - reg_0p1a_1_counter after xpd reach target1
    #[inline(always)]
    pub fn _0p1a_cnt_target1_reach_1_hp(&self) -> _0P1A_CNT_TARGET1_REACH_1_HP_R {
        _0P1A_CNT_TARGET1_REACH_1_HP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - reg_0p2a_0 counter after xpd reach target0
    #[inline(always)]
    pub fn _0p2a_cnt_target0_reach_0_hp(&self) -> _0P2A_CNT_TARGET0_REACH_0_HP_R {
        _0P2A_CNT_TARGET0_REACH_0_HP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - reg_0p2a_1_counter after xpd reach target1
    #[inline(always)]
    pub fn _0p2a_cnt_target1_reach_0_hp(&self) -> _0P2A_CNT_TARGET1_REACH_0_HP_R {
        _0P2A_CNT_TARGET1_REACH_0_HP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - reg_0p2a_0 counter after xpd reach target0
    #[inline(always)]
    pub fn _0p2a_cnt_target0_reach_1_hp(&self) -> _0P2A_CNT_TARGET0_REACH_1_HP_R {
        _0P2A_CNT_TARGET0_REACH_1_HP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - reg_0p2a_1_counter after xpd reach target1
    #[inline(always)]
    pub fn _0p2a_cnt_target1_reach_1_hp(&self) -> _0P2A_CNT_TARGET1_REACH_1_HP_R {
        _0P2A_CNT_TARGET1_REACH_1_HP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - reg_0p3a_0 counter after xpd reach target0
    #[inline(always)]
    pub fn _0p3a_cnt_target0_reach_0_hp(&self) -> _0P3A_CNT_TARGET0_REACH_0_HP_R {
        _0P3A_CNT_TARGET0_REACH_0_HP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - reg_0p3a_1_counter after xpd reach target1
    #[inline(always)]
    pub fn _0p3a_cnt_target1_reach_0_hp(&self) -> _0P3A_CNT_TARGET1_REACH_0_HP_R {
        _0P3A_CNT_TARGET1_REACH_0_HP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - reg_0p3a_0_counter after xpd reach target0
    #[inline(always)]
    pub fn _0p3a_cnt_target0_reach_1_hp(&self) -> _0P3A_CNT_TARGET0_REACH_1_HP_R {
        _0P3A_CNT_TARGET0_REACH_1_HP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - reg_0p3a_1_counter after xpd reach target1
    #[inline(always)]
    pub fn _0p3a_cnt_target1_reach_1_hp(&self) -> _0P3A_CNT_TARGET1_REACH_1_HP_R {
        _0P3A_CNT_TARGET1_REACH_1_HP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 27 - need_des
    #[inline(always)]
    pub fn lp_cpu_exc(&self) -> LP_CPU_EXC_R {
        LP_CPU_EXC_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    pub fn sdio_idle(&self) -> SDIO_IDLE_R {
        SDIO_IDLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn soc_sleep_reject(&self) -> SOC_SLEEP_REJECT_R {
        SOC_SLEEP_REJECT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn soc_wakeup(&self) -> SOC_WAKEUP_R {
        SOC_WAKEUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "_0p1a_cnt_target0_reach_0_hp",
                &self._0p1a_cnt_target0_reach_0_hp(),
            )
            .field(
                "_0p1a_cnt_target1_reach_0_hp",
                &self._0p1a_cnt_target1_reach_0_hp(),
            )
            .field(
                "_0p1a_cnt_target0_reach_1_hp",
                &self._0p1a_cnt_target0_reach_1_hp(),
            )
            .field(
                "_0p1a_cnt_target1_reach_1_hp",
                &self._0p1a_cnt_target1_reach_1_hp(),
            )
            .field(
                "_0p2a_cnt_target0_reach_0_hp",
                &self._0p2a_cnt_target0_reach_0_hp(),
            )
            .field(
                "_0p2a_cnt_target1_reach_0_hp",
                &self._0p2a_cnt_target1_reach_0_hp(),
            )
            .field(
                "_0p2a_cnt_target0_reach_1_hp",
                &self._0p2a_cnt_target0_reach_1_hp(),
            )
            .field(
                "_0p2a_cnt_target1_reach_1_hp",
                &self._0p2a_cnt_target1_reach_1_hp(),
            )
            .field(
                "_0p3a_cnt_target0_reach_0_hp",
                &self._0p3a_cnt_target0_reach_0_hp(),
            )
            .field(
                "_0p3a_cnt_target1_reach_0_hp",
                &self._0p3a_cnt_target1_reach_0_hp(),
            )
            .field(
                "_0p3a_cnt_target0_reach_1_hp",
                &self._0p3a_cnt_target0_reach_1_hp(),
            )
            .field(
                "_0p3a_cnt_target1_reach_1_hp",
                &self._0p3a_cnt_target1_reach_1_hp(),
            )
            .field("lp_cpu_exc", &self.lp_cpu_exc())
            .field("sdio_idle", &self.sdio_idle())
            .field("sw", &self.sw())
            .field("soc_sleep_reject", &self.soc_sleep_reject())
            .field("soc_wakeup", &self.soc_wakeup())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_st::R`](R) reader structure
impl crate::Readable for INT_ST_SPEC {}
///`reset()` method sets INT_ST to value 0
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
