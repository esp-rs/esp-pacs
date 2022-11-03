#[doc = "Register `OUT_INT_ENA_CH%s` reader"]
pub struct R(crate::R<OUT_INT_ENA_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_INT_ENA_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_INT_ENA_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_INT_ENA_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_INT_ENA_CH%s` writer"]
pub struct W(crate::W<OUT_INT_ENA_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_INT_ENA_CH_SPEC>;
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
impl From<crate::W<OUT_INT_ENA_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_INT_ENA_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_DONE_CH_INT_ENA` reader - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DONE_CH_INT_ENA` writer - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_ENA_CH_SPEC, bool, O>;
#[doc = "Field `OUT_EOF_CH_INT_ENA` reader - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_CH_INT_ENA` writer - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_ENA_CH_SPEC, bool, O>;
#[doc = "Field `OUT_DSCR_ERR_CH_INT_ENA` reader - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DSCR_ERR_CH_INT_ENA` writer - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_ENA_CH_SPEC, bool, O>;
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_ENA` reader - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_ENA` writer - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_ENA_CH_SPEC, bool, O>;
#[doc = "Field `OUTFIFO_OVF_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_CH_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_OVF_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_CH_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_ENA_CH_SPEC, bool, O>;
#[doc = "Field `OUTFIFO_UDF_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_CH_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_UDF_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_CH_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OUT_INT_ENA_CH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done_ch_int_ena(&self) -> OUT_DONE_CH_INT_ENA_R {
        OUT_DONE_CH_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof_ch_int_ena(&self) -> OUT_EOF_CH_INT_ENA_R {
        OUT_EOF_CH_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err_ch_int_ena(&self) -> OUT_DSCR_ERR_CH_INT_ENA_R {
        OUT_DSCR_ERR_CH_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof_ch_int_ena(&self) -> OUT_TOTAL_EOF_CH_INT_ENA_R {
        OUT_TOTAL_EOF_CH_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_ch_int_ena(&self) -> OUTFIFO_OVF_CH_INT_ENA_R {
        OUTFIFO_OVF_CH_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_ch_int_ena(&self) -> OUTFIFO_UDF_CH_INT_ENA_R {
        OUTFIFO_UDF_CH_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_done_ch_int_ena(&mut self) -> OUT_DONE_CH_INT_ENA_W<0> {
        OUT_DONE_CH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_ch_int_ena(&mut self) -> OUT_EOF_CH_INT_ENA_W<1> {
        OUT_EOF_CH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err_ch_int_ena(&mut self) -> OUT_DSCR_ERR_CH_INT_ENA_W<2> {
        OUT_DSCR_ERR_CH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof_ch_int_ena(&mut self) -> OUT_TOTAL_EOF_CH_INT_ENA_W<3> {
        OUT_TOTAL_EOF_CH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf_ch_int_ena(&mut self) -> OUTFIFO_OVF_CH_INT_ENA_W<4> {
        OUTFIFO_OVF_CH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf_ch_int_ena(&mut self) -> OUTFIFO_UDF_CH_INT_ENA_W<5> {
        OUTFIFO_UDF_CH_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits of channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_int_ena_ch](index.html) module"]
pub struct OUT_INT_ENA_CH_SPEC;
impl crate::RegisterSpec for OUT_INT_ENA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_int_ena_ch::R](R) reader structure"]
impl crate::Readable for OUT_INT_ENA_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_int_ena_ch::W](W) writer structure"]
impl crate::Writable for OUT_INT_ENA_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_INT_ENA_CH%s to value 0"]
impl crate::Resettable for OUT_INT_ENA_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
