#[doc = "Register `RST_CONF` reader"]
pub type R = crate::R<RST_CONF_SPEC>;
#[doc = "Register `RST_CONF` writer"]
pub type W = crate::W<RST_CONF_SPEC>;
#[doc = "Field `AXIM_RD_RST` reader - Write 1 then write 0 to this bit to reset axi master read data FIFO."]
pub type AXIM_RD_RST_R = crate::BitReader;
#[doc = "Field `AXIM_RD_RST` writer - Write 1 then write 0 to this bit to reset axi master read data FIFO."]
pub type AXIM_RD_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIM_WR_RST` reader - Write 1 then write 0 to this bit to reset axi master write data FIFO."]
pub type AXIM_WR_RST_R = crate::BitReader;
#[doc = "Field `AXIM_WR_RST` writer - Write 1 then write 0 to this bit to reset axi master write data FIFO."]
pub type AXIM_WR_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 then write 0 to this bit to reset axi master read data FIFO."]
    #[inline(always)]
    pub fn axim_rd_rst(&self) -> AXIM_RD_RST_R {
        AXIM_RD_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 then write 0 to this bit to reset axi master write data FIFO."]
    #[inline(always)]
    pub fn axim_wr_rst(&self) -> AXIM_WR_RST_R {
        AXIM_WR_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RST_CONF")
            .field("axim_rd_rst", &self.axim_rd_rst())
            .field("axim_wr_rst", &self.axim_wr_rst())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 then write 0 to this bit to reset axi master read data FIFO."]
    #[inline(always)]
    pub fn axim_rd_rst(&mut self) -> AXIM_RD_RST_W<'_, RST_CONF_SPEC> {
        AXIM_RD_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 then write 0 to this bit to reset axi master write data FIFO."]
    #[inline(always)]
    pub fn axim_wr_rst(&mut self) -> AXIM_WR_RST_W<'_, RST_CONF_SPEC> {
        AXIM_WR_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, RST_CONF_SPEC> {
        CLK_EN_W::new(self, 2)
    }
}
#[doc = "Configures the reset of axi\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_CONF_SPEC;
impl crate::RegisterSpec for RST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst_conf::R`](R) reader structure"]
impl crate::Readable for RST_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rst_conf::W`](W) writer structure"]
impl crate::Writable for RST_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RST_CONF to value 0"]
impl crate::Resettable for RST_CONF_SPEC {}
