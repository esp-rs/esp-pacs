#[doc = "Register `IN_ERR_EOF_DES_ADDR` reader"]
pub type R = crate::R<IN_ERR_EOF_DES_ADDR_SPEC>;
#[doc = "Field `IN_ERR_EOF_DES_ADDR_CH0` reader - Represents the address of the receive descriptor when there are some errors in the currently received data."]
pub type IN_ERR_EOF_DES_ADDR_CH0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the receive descriptor when there are some errors in the currently received data."]
    #[inline(always)]
    pub fn in_err_eof_des_addr_ch0(&self) -> IN_ERR_EOF_DES_ADDR_CH0_R {
        IN_ERR_EOF_DES_ADDR_CH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_ERR_EOF_DES_ADDR")
            .field("in_err_eof_des_addr_ch0", &self.in_err_eof_des_addr_ch0())
            .finish()
    }
}
#[doc = "Receive descriptor address when errors occur of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_err_eof_des_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_ERR_EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for IN_ERR_EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_err_eof_des_addr::R`](R) reader structure"]
impl crate::Readable for IN_ERR_EOF_DES_ADDR_SPEC {}
#[doc = "`reset()` method sets IN_ERR_EOF_DES_ADDR to value 0"]
impl crate::Resettable for IN_ERR_EOF_DES_ADDR_SPEC {}
