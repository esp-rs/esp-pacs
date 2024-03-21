#[doc = "Register `ENA` reader"]
pub type R = crate::R<ENA_SPEC>;
#[doc = "Register `ENA` writer"]
pub type W = crate::W<ENA_SPEC>;
#[doc = "Field `OUT_DONE` reader - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_DONE` writer - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF` reader - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `OUT_EOF` writer - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR` reader - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` writer - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF` reader - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` writer - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L1` reader - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_L1_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L1` writer - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L1` reader - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_L1_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L1` writer - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L3` reader - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub type OUTFIFO_OVF_L3_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L3` writer - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub type OUTFIFO_OVF_L3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L3` reader - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub type OUTFIFO_UDF_L3_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L3` writer - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub type OUTFIFO_UDF_L3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l1(&self) -> OUTFIFO_OVF_L1_R {
        OUTFIFO_OVF_L1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l1(&self) -> OUTFIFO_UDF_L1_R {
        OUTFIFO_UDF_L1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l3(&self) -> OUTFIFO_OVF_L3_R {
        OUTFIFO_OVF_L3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l3(&self) -> OUTFIFO_UDF_L3_R {
        OUTFIFO_UDF_L3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENA")
            .field("out_done", &format_args!("{}", self.out_done().bit()))
            .field("out_eof", &format_args!("{}", self.out_eof().bit()))
            .field(
                "out_dscr_err",
                &format_args!("{}", self.out_dscr_err().bit()),
            )
            .field(
                "out_total_eof",
                &format_args!("{}", self.out_total_eof().bit()),
            )
            .field(
                "outfifo_ovf_l1",
                &format_args!("{}", self.outfifo_ovf_l1().bit()),
            )
            .field(
                "outfifo_udf_l1",
                &format_args!("{}", self.outfifo_udf_l1().bit()),
            )
            .field(
                "outfifo_ovf_l3",
                &format_args!("{}", self.outfifo_ovf_l3().bit()),
            )
            .field(
                "outfifo_udf_l3",
                &format_args!("{}", self.outfifo_udf_l3().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_done(&mut self) -> OUT_DONE_W<ENA_SPEC> {
        OUT_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<ENA_SPEC> {
        OUT_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<ENA_SPEC> {
        OUT_DSCR_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<ENA_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf_l1(&mut self) -> OUTFIFO_OVF_L1_W<ENA_SPEC> {
        OUTFIFO_OVF_L1_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf_l1(&mut self) -> OUTFIFO_UDF_L1_W<ENA_SPEC> {
        OUTFIFO_UDF_L1_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf_l3(&mut self) -> OUTFIFO_OVF_L3_W<ENA_SPEC> {
        OUTFIFO_OVF_L3_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf_l3(&mut self) -> OUTFIFO_UDF_L3_W<ENA_SPEC> {
        OUTFIFO_UDF_L3_W::new(self, 7)
    }
}
#[doc = "Interrupt enable bits of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENA_SPEC;
impl crate::RegisterSpec for ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ena::R`](R) reader structure"]
impl crate::Readable for ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ena::W`](W) writer structure"]
impl crate::Writable for ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENA to value 0"]
impl crate::Resettable for ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
