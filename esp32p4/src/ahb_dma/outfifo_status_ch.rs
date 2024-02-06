#[doc = "Register `OUTFIFO_STATUS_CH%s` reader"]
pub type R = crate::R<OUTFIFO_STATUS_CH_SPEC>;
#[doc = "Field `OUTFIFO_FULL_CH` reader - L1 Tx FIFO full signal for Tx channel 0."]
pub type OUTFIFO_FULL_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_CH` reader - L1 Tx FIFO empty signal for Tx channel 0."]
pub type OUTFIFO_EMPTY_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_CH` reader - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0."]
pub type OUTFIFO_CNT_CH_R = crate::FieldReader;
#[doc = "Field `OUT_REMAIN_UNDER_1B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_1B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_2B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_2B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_3B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_3B_CH_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_4B_CH` reader - reserved"]
pub type OUT_REMAIN_UNDER_4B_CH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - L1 Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_full_ch(&self) -> OUTFIFO_FULL_CH_R {
        OUTFIFO_FULL_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L1 Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_empty_ch(&self) -> OUTFIFO_EMPTY_CH_R {
        OUTFIFO_EMPTY_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn outfifo_cnt_ch(&self) -> OUTFIFO_CNT_CH_R {
        OUTFIFO_CNT_CH_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_1b_ch(&self) -> OUT_REMAIN_UNDER_1B_CH_R {
        OUT_REMAIN_UNDER_1B_CH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_2b_ch(&self) -> OUT_REMAIN_UNDER_2B_CH_R {
        OUT_REMAIN_UNDER_2B_CH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_3b_ch(&self) -> OUT_REMAIN_UNDER_3B_CH_R {
        OUT_REMAIN_UNDER_3B_CH_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_4b_ch(&self) -> OUT_REMAIN_UNDER_4B_CH_R {
        OUT_REMAIN_UNDER_4B_CH_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS_CH")
            .field(
                "outfifo_full_ch",
                &format_args!("{}", self.outfifo_full_ch().bit()),
            )
            .field(
                "outfifo_empty_ch",
                &format_args!("{}", self.outfifo_empty_ch().bit()),
            )
            .field(
                "outfifo_cnt_ch",
                &format_args!("{}", self.outfifo_cnt_ch().bits()),
            )
            .field(
                "out_remain_under_1b_ch",
                &format_args!("{}", self.out_remain_under_1b_ch().bit()),
            )
            .field(
                "out_remain_under_2b_ch",
                &format_args!("{}", self.out_remain_under_2b_ch().bit()),
            )
            .field(
                "out_remain_under_3b_ch",
                &format_args!("{}", self.out_remain_under_3b_ch().bit()),
            )
            .field(
                "out_remain_under_4b_ch",
                &format_args!("{}", self.out_remain_under_4b_ch().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUTFIFO_STATUS_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Transmit FIFO status of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTFIFO_STATUS_CH_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_status_ch::R`](R) reader structure"]
impl crate::Readable for OUTFIFO_STATUS_CH_SPEC {}
#[doc = "`reset()` method sets OUTFIFO_STATUS_CH%s to value 0x0780_0002"]
impl crate::Resettable for OUTFIFO_STATUS_CH_SPEC {
    const RESET_VALUE: u32 = 0x0780_0002;
}
