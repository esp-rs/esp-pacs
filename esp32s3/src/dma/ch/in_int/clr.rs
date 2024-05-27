///Register `CLR` writer
pub type W = crate::W<CLR_SPEC>;
///Field `IN_DONE` writer - Set this bit to clear the IN_DONE_CH_INT interrupt.
pub type IN_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `IN_SUC_EOF` writer - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `IN_ERR_EOF` writer - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `IN_DSCR_ERR` writer - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `IN_DSCR_EMPTY` writer - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `DMA_INFIFO_FULL_WM` writer - Set this bit to clear the INFIFO_FULL_WM_CH_INT interrupt.
pub type DMA_INFIFO_FULL_WM_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `INFIFO_OVF_L1` writer - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
pub type INFIFO_OVF_L1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `INFIFO_UDF_L1` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
pub type INFIFO_UDF_L1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `INFIFO_OVF_L3` writer - Set this bit to clear the INFIFO_OVF_L3_CH_INT interrupt.
pub type INFIFO_OVF_L3_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `INFIFO_UDF_L3` writer - Set this bit to clear the INFIFO_UDF_L3_CH_INT interrupt.
pub type INFIFO_UDF_L3_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set this bit to clear the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<CLR_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<CLR_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<CLR_SPEC> {
        IN_ERR_EOF_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<CLR_SPEC> {
        IN_DSCR_ERR_W::new(self, 3)
    }
    ///Bit 4 - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<CLR_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 4)
    }
    ///Bit 5 - Set this bit to clear the INFIFO_FULL_WM_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn dma_infifo_full_wm(&mut self) -> DMA_INFIFO_FULL_WM_W<CLR_SPEC> {
        DMA_INFIFO_FULL_WM_W::new(self, 5)
    }
    ///Bit 6 - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l1(&mut self) -> INFIFO_OVF_L1_W<CLR_SPEC> {
        INFIFO_OVF_L1_W::new(self, 6)
    }
    ///Bit 7 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l1(&mut self) -> INFIFO_UDF_L1_W<CLR_SPEC> {
        INFIFO_UDF_L1_W::new(self, 7)
    }
    ///Bit 8 - Set this bit to clear the INFIFO_OVF_L3_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l3(&mut self) -> INFIFO_OVF_L3_W<CLR_SPEC> {
        INFIFO_OVF_L3_W::new(self, 8)
    }
    ///Bit 9 - Set this bit to clear the INFIFO_UDF_L3_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l3(&mut self) -> INFIFO_UDF_L3_W<CLR_SPEC> {
        INFIFO_UDF_L3_W::new(self, 9)
    }
}
/**Interrupt clear bits of Rx channel 0

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`clr::W`](W) writer structure
impl crate::Writable for CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03ff;
}
///`reset()` method sets CLR to value 0
impl crate::Resettable for CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
