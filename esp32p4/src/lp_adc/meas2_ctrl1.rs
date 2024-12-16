#[doc = "Register `MEAS2_CTRL1` reader"]
pub type R = crate::R<MEAS2_CTRL1_SPEC>;
#[doc = "Register `MEAS2_CTRL1` writer"]
pub type W = crate::W<MEAS2_CTRL1_SPEC>;
#[doc = "Field `SAR2_CNTL_STATE` reader - saradc2_cntl_fsm."]
pub type SAR2_CNTL_STATE_R = crate::FieldReader;
#[doc = "Field `SAR2_PWDET_CAL_EN` reader - RTC control pwdet enable."]
pub type SAR2_PWDET_CAL_EN_R = crate::BitReader;
#[doc = "Field `SAR2_PWDET_CAL_EN` writer - RTC control pwdet enable."]
pub type SAR2_PWDET_CAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_PKDET_CAL_EN` reader - RTC control pkdet enable."]
pub type SAR2_PKDET_CAL_EN_R = crate::BitReader;
#[doc = "Field `SAR2_PKDET_CAL_EN` writer - RTC control pkdet enable."]
pub type SAR2_PKDET_CAL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_EN_TEST` reader - SAR2_EN_TEST."]
pub type SAR2_EN_TEST_R = crate::BitReader;
#[doc = "Field `SAR2_EN_TEST` writer - SAR2_EN_TEST."]
pub type SAR2_EN_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_RSTB_FORCE` reader - N/A"]
pub type SAR2_RSTB_FORCE_R = crate::FieldReader;
#[doc = "Field `SAR2_RSTB_FORCE` writer - N/A"]
pub type SAR2_RSTB_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR2_STANDBY_WAIT` reader - N/A"]
pub type SAR2_STANDBY_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR2_STANDBY_WAIT` writer - N/A"]
pub type SAR2_STANDBY_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR2_RSTB_WAIT` reader - N/A"]
pub type SAR2_RSTB_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR2_RSTB_WAIT` writer - N/A"]
pub type SAR2_RSTB_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR2_XPD_WAIT` reader - N/A"]
pub type SAR2_XPD_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR2_XPD_WAIT` writer - N/A"]
pub type SAR2_XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - saradc2_cntl_fsm."]
    #[inline(always)]
    pub fn sar2_cntl_state(&self) -> SAR2_CNTL_STATE_R {
        SAR2_CNTL_STATE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - RTC control pwdet enable."]
    #[inline(always)]
    pub fn sar2_pwdet_cal_en(&self) -> SAR2_PWDET_CAL_EN_R {
        SAR2_PWDET_CAL_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC control pkdet enable."]
    #[inline(always)]
    pub fn sar2_pkdet_cal_en(&self) -> SAR2_PKDET_CAL_EN_R {
        SAR2_PKDET_CAL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAR2_EN_TEST."]
    #[inline(always)]
    pub fn sar2_en_test(&self) -> SAR2_EN_TEST_R {
        SAR2_EN_TEST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    pub fn sar2_rstb_force(&self) -> SAR2_RSTB_FORCE_R {
        SAR2_RSTB_FORCE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - N/A"]
    #[inline(always)]
    pub fn sar2_standby_wait(&self) -> SAR2_STANDBY_WAIT_R {
        SAR2_STANDBY_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    pub fn sar2_rstb_wait(&self) -> SAR2_RSTB_WAIT_R {
        SAR2_RSTB_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - N/A"]
    #[inline(always)]
    pub fn sar2_xpd_wait(&self) -> SAR2_XPD_WAIT_R {
        SAR2_XPD_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEAS2_CTRL1")
            .field("sar2_cntl_state", &self.sar2_cntl_state())
            .field("sar2_pwdet_cal_en", &self.sar2_pwdet_cal_en())
            .field("sar2_pkdet_cal_en", &self.sar2_pkdet_cal_en())
            .field("sar2_en_test", &self.sar2_en_test())
            .field("sar2_rstb_force", &self.sar2_rstb_force())
            .field("sar2_standby_wait", &self.sar2_standby_wait())
            .field("sar2_rstb_wait", &self.sar2_rstb_wait())
            .field("sar2_xpd_wait", &self.sar2_xpd_wait())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - RTC control pwdet enable."]
    #[inline(always)]
    pub fn sar2_pwdet_cal_en(&mut self) -> SAR2_PWDET_CAL_EN_W<MEAS2_CTRL1_SPEC> {
        SAR2_PWDET_CAL_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - RTC control pkdet enable."]
    #[inline(always)]
    pub fn sar2_pkdet_cal_en(&mut self) -> SAR2_PKDET_CAL_EN_W<MEAS2_CTRL1_SPEC> {
        SAR2_PKDET_CAL_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - SAR2_EN_TEST."]
    #[inline(always)]
    pub fn sar2_en_test(&mut self) -> SAR2_EN_TEST_W<MEAS2_CTRL1_SPEC> {
        SAR2_EN_TEST_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    pub fn sar2_rstb_force(&mut self) -> SAR2_RSTB_FORCE_W<MEAS2_CTRL1_SPEC> {
        SAR2_RSTB_FORCE_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - N/A"]
    #[inline(always)]
    pub fn sar2_standby_wait(&mut self) -> SAR2_STANDBY_WAIT_W<MEAS2_CTRL1_SPEC> {
        SAR2_STANDBY_WAIT_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    pub fn sar2_rstb_wait(&mut self) -> SAR2_RSTB_WAIT_W<MEAS2_CTRL1_SPEC> {
        SAR2_RSTB_WAIT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - N/A"]
    #[inline(always)]
    pub fn sar2_xpd_wait(&mut self) -> SAR2_XPD_WAIT_W<MEAS2_CTRL1_SPEC> {
        SAR2_XPD_WAIT_W::new(self, 24)
    }
}
#[doc = "ADC2 configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`meas2_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meas2_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEAS2_CTRL1_SPEC;
impl crate::RegisterSpec for MEAS2_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meas2_ctrl1::R`](R) reader structure"]
impl crate::Readable for MEAS2_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`meas2_ctrl1::W`](W) writer structure"]
impl crate::Writable for MEAS2_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEAS2_CTRL1 to value 0x0702_0200"]
impl crate::Resettable for MEAS2_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x0702_0200;
}
