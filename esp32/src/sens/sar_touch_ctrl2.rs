#[doc = "Register `SAR_TOUCH_CTRL2` reader"]
pub struct R(crate::R<SAR_TOUCH_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_CTRL2` writer"]
pub struct W(crate::W<SAR_TOUCH_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_CTRL2_SPEC>;
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
impl From<crate::W<SAR_TOUCH_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_MEAS_EN` reader - 10-bit register to indicate which pads are \"touched\""]
pub type TOUCH_MEAS_EN_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_MEAS_DONE` reader - fsm set 1 to indicate touch touch meas is done"]
pub type TOUCH_MEAS_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_START_FSM_EN` reader - 1: TOUCH_START &amp; TOUCH_XPD is controlled by touch fsm 0: TOUCH_START &amp; TOUCH_XPD is controlled by registers"]
pub type TOUCH_START_FSM_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_START_FSM_EN` writer - 1: TOUCH_START &amp; TOUCH_XPD is controlled by touch fsm 0: TOUCH_START &amp; TOUCH_XPD is controlled by registers"]
pub type TOUCH_START_FSM_EN_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TOUCH_CTRL2_SPEC, O>;
#[doc = "Field `TOUCH_START_EN` reader - 1: start touch fsm valid when reg_touch_start_force is set"]
pub type TOUCH_START_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_START_EN` writer - 1: start touch fsm valid when reg_touch_start_force is set"]
pub type TOUCH_START_EN_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TOUCH_CTRL2_SPEC, O>;
#[doc = "Field `TOUCH_START_FORCE` reader - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
pub type TOUCH_START_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_START_FORCE` writer - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
pub type TOUCH_START_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TOUCH_CTRL2_SPEC, O>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` reader - sleep cycles for timer"]
pub type TOUCH_SLEEP_CYCLES_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SLEEP_CYCLES` writer - sleep cycles for timer"]
pub type TOUCH_SLEEP_CYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_TOUCH_CTRL2_SPEC, 16, O, u16>;
#[doc = "Field `TOUCH_MEAS_EN_CLR` writer - to clear reg_touch_meas_en"]
pub type TOUCH_MEAS_EN_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TOUCH_CTRL2_SPEC, O>;
impl R {
    #[doc = "Bits 0:9 - 10-bit register to indicate which pads are \"touched\""]
    #[inline(always)]
    pub fn touch_meas_en(&self) -> TOUCH_MEAS_EN_R {
        TOUCH_MEAS_EN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - fsm set 1 to indicate touch touch meas is done"]
    #[inline(always)]
    pub fn touch_meas_done(&self) -> TOUCH_MEAS_DONE_R {
        TOUCH_MEAS_DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: TOUCH_START &amp; TOUCH_XPD is controlled by touch fsm 0: TOUCH_START &amp; TOUCH_XPD is controlled by registers"]
    #[inline(always)]
    pub fn touch_start_fsm_en(&self) -> TOUCH_START_FSM_EN_R {
        TOUCH_START_FSM_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: start touch fsm valid when reg_touch_start_force is set"]
    #[inline(always)]
    pub fn touch_start_en(&self) -> TOUCH_START_EN_R {
        TOUCH_START_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
    #[inline(always)]
    pub fn touch_start_force(&self) -> TOUCH_START_FORCE_R {
        TOUCH_START_FORCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:29 - sleep cycles for timer"]
    #[inline(always)]
    pub fn touch_sleep_cycles(&self) -> TOUCH_SLEEP_CYCLES_R {
        TOUCH_SLEEP_CYCLES_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_CTRL2")
            .field(
                "touch_meas_en",
                &format_args!("{}", self.touch_meas_en().bits()),
            )
            .field(
                "touch_meas_done",
                &format_args!("{}", self.touch_meas_done().bit()),
            )
            .field(
                "touch_start_fsm_en",
                &format_args!("{}", self.touch_start_fsm_en().bit()),
            )
            .field(
                "touch_start_en",
                &format_args!("{}", self.touch_start_en().bit()),
            )
            .field(
                "touch_start_force",
                &format_args!("{}", self.touch_start_force().bit()),
            )
            .field(
                "touch_sleep_cycles",
                &format_args!("{}", self.touch_sleep_cycles().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 11 - 1: TOUCH_START &amp; TOUCH_XPD is controlled by touch fsm 0: TOUCH_START &amp; TOUCH_XPD is controlled by registers"]
    #[inline(always)]
    #[must_use]
    pub fn touch_start_fsm_en(&mut self) -> TOUCH_START_FSM_EN_W<11> {
        TOUCH_START_FSM_EN_W::new(self)
    }
    #[doc = "Bit 12 - 1: start touch fsm valid when reg_touch_start_force is set"]
    #[inline(always)]
    #[must_use]
    pub fn touch_start_en(&mut self) -> TOUCH_START_EN_W<12> {
        TOUCH_START_EN_W::new(self)
    }
    #[doc = "Bit 13 - 1: to start touch fsm by SW 0: to start touch fsm by timer"]
    #[inline(always)]
    #[must_use]
    pub fn touch_start_force(&mut self) -> TOUCH_START_FORCE_W<13> {
        TOUCH_START_FORCE_W::new(self)
    }
    #[doc = "Bits 14:29 - sleep cycles for timer"]
    #[inline(always)]
    #[must_use]
    pub fn touch_sleep_cycles(&mut self) -> TOUCH_SLEEP_CYCLES_W<14> {
        TOUCH_SLEEP_CYCLES_W::new(self)
    }
    #[doc = "Bit 30 - to clear reg_touch_meas_en"]
    #[inline(always)]
    #[must_use]
    pub fn touch_meas_en_clr(&mut self) -> TOUCH_MEAS_EN_CLR_W<30> {
        TOUCH_MEAS_EN_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_ctrl2](index.html) module"]
pub struct SAR_TOUCH_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_ctrl2::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_ctrl2::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_CTRL2 to value 0x0040_0800"]
impl crate::Resettable for SAR_TOUCH_CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0800;
}
