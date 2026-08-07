#[doc = "Register `PCNT_CONF` reader"]
pub type R = crate::R<PCNT_CONF_SPEC>;
#[doc = "Register `PCNT_CONF` writer"]
pub type W = crate::W<PCNT_CONF_SPEC>;
#[doc = "Field `PCNT_CLK_EN` reader - Set 1 to enable pcnt clock"]
pub type PCNT_CLK_EN_R = crate::BitReader;
#[doc = "Field `PCNT_CLK_EN` writer - Set 1 to enable pcnt clock"]
pub type PCNT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT_RST_EN` reader - Set 1 to reset pcnt module"]
pub type PCNT_RST_EN_R = crate::BitReader;
#[doc = "Field `PCNT_RST_EN` writer - Set 1 to reset pcnt module"]
pub type PCNT_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT_REG_CLK_EN` reader - Set 1 to enable pcnt reg clock"]
pub type PCNT_REG_CLK_EN_R = crate::BitReader;
#[doc = "Field `PCNT_REG_CLK_EN` writer - Set 1 to enable pcnt reg clock"]
pub type PCNT_REG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT_REG_RST_EN` reader - Set 1 to reset pcnt reg module"]
pub type PCNT_REG_RST_EN_R = crate::BitReader;
#[doc = "Field `PCNT_REG_RST_EN` writer - Set 1 to reset pcnt reg module"]
pub type PCNT_REG_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT_READY` reader - Query this field after reset pcnt module"]
pub type PCNT_READY_R = crate::BitReader;
#[doc = "Field `PCNT_CLK_SEL` reader - Configures the clock source of the pcnt.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F32M_CLK\\\\"]
pub type PCNT_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `PCNT_CLK_SEL` writer - Configures the clock source of the pcnt.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F32M_CLK\\\\"]
pub type PCNT_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCNT_CLK_DIV_NUM` reader - The integral part of the frequency divider factor of the pcnt function clock."]
pub type PCNT_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PCNT_CLK_DIV_NUM` writer - The integral part of the frequency divider factor of the pcnt function clock."]
pub type PCNT_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable pcnt clock"]
    #[inline(always)]
    pub fn pcnt_clk_en(&self) -> PCNT_CLK_EN_R {
        PCNT_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset pcnt module"]
    #[inline(always)]
    pub fn pcnt_rst_en(&self) -> PCNT_RST_EN_R {
        PCNT_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable pcnt reg clock"]
    #[inline(always)]
    pub fn pcnt_reg_clk_en(&self) -> PCNT_REG_CLK_EN_R {
        PCNT_REG_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to reset pcnt reg module"]
    #[inline(always)]
    pub fn pcnt_reg_rst_en(&self) -> PCNT_REG_RST_EN_R {
        PCNT_REG_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Query this field after reset pcnt module"]
    #[inline(always)]
    pub fn pcnt_ready(&self) -> PCNT_READY_R {
        PCNT_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Configures the clock source of the pcnt.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F32M_CLK\\\\"]
    #[inline(always)]
    pub fn pcnt_clk_sel(&self) -> PCNT_CLK_SEL_R {
        PCNT_CLK_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:10 - The integral part of the frequency divider factor of the pcnt function clock."]
    #[inline(always)]
    pub fn pcnt_clk_div_num(&self) -> PCNT_CLK_DIV_NUM_R {
        PCNT_CLK_DIV_NUM_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNT_CONF")
            .field("pcnt_clk_en", &self.pcnt_clk_en())
            .field("pcnt_rst_en", &self.pcnt_rst_en())
            .field("pcnt_reg_clk_en", &self.pcnt_reg_clk_en())
            .field("pcnt_reg_rst_en", &self.pcnt_reg_rst_en())
            .field("pcnt_ready", &self.pcnt_ready())
            .field("pcnt_clk_sel", &self.pcnt_clk_sel())
            .field("pcnt_clk_div_num", &self.pcnt_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable pcnt clock"]
    #[inline(always)]
    pub fn pcnt_clk_en(&mut self) -> PCNT_CLK_EN_W<'_, PCNT_CONF_SPEC> {
        PCNT_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset pcnt module"]
    #[inline(always)]
    pub fn pcnt_rst_en(&mut self) -> PCNT_RST_EN_W<'_, PCNT_CONF_SPEC> {
        PCNT_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to enable pcnt reg clock"]
    #[inline(always)]
    pub fn pcnt_reg_clk_en(&mut self) -> PCNT_REG_CLK_EN_W<'_, PCNT_CONF_SPEC> {
        PCNT_REG_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set 1 to reset pcnt reg module"]
    #[inline(always)]
    pub fn pcnt_reg_rst_en(&mut self) -> PCNT_REG_RST_EN_W<'_, PCNT_CONF_SPEC> {
        PCNT_REG_RST_EN_W::new(self, 3)
    }
    #[doc = "Bits 5:6 - Configures the clock source of the pcnt.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F32M_CLK\\\\"]
    #[inline(always)]
    pub fn pcnt_clk_sel(&mut self) -> PCNT_CLK_SEL_W<'_, PCNT_CONF_SPEC> {
        PCNT_CLK_SEL_W::new(self, 5)
    }
    #[doc = "Bits 7:10 - The integral part of the frequency divider factor of the pcnt function clock."]
    #[inline(always)]
    pub fn pcnt_clk_div_num(&mut self) -> PCNT_CLK_DIV_NUM_W<'_, PCNT_CONF_SPEC> {
        PCNT_CLK_DIV_NUM_W::new(self, 7)
    }
}
#[doc = "PCNT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCNT_CONF_SPEC;
impl crate::RegisterSpec for PCNT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnt_conf::R`](R) reader structure"]
impl crate::Readable for PCNT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcnt_conf::W`](W) writer structure"]
impl crate::Writable for PCNT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNT_CONF to value 0x10"]
impl crate::Resettable for PCNT_CONF_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
