#[doc = "Register `DHT_CODEMIN_DC1` reader"]
pub type R = crate::R<DHT_CODEMIN_DC1_SPEC>;
#[doc = "Field `DHT_CODEMIN_DC1` reader - write the minimum codeword of code length from 1~16 of dc1 table. The codeword is left shifted to the MSB position of a 16bit word"]
pub type DHT_CODEMIN_DC1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write the minimum codeword of code length from 1~16 of dc1 table. The codeword is left shifted to the MSB position of a 16bit word"]
    #[inline(always)]
    pub fn dht_codemin_dc1(&self) -> DHT_CODEMIN_DC1_R {
        DHT_CODEMIN_DC1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHT_CODEMIN_DC1")
            .field("dht_codemin_dc1", &self.dht_codemin_dc1())
            .finish()
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dht_codemin_dc1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHT_CODEMIN_DC1_SPEC;
impl crate::RegisterSpec for DHT_CODEMIN_DC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_codemin_dc1::R`](R) reader structure"]
impl crate::Readable for DHT_CODEMIN_DC1_SPEC {}
#[doc = "`reset()` method sets DHT_CODEMIN_DC1 to value 0"]
impl crate::Resettable for DHT_CODEMIN_DC1_SPEC {}
