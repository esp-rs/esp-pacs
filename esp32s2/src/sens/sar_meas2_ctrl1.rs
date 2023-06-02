#[doc = "Register `SAR_MEAS2_CTRL1` reader"]
pub struct R(crate::R<SAR_MEAS2_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_MEAS2_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_MEAS2_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_MEAS2_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_MEAS2_CTRL1` writer"]
pub struct W(crate::W<SAR_MEAS2_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_MEAS2_CTRL1_SPEC>;
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
impl From<crate::W<SAR_MEAS2_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_MEAS2_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_CNTL_STATE` reader - saradc2_cntl_fsm"]
pub type SAR2_CNTL_STATE_R = crate::FieldReader;
#[doc = "Field `SAR2_PWDET_CAL_EN` reader - rtc control pwdet enable"]
pub type SAR2_PWDET_CAL_EN_R = crate::BitReader;
#[doc = "Field `SAR2_PWDET_CAL_EN` writer - rtc control pwdet enable"]
pub type SAR2_PWDET_CAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, SAR_MEAS2_CTRL1_SPEC, O>;
#[doc = "Field `SAR2_PKDET_CAL_EN` reader - rtc control pkdet enable"]
pub type SAR2_PKDET_CAL_EN_R = crate::BitReader;
#[doc = "Field `SAR2_PKDET_CAL_EN` writer - rtc control pkdet enable"]
pub type SAR2_PKDET_CAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, SAR_MEAS2_CTRL1_SPEC, O>;
#[doc = "Field `SAR2_EN_TEST` reader - SAR2_EN_TEST"]
pub type SAR2_EN_TEST_R = crate::BitReader;
#[doc = "Field `SAR2_EN_TEST` writer - SAR2_EN_TEST"]
pub type SAR2_EN_TEST_W<'a, const O: u8> = crate::BitWriter<'a, SAR_MEAS2_CTRL1_SPEC, O>;
#[doc = "Field `SAR2_RSTB_FORCE` reader - "]
pub type SAR2_RSTB_FORCE_R = crate::FieldReader;
#[doc = "Field `SAR2_RSTB_FORCE` writer - "]
pub type SAR2_RSTB_FORCE_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS2_CTRL1_SPEC, 2, O>;
#[doc = "Field `SAR2_STANDBY_WAIT` reader - "]
pub type SAR2_STANDBY_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR2_STANDBY_WAIT` writer - "]
pub type SAR2_STANDBY_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS2_CTRL1_SPEC, 8, O>;
#[doc = "Field `SAR2_RSTB_WAIT` reader - "]
pub type SAR2_RSTB_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR2_RSTB_WAIT` writer - "]
pub type SAR2_RSTB_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS2_CTRL1_SPEC, 8, O>;
#[doc = "Field `SAR2_XPD_WAIT` reader - "]
pub type SAR2_XPD_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR2_XPD_WAIT` writer - "]
pub type SAR2_XPD_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_MEAS2_CTRL1_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:2 - saradc2_cntl_fsm"]
    #[inline(always)]
    pub fn sar2_cntl_state(&self) -> SAR2_CNTL_STATE_R {
        SAR2_CNTL_STATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - rtc control pwdet enable"]
    #[inline(always)]
    pub fn sar2_pwdet_cal_en(&self) -> SAR2_PWDET_CAL_EN_R {
        SAR2_PWDET_CAL_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - rtc control pkdet enable"]
    #[inline(always)]
    pub fn sar2_pkdet_cal_en(&self) -> SAR2_PKDET_CAL_EN_R {
        SAR2_PKDET_CAL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAR2_EN_TEST"]
    #[inline(always)]
    pub fn sar2_en_test(&self) -> SAR2_EN_TEST_R {
        SAR2_EN_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sar2_rstb_force(&self) -> SAR2_RSTB_FORCE_R {
        SAR2_RSTB_FORCE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sar2_standby_wait(&self) -> SAR2_STANDBY_WAIT_R {
        SAR2_STANDBY_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sar2_rstb_wait(&self) -> SAR2_RSTB_WAIT_R {
        SAR2_RSTB_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sar2_xpd_wait(&self) -> SAR2_XPD_WAIT_R {
        SAR2_XPD_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS2_CTRL1")
            .field(
                "sar2_cntl_state",
                &format_args!("{}", self.sar2_cntl_state().bits()),
            )
            .field(
                "sar2_pwdet_cal_en",
                &format_args!("{}", self.sar2_pwdet_cal_en().bit()),
            )
            .field(
                "sar2_pkdet_cal_en",
                &format_args!("{}", self.sar2_pkdet_cal_en().bit()),
            )
            .field(
                "sar2_en_test",
                &format_args!("{}", self.sar2_en_test().bit()),
            )
            .field(
                "sar2_rstb_force",
                &format_args!("{}", self.sar2_rstb_force().bits()),
            )
            .field(
                "sar2_standby_wait",
                &format_args!("{}", self.sar2_standby_wait().bits()),
            )
            .field(
                "sar2_rstb_wait",
                &format_args!("{}", self.sar2_rstb_wait().bits()),
            )
            .field(
                "sar2_xpd_wait",
                &format_args!("{}", self.sar2_xpd_wait().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEAS2_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - rtc control pwdet enable"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_pwdet_cal_en(&mut self) -> SAR2_PWDET_CAL_EN_W<3> {
        SAR2_PWDET_CAL_EN_W::new(self)
    }
    #[doc = "Bit 4 - rtc control pkdet enable"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_pkdet_cal_en(&mut self) -> SAR2_PKDET_CAL_EN_W<4> {
        SAR2_PKDET_CAL_EN_W::new(self)
    }
    #[doc = "Bit 5 - SAR2_EN_TEST"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_en_test(&mut self) -> SAR2_EN_TEST_W<5> {
        SAR2_EN_TEST_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_rstb_force(&mut self) -> SAR2_RSTB_FORCE_W<6> {
        SAR2_RSTB_FORCE_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_standby_wait(&mut self) -> SAR2_STANDBY_WAIT_W<8> {
        SAR2_STANDBY_WAIT_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_rstb_wait(&mut self) -> SAR2_RSTB_WAIT_W<16> {
        SAR2_RSTB_WAIT_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_xpd_wait(&mut self) -> SAR2_XPD_WAIT_W<24> {
        SAR2_XPD_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure rtc saradc2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_meas2_ctrl1](index.html) module"]
pub struct SAR_MEAS2_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_MEAS2_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_meas2_ctrl1::R](R) reader structure"]
impl crate::Readable for SAR_MEAS2_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_meas2_ctrl1::W](W) writer structure"]
impl crate::Writable for SAR_MEAS2_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_MEAS2_CTRL1 to value 0x0702_0200"]
impl crate::Resettable for SAR_MEAS2_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0702_0200;
}
