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
#[doc = "Field `RX_HEAD` reader - This register stores the header of the current received packet."]
pub type RX_HEAD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the header of the current received packet."]
    #[inline(always)]
    pub fn rx_head(&self) -> RX_HEAD_R {
        RX_HEAD_R::new(self.bits)
    }
}
#[doc = "UHCI packet header register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_head](index.html) module"]
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
