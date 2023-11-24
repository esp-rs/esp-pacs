#[doc = "Register `IN_INT_ENA_CH%s` reader"]
pub type R = crate::R<IN_INT_ENA_CH_SPEC>;
#[doc = "Register `IN_INT_ENA_CH%s` writer"]
pub type W = crate::W<IN_INT_ENA_CH_SPEC>;
#[doc = "Field `IN_DONE` reader - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_DONE` writer - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` reader - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` writer - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF` reader - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF` writer - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` reader - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` writer - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` reader - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` writer - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_FULL_WM` reader - The interrupt enable bit for the INFIFO_FULL_WM_CH_INT interrupt."]
pub type INFIFO_FULL_WM_R = crate::BitReader;
#[doc = "Field `INFIFO_FULL_WM` writer - The interrupt enable bit for the INFIFO_FULL_WM_CH_INT interrupt."]
pub type INFIFO_FULL_WM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L1` reader - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L1` writer - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L1` reader - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L1` writer - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L3` reader - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
pub type INFIFO_OVF_L3_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L3` writer - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
pub type INFIFO_OVF_L3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L3` reader - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
pub type INFIFO_UDF_L3_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L3` writer - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
pub type INFIFO_UDF_L3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the INFIFO_FULL_WM_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_full_wm(&self) -> INFIFO_FULL_WM_R {
        INFIFO_FULL_WM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l1(&self) -> INFIFO_OVF_L1_R {
        INFIFO_OVF_L1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l1(&self) -> INFIFO_UDF_L1_R {
        INFIFO_UDF_L1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l3(&self) -> INFIFO_OVF_L3_R {
        INFIFO_OVF_L3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l3(&self) -> INFIFO_UDF_L3_R {
        INFIFO_UDF_L3_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_INT_ENA_CH")
            .field("in_done", &format_args!("{}", self.in_done().bit()))
            .field("in_suc_eof", &format_args!("{}", self.in_suc_eof().bit()))
            .field("in_err_eof", &format_args!("{}", self.in_err_eof().bit()))
            .field("in_dscr_err", &format_args!("{}", self.in_dscr_err().bit()))
            .field(
                "in_dscr_empty",
                &format_args!("{}", self.in_dscr_empty().bit()),
            )
            .field(
                "infifo_full_wm",
                &format_args!("{}", self.infifo_full_wm().bit()),
            )
            .field(
                "infifo_ovf_l1",
                &format_args!("{}", self.infifo_ovf_l1().bit()),
            )
            .field(
                "infifo_udf_l1",
                &format_args!("{}", self.infifo_udf_l1().bit()),
            )
            .field(
                "infifo_ovf_l3",
                &format_args!("{}", self.infifo_ovf_l3().bit()),
            )
            .field(
                "infifo_udf_l3",
                &format_args!("{}", self.infifo_udf_l3().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_INT_ENA_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<IN_INT_ENA_CH_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<IN_INT_ENA_CH_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<IN_INT_ENA_CH_SPEC> {
        IN_ERR_EOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<IN_INT_ENA_CH_SPEC> {
        IN_DSCR_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<IN_INT_ENA_CH_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the INFIFO_FULL_WM_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_full_wm(&mut self) -> INFIFO_FULL_WM_W<IN_INT_ENA_CH_SPEC> {
        INFIFO_FULL_WM_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l1(&mut self) -> INFIFO_OVF_L1_W<IN_INT_ENA_CH_SPEC> {
        INFIFO_OVF_L1_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l1(&mut self) -> INFIFO_UDF_L1_W<IN_INT_ENA_CH_SPEC> {
        INFIFO_UDF_L1_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l3(&mut self) -> INFIFO_OVF_L3_W<IN_INT_ENA_CH_SPEC> {
        INFIFO_OVF_L3_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l3(&mut self) -> INFIFO_UDF_L3_W<IN_INT_ENA_CH_SPEC> {
        INFIFO_UDF_L3_W::new(self, 9)
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
#[doc = "Interrupt enable bits of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_INT_ENA_CH_SPEC;
impl crate::RegisterSpec for IN_INT_ENA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_int_ena_ch::R`](R) reader structure"]
impl crate::Readable for IN_INT_ENA_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_int_ena_ch::W`](W) writer structure"]
impl crate::Writable for IN_INT_ENA_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_INT_ENA_CH%s to value 0"]
impl crate::Resettable for IN_INT_ENA_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
