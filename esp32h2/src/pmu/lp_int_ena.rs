#[doc = "Register `LP_INT_ENA` reader"]
pub struct R(crate::R<LP_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LP_INT_ENA` writer"]
pub struct W(crate::W<LP_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LP_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_CPU_WAKEUP_INT_ENA` reader - need_des"]
pub type LP_CPU_WAKEUP_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_CPU_WAKEUP_INT_ENA` writer - need_des"]
pub type LP_CPU_WAKEUP_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_END_INT_ENA` reader - need_des"]
pub type MODEM_SWITCH_ACTIVE_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_ACTIVE_END_INT_ENA` writer - need_des"]
pub type MODEM_SWITCH_ACTIVE_END_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_END_INT_ENA` reader - need_des"]
pub type SLEEP_SWITCH_ACTIVE_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_END_INT_ENA` writer - need_des"]
pub type SLEEP_SWITCH_ACTIVE_END_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
#[doc = "Field `SLEEP_SWITCH_MODEM_END_INT_ENA` reader - need_des"]
pub type SLEEP_SWITCH_MODEM_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_MODEM_END_INT_ENA` writer - need_des"]
pub type SLEEP_SWITCH_MODEM_END_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
#[doc = "Field `MODEM_SWITCH_SLEEP_END_INT_ENA` reader - need_des"]
pub type MODEM_SWITCH_SLEEP_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_SLEEP_END_INT_ENA` writer - need_des"]
pub type MODEM_SWITCH_SLEEP_END_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_END_INT_ENA` reader - need_des"]
pub type ACTIVE_SWITCH_SLEEP_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_END_INT_ENA` writer - need_des"]
pub type ACTIVE_SWITCH_SLEEP_END_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_START_INT_ENA` reader - need_des"]
pub type MODEM_SWITCH_ACTIVE_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_ACTIVE_START_INT_ENA` writer - need_des"]
pub type MODEM_SWITCH_ACTIVE_START_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_START_INT_ENA` reader - need_des"]
pub type SLEEP_SWITCH_ACTIVE_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_START_INT_ENA` writer - need_des"]
pub type SLEEP_SWITCH_ACTIVE_START_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
#[doc = "Field `SLEEP_SWITCH_MODEM_START_INT_ENA` reader - need_des"]
pub type SLEEP_SWITCH_MODEM_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_MODEM_START_INT_ENA` writer - need_des"]
pub type SLEEP_SWITCH_MODEM_START_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
#[doc = "Field `MODEM_SWITCH_SLEEP_START_INT_ENA` reader - need_des"]
pub type MODEM_SWITCH_SLEEP_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_SLEEP_START_INT_ENA` writer - need_des"]
pub type MODEM_SWITCH_SLEEP_START_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_START_INT_ENA` reader - need_des"]
pub type ACTIVE_SWITCH_SLEEP_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_START_INT_ENA` writer - need_des"]
pub type ACTIVE_SWITCH_SLEEP_START_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
#[doc = "Field `HP_SW_TRIGGER_INT_ENA` reader - need_des"]
pub type HP_SW_TRIGGER_INT_ENA_R = crate::BitReader;
#[doc = "Field `HP_SW_TRIGGER_INT_ENA` writer - need_des"]
pub type HP_SW_TRIGGER_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, LP_INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup_int_ena(&self) -> LP_CPU_WAKEUP_INT_ENA_R {
        LP_CPU_WAKEUP_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn modem_switch_active_end_int_ena(&self) -> MODEM_SWITCH_ACTIVE_END_INT_ENA_R {
        MODEM_SWITCH_ACTIVE_END_INT_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_end_int_ena(&self) -> SLEEP_SWITCH_ACTIVE_END_INT_ENA_R {
        SLEEP_SWITCH_ACTIVE_END_INT_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_modem_end_int_ena(&self) -> SLEEP_SWITCH_MODEM_END_INT_ENA_R {
        SLEEP_SWITCH_MODEM_END_INT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn modem_switch_sleep_end_int_ena(&self) -> MODEM_SWITCH_SLEEP_END_INT_ENA_R {
        MODEM_SWITCH_SLEEP_END_INT_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_end_int_ena(&self) -> ACTIVE_SWITCH_SLEEP_END_INT_ENA_R {
        ACTIVE_SWITCH_SLEEP_END_INT_ENA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn modem_switch_active_start_int_ena(&self) -> MODEM_SWITCH_ACTIVE_START_INT_ENA_R {
        MODEM_SWITCH_ACTIVE_START_INT_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_start_int_ena(&self) -> SLEEP_SWITCH_ACTIVE_START_INT_ENA_R {
        SLEEP_SWITCH_ACTIVE_START_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_modem_start_int_ena(&self) -> SLEEP_SWITCH_MODEM_START_INT_ENA_R {
        SLEEP_SWITCH_MODEM_START_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn modem_switch_sleep_start_int_ena(&self) -> MODEM_SWITCH_SLEEP_START_INT_ENA_R {
        MODEM_SWITCH_SLEEP_START_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_start_int_ena(&self) -> ACTIVE_SWITCH_SLEEP_START_INT_ENA_R {
        ACTIVE_SWITCH_SLEEP_START_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_sw_trigger_int_ena(&self) -> HP_SW_TRIGGER_INT_ENA_R {
        HP_SW_TRIGGER_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ENA")
            .field(
                "lp_cpu_wakeup_int_ena",
                &format_args!("{}", self.lp_cpu_wakeup_int_ena().bit()),
            )
            .field(
                "modem_switch_active_end_int_ena",
                &format_args!("{}", self.modem_switch_active_end_int_ena().bit()),
            )
            .field(
                "sleep_switch_active_end_int_ena",
                &format_args!("{}", self.sleep_switch_active_end_int_ena().bit()),
            )
            .field(
                "sleep_switch_modem_end_int_ena",
                &format_args!("{}", self.sleep_switch_modem_end_int_ena().bit()),
            )
            .field(
                "modem_switch_sleep_end_int_ena",
                &format_args!("{}", self.modem_switch_sleep_end_int_ena().bit()),
            )
            .field(
                "active_switch_sleep_end_int_ena",
                &format_args!("{}", self.active_switch_sleep_end_int_ena().bit()),
            )
            .field(
                "modem_switch_active_start_int_ena",
                &format_args!("{}", self.modem_switch_active_start_int_ena().bit()),
            )
            .field(
                "sleep_switch_active_start_int_ena",
                &format_args!("{}", self.sleep_switch_active_start_int_ena().bit()),
            )
            .field(
                "sleep_switch_modem_start_int_ena",
                &format_args!("{}", self.sleep_switch_modem_start_int_ena().bit()),
            )
            .field(
                "modem_switch_sleep_start_int_ena",
                &format_args!("{}", self.modem_switch_sleep_start_int_ena().bit()),
            )
            .field(
                "active_switch_sleep_start_int_ena",
                &format_args!("{}", self.active_switch_sleep_start_int_ena().bit()),
            )
            .field(
                "hp_sw_trigger_int_ena",
                &format_args!("{}", self.hp_sw_trigger_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_wakeup_int_ena(&mut self) -> LP_CPU_WAKEUP_INT_ENA_W<20> {
        LP_CPU_WAKEUP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_active_end_int_ena(&mut self) -> MODEM_SWITCH_ACTIVE_END_INT_ENA_W<21> {
        MODEM_SWITCH_ACTIVE_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_active_end_int_ena(&mut self) -> SLEEP_SWITCH_ACTIVE_END_INT_ENA_W<22> {
        SLEEP_SWITCH_ACTIVE_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_modem_end_int_ena(&mut self) -> SLEEP_SWITCH_MODEM_END_INT_ENA_W<23> {
        SLEEP_SWITCH_MODEM_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_sleep_end_int_ena(&mut self) -> MODEM_SWITCH_SLEEP_END_INT_ENA_W<24> {
        MODEM_SWITCH_SLEEP_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn active_switch_sleep_end_int_ena(&mut self) -> ACTIVE_SWITCH_SLEEP_END_INT_ENA_W<25> {
        ACTIVE_SWITCH_SLEEP_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_active_start_int_ena(&mut self) -> MODEM_SWITCH_ACTIVE_START_INT_ENA_W<26> {
        MODEM_SWITCH_ACTIVE_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_active_start_int_ena(&mut self) -> SLEEP_SWITCH_ACTIVE_START_INT_ENA_W<27> {
        SLEEP_SWITCH_ACTIVE_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_modem_start_int_ena(&mut self) -> SLEEP_SWITCH_MODEM_START_INT_ENA_W<28> {
        SLEEP_SWITCH_MODEM_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_sleep_start_int_ena(&mut self) -> MODEM_SWITCH_SLEEP_START_INT_ENA_W<29> {
        MODEM_SWITCH_SLEEP_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn active_switch_sleep_start_int_ena(&mut self) -> ACTIVE_SWITCH_SLEEP_START_INT_ENA_W<30> {
        ACTIVE_SWITCH_SLEEP_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sw_trigger_int_ena(&mut self) -> HP_SW_TRIGGER_INT_ENA_W<31> {
        HP_SW_TRIGGER_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_int_ena](index.html) module"]
pub struct LP_INT_ENA_SPEC;
impl crate::RegisterSpec for LP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_int_ena::R](R) reader structure"]
impl crate::Readable for LP_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lp_int_ena::W](W) writer structure"]
impl crate::Writable for LP_INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_INT_ENA to value 0"]
impl crate::Resettable for LP_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
