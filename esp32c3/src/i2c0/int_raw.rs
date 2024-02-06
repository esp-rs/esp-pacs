#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `RXFIFO_WM_INT_RAW` reader - reg_rxfifo_wm_int_raw"]
pub type RXFIFO_WM_INT_RAW_R = crate::BitReader;
#[doc = "Field `TXFIFO_WM_INT_RAW` reader - reg_txfifo_wm_int_raw"]
pub type TXFIFO_WM_INT_RAW_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF_INT_RAW` reader - reg_rxfifo_ovf_int_raw"]
pub type RXFIFO_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `END_DETECT_INT_RAW` reader - reg_end_detect_int_raw"]
pub type END_DETECT_INT_RAW_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS_DONE_INT_RAW` reader - reg_byte_trans_done_int_raw"]
pub type BYTE_TRANS_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_RAW` reader - reg_arbitration_lost_int_raw"]
pub type ARBITRATION_LOST_INT_RAW_R = crate::BitReader;
#[doc = "Field `MST_TXFIFO_UDF_INT_RAW` reader - reg_mst_txfifo_udf_int_raw"]
pub type MST_TXFIFO_UDF_INT_RAW_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE_INT_RAW` reader - reg_trans_complete_int_raw"]
pub type TRANS_COMPLETE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TIME_OUT_INT_RAW` reader - reg_time_out_int_raw"]
pub type TIME_OUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `TRANS_START_INT_RAW` reader - reg_trans_start_int_raw"]
pub type TRANS_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `NACK_INT_RAW` reader - reg_nack_int_raw"]
pub type NACK_INT_RAW_R = crate::BitReader;
#[doc = "Field `TXFIFO_OVF_INT_RAW` reader - reg_txfifo_ovf_int_raw"]
pub type TXFIFO_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `RXFIFO_UDF_INT_RAW` reader - reg_rxfifo_udf_int_raw"]
pub type RXFIFO_UDF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SCL_ST_TO_INT_RAW` reader - reg_scl_st_to_int_raw"]
pub type SCL_ST_TO_INT_RAW_R = crate::BitReader;
#[doc = "Field `SCL_MAIN_ST_TO_INT_RAW` reader - reg_scl_main_st_to_int_raw"]
pub type SCL_MAIN_ST_TO_INT_RAW_R = crate::BitReader;
#[doc = "Field `DET_START_INT_RAW` reader - reg_det_start_int_raw"]
pub type DET_START_INT_RAW_R = crate::BitReader;
#[doc = "Field `SLAVE_STRETCH_INT_RAW` reader - reg_slave_stretch_int_raw"]
pub type SLAVE_STRETCH_INT_RAW_R = crate::BitReader;
#[doc = "Field `GENERAL_CALL_INT_RAW` reader - reg_general_call_int_raw"]
pub type GENERAL_CALL_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - reg_rxfifo_wm_int_raw"]
    #[inline(always)]
    pub fn rxfifo_wm_int_raw(&self) -> RXFIFO_WM_INT_RAW_R {
        RXFIFO_WM_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_txfifo_wm_int_raw"]
    #[inline(always)]
    pub fn txfifo_wm_int_raw(&self) -> TXFIFO_WM_INT_RAW_R {
        TXFIFO_WM_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_rxfifo_ovf_int_raw"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> RXFIFO_OVF_INT_RAW_R {
        RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_end_detect_int_raw"]
    #[inline(always)]
    pub fn end_detect_int_raw(&self) -> END_DETECT_INT_RAW_R {
        END_DETECT_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_byte_trans_done_int_raw"]
    #[inline(always)]
    pub fn byte_trans_done_int_raw(&self) -> BYTE_TRANS_DONE_INT_RAW_R {
        BYTE_TRANS_DONE_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_arbitration_lost_int_raw"]
    #[inline(always)]
    pub fn arbitration_lost_int_raw(&self) -> ARBITRATION_LOST_INT_RAW_R {
        ARBITRATION_LOST_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_mst_txfifo_udf_int_raw"]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_raw(&self) -> MST_TXFIFO_UDF_INT_RAW_R {
        MST_TXFIFO_UDF_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_trans_complete_int_raw"]
    #[inline(always)]
    pub fn trans_complete_int_raw(&self) -> TRANS_COMPLETE_INT_RAW_R {
        TRANS_COMPLETE_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_time_out_int_raw"]
    #[inline(always)]
    pub fn time_out_int_raw(&self) -> TIME_OUT_INT_RAW_R {
        TIME_OUT_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_trans_start_int_raw"]
    #[inline(always)]
    pub fn trans_start_int_raw(&self) -> TRANS_START_INT_RAW_R {
        TRANS_START_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_nack_int_raw"]
    #[inline(always)]
    pub fn nack_int_raw(&self) -> NACK_INT_RAW_R {
        NACK_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_txfifo_ovf_int_raw"]
    #[inline(always)]
    pub fn txfifo_ovf_int_raw(&self) -> TXFIFO_OVF_INT_RAW_R {
        TXFIFO_OVF_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_rxfifo_udf_int_raw"]
    #[inline(always)]
    pub fn rxfifo_udf_int_raw(&self) -> RXFIFO_UDF_INT_RAW_R {
        RXFIFO_UDF_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_scl_st_to_int_raw"]
    #[inline(always)]
    pub fn scl_st_to_int_raw(&self) -> SCL_ST_TO_INT_RAW_R {
        SCL_ST_TO_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reg_scl_main_st_to_int_raw"]
    #[inline(always)]
    pub fn scl_main_st_to_int_raw(&self) -> SCL_MAIN_ST_TO_INT_RAW_R {
        SCL_MAIN_ST_TO_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - reg_det_start_int_raw"]
    #[inline(always)]
    pub fn det_start_int_raw(&self) -> DET_START_INT_RAW_R {
        DET_START_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - reg_slave_stretch_int_raw"]
    #[inline(always)]
    pub fn slave_stretch_int_raw(&self) -> SLAVE_STRETCH_INT_RAW_R {
        SLAVE_STRETCH_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reg_general_call_int_raw"]
    #[inline(always)]
    pub fn general_call_int_raw(&self) -> GENERAL_CALL_INT_RAW_R {
        GENERAL_CALL_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "rxfifo_wm_int_raw",
                &format_args!("{}", self.rxfifo_wm_int_raw().bit()),
            )
            .field(
                "txfifo_wm_int_raw",
                &format_args!("{}", self.txfifo_wm_int_raw().bit()),
            )
            .field(
                "rxfifo_ovf_int_raw",
                &format_args!("{}", self.rxfifo_ovf_int_raw().bit()),
            )
            .field(
                "end_detect_int_raw",
                &format_args!("{}", self.end_detect_int_raw().bit()),
            )
            .field(
                "byte_trans_done_int_raw",
                &format_args!("{}", self.byte_trans_done_int_raw().bit()),
            )
            .field(
                "arbitration_lost_int_raw",
                &format_args!("{}", self.arbitration_lost_int_raw().bit()),
            )
            .field(
                "mst_txfifo_udf_int_raw",
                &format_args!("{}", self.mst_txfifo_udf_int_raw().bit()),
            )
            .field(
                "trans_complete_int_raw",
                &format_args!("{}", self.trans_complete_int_raw().bit()),
            )
            .field(
                "time_out_int_raw",
                &format_args!("{}", self.time_out_int_raw().bit()),
            )
            .field(
                "trans_start_int_raw",
                &format_args!("{}", self.trans_start_int_raw().bit()),
            )
            .field(
                "nack_int_raw",
                &format_args!("{}", self.nack_int_raw().bit()),
            )
            .field(
                "txfifo_ovf_int_raw",
                &format_args!("{}", self.txfifo_ovf_int_raw().bit()),
            )
            .field(
                "rxfifo_udf_int_raw",
                &format_args!("{}", self.rxfifo_udf_int_raw().bit()),
            )
            .field(
                "scl_st_to_int_raw",
                &format_args!("{}", self.scl_st_to_int_raw().bit()),
            )
            .field(
                "scl_main_st_to_int_raw",
                &format_args!("{}", self.scl_main_st_to_int_raw().bit()),
            )
            .field(
                "det_start_int_raw",
                &format_args!("{}", self.det_start_int_raw().bit()),
            )
            .field(
                "slave_stretch_int_raw",
                &format_args!("{}", self.slave_stretch_int_raw().bit()),
            )
            .field(
                "general_call_int_raw",
                &format_args!("{}", self.general_call_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "I2C_INT_RAW_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
