#[doc = "Register `REG` reader"]
pub type R = crate::R<REG_SPEC>;
#[doc = "Register `REG` writer"]
pub type W = crate::W<REG_SPEC>;
#[doc = "Field `DIG_REG_DBIAS_SLP` reader - Configures the regulation factor for the digital system voltage regulator when the CPU is in sleep status."]
pub type DIG_REG_DBIAS_SLP_R = crate::FieldReader;
#[doc = "Field `DIG_REG_DBIAS_SLP` writer - Configures the regulation factor for the digital system voltage regulator when the CPU is in sleep status."]
pub type DIG_REG_DBIAS_SLP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIG_REG_DBIAS_WAK` reader - Configures the regulation factor for the digital system voltage regulator when the CPU is in active status."]
pub type DIG_REG_DBIAS_WAK_R = crate::FieldReader;
#[doc = "Field `DIG_REG_DBIAS_WAK` writer - Configures the regulation factor for the digital system voltage regulator when the CPU is in active status."]
pub type DIG_REG_DBIAS_WAK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCK_DCAP` reader - Configures the frequency of the RTC clocks."]
pub type SCK_DCAP_R = crate::FieldReader;
#[doc = "Field `SCK_DCAP` writer - Configures the frequency of the RTC clocks."]
pub type SCK_DCAP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBIAS_SLP` reader - Configures the regulation factor for the low-power voltage regulator when the CPU is in sleep status."]
pub type DBIAS_SLP_R = crate::FieldReader;
#[doc = "Field `DBIAS_SLP` writer - Configures the regulation factor for the low-power voltage regulator when the CPU is in sleep status."]
pub type DBIAS_SLP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBIAS_WAK` reader - Configures the regulation factor for the low-power voltage regulator when the CPU is in active status."]
pub type DBIAS_WAK_R = crate::FieldReader;
#[doc = "Field `DBIAS_WAK` writer - Configures the regulation factor for the low-power voltage regulator when the CPU is in active status."]
pub type DBIAS_WAK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBOOST_FORCE_PD` reader - RTC_DBOOST force power down"]
pub type DBOOST_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DBOOST_FORCE_PD` writer - RTC_DBOOST force power down"]
pub type DBOOST_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBOOST_FORCE_PU` reader - RTC_DBOOST force power up"]
pub type DBOOST_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DBOOST_FORCE_PU` writer - RTC_DBOOST force power up"]
pub type DBOOST_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGULATOR_FORCE_PD` reader - Set this bit to FPD the RTC_REG, which means decreasing its voltage to 0.8 V or lower."]
pub type REGULATOR_FORCE_PD_R = crate::BitReader;
#[doc = "Field `REGULATOR_FORCE_PD` writer - Set this bit to FPD the RTC_REG, which means decreasing its voltage to 0.8 V or lower."]
pub type REGULATOR_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGULATOR_FORCE_PU` reader - Set this bit to FPU the RTC_REG."]
pub type REGULATOR_FORCE_PU_R = crate::BitReader;
#[doc = "Field `REGULATOR_FORCE_PU` writer - Set this bit to FPU the RTC_REG."]
pub type REGULATOR_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG")
            .field("dig_reg_dbias_slp", &self.dig_reg_dbias_slp())
            .field("dig_reg_dbias_wak", &self.dig_reg_dbias_wak())
            .field("sck_dcap", &self.sck_dcap())
            .field("dbias_slp", &self.dbias_slp())
            .field("dbias_wak", &self.dbias_wak())
            .field("dboost_force_pd", &self.dboost_force_pd())
            .field("dboost_force_pu", &self.dboost_force_pu())
            .field("regulator_force_pd", &self.regulator_force_pd())
            .field("regulator_force_pu", &self.regulator_force_pu())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:10 - Configures the regulation factor for the digital system voltage regulator when the CPU is in sleep status."]
    #[inline(always)]
    pub fn dig_reg_dbias_slp(&mut self) -> DIG_REG_DBIAS_SLP_W<REG_SPEC> {
        DIG_REG_DBIAS_SLP_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - Configures the regulation factor for the digital system voltage regulator when the CPU is in active status."]
    #[inline(always)]
    pub fn dig_reg_dbias_wak(&mut self) -> DIG_REG_DBIAS_WAK_W<REG_SPEC> {
        DIG_REG_DBIAS_WAK_W::new(self, 11)
    }
    #[doc = "Bits 14:21 - Configures the frequency of the RTC clocks."]
    #[inline(always)]
    pub fn sck_dcap(&mut self) -> SCK_DCAP_W<REG_SPEC> {
        SCK_DCAP_W::new(self, 14)
    }
    #[doc = "Bits 22:24 - Configures the regulation factor for the low-power voltage regulator when the CPU is in sleep status."]
    #[inline(always)]
    pub fn dbias_slp(&mut self) -> DBIAS_SLP_W<REG_SPEC> {
        DBIAS_SLP_W::new(self, 22)
    }
    #[doc = "Bits 25:27 - Configures the regulation factor for the low-power voltage regulator when the CPU is in active status."]
    #[inline(always)]
    pub fn dbias_wak(&mut self) -> DBIAS_WAK_W<REG_SPEC> {
        DBIAS_WAK_W::new(self, 25)
    }
    #[doc = "Bit 28 - RTC_DBOOST force power down"]
    #[inline(always)]
    pub fn dboost_force_pd(&mut self) -> DBOOST_FORCE_PD_W<REG_SPEC> {
        DBOOST_FORCE_PD_W::new(self, 28)
    }
    #[doc = "Bit 29 - RTC_DBOOST force power up"]
    #[inline(always)]
    pub fn dboost_force_pu(&mut self) -> DBOOST_FORCE_PU_W<REG_SPEC> {
        DBOOST_FORCE_PU_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set this bit to FPD the RTC_REG, which means decreasing its voltage to 0.8 V or lower."]
    #[inline(always)]
    pub fn regulator_force_pd(&mut self) -> REGULATOR_FORCE_PD_W<REG_SPEC> {
        REGULATOR_FORCE_PD_W::new(self, 30)
    }
    #[doc = "Bit 31 - Set this bit to FPU the RTC_REG."]
    #[inline(always)]
    pub fn regulator_force_pu(&mut self) -> REGULATOR_FORCE_PU_W<REG_SPEC> {
        REGULATOR_FORCE_PU_W::new(self, 31)
    }
}
#[doc = "RTC/DIG regulator configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_SPEC;
impl crate::RegisterSpec for REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg::R`](R) reader structure"]
impl crate::Readable for REG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg::W`](W) writer structure"]
impl crate::Writable for REG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG to value 0xa900_2400"]
impl crate::Resettable for REG_SPEC {
    const RESET_VALUE: u32 = 0xa900_2400;
}
