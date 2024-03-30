#[doc = "Register `EOF_DES_ADDR` reader"]
pub type R = crate::R<EOF_DES_ADDR_SPEC>;
#[doc = "Field `OUT_EOF_DES_ADDR` reader - This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1."]
pub type OUT_EOF_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn out_eof_des_addr(&self) -> OUT_EOF_DES_ADDR_R {
        OUT_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EOF_DES_ADDR")
            .field(
                "out_eof_des_addr",
                &format_args!("{}", self.out_eof_des_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EOF_DES_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "TX CHx eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eof_des_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eof_des_addr::R`](R) reader structure"]
impl crate::Readable for EOF_DES_ADDR_SPEC {}
#[doc = "`reset()` method sets EOF_DES_ADDR to value 0"]
impl crate::Resettable for EOF_DES_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
