///Register `DHT_TOTLEN_AC0` reader
pub type R = crate::R<DHT_TOTLEN_AC0_SPEC>;
///Field `DHT_TOTLEN_AC0` reader - write the numbers of 1~n codeword length sum from 1~16 of ac0 table
pub type DHT_TOTLEN_AC0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - write the numbers of 1~n codeword length sum from 1~16 of ac0 table
    #[inline(always)]
    pub fn dht_totlen_ac0(&self) -> DHT_TOTLEN_AC0_R {
        DHT_TOTLEN_AC0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHT_TOTLEN_AC0")
            .field("dht_totlen_ac0", &self.dht_totlen_ac0())
            .finish()
    }
}
/**Trace and Debug registers

You can [`read`](crate::generic::Reg::read) this register and get [`dht_totlen_ac0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DHT_TOTLEN_AC0_SPEC;
impl crate::RegisterSpec for DHT_TOTLEN_AC0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dht_totlen_ac0::R`](R) reader structure
impl crate::Readable for DHT_TOTLEN_AC0_SPEC {}
///`reset()` method sets DHT_TOTLEN_AC0 to value 0
impl crate::Resettable for DHT_TOTLEN_AC0_SPEC {
    const RESET_VALUE: u32 = 0;
}
