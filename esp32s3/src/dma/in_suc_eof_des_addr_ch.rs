#[doc = "Register `IN_SUC_EOF_DES_ADDR_CH%s` reader"]
pub struct R(crate::R<IN_SUC_EOF_DES_ADDR_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_SUC_EOF_DES_ADDR_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_SUC_EOF_DES_ADDR_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_SUC_EOF_DES_ADDR_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_SUC_EOF_DES_ADDR_CH` reader - This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1."]
pub type IN_SUC_EOF_DES_ADDR_CH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the inlink descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn in_suc_eof_des_addr_ch(&self) -> IN_SUC_EOF_DES_ADDR_CH_R {
        IN_SUC_EOF_DES_ADDR_CH_R::new(self.bits)
    }
}
#[doc = "Inlink descriptor address when EOF occurs of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_suc_eof_des_addr_ch](index.html) module"]
pub struct IN_SUC_EOF_DES_ADDR_CH_SPEC;
impl crate::RegisterSpec for IN_SUC_EOF_DES_ADDR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_suc_eof_des_addr_ch::R](R) reader structure"]
impl crate::Readable for IN_SUC_EOF_DES_ADDR_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_SUC_EOF_DES_ADDR_CH%s to value 0"]
impl crate::Resettable for IN_SUC_EOF_DES_ADDR_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
