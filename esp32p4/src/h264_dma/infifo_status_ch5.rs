#[doc = "Register `INFIFO_STATUS_CH5` reader"]
pub type R = crate::R<INFIFO_STATUS_CH5_SPEC>;
#[doc = "Field `INFIFO_FULL_L1_CH5` reader - Tx FIFO full signal for Tx channel 1."]
pub type INFIFO_FULL_L1_CH5_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L1_CH5` reader - Tx FIFO empty signal for Tx channel 1."]
pub type INFIFO_EMPTY_L1_CH5_R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L1_CH5` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 1."]
pub type INFIFO_CNT_L1_CH5_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Tx FIFO full signal for Tx channel 1."]
    #[inline(always)]
    pub fn infifo_full_l1_ch5(&self) -> INFIFO_FULL_L1_CH5_R {
        INFIFO_FULL_L1_CH5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx FIFO empty signal for Tx channel 1."]
    #[inline(always)]
    pub fn infifo_empty_l1_ch5(&self) -> INFIFO_EMPTY_L1_CH5_R {
        INFIFO_EMPTY_L1_CH5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - The register stores the byte number of the data in Tx FIFO for Tx channel 1."]
    #[inline(always)]
    pub fn infifo_cnt_l1_ch5(&self) -> INFIFO_CNT_L1_CH5_R {
        INFIFO_CNT_L1_CH5_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_STATUS_CH5")
            .field(
                "infifo_full_l1_ch5",
                &format_args!("{}", self.infifo_full_l1_ch5().bit()),
            )
            .field(
                "infifo_empty_l1_ch5",
                &format_args!("{}", self.infifo_empty_l1_ch5().bit()),
            )
            .field(
                "infifo_cnt_l1_ch5",
                &format_args!("{}", self.infifo_cnt_l1_ch5().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INFIFO_STATUS_CH5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RX CH5 INFIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFIFO_STATUS_CH5_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS_CH5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_status_ch5::R`](R) reader structure"]
impl crate::Readable for INFIFO_STATUS_CH5_SPEC {}
#[doc = "`reset()` method sets INFIFO_STATUS_CH5 to value 0x02"]
impl crate::Resettable for INFIFO_STATUS_CH5_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
