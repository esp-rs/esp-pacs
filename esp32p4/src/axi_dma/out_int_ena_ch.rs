#[doc = "Register `OUT_INT_ENA_CH%s` reader"]
pub type R = crate::R<OUT_INT_ENA_CH_SPEC>;
#[doc = "Register `OUT_INT_ENA_CH%s` writer"]
pub type W = crate::W<OUT_INT_ENA_CH_SPEC>;
#[doc = "Field `OUT_DONE_CH_INT_ENA` reader - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_DONE_CH_INT_ENA` writer - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF_CH_INT_ENA` reader - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_EOF_CH_INT_ENA` writer - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR_CH_INT_ENA` reader - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR_CH_INT_ENA` writer - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_ENA` reader - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_CH_INT_ENA` writer - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L1_OVF_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_L1_OVF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_OVF_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_L1_OVF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L1_UDF_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_L1_UDF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_UDF_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_L1_UDF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L2_OVF_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_OVF_L2_CH_INT interrupt."]
pub type OUTFIFO_L2_OVF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_OVF_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_OVF_L2_CH_INT interrupt."]
pub type OUTFIFO_L2_OVF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L2_UDF_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_UDF_L2_CH_INT interrupt."]
pub type OUTFIFO_L2_UDF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_UDF_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_UDF_L2_CH_INT interrupt."]
pub type OUTFIFO_L2_UDF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L3_OVF_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub type OUTFIFO_L3_OVF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_OVF_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub type OUTFIFO_L3_OVF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L3_UDF_CH_INT_ENA` reader - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub type OUTFIFO_L3_UDF_CH_INT_ENA_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_UDF_CH_INT_ENA` writer - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub type OUTFIFO_L3_UDF_CH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn outfifo_l1_ovf_ch_int_ena(&self) -> OUTFIFO_L1_OVF_CH_INT_ENA_R {
        OUTFIFO_L1_OVF_CH_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l1_udf_ch_int_ena(&self) -> OUTFIFO_L1_UDF_CH_INT_ENA_R {
        OUTFIFO_L1_UDF_CH_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUTFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l2_ovf_ch_int_ena(&self) -> OUTFIFO_L2_OVF_CH_INT_ENA_R {
        OUTFIFO_L2_OVF_CH_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the OUTFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l2_udf_ch_int_ena(&self) -> OUTFIFO_L2_UDF_CH_INT_ENA_R {
        OUTFIFO_L2_UDF_CH_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l3_ovf_ch_int_ena(&self) -> OUTFIFO_L3_OVF_CH_INT_ENA_R {
        OUTFIFO_L3_OVF_CH_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_l3_udf_ch_int_ena(&self) -> OUTFIFO_L3_UDF_CH_INT_ENA_R {
        OUTFIFO_L3_UDF_CH_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_INT_ENA_CH")
            .field(
                "out_done_ch_int_ena",
                &format_args!("{}", self.out_done_ch_int_ena().bit()),
            )
            .field(
                "out_eof_ch_int_ena",
                &format_args!("{}", self.out_eof_ch_int_ena().bit()),
            )
            .field(
                "out_dscr_err_ch_int_ena",
                &format_args!("{}", self.out_dscr_err_ch_int_ena().bit()),
            )
            .field(
                "out_total_eof_ch_int_ena",
                &format_args!("{}", self.out_total_eof_ch_int_ena().bit()),
            )
            .field(
                "outfifo_l1_ovf_ch_int_ena",
                &format_args!("{}", self.outfifo_l1_ovf_ch_int_ena().bit()),
            )
            .field(
                "outfifo_l1_udf_ch_int_ena",
                &format_args!("{}", self.outfifo_l1_udf_ch_int_ena().bit()),
            )
            .field(
                "outfifo_l2_ovf_ch_int_ena",
                &format_args!("{}", self.outfifo_l2_ovf_ch_int_ena().bit()),
            )
            .field(
                "outfifo_l2_udf_ch_int_ena",
                &format_args!("{}", self.outfifo_l2_udf_ch_int_ena().bit()),
            )
            .field(
                "outfifo_l3_ovf_ch_int_ena",
                &format_args!("{}", self.outfifo_l3_ovf_ch_int_ena().bit()),
            )
            .field(
                "outfifo_l3_udf_ch_int_ena",
                &format_args!("{}", self.outfifo_l3_udf_ch_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_INT_ENA_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_done_ch_int_ena(&mut self) -> OUT_DONE_CH_INT_ENA_W<OUT_INT_ENA_CH_SPEC> {
        OUT_DONE_CH_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_ch_int_ena(&mut self) -> OUT_EOF_CH_INT_ENA_W<OUT_INT_ENA_CH_SPEC> {
        OUT_EOF_CH_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err_ch_int_ena(&mut self) -> OUT_DSCR_ERR_CH_INT_ENA_W<OUT_INT_ENA_CH_SPEC> {
        OUT_DSCR_ERR_CH_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof_ch_int_ena(&mut self) -> OUT_TOTAL_EOF_CH_INT_ENA_W<OUT_INT_ENA_CH_SPEC> {
        OUT_TOTAL_EOF_CH_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l1_ovf_ch_int_ena(
        &mut self,
    ) -> OUTFIFO_L1_OVF_CH_INT_ENA_W<OUT_INT_ENA_CH_SPEC> {
        OUTFIFO_L1_OVF_CH_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l1_udf_ch_int_ena(
        &mut self,
    ) -> OUTFIFO_L1_UDF_CH_INT_ENA_W<OUT_INT_ENA_CH_SPEC> {
        OUTFIFO_L1_UDF_CH_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUTFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l2_ovf_ch_int_ena(
        &mut self,
    ) -> OUTFIFO_L2_OVF_CH_INT_ENA_W<OUT_INT_ENA_CH_SPEC> {
        OUTFIFO_L2_OVF_CH_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the OUTFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l2_udf_ch_int_ena(
        &mut self,
    ) -> OUTFIFO_L2_UDF_CH_INT_ENA_W<OUT_INT_ENA_CH_SPEC> {
        OUTFIFO_L2_UDF_CH_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l3_ovf_ch_int_ena(
        &mut self,
    ) -> OUTFIFO_L3_OVF_CH_INT_ENA_W<OUT_INT_ENA_CH_SPEC> {
        OUTFIFO_L3_OVF_CH_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l3_udf_ch_int_ena(
        &mut self,
    ) -> OUTFIFO_L3_UDF_CH_INT_ENA_W<OUT_INT_ENA_CH_SPEC> {
        OUTFIFO_L3_UDF_CH_INT_ENA_W::new(self, 9)
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
#[doc = "Interrupt enable bits of channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_int_ena_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_int_ena_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_INT_ENA_CH_SPEC;
impl crate::RegisterSpec for OUT_INT_ENA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_int_ena_ch::R`](R) reader structure"]
impl crate::Readable for OUT_INT_ENA_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_int_ena_ch::W`](W) writer structure"]
impl crate::Writable for OUT_INT_ENA_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_INT_ENA_CH%s to value 0"]
impl crate::Resettable for OUT_INT_ENA_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
