#[doc = "Register `SYS_HP_EMAC_TX_CTRL` reader"]
pub type R = crate::R<SYS_HP_EMAC_TX_CTRL_SPEC>;
#[doc = "Register `SYS_HP_EMAC_TX_CTRL` writer"]
pub type W = crate::W<SYS_HP_EMAC_TX_CTRL_SPEC>;
#[doc = "Field `SYS_EMAC_TX_PAD_CLK_EN` reader - "]
pub type SYS_EMAC_TX_PAD_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYS_EMAC_TX_PAD_CLK_EN` writer - "]
pub type SYS_EMAC_TX_PAD_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_EMAC_TX_PAD_CLK_INV_EN` reader - "]
pub type SYS_EMAC_TX_PAD_CLK_INV_EN_R = crate::BitReader;
#[doc = "Field `SYS_EMAC_TX_PAD_CLK_INV_EN` writer - "]
pub type SYS_EMAC_TX_PAD_CLK_INV_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_EMAC_TX_CLK_SEL` reader - "]
pub type SYS_EMAC_TX_CLK_SEL_R = crate::BitReader;
#[doc = "Field `SYS_EMAC_TX_CLK_SEL` writer - "]
pub type SYS_EMAC_TX_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_EMAC_TX_180_CLK_EN` reader - "]
pub type SYS_EMAC_TX_180_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYS_EMAC_TX_180_CLK_EN` writer - "]
pub type SYS_EMAC_TX_180_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sys_emac_tx_pad_clk_en(&self) -> SYS_EMAC_TX_PAD_CLK_EN_R {
        SYS_EMAC_TX_PAD_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sys_emac_tx_pad_clk_inv_en(&self) -> SYS_EMAC_TX_PAD_CLK_INV_EN_R {
        SYS_EMAC_TX_PAD_CLK_INV_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sys_emac_tx_clk_sel(&self) -> SYS_EMAC_TX_CLK_SEL_R {
        SYS_EMAC_TX_CLK_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sys_emac_tx_180_clk_en(&self) -> SYS_EMAC_TX_180_CLK_EN_R {
        SYS_EMAC_TX_180_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_HP_EMAC_TX_CTRL")
            .field("sys_emac_tx_pad_clk_en", &self.sys_emac_tx_pad_clk_en())
            .field(
                "sys_emac_tx_pad_clk_inv_en",
                &self.sys_emac_tx_pad_clk_inv_en(),
            )
            .field("sys_emac_tx_clk_sel", &self.sys_emac_tx_clk_sel())
            .field("sys_emac_tx_180_clk_en", &self.sys_emac_tx_180_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sys_emac_tx_pad_clk_en(
        &mut self,
    ) -> SYS_EMAC_TX_PAD_CLK_EN_W<'_, SYS_HP_EMAC_TX_CTRL_SPEC> {
        SYS_EMAC_TX_PAD_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sys_emac_tx_pad_clk_inv_en(
        &mut self,
    ) -> SYS_EMAC_TX_PAD_CLK_INV_EN_W<'_, SYS_HP_EMAC_TX_CTRL_SPEC> {
        SYS_EMAC_TX_PAD_CLK_INV_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sys_emac_tx_clk_sel(&mut self) -> SYS_EMAC_TX_CLK_SEL_W<'_, SYS_HP_EMAC_TX_CTRL_SPEC> {
        SYS_EMAC_TX_CLK_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sys_emac_tx_180_clk_en(
        &mut self,
    ) -> SYS_EMAC_TX_180_CLK_EN_W<'_, SYS_HP_EMAC_TX_CTRL_SPEC> {
        SYS_EMAC_TX_180_CLK_EN_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_emac_tx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_emac_tx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_HP_EMAC_TX_CTRL_SPEC;
impl crate::RegisterSpec for SYS_HP_EMAC_TX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_hp_emac_tx_ctrl::R`](R) reader structure"]
impl crate::Readable for SYS_HP_EMAC_TX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_hp_emac_tx_ctrl::W`](W) writer structure"]
impl crate::Writable for SYS_HP_EMAC_TX_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_HP_EMAC_TX_CTRL to value 0"]
impl crate::Resettable for SYS_HP_EMAC_TX_CTRL_SPEC {}
