#[doc = "Register `IN_SUC_EOF_DES_ADDR_CH%s` reader"]
pub type R = crate::R<IN_SUC_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Field `IN_SUC_EOF_DES_ADDR` reader - Represents the address of the receive descriptor when the EOF bit in this descriptor is 1."]
pub type IN_SUC_EOF_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the receive descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn in_suc_eof_des_addr(&self) -> IN_SUC_EOF_DES_ADDR_R {
        IN_SUC_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_SUC_EOF_DES_ADDR_CH")
            .field("in_suc_eof_des_addr", &self.in_suc_eof_des_addr())
            .finish()
    }
}
#[doc = "Receive descriptor address when EOF occurs on RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_suc_eof_des_addr_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SUC_EOF_DES_ADDR_CH_SPEC;
impl crate::RegisterSpec for IN_SUC_EOF_DES_ADDR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_suc_eof_des_addr_ch::R`](R) reader structure"]
impl crate::Readable for IN_SUC_EOF_DES_ADDR_CH_SPEC {}
#[doc = "`reset()` method sets IN_SUC_EOF_DES_ADDR_CH%s to value 0"]
impl crate::Resettable for IN_SUC_EOF_DES_ADDR_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
