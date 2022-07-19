#[doc = "Register `RTC` reader"]
pub struct R(crate::R<RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC` writer"]
pub struct W(crate::W<RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_SPEC>;
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
impl From<crate::W<RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIG_REG_CAL_EN` reader - Need add desc"]
pub type DIG_REG_CAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `DIG_REG_CAL_EN` writer - Need add desc"]
pub type DIG_REG_CAL_EN_W<'a> = crate::BitWriter<'a, u32, RTC_SPEC, bool, 7>;
#[doc = "Field `SCK_DCAP` reader - SCK_DCAP"]
pub type SCK_DCAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCK_DCAP` writer - SCK_DCAP"]
pub type SCK_DCAP_W<'a> = crate::FieldWriter<'a, u32, RTC_SPEC, u8, u8, 8, 14>;
#[doc = "Field `REGULATOR_FORCE_PD` reader - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub type REGULATOR_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `REGULATOR_FORCE_PD` writer - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
pub type REGULATOR_FORCE_PD_W<'a> = crate::BitWriter<'a, u32, RTC_SPEC, bool, 30>;
#[doc = "Field `REGULATOR_FORCE_PU` reader - Need add desc"]
pub type REGULATOR_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `REGULATOR_FORCE_PU` writer - Need add desc"]
pub type REGULATOR_FORCE_PU_W<'a> = crate::BitWriter<'a, u32, RTC_SPEC, bool, 31>;
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
impl W {
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    pub fn dig_reg_cal_en(&mut self) -> DIG_REG_CAL_EN_W {
        DIG_REG_CAL_EN_W::new(self)
    }
    #[doc = "Bits 14:21 - SCK_DCAP"]
    #[inline(always)]
    pub fn sck_dcap(&mut self) -> SCK_DCAP_W {
        SCK_DCAP_W::new(self)
    }
    #[doc = "Bit 30 - RTC_REG force power down (for RTC_REG power down means decrease the voltage to 0.8v or lower )"]
    #[inline(always)]
    pub fn regulator_force_pd(&mut self) -> REGULATOR_FORCE_PD_W {
        REGULATOR_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 31 - Need add desc"]
    #[inline(always)]
    pub fn regulator_force_pu(&mut self) -> REGULATOR_FORCE_PU_W {
        REGULATOR_FORCE_PU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc](index.html) module"]
pub struct RTC_SPEC;
impl crate::RegisterSpec for RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc::R](R) reader structure"]
impl crate::Readable for RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc::W](W) writer structure"]
impl crate::Writable for RTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC to value 0x8000_0000"]
impl crate::Resettable for RTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
