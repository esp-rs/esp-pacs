#[doc = "Register `LP_INT_RAW` reader"]
pub struct R(crate::R<LP_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LP_INT_RAW` writer"]
pub struct W(crate::W<LP_INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_INT_RAW_SPEC>;
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
impl From<crate::W<LP_INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_CPU_WAKEUP_INT_RAW` reader - need_des"]
pub type LP_CPU_WAKEUP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `LP_CPU_WAKEUP_INT_RAW` writer - need_des"]
pub type LP_CPU_WAKEUP_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_END_INT_RAW` reader - need_des"]
pub type MODEM_SWITCH_ACTIVE_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_END_INT_RAW` writer - need_des"]
pub type MODEM_SWITCH_ACTIVE_END_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_END_INT_RAW` reader - need_des"]
pub type SLEEP_SWITCH_ACTIVE_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_END_INT_RAW` writer - need_des"]
pub type SLEEP_SWITCH_ACTIVE_END_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
#[doc = "Field `SLEEP_SWITCH_MODEM_END_INT_RAW` reader - need_des"]
pub type SLEEP_SWITCH_MODEM_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP_SWITCH_MODEM_END_INT_RAW` writer - need_des"]
pub type SLEEP_SWITCH_MODEM_END_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
#[doc = "Field `MODEM_SWITCH_SLEEP_END_INT_RAW` reader - need_des"]
pub type MODEM_SWITCH_SLEEP_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `MODEM_SWITCH_SLEEP_END_INT_RAW` writer - need_des"]
pub type MODEM_SWITCH_SLEEP_END_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_END_INT_RAW` reader - need_des"]
pub type ACTIVE_SWITCH_SLEEP_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_END_INT_RAW` writer - need_des"]
pub type ACTIVE_SWITCH_SLEEP_END_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_START_INT_RAW` reader - need_des"]
pub type MODEM_SWITCH_ACTIVE_START_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_START_INT_RAW` writer - need_des"]
pub type MODEM_SWITCH_ACTIVE_START_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_START_INT_RAW` reader - need_des"]
pub type SLEEP_SWITCH_ACTIVE_START_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_START_INT_RAW` writer - need_des"]
pub type SLEEP_SWITCH_ACTIVE_START_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
#[doc = "Field `SLEEP_SWITCH_MODEM_START_INT_RAW` reader - need_des"]
pub type SLEEP_SWITCH_MODEM_START_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP_SWITCH_MODEM_START_INT_RAW` writer - need_des"]
pub type SLEEP_SWITCH_MODEM_START_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
#[doc = "Field `MODEM_SWITCH_SLEEP_START_INT_RAW` reader - need_des"]
pub type MODEM_SWITCH_SLEEP_START_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `MODEM_SWITCH_SLEEP_START_INT_RAW` writer - need_des"]
pub type MODEM_SWITCH_SLEEP_START_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_START_INT_RAW` reader - need_des"]
pub type ACTIVE_SWITCH_SLEEP_START_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_START_INT_RAW` writer - need_des"]
pub type ACTIVE_SWITCH_SLEEP_START_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
#[doc = "Field `HP_SW_TRIGGER_INT_RAW` reader - need_des"]
pub type HP_SW_TRIGGER_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `HP_SW_TRIGGER_INT_RAW` writer - need_des"]
pub type HP_SW_TRIGGER_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LP_INT_RAW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup_int_raw(&self) -> LP_CPU_WAKEUP_INT_RAW_R {
        LP_CPU_WAKEUP_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn modem_switch_active_end_int_raw(&self) -> MODEM_SWITCH_ACTIVE_END_INT_RAW_R {
        MODEM_SWITCH_ACTIVE_END_INT_RAW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_end_int_raw(&self) -> SLEEP_SWITCH_ACTIVE_END_INT_RAW_R {
        SLEEP_SWITCH_ACTIVE_END_INT_RAW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_modem_end_int_raw(&self) -> SLEEP_SWITCH_MODEM_END_INT_RAW_R {
        SLEEP_SWITCH_MODEM_END_INT_RAW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn modem_switch_sleep_end_int_raw(&self) -> MODEM_SWITCH_SLEEP_END_INT_RAW_R {
        MODEM_SWITCH_SLEEP_END_INT_RAW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_end_int_raw(&self) -> ACTIVE_SWITCH_SLEEP_END_INT_RAW_R {
        ACTIVE_SWITCH_SLEEP_END_INT_RAW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn modem_switch_active_start_int_raw(&self) -> MODEM_SWITCH_ACTIVE_START_INT_RAW_R {
        MODEM_SWITCH_ACTIVE_START_INT_RAW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_start_int_raw(&self) -> SLEEP_SWITCH_ACTIVE_START_INT_RAW_R {
        SLEEP_SWITCH_ACTIVE_START_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_modem_start_int_raw(&self) -> SLEEP_SWITCH_MODEM_START_INT_RAW_R {
        SLEEP_SWITCH_MODEM_START_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn modem_switch_sleep_start_int_raw(&self) -> MODEM_SWITCH_SLEEP_START_INT_RAW_R {
        MODEM_SWITCH_SLEEP_START_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_start_int_raw(&self) -> ACTIVE_SWITCH_SLEEP_START_INT_RAW_R {
        ACTIVE_SWITCH_SLEEP_START_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_sw_trigger_int_raw(&self) -> HP_SW_TRIGGER_INT_RAW_R {
        HP_SW_TRIGGER_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_cpu_wakeup_int_raw(&mut self) -> LP_CPU_WAKEUP_INT_RAW_W<20> {
        LP_CPU_WAKEUP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_active_end_int_raw(&mut self) -> MODEM_SWITCH_ACTIVE_END_INT_RAW_W<21> {
        MODEM_SWITCH_ACTIVE_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_active_end_int_raw(&mut self) -> SLEEP_SWITCH_ACTIVE_END_INT_RAW_W<22> {
        SLEEP_SWITCH_ACTIVE_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_modem_end_int_raw(&mut self) -> SLEEP_SWITCH_MODEM_END_INT_RAW_W<23> {
        SLEEP_SWITCH_MODEM_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_sleep_end_int_raw(&mut self) -> MODEM_SWITCH_SLEEP_END_INT_RAW_W<24> {
        MODEM_SWITCH_SLEEP_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn active_switch_sleep_end_int_raw(&mut self) -> ACTIVE_SWITCH_SLEEP_END_INT_RAW_W<25> {
        ACTIVE_SWITCH_SLEEP_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_active_start_int_raw(&mut self) -> MODEM_SWITCH_ACTIVE_START_INT_RAW_W<26> {
        MODEM_SWITCH_ACTIVE_START_INT_RAW_W::new(self)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_active_start_int_raw(&mut self) -> SLEEP_SWITCH_ACTIVE_START_INT_RAW_W<27> {
        SLEEP_SWITCH_ACTIVE_START_INT_RAW_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_switch_modem_start_int_raw(&mut self) -> SLEEP_SWITCH_MODEM_START_INT_RAW_W<28> {
        SLEEP_SWITCH_MODEM_START_INT_RAW_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn modem_switch_sleep_start_int_raw(&mut self) -> MODEM_SWITCH_SLEEP_START_INT_RAW_W<29> {
        MODEM_SWITCH_SLEEP_START_INT_RAW_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn active_switch_sleep_start_int_raw(&mut self) -> ACTIVE_SWITCH_SLEEP_START_INT_RAW_W<30> {
        ACTIVE_SWITCH_SLEEP_START_INT_RAW_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sw_trigger_int_raw(&mut self) -> HP_SW_TRIGGER_INT_RAW_W<31> {
        HP_SW_TRIGGER_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_int_raw](index.html) module"]
pub struct LP_INT_RAW_SPEC;
impl crate::RegisterSpec for LP_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_int_raw::R](R) reader structure"]
impl crate::Readable for LP_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lp_int_raw::W](W) writer structure"]
impl crate::Writable for LP_INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_INT_RAW to value 0"]
impl crate::Resettable for LP_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
