#[doc = "Register `HP_PERI_MEM_CLK_FORCE_ON` reader"]
pub type R = crate::R<HP_PERI_MEM_CLK_FORCE_ON_SPEC>;
#[doc = "Register `HP_PERI_MEM_CLK_FORCE_ON` writer"]
pub type W = crate::W<HP_PERI_MEM_CLK_FORCE_ON_SPEC>;
#[doc = "Field `HP_RMT_MEM_CLK_FORCE_ON` reader - Set this bit to force on mem clk in rmt"]
pub type HP_RMT_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `HP_RMT_MEM_CLK_FORCE_ON` writer - Set this bit to force on mem clk in rmt"]
pub type HP_RMT_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_BITSCRAMBLER_TX_MEM_CLK_FORCE_ON` reader - Set this bit to force on tx mem clk in bitscrambler"]
pub type HP_BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `HP_BITSCRAMBLER_TX_MEM_CLK_FORCE_ON` writer - Set this bit to force on tx mem clk in bitscrambler"]
pub type HP_BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_BITSCRAMBLER_RX_MEM_CLK_FORCE_ON` reader - Set this bit to force on rx mem clk in bitscrambler"]
pub type HP_BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `HP_BITSCRAMBLER_RX_MEM_CLK_FORCE_ON` writer - Set this bit to force on rx mem clk in bitscrambler"]
pub type HP_BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_GDMA_MEM_CLK_FORCE_ON` reader - Set this bit to force on mem clk in gdma"]
pub type HP_GDMA_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `HP_GDMA_MEM_CLK_FORCE_ON` writer - Set this bit to force on mem clk in gdma"]
pub type HP_GDMA_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to force on mem clk in rmt"]
    #[inline(always)]
    pub fn hp_rmt_mem_clk_force_on(&self) -> HP_RMT_MEM_CLK_FORCE_ON_R {
        HP_RMT_MEM_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force on tx mem clk in bitscrambler"]
    #[inline(always)]
    pub fn hp_bitscrambler_tx_mem_clk_force_on(&self) -> HP_BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_R {
        HP_BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force on rx mem clk in bitscrambler"]
    #[inline(always)]
    pub fn hp_bitscrambler_rx_mem_clk_force_on(&self) -> HP_BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_R {
        HP_BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force on mem clk in gdma"]
    #[inline(always)]
    pub fn hp_gdma_mem_clk_force_on(&self) -> HP_GDMA_MEM_CLK_FORCE_ON_R {
        HP_GDMA_MEM_CLK_FORCE_ON_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_PERI_MEM_CLK_FORCE_ON")
            .field("hp_rmt_mem_clk_force_on", &self.hp_rmt_mem_clk_force_on())
            .field(
                "hp_bitscrambler_tx_mem_clk_force_on",
                &self.hp_bitscrambler_tx_mem_clk_force_on(),
            )
            .field(
                "hp_bitscrambler_rx_mem_clk_force_on",
                &self.hp_bitscrambler_rx_mem_clk_force_on(),
            )
            .field("hp_gdma_mem_clk_force_on", &self.hp_gdma_mem_clk_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to force on mem clk in rmt"]
    #[inline(always)]
    pub fn hp_rmt_mem_clk_force_on(
        &mut self,
    ) -> HP_RMT_MEM_CLK_FORCE_ON_W<'_, HP_PERI_MEM_CLK_FORCE_ON_SPEC> {
        HP_RMT_MEM_CLK_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force on tx mem clk in bitscrambler"]
    #[inline(always)]
    pub fn hp_bitscrambler_tx_mem_clk_force_on(
        &mut self,
    ) -> HP_BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_W<'_, HP_PERI_MEM_CLK_FORCE_ON_SPEC> {
        HP_BITSCRAMBLER_TX_MEM_CLK_FORCE_ON_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force on rx mem clk in bitscrambler"]
    #[inline(always)]
    pub fn hp_bitscrambler_rx_mem_clk_force_on(
        &mut self,
    ) -> HP_BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_W<'_, HP_PERI_MEM_CLK_FORCE_ON_SPEC> {
        HP_BITSCRAMBLER_RX_MEM_CLK_FORCE_ON_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force on mem clk in gdma"]
    #[inline(always)]
    pub fn hp_gdma_mem_clk_force_on(
        &mut self,
    ) -> HP_GDMA_MEM_CLK_FORCE_ON_W<'_, HP_PERI_MEM_CLK_FORCE_ON_SPEC> {
        HP_GDMA_MEM_CLK_FORCE_ON_W::new(self, 3)
    }
}
#[doc = "hp peri mem clk force on regpster\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_peri_mem_clk_force_on::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_peri_mem_clk_force_on::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_PERI_MEM_CLK_FORCE_ON_SPEC;
impl crate::RegisterSpec for HP_PERI_MEM_CLK_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_peri_mem_clk_force_on::R`](R) reader structure"]
impl crate::Readable for HP_PERI_MEM_CLK_FORCE_ON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_peri_mem_clk_force_on::W`](W) writer structure"]
impl crate::Writable for HP_PERI_MEM_CLK_FORCE_ON_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_PERI_MEM_CLK_FORCE_ON to value 0"]
impl crate::Resettable for HP_PERI_MEM_CLK_FORCE_ON_SPEC {}
