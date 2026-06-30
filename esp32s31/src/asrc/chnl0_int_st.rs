#[doc = "Register `CHNL0_INT_ST` reader"]
pub type R = crate::R<CHNL0_INT_ST_SPEC>;
#[doc = "Field `CHNL0_OUTCNT_EOF_INT_ST` reader - This is the status bit for reg_out_cnt_eof_int_raw when reg_out_cnt_eof_int_ena is set to 1."]
pub type CHNL0_OUTCNT_EOF_INT_ST_R = crate::BitReader;
#[doc = "Field `CHNL0_OUTFIFO_EOF_INT_ST` reader - This is the status bit for reg_outfifo_eof_int_raw when reg_outfifo_eof_int_ena is set to 1."]
pub type CHNL0_OUTFIFO_EOF_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This is the status bit for reg_out_cnt_eof_int_raw when reg_out_cnt_eof_int_ena is set to 1."]
    #[inline(always)]
    pub fn chnl0_outcnt_eof_int_st(&self) -> CHNL0_OUTCNT_EOF_INT_ST_R {
        CHNL0_OUTCNT_EOF_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the status bit for reg_outfifo_eof_int_raw when reg_outfifo_eof_int_ena is set to 1."]
    #[inline(always)]
    pub fn chnl0_outfifo_eof_int_st(&self) -> CHNL0_OUTFIFO_EOF_INT_ST_R {
        CHNL0_OUTFIFO_EOF_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL0_INT_ST")
            .field("chnl0_outcnt_eof_int_st", &self.chnl0_outcnt_eof_int_st())
            .field("chnl0_outfifo_eof_int_st", &self.chnl0_outfifo_eof_int_st())
            .finish()
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL0_INT_ST_SPEC;
impl crate::RegisterSpec for CHNL0_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl0_int_st::R`](R) reader structure"]
impl crate::Readable for CHNL0_INT_ST_SPEC {}
#[doc = "`reset()` method sets CHNL0_INT_ST to value 0"]
impl crate::Resettable for CHNL0_INT_ST_SPEC {}
