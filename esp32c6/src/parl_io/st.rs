#[doc = "Register `ST` reader"]
pub struct R(crate::R<ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_READY` reader - Represents the status that tx is ready."]
pub type TX_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - Represents the status that tx is ready."]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ST")
            .field("tx_ready", &format_args!("{}", self.tx_ready().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Parallel IO module status register0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st](index.html) module"]
pub struct ST_SPEC;
impl crate::RegisterSpec for ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st::R](R) reader structure"]
impl crate::Readable for ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ST to value 0"]
impl crate::Resettable for ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
