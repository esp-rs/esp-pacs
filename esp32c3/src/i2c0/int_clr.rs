#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `RXFIFO_WM_INT_CLR` writer - reg_rxfifo_wm_int_clr"]
pub type RXFIFO_WM_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_WM_INT_CLR` writer - reg_txfifo_wm_int_clr"]
pub type TXFIFO_WM_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_OVF_INT_CLR` writer - reg_rxfifo_ovf_int_clr"]
pub type RXFIFO_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_DETECT_INT_CLR` writer - reg_end_detect_int_clr"]
pub type END_DETECT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_TRANS_DONE_INT_CLR` writer - reg_byte_trans_done_int_clr"]
pub type BYTE_TRANS_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBITRATION_LOST_INT_CLR` writer - reg_arbitration_lost_int_clr"]
pub type ARBITRATION_LOST_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_TXFIFO_UDF_INT_CLR` writer - reg_mst_txfifo_udf_int_clr"]
pub type MST_TXFIFO_UDF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_COMPLETE_INT_CLR` writer - reg_trans_complete_int_clr"]
pub type TRANS_COMPLETE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_OUT_INT_CLR` writer - reg_time_out_int_clr"]
pub type TIME_OUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_START_INT_CLR` writer - reg_trans_start_int_clr"]
pub type TRANS_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK_INT_CLR` writer - reg_nack_int_clr"]
pub type NACK_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_OVF_INT_CLR` writer - reg_txfifo_ovf_int_clr"]
pub type TXFIFO_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_UDF_INT_CLR` writer - reg_rxfifo_udf_int_clr"]
pub type RXFIFO_UDF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_ST_TO_INT_CLR` writer - reg_scl_st_to_int_clr"]
pub type SCL_ST_TO_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_MAIN_ST_TO_INT_CLR` writer - reg_scl_main_st_to_int_clr"]
pub type SCL_MAIN_ST_TO_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DET_START_INT_CLR` writer - reg_det_start_int_clr"]
pub type DET_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_STRETCH_INT_CLR` writer - reg_slave_stretch_int_clr"]
pub type SLAVE_STRETCH_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENERAL_CALL_INT_CLR` writer - reg_general_call_int_clr"]
pub type GENERAL_CALL_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - reg_rxfifo_wm_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_wm_int_clr(&mut self) -> RXFIFO_WM_INT_CLR_W<INT_CLR_SPEC> {
        RXFIFO_WM_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_txfifo_wm_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_wm_int_clr(&mut self) -> TXFIFO_WM_INT_CLR_W<INT_CLR_SPEC> {
        TXFIFO_WM_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - reg_rxfifo_ovf_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf_int_clr(&mut self) -> RXFIFO_OVF_INT_CLR_W<INT_CLR_SPEC> {
        RXFIFO_OVF_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - reg_end_detect_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn end_detect_int_clr(&mut self) -> END_DETECT_INT_CLR_W<INT_CLR_SPEC> {
        END_DETECT_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - reg_byte_trans_done_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn byte_trans_done_int_clr(&mut self) -> BYTE_TRANS_DONE_INT_CLR_W<INT_CLR_SPEC> {
        BYTE_TRANS_DONE_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - reg_arbitration_lost_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost_int_clr(&mut self) -> ARBITRATION_LOST_INT_CLR_W<INT_CLR_SPEC> {
        ARBITRATION_LOST_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - reg_mst_txfifo_udf_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn mst_txfifo_udf_int_clr(&mut self) -> MST_TXFIFO_UDF_INT_CLR_W<INT_CLR_SPEC> {
        MST_TXFIFO_UDF_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - reg_trans_complete_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete_int_clr(&mut self) -> TRANS_COMPLETE_INT_CLR_W<INT_CLR_SPEC> {
        TRANS_COMPLETE_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - reg_time_out_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn time_out_int_clr(&mut self) -> TIME_OUT_INT_CLR_W<INT_CLR_SPEC> {
        TIME_OUT_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - reg_trans_start_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn trans_start_int_clr(&mut self) -> TRANS_START_INT_CLR_W<INT_CLR_SPEC> {
        TRANS_START_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - reg_nack_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn nack_int_clr(&mut self) -> NACK_INT_CLR_W<INT_CLR_SPEC> {
        NACK_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - reg_txfifo_ovf_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_ovf_int_clr(&mut self) -> TXFIFO_OVF_INT_CLR_W<INT_CLR_SPEC> {
        TXFIFO_OVF_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - reg_rxfifo_udf_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_udf_int_clr(&mut self) -> RXFIFO_UDF_INT_CLR_W<INT_CLR_SPEC> {
        RXFIFO_UDF_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - reg_scl_st_to_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn scl_st_to_int_clr(&mut self) -> SCL_ST_TO_INT_CLR_W<INT_CLR_SPEC> {
        SCL_ST_TO_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - reg_scl_main_st_to_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn scl_main_st_to_int_clr(&mut self) -> SCL_MAIN_ST_TO_INT_CLR_W<INT_CLR_SPEC> {
        SCL_MAIN_ST_TO_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - reg_det_start_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn det_start_int_clr(&mut self) -> DET_START_INT_CLR_W<INT_CLR_SPEC> {
        DET_START_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16 - reg_slave_stretch_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn slave_stretch_int_clr(&mut self) -> SLAVE_STRETCH_INT_CLR_W<INT_CLR_SPEC> {
        SLAVE_STRETCH_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - reg_general_call_int_clr"]
    #[inline(always)]
    #[must_use]
    pub fn general_call_int_clr(&mut self) -> GENERAL_CALL_INT_CLR_W<INT_CLR_SPEC> {
        GENERAL_CALL_INT_CLR_W::new(self, 17)
    }
}
#[doc = "I2C_INT_CLR_REG\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
