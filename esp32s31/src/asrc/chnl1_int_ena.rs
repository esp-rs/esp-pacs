#[doc = "Register `CHNL1_INT_ENA` reader"]
pub type R = crate::R<CHNL1_INT_ENA_SPEC>;
#[doc = "Register `CHNL1_INT_ENA` writer"]
pub type W = crate::W<CHNL1_INT_ENA_SPEC>;
#[doc = "Field `CHNL1_OUTCNT_EOF_INT_ENA` reader - This is the enable bit for reg_out_cnt_eof_int_st register."]
pub type CHNL1_OUTCNT_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `CHNL1_OUTCNT_EOF_INT_ENA` writer - This is the enable bit for reg_out_cnt_eof_int_st register."]
pub type CHNL1_OUTCNT_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_OUTFIFO_EOF_INT_ENA` reader - This is the enable bit for reg_outfifo_eof_int_st register."]
pub type CHNL1_OUTFIFO_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `CHNL1_OUTFIFO_EOF_INT_ENA` writer - This is the enable bit for reg_outfifo_eof_int_st register."]
pub type CHNL1_OUTFIFO_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This is the enable bit for reg_out_cnt_eof_int_st register."]
    #[inline(always)]
    pub fn chnl1_outcnt_eof_int_ena(&self) -> CHNL1_OUTCNT_EOF_INT_ENA_R {
        CHNL1_OUTCNT_EOF_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the enable bit for reg_outfifo_eof_int_st register."]
    #[inline(always)]
    pub fn chnl1_outfifo_eof_int_ena(&self) -> CHNL1_OUTFIFO_EOF_INT_ENA_R {
        CHNL1_OUTFIFO_EOF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL1_INT_ENA")
            .field("chnl1_outcnt_eof_int_ena", &self.chnl1_outcnt_eof_int_ena())
            .field(
                "chnl1_outfifo_eof_int_ena",
                &self.chnl1_outfifo_eof_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit for reg_out_cnt_eof_int_st register."]
    #[inline(always)]
    pub fn chnl1_outcnt_eof_int_ena(
        &mut self,
    ) -> CHNL1_OUTCNT_EOF_INT_ENA_W<'_, CHNL1_INT_ENA_SPEC> {
        CHNL1_OUTCNT_EOF_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - This is the enable bit for reg_outfifo_eof_int_st register."]
    #[inline(always)]
    pub fn chnl1_outfifo_eof_int_ena(
        &mut self,
    ) -> CHNL1_OUTFIFO_EOF_INT_ENA_W<'_, CHNL1_INT_ENA_SPEC> {
        CHNL1_OUTFIFO_EOF_INT_ENA_W::new(self, 1)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl1_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL1_INT_ENA_SPEC;
impl crate::RegisterSpec for CHNL1_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl1_int_ena::R`](R) reader structure"]
impl crate::Readable for CHNL1_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chnl1_int_ena::W`](W) writer structure"]
impl crate::Writable for CHNL1_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL1_INT_ENA to value 0"]
impl crate::Resettable for CHNL1_INT_ENA_SPEC {}
