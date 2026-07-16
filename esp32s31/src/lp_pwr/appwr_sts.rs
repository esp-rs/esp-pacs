#[doc = "Register `APPWR_STS` reader"]
pub type R = crate::R<APPWR_STS_SPEC>;
#[doc = "Field `APPWR_CUR_ST` reader - current power state register for apsys"]
pub type APPWR_CUR_ST_R = crate::FieldReader;
#[doc = "Field `APSYS_IN_POWER_ON` reader - 1:apsys current in power on mode 0:apsys not in power on mode"]
pub type APSYS_IN_POWER_ON_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - current power state register for apsys"]
    #[inline(always)]
    pub fn appwr_cur_st(&self) -> APPWR_CUR_ST_R {
        APPWR_CUR_ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 1:apsys current in power on mode 0:apsys not in power on mode"]
    #[inline(always)]
    pub fn apsys_in_power_on(&self) -> APSYS_IN_POWER_ON_R {
        APSYS_IN_POWER_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPWR_STS")
            .field("appwr_cur_st", &self.appwr_cur_st())
            .field("apsys_in_power_on", &self.apsys_in_power_on())
            .finish()
    }
}
#[doc = "status register for apsys pwr\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPWR_STS_SPEC;
impl crate::RegisterSpec for APPWR_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appwr_sts::R`](R) reader structure"]
impl crate::Readable for APPWR_STS_SPEC {}
#[doc = "`reset()` method sets APPWR_STS to value 0x06"]
impl crate::Resettable for APPWR_STS_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
