///Register `ENA` reader
pub type R = crate::R<ENA_SPEC>;
///Register `ENA` writer
pub type W = crate::W<ENA_SPEC>;
///Field `OUT_DONE` reader - The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
pub type OUT_DONE_R = crate::BitReader;
///Field `OUT_DONE` writer - The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
pub type OUT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_EOF` reader - The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
pub type OUT_EOF_R = crate::BitReader;
///Field `OUT_EOF` writer - The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
pub type OUT_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_DSCR_ERR` reader - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
pub type OUT_DSCR_ERR_R = crate::BitReader;
///Field `OUT_DSCR_ERR` writer - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_TOTAL_EOF` reader - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
pub type OUT_TOTAL_EOF_R = crate::BitReader;
///Field `OUT_TOTAL_EOF` writer - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTFIFO_OVF` reader - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
pub type OUTFIFO_OVF_R = crate::BitReader;
///Field `OUTFIFO_OVF` writer - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
pub type OUTFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTFIFO_UDF` reader - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
pub type OUTFIFO_UDF_R = crate::BitReader;
///Field `OUTFIFO_UDF` writer - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
pub type OUTFIFO_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_ovf(&self) -> OUTFIFO_OVF_R {
        OUTFIFO_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn outfifo_udf(&self) -> OUTFIFO_UDF_R {
        OUTFIFO_UDF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENA")
            .field("out_done", &self.out_done())
            .field("out_eof", &self.out_eof())
            .field("out_dscr_err", &self.out_dscr_err())
            .field("out_total_eof", &self.out_total_eof())
            .field("outfifo_ovf", &self.outfifo_ovf())
            .field("outfifo_udf", &self.outfifo_udf())
            .finish()
    }
}
impl W {
    ///Bit 0 - The interrupt enable bit for the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_done(&mut self) -> OUT_DONE_W<ENA_SPEC> {
        OUT_DONE_W::new(self, 0)
    }
    ///Bit 1 - The interrupt enable bit for the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<ENA_SPEC> {
        OUT_EOF_W::new(self, 1)
    }
    ///Bit 2 - The interrupt enable bit for the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<ENA_SPEC> {
        OUT_DSCR_ERR_W::new(self, 2)
    }
    ///Bit 3 - The interrupt enable bit for the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<ENA_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 3)
    }
    ///Bit 4 - The interrupt enable bit for the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf(&mut self) -> OUTFIFO_OVF_W<ENA_SPEC> {
        OUTFIFO_OVF_W::new(self, 4)
    }
    ///Bit 5 - The interrupt enable bit for the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf(&mut self) -> OUTFIFO_UDF_W<ENA_SPEC> {
        OUTFIFO_UDF_W::new(self, 5)
    }
}
/**Interrupt enable bits of channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ENA_SPEC;
impl crate::RegisterSpec for ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ena::R`](R) reader structure
impl crate::Readable for ENA_SPEC {}
///`write(|w| ..)` method takes [`ena::W`](W) writer structure
impl crate::Writable for ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ENA to value 0
impl crate::Resettable for ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
