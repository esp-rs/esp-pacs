#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_WM_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFO_WM_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 0>;
#[doc = "Field `TXFIFO_WM_INT_ENA` reader - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFO_WM_INT_ENA` writer - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 1>;
#[doc = "Field `RXFIFO_OVF_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFO_OVF_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 2>;
#[doc = "Field `END_DETECT_INT_ENA` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `END_DETECT_INT_ENA` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 3>;
#[doc = "Field `BYTE_TRANS_DONE_INT_ENA` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `BYTE_TRANS_DONE_INT_ENA` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 4>;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` reader - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` writer - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 5>;
#[doc = "Field `MST_TXFIFO_UDF_INT_ENA` reader - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `MST_TXFIFO_UDF_INT_ENA` writer - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 6>;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` reader - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` writer - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 7>;
#[doc = "Field `TIME_OUT_INT_ENA` reader - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIME_OUT_INT_ENA` writer - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 8>;
#[doc = "Field `TRANS_START_INT_ENA` reader - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TRANS_START_INT_ENA` writer - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 9>;
#[doc = "Field `NACK_INT_ENA` reader - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `NACK_INT_ENA` writer - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 10>;
#[doc = "Field `TXFIFO_OVF_INT_ENA` reader - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFO_OVF_INT_ENA` writer - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 11>;
#[doc = "Field `RXFIFO_UDF_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFO_UDF_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 12>;
#[doc = "Field `SCL_ST_TO_INT_ENA` reader - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SCL_ST_TO_INT_ENA` writer - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 13>;
#[doc = "Field `SCL_MAIN_ST_TO_INT_ENA` reader - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SCL_MAIN_ST_TO_INT_ENA` writer - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 14>;
#[doc = "Field `DET_START_INT_ENA` reader - The interrupt enable bit for I2C_DET_START_INT interrupt."]
pub type DET_START_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `DET_START_INT_ENA` writer - The interrupt enable bit for I2C_DET_START_INT interrupt."]
pub type DET_START_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 15>;
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
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm_int_ena(&mut self) -> RXFIFO_WM_INT_ENA_W {
        RXFIFO_WM_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm_int_ena(&mut self) -> TXFIFO_WM_INT_ENA_W {
        TXFIFO_WM_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&mut self) -> RXFIFO_OVF_INT_ENA_W {
        RXFIFO_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect_int_ena(&mut self) -> END_DETECT_INT_ENA_W {
        END_DETECT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done_int_ena(&mut self) -> BYTE_TRANS_DONE_INT_ENA_W {
        BYTE_TRANS_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost_int_ena(&mut self) -> ARBITRATION_LOST_INT_ENA_W {
        ARBITRATION_LOST_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf_int_ena(&mut self) -> MST_TXFIFO_UDF_INT_ENA_W {
        MST_TXFIFO_UDF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete_int_ena(&mut self) -> TRANS_COMPLETE_INT_ENA_W {
        TRANS_COMPLETE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out_int_ena(&mut self) -> TIME_OUT_INT_ENA_W {
        TIME_OUT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start_int_ena(&mut self) -> TRANS_START_INT_ENA_W {
        TRANS_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack_int_ena(&mut self) -> NACK_INT_ENA_W {
        NACK_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf_int_ena(&mut self) -> TXFIFO_OVF_INT_ENA_W {
        TXFIFO_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf_int_ena(&mut self) -> RXFIFO_UDF_INT_ENA_W {
        RXFIFO_UDF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to_int_ena(&mut self) -> SCL_ST_TO_INT_ENA_W {
        SCL_ST_TO_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to_int_ena(&mut self) -> SCL_MAIN_ST_TO_INT_ENA_W {
        SCL_MAIN_ST_TO_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - The interrupt enable bit for I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start_int_ena(&mut self) -> DET_START_INT_ENA_W {
        DET_START_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
