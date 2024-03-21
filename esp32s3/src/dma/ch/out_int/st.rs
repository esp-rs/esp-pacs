#[doc = "Register `ST` reader"]
pub type R = crate::R<ST_SPEC>;
#[doc = "Field `OUT_DONE` reader - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_EOF` reader - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` reader - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` reader - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L1` reader - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_L1_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L1` reader - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_L1_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L3` reader - The raw interrupt status bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub type OUTFIFO_OVF_L3_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L3` reader - The raw interrupt status bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub type OUTFIFO_UDF_L3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status bit for the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l1(&self) -> OUTFIFO_OVF_L1_R {
        OUTFIFO_OVF_L1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status bit for the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l1(&self) -> OUTFIFO_UDF_L1_R {
        OUTFIFO_UDF_L1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status bit for the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l3(&self) -> OUTFIFO_OVF_L3_R {
        OUTFIFO_OVF_L3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status bit for the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l3(&self) -> OUTFIFO_UDF_L3_R {
        OUTFIFO_UDF_L3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ST")
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
impl core::fmt::Debug for crate::generic::Reg<ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Masked interrupt of Tx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST_SPEC;
impl crate::RegisterSpec for ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st::R`](R) reader structure"]
impl crate::Readable for ST_SPEC {}
#[doc = "`reset()` method sets ST to value 0"]
impl crate::Resettable for ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
