#[doc = "Register `MODEM_CONF` reader"]
pub type R = crate::R<MODEM_CONF_SPEC>;
#[doc = "Register `MODEM_CONF` writer"]
pub type W = crate::W<MODEM_CONF_SPEC>;
#[doc = "Field `MODEM_CLK_SEL` reader - xxxx"]
pub type MODEM_CLK_SEL_R = crate::BitReader;
#[doc = "Field `MODEM_CLK_SEL` writer - xxxx"]
pub type MODEM_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_CLK_EN` reader - xxxx"]
pub type MODEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `MODEM_CLK_EN` writer - xxxx"]
pub type MODEM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_RST_EN` reader - Set this file as 1 to reset modem-subsystem."]
pub type MODEM_RST_EN_R = crate::BitReader;
#[doc = "Field `MODEM_RST_EN` writer - Set this file as 1 to reset modem-subsystem."]
pub type MODEM_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn modem_clk_sel(&self) -> MODEM_CLK_SEL_R {
        MODEM_CLK_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - xxxx"]
    #[inline(always)]
    pub fn modem_clk_en(&self) -> MODEM_CLK_EN_R {
        MODEM_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this file as 1 to reset modem-subsystem."]
    #[inline(always)]
    pub fn modem_rst_en(&self) -> MODEM_RST_EN_R {
        MODEM_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_CONF")
            .field("modem_clk_sel", &self.modem_clk_sel())
            .field("modem_clk_en", &self.modem_clk_en())
            .field("modem_rst_en", &self.modem_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn modem_clk_sel(&mut self) -> MODEM_CLK_SEL_W<'_, MODEM_CONF_SPEC> {
        MODEM_CLK_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - xxxx"]
    #[inline(always)]
    pub fn modem_clk_en(&mut self) -> MODEM_CLK_EN_W<'_, MODEM_CONF_SPEC> {
        MODEM_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this file as 1 to reset modem-subsystem."]
    #[inline(always)]
    pub fn modem_rst_en(&mut self) -> MODEM_RST_EN_W<'_, MODEM_CONF_SPEC> {
        MODEM_RST_EN_W::new(self, 2)
    }
}
#[doc = "MODEM_APB configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_CONF_SPEC;
impl crate::RegisterSpec for MODEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_conf::R`](R) reader structure"]
impl crate::Readable for MODEM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_conf::W`](W) writer structure"]
impl crate::Writable for MODEM_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEM_CONF to value 0x02"]
impl crate::Resettable for MODEM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
