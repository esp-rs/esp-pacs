#[doc = "Register `OUT_EOF_DES_ADDR_CH0` reader"]
pub type R = crate::R<OUT_EOF_DES_ADDR_CH0_SPEC>;
#[doc = "Field `OUT_EOF_DES_ADDR_CH0` reader - Represents the address of the transmit descriptor when the EOF bit in this descriptor is 1."]
pub type OUT_EOF_DES_ADDR_CH0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the transmit descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn out_eof_des_addr_ch0(&self) -> OUT_EOF_DES_ADDR_CH0_R {
        OUT_EOF_DES_ADDR_CH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_EOF_DES_ADDR_CH0")
            .field("out_eof_des_addr_ch0", &self.out_eof_des_addr_ch0())
            .finish()
    }
}
#[doc = "Transmit descriptor address when EOF occurs on TX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr_ch0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_EOF_DES_ADDR_CH0_SPEC;
impl crate::RegisterSpec for OUT_EOF_DES_ADDR_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_eof_des_addr_ch0::R`](R) reader structure"]
impl crate::Readable for OUT_EOF_DES_ADDR_CH0_SPEC {}
#[doc = "`reset()` method sets OUT_EOF_DES_ADDR_CH0 to value 0"]
impl crate::Resettable for OUT_EOF_DES_ADDR_CH0_SPEC {}
