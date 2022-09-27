#[doc = "Register `RXFIFO_ST` reader"]
pub struct R(crate::R<RXFIFO_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIFO_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIFO_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIFO_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO_START_ADDR` reader - This is the offset address of the last receiving data as described in nonfifo_rx_thres_register."]
pub type RXFIFO_START_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFIFO_END_ADDR` reader - This is the offset address of the first receiving data as described in nonfifo_rx_thres_register."]
pub type RXFIFO_END_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFIFO_START_ADDR` reader - This is the offset address of the first sending data as described in nonfifo_tx_thres register."]
pub type TXFIFO_START_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFIFO_END_ADDR` reader - This is the offset address of the last sending data as described in nonfifo_tx_thres register."]
pub type TXFIFO_END_ADDR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - This is the offset address of the last receiving data as described in nonfifo_rx_thres_register."]
    #[inline(always)]
    pub fn rxfifo_start_addr(&self) -> RXFIFO_START_ADDR_R {
        RXFIFO_START_ADDR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - This is the offset address of the first receiving data as described in nonfifo_rx_thres_register."]
    #[inline(always)]
    pub fn rxfifo_end_addr(&self) -> RXFIFO_END_ADDR_R {
        RXFIFO_END_ADDR_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - This is the offset address of the first sending data as described in nonfifo_tx_thres register."]
    #[inline(always)]
    pub fn txfifo_start_addr(&self) -> TXFIFO_START_ADDR_R {
        TXFIFO_START_ADDR_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - This is the offset address of the last sending data as described in nonfifo_tx_thres register."]
    #[inline(always)]
    pub fn txfifo_end_addr(&self) -> TXFIFO_END_ADDR_R {
        TXFIFO_END_ADDR_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifo_st](index.html) module"]
pub struct RXFIFO_ST_SPEC;
impl crate::RegisterSpec for RXFIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfifo_st::R](R) reader structure"]
impl crate::Readable for RXFIFO_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFIFO_ST to value 0"]
impl crate::Resettable for RXFIFO_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
