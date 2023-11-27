#[doc = "Register `DHT_VAl_DC0` reader"]
pub type R = crate::R<DHT_VAL_DC0_SPEC>;
#[doc = "Field `DHT_VAL_DC0` reader - write codeword corresponding huffman values of dc0 table"]
pub type DHT_VAL_DC0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write codeword corresponding huffman values of dc0 table"]
    #[inline(always)]
    pub fn dht_val_dc0(&self) -> DHT_VAL_DC0_R {
        DHT_VAL_DC0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHT_VAl_DC0")
            .field(
                "dht_val_dc0",
                &format_args!("{}", self.dht_val_dc0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DHT_VAL_DC0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dht_val_dc0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DHT_VAL_DC0_SPEC;
impl crate::RegisterSpec for DHT_VAL_DC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_val_dc0::R`](R) reader structure"]
impl crate::Readable for DHT_VAL_DC0_SPEC {}
#[doc = "`reset()` method sets DHT_VAl_DC0 to value 0"]
impl crate::Resettable for DHT_VAL_DC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
