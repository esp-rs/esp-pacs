#[doc = "Register `I2C_INT_CLR` writer"]
pub type W = crate::W<I2C_INT_CLR_SPEC>;
#[doc = "Field `I2C_RXFIFO_WM_INT_CLR` writer - Set this bit to clear I2C_RXFIFO_WM_INT interrupt."]
pub type I2C_RXFIFO_WM_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TXFIFO_WM_INT_CLR` writer - Set this bit to clear I2C_TXFIFO_WM_INT interrupt."]
pub type I2C_TXFIFO_WM_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_RXFIFO_OVF_INT_CLR` writer - Set this bit to clear I2C_RXFIFO_OVF_INT interrupt."]
pub type I2C_RXFIFO_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_END_DETECT_INT_CLR` writer - Set this bit to clear the I2C_END_DETECT_INT interrupt."]
pub type I2C_END_DETECT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_BYTE_TRANS_DONE_INT_CLR` writer - Set this bit to clear the I2C_END_DETECT_INT interrupt."]
pub type I2C_BYTE_TRANS_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ARBITRATION_LOST_INT_CLR` writer - Set this bit to clear the I2C_ARBITRATION_LOST_INT interrupt."]
pub type I2C_ARBITRATION_LOST_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_MST_TXFIFO_UDF_INT_CLR` writer - Set this bit to clear I2C_TRANS_COMPLETE_INT interrupt."]
pub type I2C_MST_TXFIFO_UDF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TRANS_COMPLETE_INT_CLR` writer - Set this bit to clear the I2C_TRANS_COMPLETE_INT interrupt."]
pub type I2C_TRANS_COMPLETE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TIME_OUT_INT_CLR` writer - Set this bit to clear the I2C_TIME_OUT_INT interrupt."]
pub type I2C_TIME_OUT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TRANS_START_INT_CLR` writer - Set this bit to clear the I2C_TRANS_START_INT interrupt."]
pub type I2C_TRANS_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_NACK_INT_CLR` writer - Set this bit to clear I2C_SLAVE_STRETCH_INT interrupt."]
pub type I2C_NACK_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TXFIFO_OVF_INT_CLR` writer - Set this bit to clear I2C_TXFIFO_OVF_INT interrupt."]
pub type I2C_TXFIFO_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_RXFIFO_UDF_INT_CLR` writer - Set this bit to clear I2C_RXFIFO_UDF_INT interrupt."]
pub type I2C_RXFIFO_UDF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_SCL_ST_TO_INT_CLR` writer - Set this bit to clear I2C_SCL_ST_TO_INT interrupt."]
pub type I2C_SCL_ST_TO_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_SCL_MAIN_ST_TO_INT_CLR` writer - Set this bit to clear I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type I2C_SCL_MAIN_ST_TO_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_DET_START_INT_CLR` writer - Set this bit to clear I2C_DET_START_INT interrupt."]
pub type I2C_DET_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_rxfifo_wm_int_clr(&mut self) -> I2C_RXFIFO_WM_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_RXFIFO_WM_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_txfifo_wm_int_clr(&mut self) -> I2C_TXFIFO_WM_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_TXFIFO_WM_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_rxfifo_ovf_int_clr(&mut self) -> I2C_RXFIFO_OVF_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_RXFIFO_OVF_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_end_detect_int_clr(&mut self) -> I2C_END_DETECT_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_END_DETECT_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_byte_trans_done_int_clr(
        &mut self,
    ) -> I2C_BYTE_TRANS_DONE_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_BYTE_TRANS_DONE_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_arbitration_lost_int_clr(
        &mut self,
    ) -> I2C_ARBITRATION_LOST_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_ARBITRATION_LOST_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mst_txfifo_udf_int_clr(&mut self) -> I2C_MST_TXFIFO_UDF_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_MST_TXFIFO_UDF_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_trans_complete_int_clr(&mut self) -> I2C_TRANS_COMPLETE_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_TRANS_COMPLETE_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_time_out_int_clr(&mut self) -> I2C_TIME_OUT_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_TIME_OUT_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_trans_start_int_clr(&mut self) -> I2C_TRANS_START_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_TRANS_START_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_nack_int_clr(&mut self) -> I2C_NACK_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_NACK_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to clear I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_txfifo_ovf_int_clr(&mut self) -> I2C_TXFIFO_OVF_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_TXFIFO_OVF_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to clear I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_rxfifo_udf_int_clr(&mut self) -> I2C_RXFIFO_UDF_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_RXFIFO_UDF_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to clear I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_st_to_int_clr(&mut self) -> I2C_SCL_ST_TO_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_SCL_ST_TO_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to clear I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_main_st_to_int_clr(&mut self) -> I2C_SCL_MAIN_ST_TO_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_SCL_MAIN_ST_TO_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to clear I2C_DET_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_det_start_int_clr(&mut self) -> I2C_DET_START_INT_CLR_W<I2C_INT_CLR_SPEC> {
        I2C_DET_START_INT_CLR_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_INT_CLR_SPEC;
impl crate::RegisterSpec for I2C_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2c_int_clr::W`](W) writer structure"]
impl crate::Writable for I2C_INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_INT_CLR to value 0"]
impl crate::Resettable for I2C_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
