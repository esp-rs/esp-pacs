#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `RXFIFO_WM_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_INT_ENA_R = crate::BitReader;
#[doc = "Field `RXFIFO_WM_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_WM_INT_ENA` reader - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_INT_ENA_R = crate::BitReader;
#[doc = "Field `TXFIFO_WM_INT_ENA` writer - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_OVF_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_DETECT_INT_ENA` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `END_DETECT_INT_ENA` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_TRANS_DONE_INT_ENA` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS_DONE_INT_ENA` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` reader - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_INT_ENA_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` writer - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_TXFIFO_UDF_INT_ENA` reader - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `MST_TXFIFO_UDF_INT_ENA` writer - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` reader - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_INT_ENA_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` writer - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_OUT_INT_ENA` reader - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `TIME_OUT_INT_ENA` writer - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_START_INT_ENA` reader - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `TRANS_START_INT_ENA` writer - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK_INT_ENA` reader - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_INT_ENA_R = crate::BitReader;
#[doc = "Field `NACK_INT_ENA` writer - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_OVF_INT_ENA` reader - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `TXFIFO_OVF_INT_ENA` writer - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_UDF_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `RXFIFO_UDF_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_ST_TO_INT_ENA` reader - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_INT_ENA_R = crate::BitReader;
#[doc = "Field `SCL_ST_TO_INT_ENA` writer - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_MAIN_ST_TO_INT_ENA` reader - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_INT_ENA_R = crate::BitReader;
#[doc = "Field `SCL_MAIN_ST_TO_INT_ENA` writer - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DET_START_INT_ENA` reader - The interrupt enable bit for I2C_DET_START_INT interrupt."]
pub type DET_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `DET_START_INT_ENA` writer - The interrupt enable bit for I2C_DET_START_INT interrupt."]
pub type DET_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm_int_ena(&self) -> RXFIFO_WM_INT_ENA_R {
        RXFIFO_WM_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm_int_ena(&self) -> TXFIFO_WM_INT_ENA_R {
        TXFIFO_WM_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&self) -> RXFIFO_OVF_INT_ENA_R {
        RXFIFO_OVF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect_int_ena(&self) -> END_DETECT_INT_ENA_R {
        END_DETECT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done_int_ena(&self) -> BYTE_TRANS_DONE_INT_ENA_R {
        BYTE_TRANS_DONE_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&self) -> ARBITRATION_LOST_INT_ENA_R {
        ARBITRATION_LOST_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_ena(&self) -> MST_TXFIFO_UDF_INT_ENA_R {
        MST_TXFIFO_UDF_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_ena(&self) -> TRANS_COMPLETE_INT_ENA_R {
        TRANS_COMPLETE_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out_int_ena(&self) -> TIME_OUT_INT_ENA_R {
        TIME_OUT_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start_int_ena(&self) -> TRANS_START_INT_ENA_R {
        TRANS_START_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack_int_ena(&self) -> NACK_INT_ENA_R {
        NACK_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf_int_ena(&self) -> TXFIFO_OVF_INT_ENA_R {
        TXFIFO_OVF_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf_int_ena(&self) -> RXFIFO_UDF_INT_ENA_R {
        RXFIFO_UDF_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to_int_ena(&self) -> SCL_ST_TO_INT_ENA_R {
        SCL_ST_TO_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to_int_ena(&self) -> SCL_MAIN_ST_TO_INT_ENA_R {
        SCL_MAIN_ST_TO_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start_int_ena(&self) -> DET_START_INT_ENA_R {
        DET_START_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "rxfifo_wm_int_ena",
                &format_args!("{}", self.rxfifo_wm_int_ena().bit()),
            )
            .field(
                "txfifo_wm_int_ena",
                &format_args!("{}", self.txfifo_wm_int_ena().bit()),
            )
            .field(
                "rxfifo_ovf_int_ena",
                &format_args!("{}", self.rxfifo_ovf_int_ena().bit()),
            )
            .field(
                "end_detect_int_ena",
                &format_args!("{}", self.end_detect_int_ena().bit()),
            )
            .field(
                "byte_trans_done_int_ena",
                &format_args!("{}", self.byte_trans_done_int_ena().bit()),
            )
            .field(
                "arbitration_lost_int_ena",
                &format_args!("{}", self.arbitration_lost_int_ena().bit()),
            )
            .field(
                "mst_txfifo_udf_int_ena",
                &format_args!("{}", self.mst_txfifo_udf_int_ena().bit()),
            )
            .field(
                "trans_complete_int_ena",
                &format_args!("{}", self.trans_complete_int_ena().bit()),
            )
            .field(
                "time_out_int_ena",
                &format_args!("{}", self.time_out_int_ena().bit()),
            )
            .field(
                "trans_start_int_ena",
                &format_args!("{}", self.trans_start_int_ena().bit()),
            )
            .field(
                "nack_int_ena",
                &format_args!("{}", self.nack_int_ena().bit()),
            )
            .field(
                "txfifo_ovf_int_ena",
                &format_args!("{}", self.txfifo_ovf_int_ena().bit()),
            )
            .field(
                "rxfifo_udf_int_ena",
                &format_args!("{}", self.rxfifo_udf_int_ena().bit()),
            )
            .field(
                "scl_st_to_int_ena",
                &format_args!("{}", self.scl_st_to_int_ena().bit()),
            )
            .field(
                "scl_main_st_to_int_ena",
                &format_args!("{}", self.scl_main_st_to_int_ena().bit()),
            )
            .field(
                "det_start_int_ena",
                &format_args!("{}", self.det_start_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_wm_int_ena(&mut self) -> RXFIFO_WM_INT_ENA_W<INT_ENA_SPEC> {
        RXFIFO_WM_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_wm_int_ena(&mut self) -> TXFIFO_WM_INT_ENA_W<INT_ENA_SPEC> {
        TXFIFO_WM_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf_int_ena(&mut self) -> RXFIFO_OVF_INT_ENA_W<INT_ENA_SPEC> {
        RXFIFO_OVF_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn end_detect_int_ena(&mut self) -> END_DETECT_INT_ENA_W<INT_ENA_SPEC> {
        END_DETECT_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn byte_trans_done_int_ena(&mut self) -> BYTE_TRANS_DONE_INT_ENA_W<INT_ENA_SPEC> {
        BYTE_TRANS_DONE_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost_int_ena(&mut self) -> ARBITRATION_LOST_INT_ENA_W<INT_ENA_SPEC> {
        ARBITRATION_LOST_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mst_txfifo_udf_int_ena(&mut self) -> MST_TXFIFO_UDF_INT_ENA_W<INT_ENA_SPEC> {
        MST_TXFIFO_UDF_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete_int_ena(&mut self) -> TRANS_COMPLETE_INT_ENA_W<INT_ENA_SPEC> {
        TRANS_COMPLETE_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn time_out_int_ena(&mut self) -> TIME_OUT_INT_ENA_W<INT_ENA_SPEC> {
        TIME_OUT_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_start_int_ena(&mut self) -> TRANS_START_INT_ENA_W<INT_ENA_SPEC> {
        TRANS_START_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn nack_int_ena(&mut self) -> NACK_INT_ENA_W<INT_ENA_SPEC> {
        NACK_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_ovf_int_ena(&mut self) -> TXFIFO_OVF_INT_ENA_W<INT_ENA_SPEC> {
        TXFIFO_OVF_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_udf_int_ena(&mut self) -> RXFIFO_UDF_INT_ENA_W<INT_ENA_SPEC> {
        RXFIFO_UDF_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn scl_st_to_int_ena(&mut self) -> SCL_ST_TO_INT_ENA_W<INT_ENA_SPEC> {
        SCL_ST_TO_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn scl_main_st_to_int_ena(&mut self) -> SCL_MAIN_ST_TO_INT_ENA_W<INT_ENA_SPEC> {
        SCL_MAIN_ST_TO_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for I2C_DET_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn det_start_int_ena(&mut self) -> DET_START_INT_ENA_W<INT_ENA_SPEC> {
        DET_START_INT_ENA_W::new(self, 15)
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
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
