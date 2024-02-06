#[doc = "Register `OUTFIFO_STATUS_CH0` reader"]
pub type R = crate::R<OUTFIFO_STATUS_CH0_SPEC>;
#[doc = "Field `OUTFIFO_FULL` reader - L1 Tx FIFO full signal for Tx channel 0."]
pub type OUTFIFO_FULL_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY` reader - L1 Tx FIFO empty signal for Tx channel 0."]
pub type OUTFIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT` reader - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0."]
pub type OUTFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `OUT_REMAIN_UNDER_1B` reader - reserved"]
pub type OUT_REMAIN_UNDER_1B_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_2B` reader - reserved"]
pub type OUT_REMAIN_UNDER_2B_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_3B` reader - reserved"]
pub type OUT_REMAIN_UNDER_3B_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_4B` reader - reserved"]
pub type OUT_REMAIN_UNDER_4B_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - L1 Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_full(&self) -> OUTFIFO_FULL_R {
        OUTFIFO_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L1 Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_empty(&self) -> OUTFIFO_EMPTY_R {
        OUTFIFO_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_cnt(&self) -> OUTFIFO_CNT_R {
        OUTFIFO_CNT_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_1b(&self) -> OUT_REMAIN_UNDER_1B_R {
        OUT_REMAIN_UNDER_1B_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_2b(&self) -> OUT_REMAIN_UNDER_2B_R {
        OUT_REMAIN_UNDER_2B_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_3b(&self) -> OUT_REMAIN_UNDER_3B_R {
        OUT_REMAIN_UNDER_3B_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_4b(&self) -> OUT_REMAIN_UNDER_4B_R {
        OUT_REMAIN_UNDER_4B_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS_CH0")
            .field(
                "outfifo_full",
                &format_args!("{}", self.outfifo_full().bit()),
            )
            .field(
                "outfifo_empty",
                &format_args!("{}", self.outfifo_empty().bit()),
            )
            .field(
                "outfifo_cnt",
                &format_args!("{}", self.outfifo_cnt().bits()),
            )
            .field(
                "out_remain_under_1b",
                &format_args!("{}", self.out_remain_under_1b().bit()),
            )
            .field(
                "out_remain_under_2b",
                &format_args!("{}", self.out_remain_under_2b().bit()),
            )
            .field(
                "out_remain_under_3b",
                &format_args!("{}", self.out_remain_under_3b().bit()),
            )
            .field(
                "out_remain_under_4b",
                &format_args!("{}", self.out_remain_under_4b().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUTFIFO_STATUS_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "DMA_OUTFIFO_STATUS_CH0_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTFIFO_STATUS_CH0_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_status_ch0::R`](R) reader structure"]
impl crate::Readable for OUTFIFO_STATUS_CH0_SPEC {}
#[doc = "`reset()` method sets OUTFIFO_STATUS_CH0 to value 0x0780_0002"]
impl crate::Resettable for OUTFIFO_STATUS_CH0_SPEC {
    const RESET_VALUE: u32 = 0x0780_0002;
}
