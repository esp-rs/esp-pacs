#[doc = "Register `I2C_INT_ENA` reader"]
pub type R = crate::R<I2C_INT_ENA_SPEC>;
#[doc = "Register `I2C_INT_ENA` writer"]
pub type W = crate::W<I2C_INT_ENA_SPEC>;
#[doc = "Field `I2C_RXFIFO_WM_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
pub type I2C_RXFIFO_WM_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_RXFIFO_WM_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
pub type I2C_RXFIFO_WM_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TXFIFO_WM_INT_ENA` reader - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
pub type I2C_TXFIFO_WM_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_TXFIFO_WM_INT_ENA` writer - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
pub type I2C_TXFIFO_WM_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_RXFIFO_OVF_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
pub type I2C_RXFIFO_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_RXFIFO_OVF_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
pub type I2C_RXFIFO_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_END_DETECT_INT_ENA` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type I2C_END_DETECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_END_DETECT_INT_ENA` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type I2C_END_DETECT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_BYTE_TRANS_DONE_INT_ENA` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type I2C_BYTE_TRANS_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_BYTE_TRANS_DONE_INT_ENA` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type I2C_BYTE_TRANS_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ARBITRATION_LOST_INT_ENA` reader - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub type I2C_ARBITRATION_LOST_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_ARBITRATION_LOST_INT_ENA` writer - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub type I2C_ARBITRATION_LOST_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_MST_TXFIFO_UDF_INT_ENA` reader - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub type I2C_MST_TXFIFO_UDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_MST_TXFIFO_UDF_INT_ENA` writer - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub type I2C_MST_TXFIFO_UDF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TRANS_COMPLETE_INT_ENA` reader - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub type I2C_TRANS_COMPLETE_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_TRANS_COMPLETE_INT_ENA` writer - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub type I2C_TRANS_COMPLETE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TIME_OUT_INT_ENA` reader - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
pub type I2C_TIME_OUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_TIME_OUT_INT_ENA` writer - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
pub type I2C_TIME_OUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TRANS_START_INT_ENA` reader - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
pub type I2C_TRANS_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_TRANS_START_INT_ENA` writer - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
pub type I2C_TRANS_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_NACK_INT_ENA` reader - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub type I2C_NACK_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_NACK_INT_ENA` writer - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub type I2C_NACK_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_TXFIFO_OVF_INT_ENA` reader - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
pub type I2C_TXFIFO_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_TXFIFO_OVF_INT_ENA` writer - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
pub type I2C_TXFIFO_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_RXFIFO_UDF_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
pub type I2C_RXFIFO_UDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_RXFIFO_UDF_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
pub type I2C_RXFIFO_UDF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_SCL_ST_TO_INT_ENA` reader - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
pub type I2C_SCL_ST_TO_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_SCL_ST_TO_INT_ENA` writer - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
pub type I2C_SCL_ST_TO_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_SCL_MAIN_ST_TO_INT_ENA` reader - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type I2C_SCL_MAIN_ST_TO_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_SCL_MAIN_ST_TO_INT_ENA` writer - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type I2C_SCL_MAIN_ST_TO_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_DET_START_INT_ENA` reader - The interrupt enable bit for I2C_DET_START_INT interrupt."]
pub type I2C_DET_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `I2C_DET_START_INT_ENA` writer - The interrupt enable bit for I2C_DET_START_INT interrupt."]
pub type I2C_DET_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn i2c_rxfifo_wm_int_ena(&self) -> I2C_RXFIFO_WM_INT_ENA_R {
        I2C_RXFIFO_WM_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn i2c_txfifo_wm_int_ena(&self) -> I2C_TXFIFO_WM_INT_ENA_R {
        I2C_TXFIFO_WM_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn i2c_rxfifo_ovf_int_ena(&self) -> I2C_RXFIFO_OVF_INT_ENA_R {
        I2C_RXFIFO_OVF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn i2c_end_detect_int_ena(&self) -> I2C_END_DETECT_INT_ENA_R {
        I2C_END_DETECT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn i2c_byte_trans_done_int_ena(&self) -> I2C_BYTE_TRANS_DONE_INT_ENA_R {
        I2C_BYTE_TRANS_DONE_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn i2c_arbitration_lost_int_ena(&self) -> I2C_ARBITRATION_LOST_INT_ENA_R {
        I2C_ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn i2c_mst_txfifo_udf_int_ena(&self) -> I2C_MST_TXFIFO_UDF_INT_ENA_R {
        I2C_MST_TXFIFO_UDF_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn i2c_trans_complete_int_ena(&self) -> I2C_TRANS_COMPLETE_INT_ENA_R {
        I2C_TRANS_COMPLETE_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn i2c_time_out_int_ena(&self) -> I2C_TIME_OUT_INT_ENA_R {
        I2C_TIME_OUT_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn i2c_trans_start_int_ena(&self) -> I2C_TRANS_START_INT_ENA_R {
        I2C_TRANS_START_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn i2c_nack_int_ena(&self) -> I2C_NACK_INT_ENA_R {
        I2C_NACK_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn i2c_txfifo_ovf_int_ena(&self) -> I2C_TXFIFO_OVF_INT_ENA_R {
        I2C_TXFIFO_OVF_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn i2c_rxfifo_udf_int_ena(&self) -> I2C_RXFIFO_UDF_INT_ENA_R {
        I2C_RXFIFO_UDF_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn i2c_scl_st_to_int_ena(&self) -> I2C_SCL_ST_TO_INT_ENA_R {
        I2C_SCL_ST_TO_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn i2c_scl_main_st_to_int_ena(&self) -> I2C_SCL_MAIN_ST_TO_INT_ENA_R {
        I2C_SCL_MAIN_ST_TO_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn i2c_det_start_int_ena(&self) -> I2C_DET_START_INT_ENA_R {
        I2C_DET_START_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_INT_ENA")
            .field(
                "i2c_rxfifo_wm_int_ena",
                &format_args!("{}", self.i2c_rxfifo_wm_int_ena().bit()),
            )
            .field(
                "i2c_txfifo_wm_int_ena",
                &format_args!("{}", self.i2c_txfifo_wm_int_ena().bit()),
            )
            .field(
                "i2c_rxfifo_ovf_int_ena",
                &format_args!("{}", self.i2c_rxfifo_ovf_int_ena().bit()),
            )
            .field(
                "i2c_end_detect_int_ena",
                &format_args!("{}", self.i2c_end_detect_int_ena().bit()),
            )
            .field(
                "i2c_byte_trans_done_int_ena",
                &format_args!("{}", self.i2c_byte_trans_done_int_ena().bit()),
            )
            .field(
                "i2c_arbitration_lost_int_ena",
                &format_args!("{}", self.i2c_arbitration_lost_int_ena().bit()),
            )
            .field(
                "i2c_mst_txfifo_udf_int_ena",
                &format_args!("{}", self.i2c_mst_txfifo_udf_int_ena().bit()),
            )
            .field(
                "i2c_trans_complete_int_ena",
                &format_args!("{}", self.i2c_trans_complete_int_ena().bit()),
            )
            .field(
                "i2c_time_out_int_ena",
                &format_args!("{}", self.i2c_time_out_int_ena().bit()),
            )
            .field(
                "i2c_trans_start_int_ena",
                &format_args!("{}", self.i2c_trans_start_int_ena().bit()),
            )
            .field(
                "i2c_nack_int_ena",
                &format_args!("{}", self.i2c_nack_int_ena().bit()),
            )
            .field(
                "i2c_txfifo_ovf_int_ena",
                &format_args!("{}", self.i2c_txfifo_ovf_int_ena().bit()),
            )
            .field(
                "i2c_rxfifo_udf_int_ena",
                &format_args!("{}", self.i2c_rxfifo_udf_int_ena().bit()),
            )
            .field(
                "i2c_scl_st_to_int_ena",
                &format_args!("{}", self.i2c_scl_st_to_int_ena().bit()),
            )
            .field(
                "i2c_scl_main_st_to_int_ena",
                &format_args!("{}", self.i2c_scl_main_st_to_int_ena().bit()),
            )
            .field(
                "i2c_det_start_int_ena",
                &format_args!("{}", self.i2c_det_start_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_rxfifo_wm_int_ena(&mut self) -> I2C_RXFIFO_WM_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_RXFIFO_WM_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_txfifo_wm_int_ena(&mut self) -> I2C_TXFIFO_WM_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_TXFIFO_WM_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_rxfifo_ovf_int_ena(&mut self) -> I2C_RXFIFO_OVF_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_RXFIFO_OVF_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_end_detect_int_ena(&mut self) -> I2C_END_DETECT_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_END_DETECT_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_byte_trans_done_int_ena(
        &mut self,
    ) -> I2C_BYTE_TRANS_DONE_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_BYTE_TRANS_DONE_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_arbitration_lost_int_ena(
        &mut self,
    ) -> I2C_ARBITRATION_LOST_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_ARBITRATION_LOST_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mst_txfifo_udf_int_ena(&mut self) -> I2C_MST_TXFIFO_UDF_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_MST_TXFIFO_UDF_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_trans_complete_int_ena(&mut self) -> I2C_TRANS_COMPLETE_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_TRANS_COMPLETE_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_time_out_int_ena(&mut self) -> I2C_TIME_OUT_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_TIME_OUT_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_trans_start_int_ena(&mut self) -> I2C_TRANS_START_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_TRANS_START_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_nack_int_ena(&mut self) -> I2C_NACK_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_NACK_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_txfifo_ovf_int_ena(&mut self) -> I2C_TXFIFO_OVF_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_TXFIFO_OVF_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_rxfifo_udf_int_ena(&mut self) -> I2C_RXFIFO_UDF_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_RXFIFO_UDF_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_st_to_int_ena(&mut self) -> I2C_SCL_ST_TO_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_SCL_ST_TO_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_main_st_to_int_ena(&mut self) -> I2C_SCL_MAIN_ST_TO_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_SCL_MAIN_ST_TO_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for I2C_DET_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_det_start_int_ena(&mut self) -> I2C_DET_START_INT_ENA_W<I2C_INT_ENA_SPEC> {
        I2C_DET_START_INT_ENA_W::new(self, 15)
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
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_INT_ENA_SPEC;
impl crate::RegisterSpec for I2C_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_int_ena::R`](R) reader structure"]
impl crate::Readable for I2C_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_int_ena::W`](W) writer structure"]
impl crate::Writable for I2C_INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_INT_ENA to value 0"]
impl crate::Resettable for I2C_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
