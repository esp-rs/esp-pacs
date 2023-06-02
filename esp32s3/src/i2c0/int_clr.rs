#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
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
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_WM_INT_CLR` writer - Set this bit to clear I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TXFIFO_WM_INT_CLR` writer - Set this bit to clear I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `RXFIFO_OVF_INT_CLR` writer - Set this bit to clear I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `END_DETECT_INT_CLR` writer - Set this bit to clear the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `BYTE_TRANS_DONE_INT_CLR` writer - Set this bit to clear the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `ARBITRATION_LOST_INT_CLR` writer - Set this bit to clear the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `MST_TXFIFO_UDF_INT_CLR` writer - Set this bit to clear I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TRANS_COMPLETE_INT_CLR` writer - Set this bit to clear the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TIME_OUT_INT_CLR` writer - Set this bit to clear the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TRANS_START_INT_CLR` writer - Set this bit to clear the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `NACK_INT_CLR` writer - Set this bit to clear I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `TXFIFO_OVF_INT_CLR` writer - Set this bit to clear I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `RXFIFO_UDF_INT_CLR` writer - Set this bit to clear I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `SCL_ST_TO_INT_CLR` writer - Set this bit to clear I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `SCL_MAIN_ST_TO_INT_CLR` writer - Set this bit to clear I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `DET_START_INT_CLR` writer - Set this bit to clear I2C_DET_START_INT interrupt."]
pub type DET_START_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `SLAVE_STRETCH_INT_CLR` writer - Set this bit to clear I2C_SLAVE_STRETCH_INT interrupt."]
pub type SLAVE_STRETCH_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `GENERAL_CALL_INT_CLR` writer - Set this bit for I2C_GENARAL_CALL_INT interrupt."]
pub type GENERAL_CALL_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_wm_int_clr(&mut self) -> RXFIFO_WM_INT_CLR_W<0> {
        RXFIFO_WM_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_wm_int_clr(&mut self) -> TXFIFO_WM_INT_CLR_W<1> {
        TXFIFO_WM_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf_int_clr(&mut self) -> RXFIFO_OVF_INT_CLR_W<2> {
        RXFIFO_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn end_detect_int_clr(&mut self) -> END_DETECT_INT_CLR_W<3> {
        END_DETECT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn byte_trans_done_int_clr(&mut self) -> BYTE_TRANS_DONE_INT_CLR_W<4> {
        BYTE_TRANS_DONE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost_int_clr(&mut self) -> ARBITRATION_LOST_INT_CLR_W<5> {
        ARBITRATION_LOST_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mst_txfifo_udf_int_clr(&mut self) -> MST_TXFIFO_UDF_INT_CLR_W<6> {
        MST_TXFIFO_UDF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_complete_int_clr(&mut self) -> TRANS_COMPLETE_INT_CLR_W<7> {
        TRANS_COMPLETE_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn time_out_int_clr(&mut self) -> TIME_OUT_INT_CLR_W<8> {
        TIME_OUT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_start_int_clr(&mut self) -> TRANS_START_INT_CLR_W<9> {
        TRANS_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn nack_int_clr(&mut self) -> NACK_INT_CLR_W<10> {
        NACK_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_ovf_int_clr(&mut self) -> TXFIFO_OVF_INT_CLR_W<11> {
        TXFIFO_OVF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_udf_int_clr(&mut self) -> RXFIFO_UDF_INT_CLR_W<12> {
        RXFIFO_UDF_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn scl_st_to_int_clr(&mut self) -> SCL_ST_TO_INT_CLR_W<13> {
        SCL_ST_TO_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn scl_main_st_to_int_clr(&mut self) -> SCL_MAIN_ST_TO_INT_CLR_W<14> {
        SCL_MAIN_ST_TO_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to clear I2C_DET_START_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn det_start_int_clr(&mut self) -> DET_START_INT_CLR_W<15> {
        DET_START_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to clear I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slave_stretch_int_clr(&mut self) -> SLAVE_STRETCH_INT_CLR_W<16> {
        SLAVE_STRETCH_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit for I2C_GENARAL_CALL_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn general_call_int_clr(&mut self) -> GENERAL_CALL_INT_CLR_W<17> {
        GENERAL_CALL_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
