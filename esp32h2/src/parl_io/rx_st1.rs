#[doc = "Register `RX_ST1` reader"]
pub struct R(crate::R<RX_ST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_FIFO_RD_BIT_CNT` reader - Indicates the current read bit number from Rx FIFO."]
pub type RX_FIFO_RD_BIT_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 13:31 - Indicates the current read bit number from Rx FIFO."]
    #[inline(always)]
    pub fn rx_fifo_rd_bit_cnt(&self) -> RX_FIFO_RD_BIT_CNT_R {
        RX_FIFO_RD_BIT_CNT_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ST1")
            .field(
                "rx_fifo_rd_bit_cnt",
                &format_args!("{}", self.rx_fifo_rd_bit_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_ST1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Parallel IO RX status register1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_st1](index.html) module"]
pub struct RX_ST1_SPEC;
impl crate::RegisterSpec for RX_ST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_st1::R](R) reader structure"]
impl crate::Readable for RX_ST1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_ST1 to value 0"]
impl crate::Resettable for RX_ST1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
