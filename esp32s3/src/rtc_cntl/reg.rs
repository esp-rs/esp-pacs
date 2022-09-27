#[doc = "Register `REG` reader"]
pub struct R(crate::R<REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG` writer"]
pub struct W(crate::W<REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_SPEC>;
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
impl From<crate::W<REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIG_REG_CAL_EN` reader - enable dig regulator cali"]
pub type DIG_REG_CAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIG_REG_CAL_EN` writer - enable dig regulator cali"]
pub type DIG_REG_CAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_SPEC, bool, O>;
#[doc = "Field `SCK_DCAP` reader - SCK_DCAP"]
pub type SCK_DCAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCK_DCAP` writer - SCK_DCAP"]
pub type SCK_DCAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_SPEC, u8, u8, 8, O>;
#[doc = "Field `RTC_DBOOST_FORCE_PD` reader - RTC_DBOOST force power down"]
pub type RTC_DBOOST_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_DBOOST_FORCE_PD` writer - RTC_DBOOST force power down"]
pub type RTC_DBOOST_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_SPEC, bool, O>;
#[doc = "Field `RTC_DBOOST_FORCE_PU` reader - RTC_DBOOST force power up"]
pub type RTC_DBOOST_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `RTC_DBOOST_FORCE_PU` writer - RTC_DBOOST force power up"]
pub type RTC_DBOOST_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_SPEC, bool, O>;
#[doc = "Field `RTC_REGULATOR_FORCE_PD` reader - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub type RTC_REGULATOR_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_REGULATOR_FORCE_PD` writer - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub type RTC_REGULATOR_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_SPEC, bool, O>;
#[doc = "Field `RTC_REGULATOR_FORCE_PU` reader - RTC_REG force power on (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub type RTC_REGULATOR_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `RTC_REGULATOR_FORCE_PU` writer - RTC_REG force power on (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub type RTC_REGULATOR_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - enable dig regulator cali"]
    #[inline(always)]
    pub fn dig_reg_cal_en(&self) -> DIG_REG_CAL_EN_R {
        DIG_REG_CAL_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn sck_dcap(&self) -> SCK_DCAP_R {
        SCK_DCAP_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn rtc_dboost_force_pd(&self) -> RTC_DBOOST_FORCE_PD_R {
        RTC_DBOOST_FORCE_PD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn rtc_dboost_force_pu(&self) -> RTC_DBOOST_FORCE_PU_R {
        RTC_DBOOST_FORCE_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn rtc_regulator_force_pd(&self) -> RTC_REGULATOR_FORCE_PD_R {
        RTC_REGULATOR_FORCE_PD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RTC_REG force power on (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn rtc_regulator_force_pu(&self) -> RTC_REGULATOR_FORCE_PU_R {
        RTC_REGULATOR_FORCE_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - enable dig regulator cali"]
    #[inline(always)]
    pub fn dig_reg_cal_en(&mut self) -> DIG_REG_CAL_EN_W<7> {
        DIG_REG_CAL_EN_W::new(self)
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn sck_dcap(&mut self) -> SCK_DCAP_W<14> {
        SCK_DCAP_W::new(self)
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn rtc_dboost_force_pd(&mut self) -> RTC_DBOOST_FORCE_PD_W<28> {
        RTC_DBOOST_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn rtc_dboost_force_pu(&mut self) -> RTC_DBOOST_FORCE_PU_W<29> {
        RTC_DBOOST_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn rtc_regulator_force_pd(&mut self) -> RTC_REGULATOR_FORCE_PD_W<30> {
        RTC_REGULATOR_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 31 - RTC_REG force power on (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn rtc_regulator_force_pu(&mut self) -> RTC_REGULATOR_FORCE_PU_W<31> {
        RTC_REGULATOR_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure rtc regulator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg](index.html) module"]
pub struct REG_SPEC;
impl crate::RegisterSpec for REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg::R](R) reader structure"]
impl crate::Readable for REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg::W](W) writer structure"]
impl crate::Writable for REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG to value 0xa000_0000"]
impl crate::Resettable for REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa000_0000
    }
}
