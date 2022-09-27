#[doc = "Register `OUT_EOF_DES_ADDR_CH%s` reader"]
pub struct R(crate::R<OUT_EOF_DES_ADDR_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_EOF_DES_ADDR_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_EOF_DES_ADDR_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_EOF_DES_ADDR_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT_EOF_DES_ADDR_CH` reader - This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1."]
pub type OUT_EOF_DES_ADDR_CH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn out_eof_des_addr_ch(&self) -> OUT_EOF_DES_ADDR_CH_R {
        OUT_EOF_DES_ADDR_CH_R::new(self.bits)
    }
}
#[doc = "Outlink descriptor address when EOF occurs of Tx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_eof_des_addr_ch](index.html) module"]
pub struct OUT_EOF_DES_ADDR_CH_SPEC;
impl crate::RegisterSpec for OUT_EOF_DES_ADDR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_eof_des_addr_ch::R](R) reader structure"]
impl crate::Readable for OUT_EOF_DES_ADDR_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_EOF_DES_ADDR_CH%s to value 0"]
impl crate::Resettable for OUT_EOF_DES_ADDR_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
