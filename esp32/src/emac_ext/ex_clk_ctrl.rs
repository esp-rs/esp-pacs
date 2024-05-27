#[doc = "Register `EX_CLK_CTRL` reader"]
pub type R = crate::R<EX_CLK_CTRL_SPEC>;
#[doc = "Register `EX_CLK_CTRL` writer"]
pub type W = crate::W<EX_CLK_CTRL_SPEC>;
#[doc = "Field `EXT_EN` reader - "]
pub type EXT_EN_R = crate::BitReader;
#[doc = "Field `EXT_EN` writer - "]
pub type EXT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_EN` reader - "]
pub type INT_EN_R = crate::BitReader;
#[doc = "Field `INT_EN` writer - "]
pub type INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_125_CLK_EN` reader - "]
pub type RX_125_CLK_EN_R = crate::BitReader;
#[doc = "Field `RX_125_CLK_EN` writer - "]
pub type RX_125_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MII_CLK_TX_EN` reader - "]
pub type MII_CLK_TX_EN_R = crate::BitReader;
#[doc = "Field `MII_CLK_TX_EN` writer - "]
pub type MII_CLK_TX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MII_CLK_RX_EN` reader - "]
pub type MII_CLK_RX_EN_R = crate::BitReader;
#[doc = "Field `MII_CLK_RX_EN` writer - "]
pub type MII_CLK_RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - "]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ext_en(&self) -> EXT_EN_R {
        EXT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_125_clk_en(&self) -> RX_125_CLK_EN_R {
        RX_125_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mii_clk_tx_en(&self) -> MII_CLK_TX_EN_R {
        MII_CLK_TX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mii_clk_rx_en(&self) -> MII_CLK_RX_EN_R {
        MII_CLK_RX_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EX_CLK_CTRL")
            .field("ext_en", &self.ext_en())
            .field("int_en", &self.int_en())
            .field("rx_125_clk_en", &self.rx_125_clk_en())
            .field("mii_clk_tx_en", &self.mii_clk_tx_en())
            .field("mii_clk_rx_en", &self.mii_clk_rx_en())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ext_en(&mut self) -> EXT_EN_W<EX_CLK_CTRL_SPEC> {
        EXT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> INT_EN_W<EX_CLK_CTRL_SPEC> {
        INT_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rx_125_clk_en(&mut self) -> RX_125_CLK_EN_W<EX_CLK_CTRL_SPEC> {
        RX_125_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn mii_clk_tx_en(&mut self) -> MII_CLK_TX_EN_W<EX_CLK_CTRL_SPEC> {
        MII_CLK_TX_EN_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn mii_clk_rx_en(&mut self) -> MII_CLK_RX_EN_W<EX_CLK_CTRL_SPEC> {
        MII_CLK_RX_EN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<EX_CLK_CTRL_SPEC> {
        CLK_EN_W::new(self, 5)
    }
}
#[doc = "Clock enable and external/internal clock selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ex_clk_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ex_clk_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EX_CLK_CTRL_SPEC;
impl crate::RegisterSpec for EX_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ex_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for EX_CLK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ex_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for EX_CLK_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EX_CLK_CTRL to value 0"]
impl crate::Resettable for EX_CLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
