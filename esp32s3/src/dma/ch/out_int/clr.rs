#[doc = "Register `CLR` writer"]
pub type W = crate::W<CLR_SPEC>;
#[doc = "Field `OUT_DONE` writer - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
pub type OUT_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_EOF` writer - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
pub type OUT_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR` writer - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF` writer - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L1` writer - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
pub type OUTFIFO_OVF_L1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L1` writer - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
pub type OUTFIFO_UDF_L1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L3` writer - Set this bit to clear the OUTFIFO_OVF_L3_CH_INT interrupt."]
pub type OUTFIFO_OVF_L3_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L3` writer - Set this bit to clear the OUTFIFO_UDF_L3_CH_INT interrupt."]
pub type OUTFIFO_UDF_L3_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the OUT_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_done(&mut self) -> OUT_DONE_W<CLR_SPEC> {
        OUT_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the OUT_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_eof(&mut self) -> OUT_EOF_W<CLR_SPEC> {
        OUT_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<CLR_SPEC> {
        OUT_DSCR_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<CLR_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l1(&mut self) -> OUTFIFO_OVF_L1_W<CLR_SPEC> {
        OUTFIFO_OVF_L1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l1(&mut self) -> OUTFIFO_UDF_L1_W<CLR_SPEC> {
        OUTFIFO_UDF_L1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the OUTFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_ovf_l3(&mut self) -> OUTFIFO_OVF_L3_W<CLR_SPEC> {
        OUTFIFO_OVF_L3_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the OUTFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn outfifo_udf_l3(&mut self) -> OUTFIFO_UDF_L3_W<CLR_SPEC> {
        OUTFIFO_UDF_L3_W::new(self, 7)
    }
}
#[doc = "Interrupt clear bits of Tx channel 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {}
