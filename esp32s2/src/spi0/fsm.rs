#[doc = "Register `FSM` reader"]
pub struct R(crate::R<FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM` writer"]
pub struct W(crate::W<FSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST` reader - The status of spi state machine. 0: idle state, 1: preparation state, 2: send command state, 3: send data state, 4: red data state, 5:write data state, 6: wait state, 7: done state."]
pub type ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MST_DMA_RD_BYTELEN` reader - Define the master DMA read byte length in non seg-conf-trans or seg-conf-trans mode. Invalid when SPI_RX_EOF_EN is 0. Can be configured in CONF state.."]
pub type MST_DMA_RD_BYTELEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MST_DMA_RD_BYTELEN` writer - Define the master DMA read byte length in non seg-conf-trans or seg-conf-trans mode. Invalid when SPI_RX_EOF_EN is 0. Can be configured in CONF state.."]
pub type MST_DMA_RD_BYTELEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:3 - The status of spi state machine. 0: idle state, 1: preparation state, 2: send command state, 3: send data state, 4: red data state, 5:write data state, 6: wait state, 7: done state."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - Define the master DMA read byte length in non seg-conf-trans or seg-conf-trans mode. Invalid when SPI_RX_EOF_EN is 0. Can be configured in CONF state.."]
    #[inline(always)]
    pub fn mst_dma_rd_bytelen(&self) -> MST_DMA_RD_BYTELEN_R {
        MST_DMA_RD_BYTELEN_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:31 - Define the master DMA read byte length in non seg-conf-trans or seg-conf-trans mode. Invalid when SPI_RX_EOF_EN is 0. Can be configured in CONF state.."]
    #[inline(always)]
    pub fn mst_dma_rd_bytelen(&mut self) -> MST_DMA_RD_BYTELEN_W<12> {
        MST_DMA_RD_BYTELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI master status and DMA read byte control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm](index.html) module"]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm::R](R) reader structure"]
impl crate::Readable for FSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm::W](W) writer structure"]
impl crate::Writable for FSM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSM to value 0"]
impl crate::Resettable for FSM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
