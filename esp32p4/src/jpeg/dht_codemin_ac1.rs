#[doc = "Register `DHT_CODEMIN_AC1` reader"]
pub type R = crate::R<DHT_CODEMIN_AC1_SPEC>;
#[doc = "Field `DHT_CODEMIN_AC1` reader - write the minimum codeword of code length from 1~16 of ac1 table. The codeword is left shifted to the MSB position of a 16bit word"]
pub type DHT_CODEMIN_AC1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write the minimum codeword of code length from 1~16 of ac1 table. The codeword is left shifted to the MSB position of a 16bit word"]
    #[inline(always)]
    pub fn dht_codemin_ac1(&self) -> DHT_CODEMIN_AC1_R {
        DHT_CODEMIN_AC1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHT_CODEMIN_AC1")
            .field(
                "dht_codemin_ac1",
                &format_args!("{}", self.dht_codemin_ac1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DHT_CODEMIN_AC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_codemin_ac1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHT_CODEMIN_AC1_SPEC;
impl crate::RegisterSpec for DHT_CODEMIN_AC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_codemin_ac1::R`](R) reader structure"]
impl crate::Readable for DHT_CODEMIN_AC1_SPEC {}
#[doc = "`reset()` method sets DHT_CODEMIN_AC1 to value 0"]
impl crate::Resettable for DHT_CODEMIN_AC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
