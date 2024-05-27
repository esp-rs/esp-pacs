#[doc = "Register `_1_TX_ERREOF_DES_ADDR` reader"]
pub type R = crate::R<_1_TX_ERREOF_DES_ADDR_SPEC>;
#[doc = "Field `SLC1_TX_ERR_EOF_DES_ADDR` reader - "]
pub type SLC1_TX_ERR_EOF_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc1_tx_err_eof_des_addr(&self) -> SLC1_TX_ERR_EOF_DES_ADDR_R {
        SLC1_TX_ERR_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1_TX_ERREOF_DES_ADDR")
            .field("slc1_tx_err_eof_des_addr", &self.slc1_tx_err_eof_des_addr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1_tx_erreof_des_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1_TX_ERREOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for _1_TX_ERREOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1_tx_erreof_des_addr::R`](R) reader structure"]
impl crate::Readable for _1_TX_ERREOF_DES_ADDR_SPEC {}
#[doc = "`reset()` method sets _1_TX_ERREOF_DES_ADDR to value 0"]
impl crate::Resettable for _1_TX_ERREOF_DES_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
