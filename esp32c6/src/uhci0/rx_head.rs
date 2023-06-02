#[doc = "Register `RX_HEAD` reader"]
pub struct R(crate::R<RX_HEAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_HEAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_HEAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_HEAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_HEAD` reader - Stores the head of received packet."]
pub type RX_HEAD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the head of received packet."]
    #[inline(always)]
    pub fn rx_head(&self) -> RX_HEAD_R {
        RX_HEAD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_HEAD")
            .field("rx_head", &format_args!("{}", self.rx_head().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_HEAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "UHCI Head Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_head](index.html) module"]
pub struct RX_HEAD_SPEC;
impl crate::RegisterSpec for RX_HEAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_head::R](R) reader structure"]
impl crate::Readable for RX_HEAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_HEAD to value 0"]
impl crate::Resettable for RX_HEAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
