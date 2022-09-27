#[doc = "Register `IN_ERR_EOF_DES_ADDR_CH0` reader"]
pub struct R(crate::R<IN_ERR_EOF_DES_ADDR_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_ERR_EOF_DES_ADDR_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_ERR_EOF_DES_ADDR_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_ERR_EOF_DES_ADDR_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_ERR_EOF_DES_ADDR_CH0` reader - This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0."]
pub type IN_ERR_EOF_DES_ADDR_CH0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the inlink descriptor when there are some errors in current receiving data. Only used when peripheral is UHCI0."]
    #[inline(always)]
    pub fn in_err_eof_des_addr_ch0(&self) -> IN_ERR_EOF_DES_ADDR_CH0_R {
        IN_ERR_EOF_DES_ADDR_CH0_R::new(self.bits)
    }
}
#[doc = "DMA_IN_ERR_EOF_DES_ADDR_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_err_eof_des_addr_ch0](index.html) module"]
pub struct IN_ERR_EOF_DES_ADDR_CH0_SPEC;
impl crate::RegisterSpec for IN_ERR_EOF_DES_ADDR_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_err_eof_des_addr_ch0::R](R) reader structure"]
impl crate::Readable for IN_ERR_EOF_DES_ADDR_CH0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_ERR_EOF_DES_ADDR_CH0 to value 0"]
impl crate::Resettable for IN_ERR_EOF_DES_ADDR_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
