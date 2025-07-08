#[doc = "Register `OUTFIFO_STATUS_CH0` reader"]
pub type R = crate::R<OUTFIFO_STATUS_CH0_SPEC>;
#[doc = "Field `OUTFIFO_FULL_CH0` reader - Represents whether L1 TX FIFO is full.\\\\0: Not Full\\\\1: Full\\\\"]
pub type OUTFIFO_FULL_CH0_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_CH0` reader - Represents whether L1 TX FIFO is empty.\\\\0: Not empty\\\\1: Empty\\\\"]
pub type OUTFIFO_EMPTY_CH0_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_CH0` reader - Represents the number of data bytes in L1 TX FIFO for TX channel 0"]
pub type OUTFIFO_CNT_CH0_R = crate::FieldReader;
#[doc = "Field `OUT_REMAIN_UNDER_1B_CH0` reader - reserved"]
pub type OUT_REMAIN_UNDER_1B_CH0_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_2B_CH0` reader - reserved"]
pub type OUT_REMAIN_UNDER_2B_CH0_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_3B_CH0` reader - reserved"]
pub type OUT_REMAIN_UNDER_3B_CH0_R = crate::BitReader;
#[doc = "Field `OUT_REMAIN_UNDER_4B_CH0` reader - reserved"]
pub type OUT_REMAIN_UNDER_4B_CH0_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents whether L1 TX FIFO is full.\\\\0: Not Full\\\\1: Full\\\\"]
    #[inline(always)]
    pub fn outfifo_full_ch0(&self) -> OUTFIFO_FULL_CH0_R {
        OUTFIFO_FULL_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents whether L1 TX FIFO is empty.\\\\0: Not empty\\\\1: Empty\\\\"]
    #[inline(always)]
    pub fn outfifo_empty_ch0(&self) -> OUTFIFO_EMPTY_CH0_R {
        OUTFIFO_EMPTY_CH0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Represents the number of data bytes in L1 TX FIFO for TX channel 0"]
    #[inline(always)]
    pub fn outfifo_cnt_ch0(&self) -> OUTFIFO_CNT_CH0_R {
        OUTFIFO_CNT_CH0_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_1b_ch0(&self) -> OUT_REMAIN_UNDER_1B_CH0_R {
        OUT_REMAIN_UNDER_1B_CH0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_2b_ch0(&self) -> OUT_REMAIN_UNDER_2B_CH0_R {
        OUT_REMAIN_UNDER_2B_CH0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_3b_ch0(&self) -> OUT_REMAIN_UNDER_3B_CH0_R {
        OUT_REMAIN_UNDER_3B_CH0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - reserved"]
    #[inline(always)]
    pub fn out_remain_under_4b_ch0(&self) -> OUT_REMAIN_UNDER_4B_CH0_R {
        OUT_REMAIN_UNDER_4B_CH0_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS_CH0")
            .field("outfifo_full_ch0", &self.outfifo_full_ch0())
            .field("outfifo_empty_ch0", &self.outfifo_empty_ch0())
            .field("outfifo_cnt_ch0", &self.outfifo_cnt_ch0())
            .field("out_remain_under_1b_ch0", &self.out_remain_under_1b_ch0())
            .field("out_remain_under_2b_ch0", &self.out_remain_under_2b_ch0())
            .field("out_remain_under_3b_ch0", &self.out_remain_under_3b_ch0())
            .field("out_remain_under_4b_ch0", &self.out_remain_under_4b_ch0())
            .finish()
    }
}
#[doc = "Receive FIFO status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status_ch0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
