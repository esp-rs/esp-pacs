#[doc = "Register `BS_FUNC_CONF` reader"]
pub type R = crate::R<BS_FUNC_CONF_SPEC>;
#[doc = "Register `BS_FUNC_CONF` writer"]
pub type W = crate::W<BS_FUNC_CONF_SPEC>;
#[doc = "Field `BS_TX_RST_EN` reader - Set 0 to reset bs tx module"]
pub type BS_TX_RST_EN_R = crate::BitReader;
#[doc = "Field `BS_TX_RST_EN` writer - Set 0 to reset bs tx module"]
pub type BS_TX_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS_RX_RST_EN` reader - Set 0 to reset bs rx module"]
pub type BS_RX_RST_EN_R = crate::BitReader;
#[doc = "Field `BS_RX_RST_EN` writer - Set 0 to reset bs rx module"]
pub type BS_RX_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 23 - Set 0 to reset bs tx module"]
    #[inline(always)]
    pub fn bs_tx_rst_en(&self) -> BS_TX_RST_EN_R {
        BS_TX_RST_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set 0 to reset bs rx module"]
    #[inline(always)]
    pub fn bs_rx_rst_en(&self) -> BS_RX_RST_EN_R {
        BS_RX_RST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BS_FUNC_CONF")
            .field("bs_tx_rst_en", &self.bs_tx_rst_en())
            .field("bs_rx_rst_en", &self.bs_rx_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 23 - Set 0 to reset bs tx module"]
    #[inline(always)]
    pub fn bs_tx_rst_en(&mut self) -> BS_TX_RST_EN_W<'_, BS_FUNC_CONF_SPEC> {
        BS_TX_RST_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - Set 0 to reset bs rx module"]
    #[inline(always)]
    pub fn bs_rx_rst_en(&mut self) -> BS_RX_RST_EN_W<'_, BS_FUNC_CONF_SPEC> {
        BS_RX_RST_EN_W::new(self, 24)
    }
}
#[doc = "BS_FUNC_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`bs_func_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bs_func_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BS_FUNC_CONF_SPEC;
impl crate::RegisterSpec for BS_FUNC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bs_func_conf::R`](R) reader structure"]
impl crate::Readable for BS_FUNC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bs_func_conf::W`](W) writer structure"]
impl crate::Writable for BS_FUNC_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BS_FUNC_CONF to value 0"]
impl crate::Resettable for BS_FUNC_CONF_SPEC {}
