#[doc = "Register `LP_INT_ST` reader"]
pub struct R(crate::R<LP_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LP_CPU_WAKEUP_INT_ST` reader - need_des"]
pub type LP_CPU_WAKEUP_INT_ST_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_ACTIVE_END_INT_ST` reader - need_des"]
pub type MODEM_SWITCH_ACTIVE_END_INT_ST_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_END_INT_ST` reader - need_des"]
pub type SLEEP_SWITCH_ACTIVE_END_INT_ST_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_MODEM_END_INT_ST` reader - need_des"]
pub type SLEEP_SWITCH_MODEM_END_INT_ST_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_SLEEP_END_INT_ST` reader - need_des"]
pub type MODEM_SWITCH_SLEEP_END_INT_ST_R = crate::BitReader;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_END_INT_ST` reader - need_des"]
pub type ACTIVE_SWITCH_SLEEP_END_INT_ST_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_ACTIVE_START_INT_ST` reader - need_des"]
pub type MODEM_SWITCH_ACTIVE_START_INT_ST_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_START_INT_ST` reader - need_des"]
pub type SLEEP_SWITCH_ACTIVE_START_INT_ST_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_MODEM_START_INT_ST` reader - need_des"]
pub type SLEEP_SWITCH_MODEM_START_INT_ST_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_SLEEP_START_INT_ST` reader - need_des"]
pub type MODEM_SWITCH_SLEEP_START_INT_ST_R = crate::BitReader;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_START_INT_ST` reader - need_des"]
pub type ACTIVE_SWITCH_SLEEP_START_INT_ST_R = crate::BitReader;
#[doc = "Field `HP_SW_TRIGGER_INT_ST` reader - need_des"]
pub type HP_SW_TRIGGER_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup_int_st(&self) -> LP_CPU_WAKEUP_INT_ST_R {
        LP_CPU_WAKEUP_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn modem_switch_active_end_int_st(&self) -> MODEM_SWITCH_ACTIVE_END_INT_ST_R {
        MODEM_SWITCH_ACTIVE_END_INT_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_end_int_st(&self) -> SLEEP_SWITCH_ACTIVE_END_INT_ST_R {
        SLEEP_SWITCH_ACTIVE_END_INT_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_modem_end_int_st(&self) -> SLEEP_SWITCH_MODEM_END_INT_ST_R {
        SLEEP_SWITCH_MODEM_END_INT_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn modem_switch_sleep_end_int_st(&self) -> MODEM_SWITCH_SLEEP_END_INT_ST_R {
        MODEM_SWITCH_SLEEP_END_INT_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_end_int_st(&self) -> ACTIVE_SWITCH_SLEEP_END_INT_ST_R {
        ACTIVE_SWITCH_SLEEP_END_INT_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn modem_switch_active_start_int_st(&self) -> MODEM_SWITCH_ACTIVE_START_INT_ST_R {
        MODEM_SWITCH_ACTIVE_START_INT_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_start_int_st(&self) -> SLEEP_SWITCH_ACTIVE_START_INT_ST_R {
        SLEEP_SWITCH_ACTIVE_START_INT_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_modem_start_int_st(&self) -> SLEEP_SWITCH_MODEM_START_INT_ST_R {
        SLEEP_SWITCH_MODEM_START_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn modem_switch_sleep_start_int_st(&self) -> MODEM_SWITCH_SLEEP_START_INT_ST_R {
        MODEM_SWITCH_SLEEP_START_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_start_int_st(&self) -> ACTIVE_SWITCH_SLEEP_START_INT_ST_R {
        ACTIVE_SWITCH_SLEEP_START_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_sw_trigger_int_st(&self) -> HP_SW_TRIGGER_INT_ST_R {
        HP_SW_TRIGGER_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ST")
            .field(
                "lp_cpu_wakeup_int_st",
                &format_args!("{}", self.lp_cpu_wakeup_int_st().bit()),
            )
            .field(
                "modem_switch_active_end_int_st",
                &format_args!("{}", self.modem_switch_active_end_int_st().bit()),
            )
            .field(
                "sleep_switch_active_end_int_st",
                &format_args!("{}", self.sleep_switch_active_end_int_st().bit()),
            )
            .field(
                "sleep_switch_modem_end_int_st",
                &format_args!("{}", self.sleep_switch_modem_end_int_st().bit()),
            )
            .field(
                "modem_switch_sleep_end_int_st",
                &format_args!("{}", self.modem_switch_sleep_end_int_st().bit()),
            )
            .field(
                "active_switch_sleep_end_int_st",
                &format_args!("{}", self.active_switch_sleep_end_int_st().bit()),
            )
            .field(
                "modem_switch_active_start_int_st",
                &format_args!("{}", self.modem_switch_active_start_int_st().bit()),
            )
            .field(
                "sleep_switch_active_start_int_st",
                &format_args!("{}", self.sleep_switch_active_start_int_st().bit()),
            )
            .field(
                "sleep_switch_modem_start_int_st",
                &format_args!("{}", self.sleep_switch_modem_start_int_st().bit()),
            )
            .field(
                "modem_switch_sleep_start_int_st",
                &format_args!("{}", self.modem_switch_sleep_start_int_st().bit()),
            )
            .field(
                "active_switch_sleep_start_int_st",
                &format_args!("{}", self.active_switch_sleep_start_int_st().bit()),
            )
            .field(
                "hp_sw_trigger_int_st",
                &format_args!("{}", self.hp_sw_trigger_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_int_st](index.html) module"]
pub struct LP_INT_ST_SPEC;
impl crate::RegisterSpec for LP_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_int_st::R](R) reader structure"]
impl crate::Readable for LP_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LP_INT_ST to value 0"]
impl crate::Resettable for LP_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
