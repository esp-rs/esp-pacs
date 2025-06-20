#[doc = "Register `OUT_EOF_DES_ADDR_CH%s` reader"]
pub type R = crate::R<OUT_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Field `OUT_EOF_DES_ADDR_CH` reader - Represents the address of the transmit descriptor when the EOF bit in this descriptor is 1."]
pub type OUT_EOF_DES_ADDR_CH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the transmit descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn out_eof_des_addr_ch(&self) -> OUT_EOF_DES_ADDR_CH_R {
        OUT_EOF_DES_ADDR_CH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_EOF_DES_ADDR_CH")
            .field("out_eof_des_addr_ch", &self.out_eof_des_addr_ch())
            .finish()
    }
}
#[doc = "Transmit descriptor address when EOF occurs on TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_eof_des_addr_ch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_EOF_DES_ADDR_CH_SPEC;
impl crate::RegisterSpec for OUT_EOF_DES_ADDR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_eof_des_addr_ch::R`](R) reader structure"]
impl crate::Readable for OUT_EOF_DES_ADDR_CH_SPEC {}
#[doc = "`reset()` method sets OUT_EOF_DES_ADDR_CH%s to value 0"]
impl crate::Resettable for OUT_EOF_DES_ADDR_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
