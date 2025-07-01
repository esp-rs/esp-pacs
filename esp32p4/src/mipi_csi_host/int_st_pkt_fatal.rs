#[doc = "Register `INT_ST_PKT_FATAL` reader"]
pub type R = crate::R<INT_ST_PKT_FATAL_SPEC>;
#[doc = "Field `ST_ERR_ECC_DOUBLE` reader - NA"]
pub type ST_ERR_ECC_DOUBLE_R = crate::BitReader;
#[doc = "Field `ST_SHORTER_PAYLOAD` reader - NA"]
pub type ST_SHORTER_PAYLOAD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_double(&self) -> ST_ERR_ECC_DOUBLE_R {
        ST_ERR_ECC_DOUBLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn st_shorter_payload(&self) -> ST_SHORTER_PAYLOAD_R {
        ST_SHORTER_PAYLOAD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST_PKT_FATAL")
            .field("st_err_ecc_double", &self.st_err_ecc_double())
            .field("st_shorter_payload", &self.st_shorter_payload())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_pkt_fatal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_PKT_FATAL_SPEC;
impl crate::RegisterSpec for INT_ST_PKT_FATAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_pkt_fatal::R`](R) reader structure"]
impl crate::Readable for INT_ST_PKT_FATAL_SPEC {}
#[doc = "`reset()` method sets INT_ST_PKT_FATAL to value 0"]
impl crate::Resettable for INT_ST_PKT_FATAL_SPEC {}
