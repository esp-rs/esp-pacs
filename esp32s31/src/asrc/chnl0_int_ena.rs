#[doc = "Register `CHNL0_INT_ENA` reader"]
pub type R = crate::R<CHNL0_INT_ENA_SPEC>;
#[doc = "Register `CHNL0_INT_ENA` writer"]
pub type W = crate::W<CHNL0_INT_ENA_SPEC>;
#[doc = "Field `CHNL0_OUTCNT_EOF_INT_ENA` reader - This is the enable bit for reg_out_cnt_eof_int_st register."]
pub type CHNL0_OUTCNT_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `CHNL0_OUTCNT_EOF_INT_ENA` writer - This is the enable bit for reg_out_cnt_eof_int_st register."]
pub type CHNL0_OUTCNT_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_OUTFIFO_EOF_INT_ENA` reader - This is the enable bit for reg_outfifo_eof_int_st register."]
pub type CHNL0_OUTFIFO_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `CHNL0_OUTFIFO_EOF_INT_ENA` writer - This is the enable bit for reg_outfifo_eof_int_st register."]
pub type CHNL0_OUTFIFO_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This is the enable bit for reg_out_cnt_eof_int_st register."]
    #[inline(always)]
    pub fn chnl0_outcnt_eof_int_ena(&self) -> CHNL0_OUTCNT_EOF_INT_ENA_R {
        CHNL0_OUTCNT_EOF_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the enable bit for reg_outfifo_eof_int_st register."]
    #[inline(always)]
    pub fn chnl0_outfifo_eof_int_ena(&self) -> CHNL0_OUTFIFO_EOF_INT_ENA_R {
        CHNL0_OUTFIFO_EOF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL0_INT_ENA")
            .field("chnl0_outcnt_eof_int_ena", &self.chnl0_outcnt_eof_int_ena())
            .field(
                "chnl0_outfifo_eof_int_ena",
                &self.chnl0_outfifo_eof_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit for reg_out_cnt_eof_int_st register."]
    #[inline(always)]
    pub fn chnl0_outcnt_eof_int_ena(
        &mut self,
    ) -> CHNL0_OUTCNT_EOF_INT_ENA_W<'_, CHNL0_INT_ENA_SPEC> {
        CHNL0_OUTCNT_EOF_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - This is the enable bit for reg_outfifo_eof_int_st register."]
    #[inline(always)]
    pub fn chnl0_outfifo_eof_int_ena(
        &mut self,
    ) -> CHNL0_OUTFIFO_EOF_INT_ENA_W<'_, CHNL0_INT_ENA_SPEC> {
        CHNL0_OUTFIFO_EOF_INT_ENA_W::new(self, 1)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL0_INT_ENA_SPEC;
impl crate::RegisterSpec for CHNL0_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl0_int_ena::R`](R) reader structure"]
impl crate::Readable for CHNL0_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chnl0_int_ena::W`](W) writer structure"]
impl crate::Writable for CHNL0_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL0_INT_ENA to value 0"]
impl crate::Resettable for CHNL0_INT_ENA_SPEC {}
