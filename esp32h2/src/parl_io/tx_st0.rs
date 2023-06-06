#[doc = "Register `TX_ST0` reader"]
pub struct R(crate::R<TX_ST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_ST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_ST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_ST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TX_CNT` reader - Indicates the cycle number of reading Tx FIFO."]
pub type TX_CNT_R = crate::FieldReader;
#[doc = "Field `TX_FIFO_RD_BIT_CNT` reader - Indicates the current read bit number from Tx FIFO."]
pub type TX_FIFO_RD_BIT_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 6:12 - Indicates the cycle number of reading Tx FIFO."]
    #[inline(always)]
    pub fn tx_cnt(&self) -> TX_CNT_R {
        TX_CNT_R::new(((self.bits >> 6) & 0x7f) as u8)
    }
    #[doc = "Bits 13:31 - Indicates the current read bit number from Tx FIFO."]
    #[inline(always)]
    pub fn tx_fifo_rd_bit_cnt(&self) -> TX_FIFO_RD_BIT_CNT_R {
        TX_FIFO_RD_BIT_CNT_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_ST0")
            .field("tx_cnt", &format_args!("{}", self.tx_cnt().bits()))
            .field(
                "tx_fifo_rd_bit_cnt",
                &format_args!("{}", self.tx_fifo_rd_bit_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_ST0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Parallel IO TX status register0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_st0](index.html) module"]
pub struct TX_ST0_SPEC;
impl crate::RegisterSpec for TX_ST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_st0::R](R) reader structure"]
impl crate::Readable for TX_ST0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_ST0 to value 0"]
impl crate::Resettable for TX_ST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
