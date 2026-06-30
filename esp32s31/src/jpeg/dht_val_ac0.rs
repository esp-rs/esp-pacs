#[doc = "Register `DHT_VAl_AC0` reader"]
pub type R = crate::R<DHT_VAL_AC0_SPEC>;
#[doc = "Field `DHT_VAL_AC0` reader - write codeword corresponding huffman values of ac0 table"]
pub type DHT_VAL_AC0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write codeword corresponding huffman values of ac0 table"]
    #[inline(always)]
    pub fn dht_val_ac0(&self) -> DHT_VAL_AC0_R {
        DHT_VAL_AC0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHT_VAl_AC0")
            .field("dht_val_ac0", &self.dht_val_ac0())
            .finish()
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dht_val_ac0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHT_VAL_AC0_SPEC;
impl crate::RegisterSpec for DHT_VAL_AC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_val_ac0::R`](R) reader structure"]
impl crate::Readable for DHT_VAL_AC0_SPEC {}
#[doc = "`reset()` method sets DHT_VAl_AC0 to value 0"]
impl crate::Resettable for DHT_VAL_AC0_SPEC {}
