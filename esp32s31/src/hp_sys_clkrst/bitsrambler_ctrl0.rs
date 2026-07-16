#[doc = "Register `BITSRAMBLER_CTRL0` reader"]
pub type R = crate::R<BITSRAMBLER_CTRL0_SPEC>;
#[doc = "Register `BITSRAMBLER_CTRL0` writer"]
pub type W = crate::W<BITSRAMBLER_CTRL0_SPEC>;
#[doc = "Field `BITSRAMBLER_SYS_CLK_EN` reader - need_des"]
pub type BITSRAMBLER_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `BITSRAMBLER_SYS_CLK_EN` writer - need_des"]
pub type BITSRAMBLER_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSRAMBLER_RST_EN` reader - need_des"]
pub type BITSRAMBLER_RST_EN_R = crate::BitReader;
#[doc = "Field `BITSRAMBLER_RST_EN` writer - need_des"]
pub type BITSRAMBLER_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSRAMBLER_FORCE_NORST` reader - need_des"]
pub type BITSRAMBLER_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `BITSRAMBLER_FORCE_NORST` writer - need_des"]
pub type BITSRAMBLER_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSRAMBLER_RX_SYS_CLK_EN` reader - need_des"]
pub type BITSRAMBLER_RX_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `BITSRAMBLER_RX_SYS_CLK_EN` writer - need_des"]
pub type BITSRAMBLER_RX_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSRAMBLER_RX_RST_EN` reader - need_des"]
pub type BITSRAMBLER_RX_RST_EN_R = crate::BitReader;
#[doc = "Field `BITSRAMBLER_RX_RST_EN` writer - need_des"]
pub type BITSRAMBLER_RX_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSRAMBLER_RX_FORCE_NORST` reader - need_des"]
pub type BITSRAMBLER_RX_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `BITSRAMBLER_RX_FORCE_NORST` writer - need_des"]
pub type BITSRAMBLER_RX_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSRAMBLER_TX_SYS_CLK_EN` reader - need_des"]
pub type BITSRAMBLER_TX_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `BITSRAMBLER_TX_SYS_CLK_EN` writer - need_des"]
pub type BITSRAMBLER_TX_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSRAMBLER_TX_RST_EN` reader - need_des"]
pub type BITSRAMBLER_TX_RST_EN_R = crate::BitReader;
#[doc = "Field `BITSRAMBLER_TX_RST_EN` writer - need_des"]
pub type BITSRAMBLER_TX_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BITSRAMBLER_TX_FORCE_NORST` reader - need_des"]
pub type BITSRAMBLER_TX_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `BITSRAMBLER_TX_FORCE_NORST` writer - need_des"]
pub type BITSRAMBLER_TX_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_sys_clk_en(&self) -> BITSRAMBLER_SYS_CLK_EN_R {
        BITSRAMBLER_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_rst_en(&self) -> BITSRAMBLER_RST_EN_R {
        BITSRAMBLER_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_force_norst(&self) -> BITSRAMBLER_FORCE_NORST_R {
        BITSRAMBLER_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_rx_sys_clk_en(&self) -> BITSRAMBLER_RX_SYS_CLK_EN_R {
        BITSRAMBLER_RX_SYS_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_rx_rst_en(&self) -> BITSRAMBLER_RX_RST_EN_R {
        BITSRAMBLER_RX_RST_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_rx_force_norst(&self) -> BITSRAMBLER_RX_FORCE_NORST_R {
        BITSRAMBLER_RX_FORCE_NORST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_tx_sys_clk_en(&self) -> BITSRAMBLER_TX_SYS_CLK_EN_R {
        BITSRAMBLER_TX_SYS_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_tx_rst_en(&self) -> BITSRAMBLER_TX_RST_EN_R {
        BITSRAMBLER_TX_RST_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_tx_force_norst(&self) -> BITSRAMBLER_TX_FORCE_NORST_R {
        BITSRAMBLER_TX_FORCE_NORST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BITSRAMBLER_CTRL0")
            .field("bitsrambler_sys_clk_en", &self.bitsrambler_sys_clk_en())
            .field("bitsrambler_rst_en", &self.bitsrambler_rst_en())
            .field("bitsrambler_force_norst", &self.bitsrambler_force_norst())
            .field(
                "bitsrambler_rx_sys_clk_en",
                &self.bitsrambler_rx_sys_clk_en(),
            )
            .field("bitsrambler_rx_rst_en", &self.bitsrambler_rx_rst_en())
            .field(
                "bitsrambler_rx_force_norst",
                &self.bitsrambler_rx_force_norst(),
            )
            .field(
                "bitsrambler_tx_sys_clk_en",
                &self.bitsrambler_tx_sys_clk_en(),
            )
            .field("bitsrambler_tx_rst_en", &self.bitsrambler_tx_rst_en())
            .field(
                "bitsrambler_tx_force_norst",
                &self.bitsrambler_tx_force_norst(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_sys_clk_en(
        &mut self,
    ) -> BITSRAMBLER_SYS_CLK_EN_W<'_, BITSRAMBLER_CTRL0_SPEC> {
        BITSRAMBLER_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_rst_en(&mut self) -> BITSRAMBLER_RST_EN_W<'_, BITSRAMBLER_CTRL0_SPEC> {
        BITSRAMBLER_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_force_norst(
        &mut self,
    ) -> BITSRAMBLER_FORCE_NORST_W<'_, BITSRAMBLER_CTRL0_SPEC> {
        BITSRAMBLER_FORCE_NORST_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_rx_sys_clk_en(
        &mut self,
    ) -> BITSRAMBLER_RX_SYS_CLK_EN_W<'_, BITSRAMBLER_CTRL0_SPEC> {
        BITSRAMBLER_RX_SYS_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_rx_rst_en(&mut self) -> BITSRAMBLER_RX_RST_EN_W<'_, BITSRAMBLER_CTRL0_SPEC> {
        BITSRAMBLER_RX_RST_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_rx_force_norst(
        &mut self,
    ) -> BITSRAMBLER_RX_FORCE_NORST_W<'_, BITSRAMBLER_CTRL0_SPEC> {
        BITSRAMBLER_RX_FORCE_NORST_W::new(self, 5)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_tx_sys_clk_en(
        &mut self,
    ) -> BITSRAMBLER_TX_SYS_CLK_EN_W<'_, BITSRAMBLER_CTRL0_SPEC> {
        BITSRAMBLER_TX_SYS_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_tx_rst_en(&mut self) -> BITSRAMBLER_TX_RST_EN_W<'_, BITSRAMBLER_CTRL0_SPEC> {
        BITSRAMBLER_TX_RST_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn bitsrambler_tx_force_norst(
        &mut self,
    ) -> BITSRAMBLER_TX_FORCE_NORST_W<'_, BITSRAMBLER_CTRL0_SPEC> {
        BITSRAMBLER_TX_FORCE_NORST_W::new(self, 8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`bitsrambler_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bitsrambler_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BITSRAMBLER_CTRL0_SPEC;
impl crate::RegisterSpec for BITSRAMBLER_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bitsrambler_ctrl0::R`](R) reader structure"]
impl crate::Readable for BITSRAMBLER_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bitsrambler_ctrl0::W`](W) writer structure"]
impl crate::Writable for BITSRAMBLER_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BITSRAMBLER_CTRL0 to value 0"]
impl crate::Resettable for BITSRAMBLER_CTRL0_SPEC {}
