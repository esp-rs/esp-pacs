///Register `EDMA_CTRL` reader
pub type R = crate::R<EDMA_CTRL_SPEC>;
///Register `EDMA_CTRL` writer
pub type W = crate::W<EDMA_CTRL_SPEC>;
///Field `EDMA_CLK_ON` reader - Set 1 to enable EDMA clock.
pub type EDMA_CLK_ON_R = crate::BitReader;
///Field `EDMA_CLK_ON` writer - Set 1 to enable EDMA clock.
pub type EDMA_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EDMA_RESET` reader - Set 1 to let EDMA reset
pub type EDMA_RESET_R = crate::BitReader;
///Field `EDMA_RESET` writer - Set 1 to let EDMA reset
pub type EDMA_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable EDMA clock.
    #[inline(always)]
    pub fn edma_clk_on(&self) -> EDMA_CLK_ON_R {
        EDMA_CLK_ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 1 to let EDMA reset
    #[inline(always)]
    pub fn edma_reset(&self) -> EDMA_RESET_R {
        EDMA_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDMA_CTRL")
            .field("edma_clk_on", &self.edma_clk_on())
            .field("edma_reset", &self.edma_reset())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable EDMA clock.
    #[inline(always)]
    #[must_use]
    pub fn edma_clk_on(&mut self) -> EDMA_CLK_ON_W<EDMA_CTRL_SPEC> {
        EDMA_CLK_ON_W::new(self, 0)
    }
    ///Bit 1 - Set 1 to let EDMA reset
    #[inline(always)]
    #[must_use]
    pub fn edma_reset(&mut self) -> EDMA_RESET_W<EDMA_CTRL_SPEC> {
        EDMA_RESET_W::new(self, 1)
    }
}
/**EDMA control register

You can [`read`](crate::generic::Reg::read) this register and get [`edma_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EDMA_CTRL_SPEC;
impl crate::RegisterSpec for EDMA_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`edma_ctrl::R`](R) reader structure
impl crate::Readable for EDMA_CTRL_SPEC {}
///`write(|w| ..)` method takes [`edma_ctrl::W`](W) writer structure
impl crate::Writable for EDMA_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EDMA_CTRL to value 0x01
impl crate::Resettable for EDMA_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
