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
#[doc = "Field `DIG_REG_DBIAS_SLP` reader - Configures the regulation factor for the digital system voltage regulator when the CPU is in sleep status."]
pub type DIG_REG_DBIAS_SLP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIG_REG_DBIAS_SLP` writer - Configures the regulation factor for the digital system voltage regulator when the CPU is in sleep status."]
pub type DIG_REG_DBIAS_SLP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_SPEC, u8, u8, 3, O>;
#[doc = "Field `DIG_REG_DBIAS_WAK` reader - Configures the regulation factor for the digital system voltage regulator when the CPU is in active status."]
pub type DIG_REG_DBIAS_WAK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIG_REG_DBIAS_WAK` writer - Configures the regulation factor for the digital system voltage regulator when the CPU is in active status."]
pub type DIG_REG_DBIAS_WAK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_SPEC, u8, u8, 3, O>;
#[doc = "Field `SCK_DCAP` reader - Configures the frequency of the RTC clocks."]
pub type SCK_DCAP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCK_DCAP` writer - Configures the frequency of the RTC clocks."]
pub type SCK_DCAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_SPEC, u8, u8, 8, O>;
#[doc = "Field `DBIAS_SLP` reader - Configures the regulation factor for the low-power voltage regulator when the CPU is in sleep status."]
pub type DBIAS_SLP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBIAS_SLP` writer - Configures the regulation factor for the low-power voltage regulator when the CPU is in sleep status."]
pub type DBIAS_SLP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBIAS_WAK` reader - Configures the regulation factor for the low-power voltage regulator when the CPU is in active status."]
pub type DBIAS_WAK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBIAS_WAK` writer - Configures the regulation factor for the low-power voltage regulator when the CPU is in active status."]
pub type DBIAS_WAK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBOOST_FORCE_PD` reader - RTC_DBOOST force power down"]
pub type DBOOST_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `DBOOST_FORCE_PD` writer - RTC_DBOOST force power down"]
pub type DBOOST_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_SPEC, bool, O>;
#[doc = "Field `DBOOST_FORCE_PU` reader - RTC_DBOOST force power up"]
pub type DBOOST_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `DBOOST_FORCE_PU` writer - RTC_DBOOST force power up"]
pub type DBOOST_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_SPEC, bool, O>;
#[doc = "Field `REGULATOR_FORCE_PD` reader - Set this bit to FPD the RTC_REG, which means decreasing its voltage to 0.8 V or lower."]
pub type REGULATOR_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `REGULATOR_FORCE_PD` writer - Set this bit to FPD the RTC_REG, which means decreasing its voltage to 0.8 V or lower."]
pub type REGULATOR_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_SPEC, bool, O>;
#[doc = "Field `REGULATOR_FORCE_PU` reader - Set this bit to FPU the RTC_REG."]
pub type REGULATOR_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `REGULATOR_FORCE_PU` writer - Set this bit to FPU the RTC_REG."]
pub type REGULATOR_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 8:10 - Configures the regulation factor for the digital system voltage regulator when the CPU is in sleep status."]
    #[inline(always)]
    pub fn dig_reg_dbias_slp(&self) -> DIG_REG_DBIAS_SLP_R {
        DIG_REG_DBIAS_SLP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Configures the regulation factor for the digital system voltage regulator when the CPU is in active status."]
    #[inline(always)]
    pub fn dig_reg_dbias_wak(&self) -> DIG_REG_DBIAS_WAK_R {
        DIG_REG_DBIAS_WAK_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:21 - Configures the frequency of the RTC clocks."]
    #[inline(always)]
    pub fn sck_dcap(&self) -> SCK_DCAP_R {
        SCK_DCAP_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:24 - Configures the regulation factor for the low-power voltage regulator when the CPU is in sleep status."]
    #[inline(always)]
    pub fn dbias_slp(&self) -> DBIAS_SLP_R {
        DBIAS_SLP_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - Configures the regulation factor for the low-power voltage regulator when the CPU is in active status."]
    #[inline(always)]
    pub fn dbias_wak(&self) -> DBIAS_WAK_R {
        DBIAS_WAK_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn dboost_force_pd(&self) -> DBOOST_FORCE_PD_R {
        DBOOST_FORCE_PD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn dboost_force_pu(&self) -> DBOOST_FORCE_PU_R {
        DBOOST_FORCE_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to FPD the RTC_REG, which means decreasing its voltage to 0.8 V or lower."]
    #[inline(always)]
    pub fn regulator_force_pd(&self) -> REGULATOR_FORCE_PD_R {
        REGULATOR_FORCE_PD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to FPU the RTC_REG."]
    #[inline(always)]
    pub fn regulator_force_pu(&self) -> REGULATOR_FORCE_PU_R {
        REGULATOR_FORCE_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - Configures the regulation factor for the digital system voltage regulator when the CPU is in sleep status."]
    #[inline(always)]
    pub fn dig_reg_dbias_slp(&mut self) -> DIG_REG_DBIAS_SLP_W<8> {
        DIG_REG_DBIAS_SLP_W::new(self)
    }
    #[doc = "Bits 11:13 - Configures the regulation factor for the digital system voltage regulator when the CPU is in active status."]
    #[inline(always)]
    pub fn dig_reg_dbias_wak(&mut self) -> DIG_REG_DBIAS_WAK_W<11> {
        DIG_REG_DBIAS_WAK_W::new(self)
    }
    #[doc = "Bits 14:21 - Configures the frequency of the RTC clocks."]
    #[inline(always)]
    pub fn sck_dcap(&mut self) -> SCK_DCAP_W<14> {
        SCK_DCAP_W::new(self)
    }
    #[doc = "Bits 22:24 - Configures the regulation factor for the low-power voltage regulator when the CPU is in sleep status."]
    #[inline(always)]
    pub fn dbias_slp(&mut self) -> DBIAS_SLP_W<22> {
        DBIAS_SLP_W::new(self)
    }
    #[doc = "Bits 25:27 - Configures the regulation factor for the low-power voltage regulator when the CPU is in active status."]
    #[inline(always)]
    pub fn dbias_wak(&mut self) -> DBIAS_WAK_W<25> {
        DBIAS_WAK_W::new(self)
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn dboost_force_pd(&mut self) -> DBOOST_FORCE_PD_W<28> {
        DBOOST_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn dboost_force_pu(&mut self) -> DBOOST_FORCE_PU_W<29> {
        DBOOST_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 30 - Set this bit to FPD the RTC_REG, which means decreasing its voltage to 0.8 V or lower."]
    #[inline(always)]
    pub fn regulator_force_pd(&mut self) -> REGULATOR_FORCE_PD_W<30> {
        REGULATOR_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to FPU the RTC_REG."]
    #[inline(always)]
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
#[doc = "RTC/DIG regulator configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg](index.html) module"]
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
#[doc = "`reset()` method sets REG to value 0xa900_2400"]
impl crate::Resettable for REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa900_2400
    }
}
