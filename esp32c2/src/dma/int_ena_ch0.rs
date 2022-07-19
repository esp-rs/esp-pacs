#[doc = "Register `INT_ENA_CH0` reader"]
pub struct R(crate::R<INT_ENA_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA_CH0` writer"]
pub struct W(crate::W<INT_ENA_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_CH0_SPEC>;
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
impl From<crate::W<INT_ENA_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IN_DONE_CH0_INT_ENA` reader - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `IN_DONE_CH0_INT_ENA` writer - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 0>;
#[doc = "Field `IN_SUC_EOF_CH0_INT_ENA` reader - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `IN_SUC_EOF_CH0_INT_ENA` writer - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 1>;
#[doc = "Field `IN_ERR_EOF_CH0_INT_ENA` reader - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `IN_ERR_EOF_CH0_INT_ENA` writer - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 2>;
#[doc = "Field `OUT_DONE_CH0_INT_ENA` reader - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DONE_CH0_INT_ENA` writer - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 3>;
#[doc = "Field `OUT_EOF_CH0_INT_ENA` reader - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_CH0_INT_ENA` writer - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 4>;
#[doc = "Field `IN_DSCR_ERR_CH0_INT_ENA` reader - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_ERR_CH0_INT_ENA` writer - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 5>;
#[doc = "Field `OUT_DSCR_ERR_CH0_INT_ENA` reader - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DSCR_ERR_CH0_INT_ENA` writer - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 6>;
#[doc = "Field `IN_DSCR_EMPTY_CH0_INT_ENA` reader - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `IN_DSCR_EMPTY_CH0_INT_ENA` writer - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 7>;
#[doc = "Field `OUT_TOTAL_EOF_CH0_INT_ENA` reader - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_TOTAL_EOF_CH0_INT_ENA` writer - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 8>;
#[doc = "Field `INFIFO_OVF_CH0_INT_ENA` reader - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_OVF_CH0_INT_ENA` writer - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 9>;
#[doc = "Field `INFIFO_UDF_CH0_INT_ENA` reader - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_UDF_CH0_INT_ENA` writer - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 10>;
#[doc = "Field `OUTFIFO_OVF_CH0_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_OVF_CH0_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 11>;
#[doc = "Field `OUTFIFO_UDF_CH0_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_CH0_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_UDF_CH0_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_CH0_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_CH0_SPEC, bool, 12>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done_ch0_int_ena(&self) -> IN_DONE_CH0_INT_ENA_R {
        IN_DONE_CH0_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_ch0_int_ena(&self) -> IN_SUC_EOF_CH0_INT_ENA_R {
        IN_SUC_EOF_CH0_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof_ch0_int_ena(&self) -> IN_ERR_EOF_CH0_INT_ENA_R {
        IN_ERR_EOF_CH0_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch0_int_ena(&self) -> OUT_DONE_CH0_INT_ENA_R {
        OUT_DONE_CH0_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch0_int_ena(&self) -> OUT_EOF_CH0_INT_ENA_R {
        OUT_EOF_CH0_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_ch0_int_ena(&self) -> IN_DSCR_ERR_CH0_INT_ENA_R {
        IN_DSCR_ERR_CH0_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch0_int_ena(&self) -> OUT_DSCR_ERR_CH0_INT_ENA_R {
        OUT_DSCR_ERR_CH0_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_ch0_int_ena(&self) -> IN_DSCR_EMPTY_CH0_INT_ENA_R {
        IN_DSCR_EMPTY_CH0_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch0_int_ena(&self) -> OUT_TOTAL_EOF_CH0_INT_ENA_R {
        OUT_TOTAL_EOF_CH0_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_ch0_int_ena(&self) -> INFIFO_OVF_CH0_INT_ENA_R {
        INFIFO_OVF_CH0_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_ch0_int_ena(&self) -> INFIFO_UDF_CH0_INT_ENA_R {
        INFIFO_UDF_CH0_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_ch0_int_ena(&self) -> OUTFIFO_OVF_CH0_INT_ENA_R {
        OUTFIFO_OVF_CH0_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_ch0_int_ena(&self) -> OUTFIFO_UDF_CH0_INT_ENA_R {
        OUTFIFO_UDF_CH0_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done_ch0_int_ena(&mut self) -> IN_DONE_CH0_INT_ENA_W {
        IN_DONE_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_ch0_int_ena(&mut self) -> IN_SUC_EOF_CH0_INT_ENA_W {
        IN_SUC_EOF_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof_ch0_int_ena(&mut self) -> IN_ERR_EOF_CH0_INT_ENA_W {
        IN_ERR_EOF_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch0_int_ena(&mut self) -> OUT_DONE_CH0_INT_ENA_W {
        OUT_DONE_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch0_int_ena(&mut self) -> OUT_EOF_CH0_INT_ENA_W {
        OUT_EOF_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_ch0_int_ena(&mut self) -> IN_DSCR_ERR_CH0_INT_ENA_W {
        IN_DSCR_ERR_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch0_int_ena(&mut self) -> OUT_DSCR_ERR_CH0_INT_ENA_W {
        OUT_DSCR_ERR_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_ch0_int_ena(&mut self) -> IN_DSCR_EMPTY_CH0_INT_ENA_W {
        IN_DSCR_EMPTY_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch0_int_ena(&mut self) -> OUT_TOTAL_EOF_CH0_INT_ENA_W {
        OUT_TOTAL_EOF_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_ch0_int_ena(&mut self) -> INFIFO_OVF_CH0_INT_ENA_W {
        INFIFO_OVF_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_ch0_int_ena(&mut self) -> INFIFO_UDF_CH0_INT_ENA_W {
        INFIFO_UDF_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_ch0_int_ena(&mut self) -> OUTFIFO_OVF_CH0_INT_ENA_W {
        OUTFIFO_OVF_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_ch0_int_ena(&mut self) -> OUTFIFO_UDF_CH0_INT_ENA_W {
        OUTFIFO_UDF_CH0_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_INT_ENA_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_ch0](index.html) module"]
pub struct INT_ENA_CH0_SPEC;
impl crate::RegisterSpec for INT_ENA_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena_ch0::R](R) reader structure"]
impl crate::Readable for INT_ENA_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena_ch0::W](W) writer structure"]
impl crate::Writable for INT_ENA_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA_CH0 to value 0"]
impl crate::Resettable for INT_ENA_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
