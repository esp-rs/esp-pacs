#[doc = "Register `DMA_ADDR_CTRL` reader"]
pub type R = crate::R<DMA_ADDR_CTRL_SPEC>;
#[doc = "Register `DMA_ADDR_CTRL` writer"]
pub type W = crate::W<DMA_ADDR_CTRL_SPEC>;
#[doc = "Field `REG_SYS_DMA_ADDR_SEL` reader - 0 means dma access extmem use 8xxx_xxxx else use 4xxx_xxxx"]
pub type REG_SYS_DMA_ADDR_SEL_R = crate::BitReader;
#[doc = "Field `REG_SYS_DMA_ADDR_SEL` writer - 0 means dma access extmem use 8xxx_xxxx else use 4xxx_xxxx"]
pub type REG_SYS_DMA_ADDR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0 means dma access extmem use 8xxx_xxxx else use 4xxx_xxxx"]
    #[inline(always)]
    pub fn reg_sys_dma_addr_sel(&self) -> REG_SYS_DMA_ADDR_SEL_R {
        REG_SYS_DMA_ADDR_SEL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_ADDR_CTRL")
            .field(
                "reg_sys_dma_addr_sel",
                &format_args!("{}", self.reg_sys_dma_addr_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_ADDR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - 0 means dma access extmem use 8xxx_xxxx else use 4xxx_xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sys_dma_addr_sel(&mut self) -> REG_SYS_DMA_ADDR_SEL_W<DMA_ADDR_CTRL_SPEC> {
        REG_SYS_DMA_ADDR_SEL_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_addr_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_addr_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_ADDR_CTRL_SPEC;
impl crate::RegisterSpec for DMA_ADDR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_addr_ctrl::R`](R) reader structure"]
impl crate::Readable for DMA_ADDR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_addr_ctrl::W`](W) writer structure"]
impl crate::Writable for DMA_ADDR_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_ADDR_CTRL to value 0"]
impl crate::Resettable for DMA_ADDR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
