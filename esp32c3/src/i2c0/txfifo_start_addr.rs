#[doc = "Register `TXFIFO_START_ADDR` reader"]
pub struct R(crate::R<TXFIFO_START_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFIFO_START_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFIFO_START_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFIFO_START_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXFIFO_START_ADDR` reader - reg_txfifo_start_addr."]
pub type TXFIFO_START_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reg_txfifo_start_addr."]
    #[inline(always)]
    pub fn txfifo_start_addr(&self) -> TXFIFO_START_ADDR_R {
        TXFIFO_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXFIFO_START_ADDR")
            .field(
                "txfifo_start_addr",
                &format_args!("{}", self.txfifo_start_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TXFIFO_START_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "I2C_TXFIFO_START_ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifo_start_addr](index.html) module"]
pub struct TXFIFO_START_ADDR_SPEC;
impl crate::RegisterSpec for TXFIFO_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfifo_start_addr::R](R) reader structure"]
impl crate::Readable for TXFIFO_START_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXFIFO_START_ADDR to value 0"]
impl crate::Resettable for TXFIFO_START_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
