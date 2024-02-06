#[doc = "Register `OUT_EOF_BFR_DES_ADDR_CH1` reader"]
pub type R = crate::R<OUT_EOF_BFR_DES_ADDR_CH1_SPEC>;
#[doc = "Field `OUT_EOF_BFR_DES_ADDR` reader - This register stores the address of the outlink descriptor before the last outlink descriptor."]
pub type OUT_EOF_BFR_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the outlink descriptor before the last outlink descriptor."]
    #[inline(always)]
    pub fn out_eof_bfr_des_addr(&self) -> OUT_EOF_BFR_DES_ADDR_R {
        OUT_EOF_BFR_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_EOF_BFR_DES_ADDR_CH1")
            .field(
                "out_eof_bfr_des_addr",
                &format_args!("{}", self.out_eof_bfr_des_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_EOF_BFR_DES_ADDR_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "DMA_OUT_EOF_BFR_DES_ADDR_CH1_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_bfr_des_addr_ch1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_EOF_BFR_DES_ADDR_CH1_SPEC;
impl crate::RegisterSpec for OUT_EOF_BFR_DES_ADDR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_eof_bfr_des_addr_ch1::R`](R) reader structure"]
impl crate::Readable for OUT_EOF_BFR_DES_ADDR_CH1_SPEC {}
#[doc = "`reset()` method sets OUT_EOF_BFR_DES_ADDR_CH1 to value 0"]
impl crate::Resettable for OUT_EOF_BFR_DES_ADDR_CH1_SPEC {
    const RESET_VALUE: u32 = 0;
}
