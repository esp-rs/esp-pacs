#[doc = "Register `OUTFIFO_STATUS_CH4` reader"]
pub type R = crate::R<OUTFIFO_STATUS_CH4_SPEC>;
#[doc = "Field `OUTFIFO_FULL_L2_CH4` reader - Tx FIFO full signal for Tx channel 2."]
pub type OUTFIFO_FULL_L2_CH4_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_L2_CH4` reader - Tx FIFO empty signal for Tx channel 2."]
pub type OUTFIFO_EMPTY_L2_CH4_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_L2_CH4` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 2."]
pub type OUTFIFO_CNT_L2_CH4_R = crate::FieldReader;
#[doc = "Field `OUTFIFO_FULL_L1_CH4` reader - Tx FIFO full signal for Tx channel 2."]
pub type OUTFIFO_FULL_L1_CH4_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_L1_CH4` reader - Tx FIFO empty signal for Tx channel 2."]
pub type OUTFIFO_EMPTY_L1_CH4_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_L1_CH4` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 2."]
pub type OUTFIFO_CNT_L1_CH4_R = crate::FieldReader;
#[doc = "Field `OUTFIFO_FULL_L3_CH4` reader - Tx FIFO full signal for Tx channel 2."]
pub type OUTFIFO_FULL_L3_CH4_R = crate::BitReader;
#[doc = "Field `OUTFIFO_EMPTY_L3_CH4` reader - Tx FIFO empty signal for Tx channel 2."]
pub type OUTFIFO_EMPTY_L3_CH4_R = crate::BitReader;
#[doc = "Field `OUTFIFO_CNT_L3_CH4` reader - The register stores the byte number of the data in Tx FIFO for Tx channel 2."]
pub type OUTFIFO_CNT_L3_CH4_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Tx FIFO full signal for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_full_l2_ch4(&self) -> OUTFIFO_FULL_L2_CH4_R {
        OUTFIFO_FULL_L2_CH4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx FIFO empty signal for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_empty_l2_ch4(&self) -> OUTFIFO_EMPTY_L2_CH4_R {
        OUTFIFO_EMPTY_L2_CH4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The register stores the byte number of the data in Tx FIFO for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_cnt_l2_ch4(&self) -> OUTFIFO_CNT_L2_CH4_R {
        OUTFIFO_CNT_L2_CH4_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Tx FIFO full signal for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_full_l1_ch4(&self) -> OUTFIFO_FULL_L1_CH4_R {
        OUTFIFO_FULL_L1_CH4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx FIFO empty signal for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_empty_l1_ch4(&self) -> OUTFIFO_EMPTY_L1_CH4_R {
        OUTFIFO_EMPTY_L1_CH4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - The register stores the byte number of the data in Tx FIFO for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_cnt_l1_ch4(&self) -> OUTFIFO_CNT_L1_CH4_R {
        OUTFIFO_CNT_L1_CH4_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Tx FIFO full signal for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_full_l3_ch4(&self) -> OUTFIFO_FULL_L3_CH4_R {
        OUTFIFO_FULL_L3_CH4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tx FIFO empty signal for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_empty_l3_ch4(&self) -> OUTFIFO_EMPTY_L3_CH4_R {
        OUTFIFO_EMPTY_L3_CH4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - The register stores the byte number of the data in Tx FIFO for Tx channel 2."]
    #[inline(always)]
    pub fn outfifo_cnt_l3_ch4(&self) -> OUTFIFO_CNT_L3_CH4_R {
        OUTFIFO_CNT_L3_CH4_R::new(((self.bits >> 18) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFIFO_STATUS_CH4")
            .field(
                "outfifo_full_l2_ch4",
                &format_args!("{}", self.outfifo_full_l2_ch4().bit()),
            )
            .field(
                "outfifo_empty_l2_ch4",
                &format_args!("{}", self.outfifo_empty_l2_ch4().bit()),
            )
            .field(
                "outfifo_cnt_l2_ch4",
                &format_args!("{}", self.outfifo_cnt_l2_ch4().bits()),
            )
            .field(
                "outfifo_full_l1_ch4",
                &format_args!("{}", self.outfifo_full_l1_ch4().bit()),
            )
            .field(
                "outfifo_empty_l1_ch4",
                &format_args!("{}", self.outfifo_empty_l1_ch4().bit()),
            )
            .field(
                "outfifo_cnt_l1_ch4",
                &format_args!("{}", self.outfifo_cnt_l1_ch4().bits()),
            )
            .field(
                "outfifo_full_l3_ch4",
                &format_args!("{}", self.outfifo_full_l3_ch4().bit()),
            )
            .field(
                "outfifo_empty_l3_ch4",
                &format_args!("{}", self.outfifo_empty_l3_ch4().bit()),
            )
            .field(
                "outfifo_cnt_l3_ch4",
                &format_args!("{}", self.outfifo_cnt_l3_ch4().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUTFIFO_STATUS_CH4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "TX CH4 outfifo status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outfifo_status_ch4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUTFIFO_STATUS_CH4_SPEC;
impl crate::RegisterSpec for OUTFIFO_STATUS_CH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_status_ch4::R`](R) reader structure"]
impl crate::Readable for OUTFIFO_STATUS_CH4_SPEC {}
#[doc = "`reset()` method sets OUTFIFO_STATUS_CH4 to value 0x0002_0082"]
impl crate::Resettable for OUTFIFO_STATUS_CH4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0082;
}
