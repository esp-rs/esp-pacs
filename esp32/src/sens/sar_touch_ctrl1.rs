#[doc = "Register `SAR_TOUCH_CTRL1` reader"]
pub struct R(crate::R<SAR_TOUCH_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_TOUCH_CTRL1` writer"]
pub struct W(crate::W<SAR_TOUCH_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_TOUCH_CTRL1_SPEC>;
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
impl From<crate::W<SAR_TOUCH_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_TOUCH_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUCH_MEAS_DELAY` reader - the meas length (in 8MHz)"]
pub type TOUCH_MEAS_DELAY_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_MEAS_DELAY` writer - the meas length (in 8MHz)"]
pub type TOUCH_MEAS_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_TOUCH_CTRL1_SPEC, 16, O, u16>;
#[doc = "Field `TOUCH_XPD_WAIT` reader - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
pub type TOUCH_XPD_WAIT_R = crate::FieldReader;
#[doc = "Field `TOUCH_XPD_WAIT` writer - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
pub type TOUCH_XPD_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_TOUCH_CTRL1_SPEC, 8, O>;
#[doc = "Field `TOUCH_OUT_SEL` reader - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
pub type TOUCH_OUT_SEL_R = crate::BitReader;
#[doc = "Field `TOUCH_OUT_SEL` writer - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
pub type TOUCH_OUT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TOUCH_CTRL1_SPEC, O>;
#[doc = "Field `TOUCH_OUT_1EN` reader - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 &amp; SET2 is both \"touched\""]
pub type TOUCH_OUT_1EN_R = crate::BitReader;
#[doc = "Field `TOUCH_OUT_1EN` writer - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 &amp; SET2 is both \"touched\""]
pub type TOUCH_OUT_1EN_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TOUCH_CTRL1_SPEC, O>;
#[doc = "Field `XPD_HALL_FORCE` reader - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub type XPD_HALL_FORCE_R = crate::BitReader;
#[doc = "Field `XPD_HALL_FORCE` writer - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub type XPD_HALL_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TOUCH_CTRL1_SPEC, O>;
#[doc = "Field `HALL_PHASE_FORCE` reader - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub type HALL_PHASE_FORCE_R = crate::BitReader;
#[doc = "Field `HALL_PHASE_FORCE` writer - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub type HALL_PHASE_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_TOUCH_CTRL1_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - the meas length (in 8MHz)"]
    #[inline(always)]
    pub fn touch_meas_delay(&self) -> TOUCH_MEAS_DELAY_R {
        TOUCH_MEAS_DELAY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    pub fn touch_xpd_wait(&self) -> TOUCH_XPD_WAIT_R {
        TOUCH_XPD_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
    #[inline(always)]
    pub fn touch_out_sel(&self) -> TOUCH_OUT_SEL_R {
        TOUCH_OUT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 &amp; SET2 is both \"touched\""]
    #[inline(always)]
    pub fn touch_out_1en(&self) -> TOUCH_OUT_1EN_R {
        TOUCH_OUT_1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn xpd_hall_force(&self) -> XPD_HALL_FORCE_R {
        XPD_HALL_FORCE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn hall_phase_force(&self) -> HALL_PHASE_FORCE_R {
        HALL_PHASE_FORCE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_CTRL1")
            .field(
                "touch_meas_delay",
                &format_args!("{}", self.touch_meas_delay().bits()),
            )
            .field(
                "touch_xpd_wait",
                &format_args!("{}", self.touch_xpd_wait().bits()),
            )
            .field(
                "touch_out_sel",
                &format_args!("{}", self.touch_out_sel().bit()),
            )
            .field(
                "touch_out_1en",
                &format_args!("{}", self.touch_out_1en().bit()),
            )
            .field(
                "xpd_hall_force",
                &format_args!("{}", self.xpd_hall_force().bit()),
            )
            .field(
                "hall_phase_force",
                &format_args!("{}", self.hall_phase_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - the meas length (in 8MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn touch_meas_delay(&mut self) -> TOUCH_MEAS_DELAY_W<0> {
        TOUCH_MEAS_DELAY_W::new(self)
    }
    #[doc = "Bits 16:23 - the waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD"]
    #[inline(always)]
    #[must_use]
    pub fn touch_xpd_wait(&mut self) -> TOUCH_XPD_WAIT_W<16> {
        TOUCH_XPD_WAIT_W::new(self)
    }
    #[doc = "Bit 24 - 1: when the counter is greater then the threshold the touch pad is considered as \"touched\" 0: when the counter is less than the threshold the touch pad is considered as \"touched\""]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_sel(&mut self) -> TOUCH_OUT_SEL_W<24> {
        TOUCH_OUT_SEL_W::new(self)
    }
    #[doc = "Bit 25 - 1: wakeup interrupt is generated if SET1 is \"touched\" 0: wakeup interrupt is generated only if SET1 &amp; SET2 is both \"touched\""]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_1en(&mut self) -> TOUCH_OUT_1EN_W<25> {
        TOUCH_OUT_1EN_W::new(self)
    }
    #[doc = "Bit 26 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_hall_force(&mut self) -> XPD_HALL_FORCE_W<26> {
        XPD_HALL_FORCE_W::new(self)
    }
    #[doc = "Bit 27 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    #[must_use]
    pub fn hall_phase_force(&mut self) -> HALL_PHASE_FORCE_W<27> {
        HALL_PHASE_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_ctrl1](index.html) module"]
pub struct SAR_TOUCH_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_ctrl1::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_touch_ctrl1::W](W) writer structure"]
impl crate::Writable for SAR_TOUCH_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_TOUCH_CTRL1 to value 0x0204_1000"]
impl crate::Resettable for SAR_TOUCH_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0204_1000;
}
