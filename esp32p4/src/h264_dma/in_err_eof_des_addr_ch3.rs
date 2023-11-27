#[doc = "Register `IN_ERR_EOF_DES_ADDR_CH3` reader"]
pub type R = crate::R<IN_ERR_EOF_DES_ADDR_CH3_SPEC>;
#[doc = "Field `IN_ERR_EOF_DES_ADDR_CH3` reader - This register stores the address of the inlink descriptor when there are some errors in current receiving data."]
pub type IN_ERR_EOF_DES_ADDR_CH3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the inlink descriptor when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn in_err_eof_des_addr_ch3(&self) -> IN_ERR_EOF_DES_ADDR_CH3_R {
        IN_ERR_EOF_DES_ADDR_CH3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_ERR_EOF_DES_ADDR_CH3")
            .field(
                "in_err_eof_des_addr_ch3",
                &format_args!("{}", self.in_err_eof_des_addr_ch3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_ERR_EOF_DES_ADDR_CH3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RX CH3 err eof des addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_err_eof_des_addr_ch3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_ERR_EOF_DES_ADDR_CH3_SPEC;
impl crate::RegisterSpec for IN_ERR_EOF_DES_ADDR_CH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_err_eof_des_addr_ch3::R`](R) reader structure"]
impl crate::Readable for IN_ERR_EOF_DES_ADDR_CH3_SPEC {}
#[doc = "`reset()` method sets IN_ERR_EOF_DES_ADDR_CH3 to value 0"]
impl crate::Resettable for IN_ERR_EOF_DES_ADDR_CH3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
