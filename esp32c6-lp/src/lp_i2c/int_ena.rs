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
pub type RXFIFO_WM_INT_ENA_R = crate::BitReader;
#[doc = "Field `RXFIFO_WM_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TXFIFO_WM_INT_ENA` reader - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_INT_ENA_R = crate::BitReader;
#[doc = "Field `TXFIFO_WM_INT_ENA` writer - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `RXFIFO_OVF_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `END_DETECT_INT_ENA` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `END_DETECT_INT_ENA` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `BYTE_TRANS_DONE_INT_ENA` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS_DONE_INT_ENA` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` reader - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_INT_ENA_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST_INT_ENA` writer - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `MST_TXFIFO_UDF_INT_ENA` reader - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `MST_TXFIFO_UDF_INT_ENA` writer - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` reader - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_INT_ENA_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE_INT_ENA` writer - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TIME_OUT_INT_ENA` reader - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `TIME_OUT_INT_ENA` writer - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TRANS_START_INT_ENA` reader - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `TRANS_START_INT_ENA` writer - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `NACK_INT_ENA` reader - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_INT_ENA_R = crate::BitReader;
#[doc = "Field `NACK_INT_ENA` writer - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TXFIFO_OVF_INT_ENA` reader - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `TXFIFO_OVF_INT_ENA` writer - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `RXFIFO_UDF_INT_ENA` reader - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `RXFIFO_UDF_INT_ENA` writer - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `SCL_ST_TO_INT_ENA` reader - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_INT_ENA_R = crate::BitReader;
#[doc = "Field `SCL_ST_TO_INT_ENA` writer - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `SCL_MAIN_ST_TO_INT_ENA` reader - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_INT_ENA_R = crate::BitReader;
#[doc = "Field `SCL_MAIN_ST_TO_INT_ENA` writer - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `DET_START_INT_ENA` reader - The interrupt enable bit for I2C_DET_START_INT interrupt."]
pub type DET_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `DET_START_INT_ENA` writer - The interrupt enable bit for I2C_DET_START_INT interrupt."]
pub type DET_START_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_wm_int_ena(&mut self) -> RXFIFO_WM_INT_ENA_W<0> {
        RXFIFO_WM_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_wm_int_ena(&mut self) -> TXFIFO_WM_INT_ENA_W<1> {
        TXFIFO_WM_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf_int_ena(&mut self) -> RXFIFO_OVF_INT_ENA_W<2> {
        RXFIFO_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn end_detect_int_ena(&mut self) -> END_DETECT_INT_ENA_W<3> {
        END_DETECT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn byte_trans_done_int_ena(&mut self) -> BYTE_TRANS_DONE_INT_ENA_W<4> {
        BYTE_TRANS_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost_int_ena(&mut self) -> ARBITRATION_LOST_INT_ENA_W<5> {
        ARBITRATION_LOST_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mst_txfifo_udf_int_ena(&mut self) -> MST_TXFIFO_UDF_INT_ENA_W<6> {
        MST_TXFIFO_UDF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete_int_ena(&mut self) -> TRANS_COMPLETE_INT_ENA_W<7> {
        TRANS_COMPLETE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn time_out_int_ena(&mut self) -> TIME_OUT_INT_ENA_W<8> {
        TIME_OUT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_start_int_ena(&mut self) -> TRANS_START_INT_ENA_W<9> {
        TRANS_START_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn nack_int_ena(&mut self) -> NACK_INT_ENA_W<10> {
        NACK_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_ovf_int_ena(&mut self) -> TXFIFO_OVF_INT_ENA_W<11> {
        TXFIFO_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_udf_int_ena(&mut self) -> RXFIFO_UDF_INT_ENA_W<12> {
        RXFIFO_UDF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn scl_st_to_int_ena(&mut self) -> SCL_ST_TO_INT_ENA_W<13> {
        SCL_ST_TO_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn scl_main_st_to_int_ena(&mut self) -> SCL_MAIN_ST_TO_INT_ENA_W<14> {
        SCL_MAIN_ST_TO_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - The interrupt enable bit for I2C_DET_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn det_start_int_ena(&mut self) -> DET_START_INT_ENA_W<15> {
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
