#[doc = "Register `cali_data` reader"]
pub type R = crate::R<CALI_DATA_SPEC>;
#[doc = "Field `CALI_VALUE_SYNC2` reader - "]
pub type CALI_VALUE_SYNC2_R = crate::FieldReader<u32>;
#[doc = "Field `CALI_RDY_REAL` reader - "]
pub type CALI_RDY_REAL_R = crate::BitReader;
#[doc = "Field `CALI_RDY_SYNC2` reader - "]
pub type CALI_RDY_SYNC2_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn cali_value_sync2(&self) -> CALI_VALUE_SYNC2_R {
        CALI_VALUE_SYNC2_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cali_rdy_real(&self) -> CALI_RDY_REAL_R {
        CALI_RDY_REAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cali_rdy_sync2(&self) -> CALI_RDY_SYNC2_R {
        CALI_RDY_SYNC2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("cali_data")
            .field("cali_value_sync2", &self.cali_value_sync2())
            .field("cali_rdy_real", &self.cali_rdy_real())
            .field("cali_rdy_sync2", &self.cali_rdy_sync2())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cali_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALI_DATA_SPEC;
impl crate::RegisterSpec for CALI_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cali_data::R`](R) reader structure"]
impl crate::Readable for CALI_DATA_SPEC {}
#[doc = "`reset()` method sets cali_data to value 0"]
impl crate::Resettable for CALI_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
