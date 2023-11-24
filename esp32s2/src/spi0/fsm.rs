#[doc = "Register `FSM` reader"]
pub type R = crate::R<FSM_SPEC>;
#[doc = "Register `FSM` writer"]
pub type W = crate::W<FSM_SPEC>;
#[doc = "Field `ST` reader - The status of spi state machine. 0: idle state, 1: preparation state, 2: send command state, 3: send data state, 4: red data state, 5:write data state, 6: wait state, 7: done state."]
pub type ST_R = crate::FieldReader;
#[doc = "Field `MST_DMA_RD_BYTELEN` reader - Define the master DMA read byte length in non seg-conf-trans or seg-conf-trans mode. Invalid when SPI_RX_EOF_EN is 0. Can be configured in CONF state.."]
pub type MST_DMA_RD_BYTELEN_R = crate::FieldReader<u32>;
#[doc = "Field `MST_DMA_RD_BYTELEN` writer - Define the master DMA read byte length in non seg-conf-trans or seg-conf-trans mode. Invalid when SPI_RX_EOF_EN is 0. Can be configured in CONF state.."]
pub type MST_DMA_RD_BYTELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:3 - The status of spi state machine. 0: idle state, 1: preparation state, 2: send command state, 3: send data state, 4: red data state, 5:write data state, 6: wait state, 7: done state."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - Define the master DMA read byte length in non seg-conf-trans or seg-conf-trans mode. Invalid when SPI_RX_EOF_EN is 0. Can be configured in CONF state.."]
    #[inline(always)]
    pub fn mst_dma_rd_bytelen(&self) -> MST_DMA_RD_BYTELEN_R {
        MST_DMA_RD_BYTELEN_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM")
            .field("st", &format_args!("{}", self.st().bits()))
            .field(
                "mst_dma_rd_bytelen",
                &format_args!("{}", self.mst_dma_rd_bytelen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FSM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 12:31 - Define the master DMA read byte length in non seg-conf-trans or seg-conf-trans mode. Invalid when SPI_RX_EOF_EN is 0. Can be configured in CONF state.."]
    #[inline(always)]
    #[must_use]
    pub fn mst_dma_rd_bytelen(&mut self) -> MST_DMA_RD_BYTELEN_W<FSM_SPEC> {
        MST_DMA_RD_BYTELEN_W::new(self, 12)
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
#[doc = "SPI master status and DMA read byte control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm::R`](R) reader structure"]
impl crate::Readable for FSM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsm::W`](W) writer structure"]
impl crate::Writable for FSM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM to value 0"]
impl crate::Resettable for FSM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
