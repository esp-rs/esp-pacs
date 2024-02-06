#[doc = "Register `INFIFO_STATUS_CH0` reader"]
pub type R = crate::R<INFIFO_STATUS_CH0_SPEC>;
#[doc = "Field `INFIFO_FULL_L2_CH0` reader - Rx FIFO full signal for Rx channel."]
pub type INFIFO_FULL_L2_CH0_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L2_CH0` reader - Rx FIFO empty signal for Rx channel."]
pub type INFIFO_EMPTY_L2_CH0_R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L2_CH0` reader - The register stores the byte number of the data in Rx FIFO for Rx channel."]
pub type INFIFO_CNT_L2_CH0_R = crate::FieldReader;
#[doc = "Field `INFIFO_FULL_L1_CH0` reader - Tx FIFO full signal for Tx channel 0."]
pub type INFIFO_FULL_L1_CH0_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L1_CH0` reader - Tx FIFO empty signal for Tx channel 0."]
pub type INFIFO_EMPTY_L1_CH0_R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L1_CH0` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
pub type INFIFO_CNT_L1_CH0_R = crate::FieldReader;
#[doc = "Field `INFIFO_FULL_L3_CH0` reader - Tx FIFO full signal for Tx channel 0."]
pub type INFIFO_FULL_L3_CH0_R = crate::BitReader;
#[doc = "Field `INFIFO_EMPTY_L3_CH0` reader - Tx FIFO empty signal for Tx channel 0."]
pub type INFIFO_EMPTY_L3_CH0_R = crate::BitReader;
#[doc = "Field `INFIFO_CNT_L3_CH0` reader - The register stores the 8byte number of the data in Tx FIFO for Tx channel 0."]
pub type INFIFO_CNT_L3_CH0_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Rx FIFO full signal for Rx channel."]
    #[inline(always)]
    pub fn infifo_full_l2_ch0(&self) -> INFIFO_FULL_L2_CH0_R {
        INFIFO_FULL_L2_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO empty signal for Rx channel."]
    #[inline(always)]
    pub fn infifo_empty_l2_ch0(&self) -> INFIFO_EMPTY_L2_CH0_R {
        INFIFO_EMPTY_L2_CH0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The register stores the byte number of the data in Rx FIFO for Rx channel."]
    #[inline(always)]
    pub fn infifo_cnt_l2_ch0(&self) -> INFIFO_CNT_L2_CH0_R {
        INFIFO_CNT_L2_CH0_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l1_ch0(&self) -> INFIFO_FULL_L1_CH0_R {
        INFIFO_FULL_L1_CH0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l1_ch0(&self) -> INFIFO_EMPTY_L1_CH0_R {
        INFIFO_EMPTY_L1_CH0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - The register stores the byte number of the data in Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l1_ch0(&self) -> INFIFO_CNT_L1_CH0_R {
        INFIFO_CNT_L1_CH0_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Tx FIFO full signal for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_full_l3_ch0(&self) -> INFIFO_FULL_L3_CH0_R {
        INFIFO_FULL_L3_CH0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tx FIFO empty signal for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_empty_l3_ch0(&self) -> INFIFO_EMPTY_L3_CH0_R {
        INFIFO_EMPTY_L3_CH0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - The register stores the 8byte number of the data in Tx FIFO for Tx channel 0."]
    #[inline(always)]
    pub fn infifo_cnt_l3_ch0(&self) -> INFIFO_CNT_L3_CH0_R {
        INFIFO_CNT_L3_CH0_R::new(((self.bits >> 18) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFIFO_STATUS_CH0")
            .field(
                "infifo_full_l2_ch0",
                &format_args!("{}", self.infifo_full_l2_ch0().bit()),
            )
            .field(
                "infifo_empty_l2_ch0",
                &format_args!("{}", self.infifo_empty_l2_ch0().bit()),
            )
            .field(
                "infifo_cnt_l2_ch0",
                &format_args!("{}", self.infifo_cnt_l2_ch0().bits()),
            )
            .field(
                "infifo_full_l1_ch0",
                &format_args!("{}", self.infifo_full_l1_ch0().bit()),
            )
            .field(
                "infifo_empty_l1_ch0",
                &format_args!("{}", self.infifo_empty_l1_ch0().bit()),
            )
            .field(
                "infifo_cnt_l1_ch0",
                &format_args!("{}", self.infifo_cnt_l1_ch0().bits()),
            )
            .field(
                "infifo_full_l3_ch0",
                &format_args!("{}", self.infifo_full_l3_ch0().bit()),
            )
            .field(
                "infifo_empty_l3_ch0",
                &format_args!("{}", self.infifo_empty_l3_ch0().bit()),
            )
            .field(
                "infifo_cnt_l3_ch0",
                &format_args!("{}", self.infifo_cnt_l3_ch0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INFIFO_STATUS_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RX CH0 INFIFO status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infifo_status_ch0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFIFO_STATUS_CH0_SPEC;
impl crate::RegisterSpec for INFIFO_STATUS_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_status_ch0::R`](R) reader structure"]
impl crate::Readable for INFIFO_STATUS_CH0_SPEC {}
#[doc = "`reset()` method sets INFIFO_STATUS_CH0 to value 0x0002_0082"]
impl crate::Resettable for INFIFO_STATUS_CH0_SPEC {
    const RESET_VALUE: u32 = 0x0002_0082;
}
