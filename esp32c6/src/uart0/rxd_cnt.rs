#[doc = "Register `RXD_CNT` reader"]
pub struct R(crate::R<RXD_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXD_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXD_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXD_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXD_EDGE_CNT` reader - This register stores the count of rxd edge change. It is used in baud rate-detect process."]
pub type RXD_EDGE_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - This register stores the count of rxd edge change. It is used in baud rate-detect process."]
    #[inline(always)]
    pub fn rxd_edge_cnt(&self) -> RXD_EDGE_CNT_R {
        RXD_EDGE_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXD_CNT")
            .field(
                "rxd_edge_cnt",
                &format_args!("{}", self.rxd_edge_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RXD_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Autobaud edge change count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd_cnt](index.html) module"]
pub struct RXD_CNT_SPEC;
impl crate::RegisterSpec for RXD_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxd_cnt::R](R) reader structure"]
impl crate::Readable for RXD_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXD_CNT to value 0"]
impl crate::Resettable for RXD_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
