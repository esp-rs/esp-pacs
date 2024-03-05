#[doc = "Register `IN_INT_ENA_CH4` reader"]
pub type R = crate::R<IN_INT_ENA_CH4_SPEC>;
#[doc = "Register `IN_INT_ENA_CH4` writer"]
pub type W = crate::W<IN_INT_ENA_CH4_SPEC>;
#[doc = "Field `IN_DONE_CH4_INT_ENA` reader - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_CH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DONE_CH4_INT_ENA` writer - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_CH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF_CH4_INT_ENA` reader - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_CH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_CH4_INT_ENA` writer - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_CH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF_CH4_INT_ENA` reader - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_CH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_CH4_INT_ENA` writer - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_CH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR_CH4_INT_ENA` reader - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_CH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR_CH4_INT_ENA` writer - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_CH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L1_CH4_INT_ENA` reader - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_L1_CH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L1_CH4_INT_ENA` writer - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_OVF_L1_CH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L1_CH4_INT_ENA` reader - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_L1_CH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L1_CH4_INT_ENA` writer - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_UDF_L1_CH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L2_CH4_INT_ENA` reader - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
pub type INFIFO_OVF_L2_CH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L2_CH4_INT_ENA` writer - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
pub type INFIFO_OVF_L2_CH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L2_CH4_INT_ENA` reader - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
pub type INFIFO_UDF_L2_CH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L2_CH4_INT_ENA` writer - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
pub type INFIFO_UDF_L2_CH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY_CH4_INT_ENA` reader - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_CH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY_CH4_INT_ENA` writer - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_CH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_TASK_OVF_CH4_INT_ENA` reader - The interrupt enable bit for the IN_DSCR_TASK_OVF_CH_INT interrupt."]
pub type IN_DSCR_TASK_OVF_CH4_INT_ENA_R = crate::BitReader;
#[doc = "Field `IN_DSCR_TASK_OVF_CH4_INT_ENA` writer - The interrupt enable bit for the IN_DSCR_TASK_OVF_CH_INT interrupt."]
pub type IN_DSCR_TASK_OVF_CH4_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_done_ch4_int_ena(&self) -> IN_DONE_CH4_INT_ENA_R {
        IN_DONE_CH4_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_suc_eof_ch4_int_ena(&self) -> IN_SUC_EOF_CH4_INT_ENA_R {
        IN_SUC_EOF_CH4_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_err_eof_ch4_int_ena(&self) -> IN_ERR_EOF_CH4_INT_ENA_R {
        IN_ERR_EOF_CH4_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_err_ch4_int_ena(&self) -> IN_DSCR_ERR_CH4_INT_ENA_R {
        IN_DSCR_ERR_CH4_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l1_ch4_int_ena(&self) -> INFIFO_OVF_L1_CH4_INT_ENA_R {
        INFIFO_OVF_L1_CH4_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l1_ch4_int_ena(&self) -> INFIFO_UDF_L1_CH4_INT_ENA_R {
        INFIFO_UDF_L1_CH4_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_ovf_l2_ch4_int_ena(&self) -> INFIFO_OVF_L2_CH4_INT_ENA_R {
        INFIFO_OVF_L2_CH4_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    pub fn infifo_udf_l2_ch4_int_ena(&self) -> INFIFO_UDF_L2_CH4_INT_ENA_R {
        INFIFO_UDF_L2_CH4_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_empty_ch4_int_ena(&self) -> IN_DSCR_EMPTY_CH4_INT_ENA_R {
        IN_DSCR_EMPTY_CH4_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the IN_DSCR_TASK_OVF_CH_INT interrupt."]
    #[inline(always)]
    pub fn in_dscr_task_ovf_ch4_int_ena(&self) -> IN_DSCR_TASK_OVF_CH4_INT_ENA_R {
        IN_DSCR_TASK_OVF_CH4_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_INT_ENA_CH4")
            .field(
                "in_done_ch4_int_ena",
                &format_args!("{}", self.in_done_ch4_int_ena().bit()),
            )
            .field(
                "in_suc_eof_ch4_int_ena",
                &format_args!("{}", self.in_suc_eof_ch4_int_ena().bit()),
            )
            .field(
                "in_err_eof_ch4_int_ena",
                &format_args!("{}", self.in_err_eof_ch4_int_ena().bit()),
            )
            .field(
                "in_dscr_err_ch4_int_ena",
                &format_args!("{}", self.in_dscr_err_ch4_int_ena().bit()),
            )
            .field(
                "infifo_ovf_l1_ch4_int_ena",
                &format_args!("{}", self.infifo_ovf_l1_ch4_int_ena().bit()),
            )
            .field(
                "infifo_udf_l1_ch4_int_ena",
                &format_args!("{}", self.infifo_udf_l1_ch4_int_ena().bit()),
            )
            .field(
                "infifo_ovf_l2_ch4_int_ena",
                &format_args!("{}", self.infifo_ovf_l2_ch4_int_ena().bit()),
            )
            .field(
                "infifo_udf_l2_ch4_int_ena",
                &format_args!("{}", self.infifo_udf_l2_ch4_int_ena().bit()),
            )
            .field(
                "in_dscr_empty_ch4_int_ena",
                &format_args!("{}", self.in_dscr_empty_ch4_int_ena().bit()),
            )
            .field(
                "in_dscr_task_ovf_ch4_int_ena",
                &format_args!("{}", self.in_dscr_task_ovf_ch4_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_INT_ENA_CH4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_done_ch4_int_ena(&mut self) -> IN_DONE_CH4_INT_ENA_W<IN_INT_ENA_CH4_SPEC> {
        IN_DONE_CH4_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof_ch4_int_ena(&mut self) -> IN_SUC_EOF_CH4_INT_ENA_W<IN_INT_ENA_CH4_SPEC> {
        IN_SUC_EOF_CH4_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof_ch4_int_ena(&mut self) -> IN_ERR_EOF_CH4_INT_ENA_W<IN_INT_ENA_CH4_SPEC> {
        IN_ERR_EOF_CH4_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err_ch4_int_ena(&mut self) -> IN_DSCR_ERR_CH4_INT_ENA_W<IN_INT_ENA_CH4_SPEC> {
        IN_DSCR_ERR_CH4_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l1_ch4_int_ena(
        &mut self,
    ) -> INFIFO_OVF_L1_CH4_INT_ENA_W<IN_INT_ENA_CH4_SPEC> {
        INFIFO_OVF_L1_CH4_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l1_ch4_int_ena(
        &mut self,
    ) -> INFIFO_UDF_L1_CH4_INT_ENA_W<IN_INT_ENA_CH4_SPEC> {
        INFIFO_UDF_L1_CH4_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l2_ch4_int_ena(
        &mut self,
    ) -> INFIFO_OVF_L2_CH4_INT_ENA_W<IN_INT_ENA_CH4_SPEC> {
        INFIFO_OVF_L2_CH4_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l2_ch4_int_ena(
        &mut self,
    ) -> INFIFO_UDF_L2_CH4_INT_ENA_W<IN_INT_ENA_CH4_SPEC> {
        INFIFO_UDF_L2_CH4_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty_ch4_int_ena(
        &mut self,
    ) -> IN_DSCR_EMPTY_CH4_INT_ENA_W<IN_INT_ENA_CH4_SPEC> {
        IN_DSCR_EMPTY_CH4_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for the IN_DSCR_TASK_OVF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_task_ovf_ch4_int_ena(
        &mut self,
    ) -> IN_DSCR_TASK_OVF_CH4_INT_ENA_W<IN_INT_ENA_CH4_SPEC> {
        IN_DSCR_TASK_OVF_CH4_INT_ENA_W::new(self, 9)
    }
}
#[doc = "RX CH4 interrupt ena register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_int_ena_ch4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_int_ena_ch4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_INT_ENA_CH4_SPEC;
impl crate::RegisterSpec for IN_INT_ENA_CH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_int_ena_ch4::R`](R) reader structure"]
impl crate::Readable for IN_INT_ENA_CH4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_int_ena_ch4::W`](W) writer structure"]
impl crate::Writable for IN_INT_ENA_CH4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_INT_ENA_CH4 to value 0"]
impl crate::Resettable for IN_INT_ENA_CH4_SPEC {
    const RESET_VALUE: u32 = 0;
}
