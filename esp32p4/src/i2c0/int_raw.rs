#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `RXFIFO_WM` reader - The raw interrupt status of I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_R = crate::BitReader;
#[doc = "Field `TXFIFO_WM` reader - The raw interrupt status of I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` reader - The raw interrupt status of I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `END_DETECT` reader - The raw interrupt status of the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS_DONE` reader - The raw interrupt status of the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST` reader - The raw interrupt status of the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_R = crate::BitReader;
#[doc = "Field `MST_TXFIFO_UDF` reader - The raw interrupt status of I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` reader - The raw interrupt status of the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `TIME_OUT` reader - The raw interrupt status of the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `TRANS_START` reader - The raw interrupt status of the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_R = crate::BitReader;
#[doc = "Field `NACK` reader - The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_R = crate::BitReader;
#[doc = "Field `TXFIFO_OVF` reader - The raw interrupt status of I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `RXFIFO_UDF` reader - The raw interrupt status of I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_R = crate::BitReader;
#[doc = "Field `SCL_ST_TO` reader - The raw interrupt status of I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_R = crate::BitReader;
#[doc = "Field `SCL_MAIN_ST_TO` reader - The raw interrupt status of I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_R = crate::BitReader;
#[doc = "Field `DET_START` reader - The raw interrupt status of I2C_DET_START_INT interrupt."]
pub type DET_START_R = crate::BitReader;
#[doc = "Field `SLAVE_STRETCH` reader - The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt."]
pub type SLAVE_STRETCH_R = crate::BitReader;
#[doc = "Field `GENERAL_CALL` reader - The raw interrupt status of I2C_GENARAL_CALL_INT interrupt."]
pub type GENERAL_CALL_R = crate::BitReader;
#[doc = "Field `SLAVE_ADDR_UNMATCH` reader - The raw interrupt status of I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt."]
pub type SLAVE_ADDR_UNMATCH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm(&self) -> RXFIFO_WM_R {
        RXFIFO_WM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm(&self) -> TXFIFO_WM_R {
        TXFIFO_WM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RXFIFO_OVF_R {
        RXFIFO_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status of the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect(&self) -> END_DETECT_R {
        END_DETECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status of the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done(&self) -> BYTE_TRANS_DONE_R {
        BYTE_TRANS_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status of the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status of I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf(&self) -> MST_TXFIFO_UDF_R {
        MST_TXFIFO_UDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status of the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status of the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status of the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status of I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf(&self) -> TXFIFO_OVF_R {
        TXFIFO_OVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt status of I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf(&self) -> RXFIFO_UDF_R {
        RXFIFO_UDF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt status of I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to(&self) -> SCL_ST_TO_R {
        SCL_ST_TO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt status of I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to(&self) -> SCL_MAIN_ST_TO_R {
        SCL_MAIN_ST_TO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw interrupt status of I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start(&self) -> DET_START_R {
        DET_START_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The raw interrupt status of I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_stretch(&self) -> SLAVE_STRETCH_R {
        SLAVE_STRETCH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The raw interrupt status of I2C_GENARAL_CALL_INT interrupt."]
    #[inline(always)]
    pub fn general_call(&self) -> GENERAL_CALL_R {
        GENERAL_CALL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The raw interrupt status of I2C_SLAVE_ADDR_UNMATCH_INT_RAW interrupt."]
    #[inline(always)]
    pub fn slave_addr_unmatch(&self) -> SLAVE_ADDR_UNMATCH_R {
        SLAVE_ADDR_UNMATCH_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("rxfifo_wm", &self.rxfifo_wm().bit())
            .field("txfifo_wm", &self.txfifo_wm().bit())
            .field("rxfifo_ovf", &self.rxfifo_ovf().bit())
            .field("end_detect", &self.end_detect().bit())
            .field("byte_trans_done", &self.byte_trans_done().bit())
            .field("arbitration_lost", &self.arbitration_lost().bit())
            .field("mst_txfifo_udf", &self.mst_txfifo_udf().bit())
            .field("trans_complete", &self.trans_complete().bit())
            .field("time_out", &self.time_out().bit())
            .field("trans_start", &self.trans_start().bit())
            .field("nack", &self.nack().bit())
            .field("txfifo_ovf", &self.txfifo_ovf().bit())
            .field("rxfifo_udf", &self.rxfifo_udf().bit())
            .field("scl_st_to", &self.scl_st_to().bit())
            .field("scl_main_st_to", &self.scl_main_st_to().bit())
            .field("det_start", &self.det_start().bit())
            .field("slave_stretch", &self.slave_stretch().bit())
            .field("general_call", &self.general_call().bit())
            .field("slave_addr_unmatch", &self.slave_addr_unmatch().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0x02"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
