#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `_0P1A_CNT_TARGET0_REACH_0_HP` writer - reg_0p1a_0_counter after xpd reach target0"]
pub type _0P1A_CNT_TARGET0_REACH_0_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_0P1A_CNT_TARGET1_REACH_0_HP` writer - reg_0p1a_1_counter after xpd reach target1"]
pub type _0P1A_CNT_TARGET1_REACH_0_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_0P1A_CNT_TARGET0_REACH_1_HP` writer - reg_0p1a_0 counter after xpd reach target0"]
pub type _0P1A_CNT_TARGET0_REACH_1_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_0P1A_CNT_TARGET1_REACH_1_HP` writer - reg_0p1a_1_counter after xpd reach target1"]
pub type _0P1A_CNT_TARGET1_REACH_1_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_0P2A_CNT_TARGET0_REACH_0_HP` writer - reg_0p2a_0 counter after xpd reach target0"]
pub type _0P2A_CNT_TARGET0_REACH_0_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_0P2A_CNT_TARGET1_REACH_0_HP` writer - reg_0p2a_1_counter after xpd reach target1"]
pub type _0P2A_CNT_TARGET1_REACH_0_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_0P2A_CNT_TARGET0_REACH_1_HP` writer - reg_0p2a_0 counter after xpd reach target0"]
pub type _0P2A_CNT_TARGET0_REACH_1_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_0P2A_CNT_TARGET1_REACH_1_HP` writer - reg_0p2a_1_counter after xpd reach target1"]
pub type _0P2A_CNT_TARGET1_REACH_1_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_0P3A_CNT_TARGET0_REACH_0_HP` writer - reg_0p3a_0 counter after xpd reach target0"]
pub type _0P3A_CNT_TARGET0_REACH_0_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_0P3A_CNT_TARGET1_REACH_0_HP` writer - reg_0p3a_1_counter after xpd reach target1"]
pub type _0P3A_CNT_TARGET1_REACH_0_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_0P3A_CNT_TARGET0_REACH_1_HP` writer - reg_0p3a_0_counter after xpd reach target0"]
pub type _0P3A_CNT_TARGET0_REACH_1_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `_0P3A_CNT_TARGET1_REACH_1_HP` writer - reg_0p3a_1_counter after xpd reach target1"]
pub type _0P3A_CNT_TARGET1_REACH_1_HP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LP_CPU_EXC` writer - need_des"]
pub type LP_CPU_EXC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SDIO_IDLE` writer - need_des"]
pub type SDIO_IDLE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SW` writer - need_des"]
pub type SW_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SOC_SLEEP_REJECT` writer - need_des"]
pub type SOC_SLEEP_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SOC_WAKEUP` writer - need_des"]
pub type SOC_WAKEUP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 14 - reg_0p1a_0_counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p1a_cnt_target0_reach_0_hp(&mut self) -> _0P1A_CNT_TARGET0_REACH_0_HP_W<INT_CLR_SPEC> {
        _0P1A_CNT_TARGET0_REACH_0_HP_W::new(self, 14)
    }
    #[doc = "Bit 15 - reg_0p1a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p1a_cnt_target1_reach_0_hp(&mut self) -> _0P1A_CNT_TARGET1_REACH_0_HP_W<INT_CLR_SPEC> {
        _0P1A_CNT_TARGET1_REACH_0_HP_W::new(self, 15)
    }
    #[doc = "Bit 16 - reg_0p1a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p1a_cnt_target0_reach_1_hp(&mut self) -> _0P1A_CNT_TARGET0_REACH_1_HP_W<INT_CLR_SPEC> {
        _0P1A_CNT_TARGET0_REACH_1_HP_W::new(self, 16)
    }
    #[doc = "Bit 17 - reg_0p1a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p1a_cnt_target1_reach_1_hp(&mut self) -> _0P1A_CNT_TARGET1_REACH_1_HP_W<INT_CLR_SPEC> {
        _0P1A_CNT_TARGET1_REACH_1_HP_W::new(self, 17)
    }
    #[doc = "Bit 18 - reg_0p2a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p2a_cnt_target0_reach_0_hp(&mut self) -> _0P2A_CNT_TARGET0_REACH_0_HP_W<INT_CLR_SPEC> {
        _0P2A_CNT_TARGET0_REACH_0_HP_W::new(self, 18)
    }
    #[doc = "Bit 19 - reg_0p2a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p2a_cnt_target1_reach_0_hp(&mut self) -> _0P2A_CNT_TARGET1_REACH_0_HP_W<INT_CLR_SPEC> {
        _0P2A_CNT_TARGET1_REACH_0_HP_W::new(self, 19)
    }
    #[doc = "Bit 20 - reg_0p2a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p2a_cnt_target0_reach_1_hp(&mut self) -> _0P2A_CNT_TARGET0_REACH_1_HP_W<INT_CLR_SPEC> {
        _0P2A_CNT_TARGET0_REACH_1_HP_W::new(self, 20)
    }
    #[doc = "Bit 21 - reg_0p2a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p2a_cnt_target1_reach_1_hp(&mut self) -> _0P2A_CNT_TARGET1_REACH_1_HP_W<INT_CLR_SPEC> {
        _0P2A_CNT_TARGET1_REACH_1_HP_W::new(self, 21)
    }
    #[doc = "Bit 22 - reg_0p3a_0 counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p3a_cnt_target0_reach_0_hp(&mut self) -> _0P3A_CNT_TARGET0_REACH_0_HP_W<INT_CLR_SPEC> {
        _0P3A_CNT_TARGET0_REACH_0_HP_W::new(self, 22)
    }
    #[doc = "Bit 23 - reg_0p3a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p3a_cnt_target1_reach_0_hp(&mut self) -> _0P3A_CNT_TARGET1_REACH_0_HP_W<INT_CLR_SPEC> {
        _0P3A_CNT_TARGET1_REACH_0_HP_W::new(self, 23)
    }
    #[doc = "Bit 24 - reg_0p3a_0_counter after xpd reach target0"]
    #[inline(always)]
    pub fn _0p3a_cnt_target0_reach_1_hp(&mut self) -> _0P3A_CNT_TARGET0_REACH_1_HP_W<INT_CLR_SPEC> {
        _0P3A_CNT_TARGET0_REACH_1_HP_W::new(self, 24)
    }
    #[doc = "Bit 25 - reg_0p3a_1_counter after xpd reach target1"]
    #[inline(always)]
    pub fn _0p3a_cnt_target1_reach_1_hp(&mut self) -> _0P3A_CNT_TARGET1_REACH_1_HP_W<INT_CLR_SPEC> {
        _0P3A_CNT_TARGET1_REACH_1_HP_W::new(self, 25)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_exc(&mut self) -> LP_CPU_EXC_W<INT_CLR_SPEC> {
        LP_CPU_EXC_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sdio_idle(&mut self) -> SDIO_IDLE_W<INT_CLR_SPEC> {
        SDIO_IDLE_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<INT_CLR_SPEC> {
        SW_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn soc_sleep_reject(&mut self) -> SOC_SLEEP_REJECT_W<INT_CLR_SPEC> {
        SOC_SLEEP_REJECT_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup(&mut self) -> SOC_WAKEUP_W<INT_CLR_SPEC> {
        SOC_WAKEUP_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xfbff_c000;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
