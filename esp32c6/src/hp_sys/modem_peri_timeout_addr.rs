#[doc = "Register `MODEM_PERI_TIMEOUT_ADDR` reader"]
pub struct R(crate::R<MODEM_PERI_TIMEOUT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODEM_PERI_TIMEOUT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODEM_PERI_TIMEOUT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODEM_PERI_TIMEOUT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MODEM_PERI_TIMEOUT_ADDR` reader - Record the address information of abnormal access"]
pub type MODEM_PERI_TIMEOUT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Record the address information of abnormal access"]
    #[inline(always)]
    pub fn modem_peri_timeout_addr(&self) -> MODEM_PERI_TIMEOUT_ADDR_R {
        MODEM_PERI_TIMEOUT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_PERI_TIMEOUT_ADDR")
            .field(
                "modem_peri_timeout_addr",
                &format_args!("{}", self.modem_peri_timeout_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEM_PERI_TIMEOUT_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "MODEM_PERI_TIMEOUT_ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modem_peri_timeout_addr](index.html) module"]
pub struct MODEM_PERI_TIMEOUT_ADDR_SPEC;
impl crate::RegisterSpec for MODEM_PERI_TIMEOUT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modem_peri_timeout_addr::R](R) reader structure"]
impl crate::Readable for MODEM_PERI_TIMEOUT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MODEM_PERI_TIMEOUT_ADDR to value 0"]
impl crate::Resettable for MODEM_PERI_TIMEOUT_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
