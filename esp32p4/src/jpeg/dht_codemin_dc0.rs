#[doc = "Register `DHT_CODEMIN_DC0` reader"]
pub type R = crate::R<DHT_CODEMIN_DC0_SPEC>;
#[doc = "Field `DHT_CODEMIN_DC0` reader - write the minimum codeword of code length from 1~16 of dc0 table. The codeword is left shifted to the MSB position of a 16bit word"]
pub type DHT_CODEMIN_DC0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write the minimum codeword of code length from 1~16 of dc0 table. The codeword is left shifted to the MSB position of a 16bit word"]
    #[inline(always)]
    pub fn dht_codemin_dc0(&self) -> DHT_CODEMIN_DC0_R {
        DHT_CODEMIN_DC0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHT_CODEMIN_DC0")
            .field("dht_codemin_dc0", &self.dht_codemin_dc0())
            .finish()
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_codemin_dc0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHT_CODEMIN_DC0_SPEC;
impl crate::RegisterSpec for DHT_CODEMIN_DC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_codemin_dc0::R`](R) reader structure"]
impl crate::Readable for DHT_CODEMIN_DC0_SPEC {}
#[doc = "`reset()` method sets DHT_CODEMIN_DC0 to value 0"]
impl crate::Resettable for DHT_CODEMIN_DC0_SPEC {
    const RESET_VALUE: u32 = 0;
}
