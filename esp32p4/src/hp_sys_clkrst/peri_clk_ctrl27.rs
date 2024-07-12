#[doc = "Register `PERI_CLK_CTRL27` reader"]
pub type R = crate::R<PERI_CLK_CTRL27_SPEC>;
#[doc = "Register `PERI_CLK_CTRL27` writer"]
pub type W = crate::W<PERI_CLK_CTRL27_SPEC>;
#[doc = "Field `PADBIST_RX_CLK_DIV_NUM` reader - Reserved"]
pub type PADBIST_RX_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PADBIST_RX_CLK_DIV_NUM` writer - Reserved"]
pub type PADBIST_RX_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PADBIST_TX_CLK_SRC_SEL` reader - Reserved"]
pub type PADBIST_TX_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `PADBIST_TX_CLK_SRC_SEL` writer - Reserved"]
pub type PADBIST_TX_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADBIST_TX_CLK_EN` reader - Reserved"]
pub type PADBIST_TX_CLK_EN_R = crate::BitReader;
#[doc = "Field `PADBIST_TX_CLK_EN` writer - Reserved"]
pub type PADBIST_TX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADBIST_TX_CLK_DIV_NUM` reader - Reserved"]
pub type PADBIST_TX_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PADBIST_TX_CLK_DIV_NUM` writer - Reserved"]
pub type PADBIST_TX_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn padbist_rx_clk_div_num(&self) -> PADBIST_RX_CLK_DIV_NUM_R {
        PADBIST_RX_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn padbist_tx_clk_src_sel(&self) -> PADBIST_TX_CLK_SRC_SEL_R {
        PADBIST_TX_CLK_SRC_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn padbist_tx_clk_en(&self) -> PADBIST_TX_CLK_EN_R {
        PADBIST_TX_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    pub fn padbist_tx_clk_div_num(&self) -> PADBIST_TX_CLK_DIV_NUM_R {
        PADBIST_TX_CLK_DIV_NUM_R::new(((self.bits >> 10) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL27")
            .field("padbist_rx_clk_div_num", &self.padbist_rx_clk_div_num())
            .field("padbist_tx_clk_src_sel", &self.padbist_tx_clk_src_sel())
            .field("padbist_tx_clk_en", &self.padbist_tx_clk_en())
            .field("padbist_tx_clk_div_num", &self.padbist_tx_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn padbist_rx_clk_div_num(&mut self) -> PADBIST_RX_CLK_DIV_NUM_W<PERI_CLK_CTRL27_SPEC> {
        PADBIST_RX_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn padbist_tx_clk_src_sel(&mut self) -> PADBIST_TX_CLK_SRC_SEL_W<PERI_CLK_CTRL27_SPEC> {
        PADBIST_TX_CLK_SRC_SEL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn padbist_tx_clk_en(&mut self) -> PADBIST_TX_CLK_EN_W<PERI_CLK_CTRL27_SPEC> {
        PADBIST_TX_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn padbist_tx_clk_div_num(&mut self) -> PADBIST_TX_CLK_DIV_NUM_W<PERI_CLK_CTRL27_SPEC> {
        PADBIST_TX_CLK_DIV_NUM_W::new(self, 10)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL27_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL27_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl27::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL27_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl27::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL27_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL27 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL27_SPEC {
    const RESET_VALUE: u32 = 0;
}
