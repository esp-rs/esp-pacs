#[doc = "Register `RTC_CNTL` reader"]
pub struct R(crate::R<RTC_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CNTL` writer"]
pub struct W(crate::W<RTC_CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CNTL_SPEC>;
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
impl From<crate::W<RTC_CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CNTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIG_REG_CAL_EN` reader - Need add desc"]
pub type DIG_REG_CAL_EN_R = crate::BitReader;
#[doc = "Field `DIG_REG_CAL_EN` writer - Need add desc"]
pub type DIG_REG_CAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, RTC_CNTL_SPEC, O>;
#[doc = "Field `SCK_DCAP` reader - SCK_DCAP"]
pub type SCK_DCAP_R = crate::FieldReader;
#[doc = "Field `SCK_DCAP` writer - SCK_DCAP"]
pub type SCK_DCAP_W<'a, const O: u8> = crate::FieldWriter<'a, RTC_CNTL_SPEC, 8, O>;
#[doc = "Field `REGULATOR_FORCE_PD` reader - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub type REGULATOR_FORCE_PD_R = crate::BitReader;
#[doc = "Field `REGULATOR_FORCE_PD` writer - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub type REGULATOR_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, RTC_CNTL_SPEC, O>;
#[doc = "Field `REGULATOR_FORCE_PU` reader - Need add desc"]
pub type REGULATOR_FORCE_PU_R = crate::BitReader;
#[doc = "Field `REGULATOR_FORCE_PU` writer - Need add desc"]
pub type REGULATOR_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, RTC_CNTL_SPEC, O>;
impl R {
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    pub fn dig_reg_cal_en(&self) -> DIG_REG_CAL_EN_R {
        DIG_REG_CAL_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn sck_dcap(&self) -> SCK_DCAP_R {
        SCK_DCAP_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn regulator_force_pd(&self) -> REGULATOR_FORCE_PD_R {
        REGULATOR_FORCE_PD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn regulator_force_pu(&self) -> REGULATOR_FORCE_PU_R {
        REGULATOR_FORCE_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_CNTL")
            .field(
                "dig_reg_cal_en",
                &format_args!("{}", self.dig_reg_cal_en().bit()),
            )
            .field("sck_dcap", &format_args!("{}", self.sck_dcap().bits()))
            .field(
                "regulator_force_pd",
                &format_args!("{}", self.regulator_force_pd().bit()),
            )
            .field(
                "regulator_force_pu",
                &format_args!("{}", self.regulator_force_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn dig_reg_cal_en(&mut self) -> DIG_REG_CAL_EN_W<7> {
        DIG_REG_CAL_EN_W::new(self)
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    #[must_use]
    pub fn sck_dcap(&mut self) -> SCK_DCAP_W<14> {
        SCK_DCAP_W::new(self)
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    #[must_use]
    pub fn regulator_force_pd(&mut self) -> REGULATOR_FORCE_PD_W<30> {
        REGULATOR_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn regulator_force_pu(&mut self) -> REGULATOR_FORCE_PU_W<31> {
        REGULATOR_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl](index.html) module"]
pub struct RTC_CNTL_SPEC;
impl crate::RegisterSpec for RTC_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cntl::R](R) reader structure"]
impl crate::Readable for RTC_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cntl::W](W) writer structure"]
impl crate::Writable for RTC_CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_CNTL to value 0x8000_0000"]
impl crate::Resettable for RTC_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
