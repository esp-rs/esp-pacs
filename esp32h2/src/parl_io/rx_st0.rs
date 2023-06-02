#[doc = "Register `RX_ST0` reader"]
pub struct R(crate::R<RX_ST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_ST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_ST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_ST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_CNT` reader - Indicates the cycle number of reading Rx FIFO."]
pub type RX_CNT_R = crate::FieldReader;
#[doc = "Field `RX_FIFO_WR_BIT_CNT` reader - Indicates the current written bit number into Rx FIFO."]
pub type RX_FIFO_WR_BIT_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 9:12 - Indicates the cycle number of reading Rx FIFO."]
    #[inline(always)]
    pub fn rx_cnt(&self) -> RX_CNT_R {
        RX_CNT_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:31 - Indicates the current written bit number into Rx FIFO."]
    #[inline(always)]
    pub fn rx_fifo_wr_bit_cnt(&self) -> RX_FIFO_WR_BIT_CNT_R {
        RX_FIFO_WR_BIT_CNT_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ST0")
            .field("rx_cnt", &format_args!("{}", self.rx_cnt().bits()))
            .field(
                "rx_fifo_wr_bit_cnt",
                &format_args!("{}", self.rx_fifo_wr_bit_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_ST0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Parallel IO RX status register0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_st0](index.html) module"]
pub struct RX_ST0_SPEC;
impl crate::RegisterSpec for RX_ST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_st0::R](R) reader structure"]
impl crate::Readable for RX_ST0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RX_ST0 to value 0"]
impl crate::Resettable for RX_ST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
