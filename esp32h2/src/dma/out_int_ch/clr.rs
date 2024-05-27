///Register `CLR` writer
pub type W = crate::W<CLR_SPEC>;
///Field `OUT_DONE` writer - Set this bit to clear the OUT_DONE_CH_INT interrupt.
pub type OUT_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `OUT_EOF` writer - Set this bit to clear the OUT_EOF_CH_INT interrupt.
pub type OUT_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `OUT_DSCR_ERR` writer - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `OUT_TOTAL_EOF` writer - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `OUTFIFO_OVF` writer - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
pub type OUTFIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `OUTFIFO_UDF` writer - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
pub type OUTFIFO_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set this bit to clear the OUT_DONE_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_done(&mut self) -> OUT_DONE_W<CLR_SPEC> {
        OUT_DONE_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to clear the OUT_EOF_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<CLR_SPEC> {
        OUT_EOF_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to clear the OUT_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<CLR_SPEC> {
        OUT_DSCR_ERR_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to clear the OUT_TOTAL_EOF_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<CLR_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 3)
    }
    ///Bit 4 - Set this bit to clear the OUTFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn outfifo_ovf(&mut self) -> OUTFIFO_OVF_W<CLR_SPEC> {
        OUTFIFO_OVF_W::new(self, 4)
    }
    ///Bit 5 - Set this bit to clear the OUTFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn outfifo_udf(&mut self) -> OUTFIFO_UDF_W<CLR_SPEC> {
        OUTFIFO_UDF_W::new(self, 5)
    }
}
/**Interrupt clear bits of channel 0

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`clr::W`](W) writer structure
impl crate::Writable for CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
///`reset()` method sets CLR to value 0
impl crate::Resettable for CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
