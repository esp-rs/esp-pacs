#[doc = "Register `OUT_EOF_DES_ADDR_CH1` reader"]
pub type R = crate::R<OUT_EOF_DES_ADDR_CH1_SPEC>;
#[doc = "Field `OUT_EOF_DES_ADDR_CH1` reader - Represents the address of the transmit descriptor when the EOF bit in this descriptor is 1."]
pub type OUT_EOF_DES_ADDR_CH1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the transmit descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn out_eof_des_addr_ch1(&self) -> OUT_EOF_DES_ADDR_CH1_R {
        OUT_EOF_DES_ADDR_CH1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_EOF_DES_ADDR_CH1")
            .field("out_eof_des_addr_ch1", &self.out_eof_des_addr_ch1())
            .finish()
    }
}
#[doc = "Transmit descriptor address when EOF occurs on TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr_ch1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_EOF_DES_ADDR_CH1_SPEC;
impl crate::RegisterSpec for OUT_EOF_DES_ADDR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_eof_des_addr_ch1::R`](R) reader structure"]
impl crate::Readable for OUT_EOF_DES_ADDR_CH1_SPEC {}
#[doc = "`reset()` method sets OUT_EOF_DES_ADDR_CH1 to value 0"]
impl crate::Resettable for OUT_EOF_DES_ADDR_CH1_SPEC {}
