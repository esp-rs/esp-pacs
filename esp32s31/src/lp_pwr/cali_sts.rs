#[doc = "Register `CALI_STS` reader"]
pub type R = crate::R<CALI_STS_SPEC>;
#[doc = "Field `CALI_CUR_ST` reader - current power state register for cali"]
pub type CALI_CUR_ST_R = crate::FieldReader;
#[doc = "Field `CALI_IN_POWER_ON` reader - 1:cali current in power on mode 0:cali not in power on mode"]
pub type CALI_IN_POWER_ON_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - current power state register for cali"]
    #[inline(always)]
    pub fn cali_cur_st(&self) -> CALI_CUR_ST_R {
        CALI_CUR_ST_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 1:cali current in power on mode 0:cali not in power on mode"]
    #[inline(always)]
    pub fn cali_in_power_on(&self) -> CALI_IN_POWER_ON_R {
        CALI_IN_POWER_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALI_STS")
            .field("cali_cur_st", &self.cali_cur_st())
            .field("cali_in_power_on", &self.cali_in_power_on())
            .finish()
    }
}
#[doc = "status register for CALI PWR\n\nYou can [`read`](crate::Reg::read) this register and get [`cali_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALI_STS_SPEC;
impl crate::RegisterSpec for CALI_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cali_sts::R`](R) reader structure"]
impl crate::Readable for CALI_STS_SPEC {}
#[doc = "`reset()` method sets CALI_STS to value 0x04"]
impl crate::Resettable for CALI_STS_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
