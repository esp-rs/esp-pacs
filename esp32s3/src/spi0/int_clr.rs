///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `TOTAL_TRANS_END` writer - The clear bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt.
pub type TOTAL_TRANS_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ECC_ERR` writer - The clear bit for SPI_MEM_ECC_ERR_INT interrupt. SPI_MEM_ECC_ERR_ADDR and SPI_MEM_ECC_ERR_CNT will be cleared by the pulse of this bit.
pub type ECC_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 2 - The clear bit for SPI_MEM_TOTAL_TRANS_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn total_trans_end(&mut self) -> TOTAL_TRANS_END_W<INT_CLR_SPEC> {
        TOTAL_TRANS_END_W::new(self, 2)
    }
    ///Bit 4 - The clear bit for SPI_MEM_ECC_ERR_INT interrupt. SPI_MEM_ECC_ERR_ADDR and SPI_MEM_ECC_ERR_CNT will be cleared by the pulse of this bit.
    #[inline(always)]
    #[must_use]
    pub fn ecc_err(&mut self) -> ECC_ERR_W<INT_CLR_SPEC> {
        ECC_ERR_W::new(self, 4)
    }
}
/**SPI1 interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x14;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
