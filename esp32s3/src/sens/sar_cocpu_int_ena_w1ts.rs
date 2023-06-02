#[doc = "Register `SAR_COCPU_INT_ENA_W1TS` writer"]
pub struct W(crate::W<SAR_COCPU_INT_ENA_W1TS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_COCPU_INT_ENA_W1TS_SPEC>;
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
impl From<crate::W<SAR_COCPU_INT_ENA_W1TS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_COCPU_INT_ENA_W1TS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_COCPU_TOUCH_DONE_INT_ENA_W1TS` writer - int enable of touch done"]
pub type SAR_COCPU_TOUCH_DONE_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[doc = "Field `SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TS` writer - int enable of from touch inactive"]
pub type SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[doc = "Field `SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TS` writer - int enable of touch active"]
pub type SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[doc = "Field `SAR_COCPU_SARADC1_INT_ENA_W1TS` writer - int enable of from saradc1"]
pub type SAR_COCPU_SARADC1_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[doc = "Field `SAR_COCPU_SARADC2_INT_ENA_W1TS` writer - int enable of from saradc2"]
pub type SAR_COCPU_SARADC2_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[doc = "Field `SAR_COCPU_TSENS_INT_ENA_W1TS` writer - int enable of tsens"]
pub type SAR_COCPU_TSENS_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[doc = "Field `SAR_COCPU_START_INT_ENA_W1TS` writer - int enable of start"]
pub type SAR_COCPU_START_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[doc = "Field `SAR_COCPU_SW_INT_ENA_W1TS` writer - int enable of software"]
pub type SAR_COCPU_SW_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[doc = "Field `SAR_COCPU_SWD_INT_ENA_W1TS` writer - int enable of super watch dog"]
pub type SAR_COCPU_SWD_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[doc = "Field `SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TS` writer - int enable of timeout done"]
pub type SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[doc = "Field `SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS` writer - int enable of approach loop done"]
pub type SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[doc = "Field `SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TS` writer - int enable of touch scan done"]
pub type SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TS_W<'a, const O: u8> =
    crate::BitWriter<'a, SAR_COCPU_INT_ENA_W1TS_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_COCPU_INT_ENA_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - int enable of touch done"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_done_int_ena_w1ts(&mut self) -> SAR_COCPU_TOUCH_DONE_INT_ENA_W1TS_W<0> {
        SAR_COCPU_TOUCH_DONE_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 1 - int enable of from touch inactive"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_inactive_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TS_W<1> {
        SAR_COCPU_TOUCH_INACTIVE_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 2 - int enable of touch active"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_active_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TS_W<2> {
        SAR_COCPU_TOUCH_ACTIVE_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 3 - int enable of from saradc1"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_saradc1_int_ena_w1ts(&mut self) -> SAR_COCPU_SARADC1_INT_ENA_W1TS_W<3> {
        SAR_COCPU_SARADC1_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 4 - int enable of from saradc2"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_saradc2_int_ena_w1ts(&mut self) -> SAR_COCPU_SARADC2_INT_ENA_W1TS_W<4> {
        SAR_COCPU_SARADC2_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 5 - int enable of tsens"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_tsens_int_ena_w1ts(&mut self) -> SAR_COCPU_TSENS_INT_ENA_W1TS_W<5> {
        SAR_COCPU_TSENS_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 6 - int enable of start"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_start_int_ena_w1ts(&mut self) -> SAR_COCPU_START_INT_ENA_W1TS_W<6> {
        SAR_COCPU_START_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 7 - int enable of software"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_sw_int_ena_w1ts(&mut self) -> SAR_COCPU_SW_INT_ENA_W1TS_W<7> {
        SAR_COCPU_SW_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 8 - int enable of super watch dog"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_swd_int_ena_w1ts(&mut self) -> SAR_COCPU_SWD_INT_ENA_W1TS_W<8> {
        SAR_COCPU_SWD_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 9 - int enable of timeout done"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_timeout_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TS_W<9> {
        SAR_COCPU_TOUCH_TIMEOUT_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 10 - int enable of approach loop done"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_approach_loop_done_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS_W<10> {
        SAR_COCPU_TOUCH_APPROACH_LOOP_DONE_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Bit 11 - int enable of touch scan done"]
    #[inline(always)]
    #[must_use]
    pub fn sar_cocpu_touch_scan_done_int_ena_w1ts(
        &mut self,
    ) -> SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TS_W<11> {
        SAR_COCPU_TOUCH_SCAN_DONE_INT_ENA_W1TS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the interrupt enable of ulp\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cocpu_int_ena_w1ts](index.html) module"]
pub struct SAR_COCPU_INT_ENA_W1TS_SPEC;
impl crate::RegisterSpec for SAR_COCPU_INT_ENA_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sar_cocpu_int_ena_w1ts::W](W) writer structure"]
impl crate::Writable for SAR_COCPU_INT_ENA_W1TS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_COCPU_INT_ENA_W1TS to value 0"]
impl crate::Resettable for SAR_COCPU_INT_ENA_W1TS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
