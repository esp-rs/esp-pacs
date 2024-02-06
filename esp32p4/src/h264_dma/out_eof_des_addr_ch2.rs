#[doc = "Register `OUT_EOF_DES_ADDR_CH2` reader"]
pub type R = crate::R<OUT_EOF_DES_ADDR_CH2_SPEC>;
#[doc = "Field `OUT_EOF_DES_ADDR_CH2` reader - This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1."]
pub type OUT_EOF_DES_ADDR_CH2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn out_eof_des_addr_ch2(&self) -> OUT_EOF_DES_ADDR_CH2_R {
        OUT_EOF_DES_ADDR_CH2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_EOF_DES_ADDR_CH2")
            .field(
                "out_eof_des_addr_ch2",
                &format_args!("{}", self.out_eof_des_addr_ch2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_EOF_DES_ADDR_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "TX CH2 eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_eof_des_addr_ch2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_EOF_DES_ADDR_CH2_SPEC;
impl crate::RegisterSpec for OUT_EOF_DES_ADDR_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_eof_des_addr_ch2::R`](R) reader structure"]
impl crate::Readable for OUT_EOF_DES_ADDR_CH2_SPEC {}
#[doc = "`reset()` method sets OUT_EOF_DES_ADDR_CH2 to value 0"]
impl crate::Resettable for OUT_EOF_DES_ADDR_CH2_SPEC {
    const RESET_VALUE: u32 = 0;
}
