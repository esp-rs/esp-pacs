#[doc = "Register `_0_TX_EOF_DES_ADDR` reader"]
pub type R = crate::R<_0_TX_EOF_DES_ADDR_SPEC>;
#[doc = "Field `SLC0_TX_SUC_EOF_DES_ADDR` reader - "]
pub type SLC0_TX_SUC_EOF_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_tx_suc_eof_des_addr(&self) -> SLC0_TX_SUC_EOF_DES_ADDR_R {
        SLC0_TX_SUC_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_TX_EOF_DES_ADDR")
            .field("slc0_tx_suc_eof_des_addr", &self.slc0_tx_suc_eof_des_addr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_tx_eof_des_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0_TX_EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for _0_TX_EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_tx_eof_des_addr::R`](R) reader structure"]
impl crate::Readable for _0_TX_EOF_DES_ADDR_SPEC {}
#[doc = "`reset()` method sets _0_TX_EOF_DES_ADDR to value 0"]
impl crate::Resettable for _0_TX_EOF_DES_ADDR_SPEC {}
