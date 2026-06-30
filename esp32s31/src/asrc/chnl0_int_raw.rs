#[doc = "Register `CHNL0_INT_RAW` reader"]
pub type R = crate::R<CHNL0_INT_RAW_SPEC>;
#[doc = "Register `CHNL0_INT_RAW` writer"]
pub type W = crate::W<CHNL0_INT_RAW_SPEC>;
#[doc = "Field `CHNL0_OUTCNT_EOF_INT_RAW` reader - This interrupt raw bit turns to high level when the counter approach to reg_out_len."]
pub type CHNL0_OUTCNT_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CHNL0_OUTCNT_EOF_INT_RAW` writer - This interrupt raw bit turns to high level when the counter approach to reg_out_len."]
pub type CHNL0_OUTCNT_EOF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_OUTFIFO_EOF_INT_RAW` reader - This interrupt raw bit turns to high level when getting the eof from DMA and write the several data to DMA."]
pub type CHNL0_OUTFIFO_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `CHNL0_OUTFIFO_EOF_INT_RAW` writer - This interrupt raw bit turns to high level when getting the eof from DMA and write the several data to DMA."]
pub type CHNL0_OUTFIFO_EOF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when the counter approach to reg_out_len."]
    #[inline(always)]
    pub fn chnl0_outcnt_eof_int_raw(&self) -> CHNL0_OUTCNT_EOF_INT_RAW_R {
        CHNL0_OUTCNT_EOF_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when getting the eof from DMA and write the several data to DMA."]
    #[inline(always)]
    pub fn chnl0_outfifo_eof_int_raw(&self) -> CHNL0_OUTFIFO_EOF_INT_RAW_R {
        CHNL0_OUTFIFO_EOF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHNL0_INT_RAW")
            .field("chnl0_outcnt_eof_int_raw", &self.chnl0_outcnt_eof_int_raw())
            .field(
                "chnl0_outfifo_eof_int_raw",
                &self.chnl0_outfifo_eof_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when the counter approach to reg_out_len."]
    #[inline(always)]
    pub fn chnl0_outcnt_eof_int_raw(
        &mut self,
    ) -> CHNL0_OUTCNT_EOF_INT_RAW_W<'_, CHNL0_INT_RAW_SPEC> {
        CHNL0_OUTCNT_EOF_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when getting the eof from DMA and write the several data to DMA."]
    #[inline(always)]
    pub fn chnl0_outfifo_eof_int_raw(
        &mut self,
    ) -> CHNL0_OUTFIFO_EOF_INT_RAW_W<'_, CHNL0_INT_RAW_SPEC> {
        CHNL0_OUTFIFO_EOF_INT_RAW_W::new(self, 1)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`chnl0_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL0_INT_RAW_SPEC;
impl crate::RegisterSpec for CHNL0_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chnl0_int_raw::R`](R) reader structure"]
impl crate::Readable for CHNL0_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chnl0_int_raw::W`](W) writer structure"]
impl crate::Writable for CHNL0_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL0_INT_RAW to value 0"]
impl crate::Resettable for CHNL0_INT_RAW_SPEC {}
