#[doc = "Register `PERI_MEM_CLK_FORCE_ON` reader"]
pub type R = crate::R<PERI_MEM_CLK_FORCE_ON_SPEC>;
#[doc = "Register `PERI_MEM_CLK_FORCE_ON` writer"]
pub type W = crate::W<PERI_MEM_CLK_FORCE_ON_SPEC>;
#[doc = "Field `RMT_MEM_CLK_FORCE_ON` reader - Set this bit to force on mem clk in rmt"]
pub type RMT_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `RMT_MEM_CLK_FORCE_ON` writer - Set this bit to force on mem clk in rmt"]
pub type RMT_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSCRAMBLER_TX_MEM_CLK_FORCE_ON` reader - Set this bit to force on tx mem clk in bitscrambler"]
pub type BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `BITSCRAMBLER_TX_MEM_CLK_FORCE_ON` writer - Set this bit to force on tx mem clk in bitscrambler"]
pub type BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSCRAMBLER_RX_MEM_CLK_FORCE_ON` reader - Set this bit to force on rx mem clk in bitscrambler"]
pub type BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `BITSCRAMBLER_RX_MEM_CLK_FORCE_ON` writer - Set this bit to force on rx mem clk in bitscrambler"]
pub type BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_MEM_CLK_FORCE_ON` reader - Set this bit to force on mem clk in gdma"]
pub type GDMA_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `GDMA_MEM_CLK_FORCE_ON` writer - Set this bit to force on mem clk in gdma"]
pub type GDMA_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to force on mem clk in rmt"]
    #[inline(always)]
    pub fn rmt_mem_clk_force_on(&self) -> RMT_MEM_CLK_FORCE_ON_R {
        RMT_MEM_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force on tx mem clk in bitscrambler"]
    #[inline(always)]
    pub fn bitscrambler_tx_mem_clk_force_on(&self) -> BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_R {
        BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force on rx mem clk in bitscrambler"]
    #[inline(always)]
    pub fn bitscrambler_rx_mem_clk_force_on(&self) -> BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_R {
        BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force on mem clk in gdma"]
    #[inline(always)]
    pub fn gdma_mem_clk_force_on(&self) -> GDMA_MEM_CLK_FORCE_ON_R {
        GDMA_MEM_CLK_FORCE_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_MEM_CLK_FORCE_ON")
            .field("rmt_mem_clk_force_on", &self.rmt_mem_clk_force_on())
            .field(
                "bitscrambler_tx_mem_clk_force_on",
                &self.bitscrambler_tx_mem_clk_force_on(),
            )
            .field(
                "bitscrambler_rx_mem_clk_force_on",
                &self.bitscrambler_rx_mem_clk_force_on(),
            )
            .field("gdma_mem_clk_force_on", &self.gdma_mem_clk_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force on mem clk in rmt"]
    #[inline(always)]
    #[must_use]
    pub fn rmt_mem_clk_force_on(&mut self) -> RMT_MEM_CLK_FORCE_ON_W<PERI_MEM_CLK_FORCE_ON_SPEC> {
        RMT_MEM_CLK_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force on tx mem clk in bitscrambler"]
    #[inline(always)]
    #[must_use]
    pub fn bitscrambler_tx_mem_clk_force_on(
        &mut self,
    ) -> BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_W<PERI_MEM_CLK_FORCE_ON_SPEC> {
        BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force on rx mem clk in bitscrambler"]
    #[inline(always)]
    #[must_use]
    pub fn bitscrambler_rx_mem_clk_force_on(
        &mut self,
    ) -> BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_W<PERI_MEM_CLK_FORCE_ON_SPEC> {
        BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force on mem clk in gdma"]
    #[inline(always)]
    #[must_use]
    pub fn gdma_mem_clk_force_on(&mut self) -> GDMA_MEM_CLK_FORCE_ON_W<PERI_MEM_CLK_FORCE_ON_SPEC> {
        GDMA_MEM_CLK_FORCE_ON_W::new(self, 3)
    }
}
#[doc = "hp peri mem clk force on regpster\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_mem_clk_force_on::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_mem_clk_force_on::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_MEM_CLK_FORCE_ON_SPEC;
impl crate::RegisterSpec for PERI_MEM_CLK_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_mem_clk_force_on::R`](R) reader structure"]
impl crate::Readable for PERI_MEM_CLK_FORCE_ON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_mem_clk_force_on::W`](W) writer structure"]
impl crate::Writable for PERI_MEM_CLK_FORCE_ON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_MEM_CLK_FORCE_ON to value 0"]
impl crate::Resettable for PERI_MEM_CLK_FORCE_ON_SPEC {
    const RESET_VALUE: u32 = 0;
}
