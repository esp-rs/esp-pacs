#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `LP_REG_DUMMY_OUT` reader - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
pub type LP_REG_DUMMY_OUT_R = crate::BitReader;
#[doc = "Field `LP_REG_DUMMY_OUT` writer - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
pub type LP_REG_DUMMY_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type LP_REG_Q_POL_R = crate::BitReader;
#[doc = "Field `LP_REG_Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type LP_REG_Q_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type LP_REG_D_POL_R = crate::BitReader;
#[doc = "Field `LP_REG_D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type LP_REG_D_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_RD_BIT_ORDER` reader - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
pub type LP_REG_RD_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `LP_REG_RD_BIT_ORDER` writer - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
pub type LP_REG_RD_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_WR_BIT_ORDER` reader - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
pub type LP_REG_WR_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `LP_REG_WR_BIT_ORDER` writer - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
pub type LP_REG_WR_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_dummy_out(&self) -> LP_REG_DUMMY_OUT_R {
        LP_REG_DUMMY_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_q_pol(&self) -> LP_REG_Q_POL_R {
        LP_REG_Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_d_pol(&self) -> LP_REG_D_POL_R {
        LP_REG_D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25 - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_rd_bit_order(&self) -> LP_REG_RD_BIT_ORDER_R {
        LP_REG_RD_BIT_ORDER_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_wr_bit_order(&self) -> LP_REG_WR_BIT_ORDER_R {
        LP_REG_WR_BIT_ORDER_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("lp_reg_dummy_out", &self.lp_reg_dummy_out())
            .field("lp_reg_q_pol", &self.lp_reg_q_pol())
            .field("lp_reg_d_pol", &self.lp_reg_d_pol())
            .field("lp_reg_rd_bit_order", &self.lp_reg_rd_bit_order())
            .field("lp_reg_wr_bit_order", &self.lp_reg_wr_bit_order())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_dummy_out(&mut self) -> LP_REG_DUMMY_OUT_W<'_, CTRL_SPEC> {
        LP_REG_DUMMY_OUT_W::new(self, 3)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_q_pol(&mut self) -> LP_REG_Q_POL_W<'_, CTRL_SPEC> {
        LP_REG_Q_POL_W::new(self, 18)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_d_pol(&mut self) -> LP_REG_D_POL_W<'_, CTRL_SPEC> {
        LP_REG_D_POL_W::new(self, 19)
    }
    #[doc = "Bit 25 - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_rd_bit_order(&mut self) -> LP_REG_RD_BIT_ORDER_W<'_, CTRL_SPEC> {
        LP_REG_RD_BIT_ORDER_W::new(self, 25)
    }
    #[doc = "Bit 26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_wr_bit_order(&mut self) -> LP_REG_WR_BIT_ORDER_W<'_, CTRL_SPEC> {
        LP_REG_WR_BIT_ORDER_W::new(self, 26)
    }
}
#[doc = "SPI control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {}
