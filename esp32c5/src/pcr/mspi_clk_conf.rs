#[doc = "Register `MSPI_CLK_CONF` reader"]
pub type R = crate::R<MSPI_CLK_CONF_SPEC>;
#[doc = "Register `MSPI_CLK_CONF` writer"]
pub type W = crate::W<MSPI_CLK_CONF_SPEC>;
#[doc = "Field `MSPI_FAST_DIV_NUM` reader - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
pub type MSPI_FAST_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `MSPI_FAST_DIV_NUM` writer - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
pub type MSPI_FAST_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MSPI_FUNC_CLK_SEL` reader - Configures the clock source for MSPI.\\\\ 0(default): XTAL_CLK\\\\ 1 RC_FAST_CLK\\\\ 2: PLL_F480M_CLK\\\\"]
pub type MSPI_FUNC_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `MSPI_FUNC_CLK_SEL` writer - Configures the clock source for MSPI.\\\\ 0(default): XTAL_CLK\\\\ 1 RC_FAST_CLK\\\\ 2: PLL_F480M_CLK\\\\"]
pub type MSPI_FUNC_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MSPI_FUNC_CLK_EN` reader - Set 1 to enable mspi func clock"]
pub type MSPI_FUNC_CLK_EN_R = crate::BitReader;
#[doc = "Field `MSPI_FUNC_CLK_EN` writer - Set 1 to enable mspi func clock"]
pub type MSPI_FUNC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSPI_AXI_RST_EN` reader - Set 0 to reset axi_clock domain of mspi module"]
pub type MSPI_AXI_RST_EN_R = crate::BitReader;
#[doc = "Field `MSPI_AXI_RST_EN` writer - Set 0 to reset axi_clock domain of mspi module"]
pub type MSPI_AXI_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
    #[inline(always)]
    pub fn mspi_fast_div_num(&self) -> MSPI_FAST_DIV_NUM_R {
        MSPI_FAST_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Configures the clock source for MSPI.\\\\ 0(default): XTAL_CLK\\\\ 1 RC_FAST_CLK\\\\ 2: PLL_F480M_CLK\\\\"]
    #[inline(always)]
    pub fn mspi_func_clk_sel(&self) -> MSPI_FUNC_CLK_SEL_R {
        MSPI_FUNC_CLK_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Set 1 to enable mspi func clock"]
    #[inline(always)]
    pub fn mspi_func_clk_en(&self) -> MSPI_FUNC_CLK_EN_R {
        MSPI_FUNC_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set 0 to reset axi_clock domain of mspi module"]
    #[inline(always)]
    pub fn mspi_axi_rst_en(&self) -> MSPI_AXI_RST_EN_R {
        MSPI_AXI_RST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSPI_CLK_CONF")
            .field("mspi_fast_div_num", &self.mspi_fast_div_num())
            .field("mspi_func_clk_sel", &self.mspi_func_clk_sel())
            .field("mspi_func_clk_en", &self.mspi_func_clk_en())
            .field("mspi_axi_rst_en", &self.mspi_axi_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
    #[inline(always)]
    pub fn mspi_fast_div_num(&mut self) -> MSPI_FAST_DIV_NUM_W<MSPI_CLK_CONF_SPEC> {
        MSPI_FAST_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Configures the clock source for MSPI.\\\\ 0(default): XTAL_CLK\\\\ 1 RC_FAST_CLK\\\\ 2: PLL_F480M_CLK\\\\"]
    #[inline(always)]
    pub fn mspi_func_clk_sel(&mut self) -> MSPI_FUNC_CLK_SEL_W<MSPI_CLK_CONF_SPEC> {
        MSPI_FUNC_CLK_SEL_W::new(self, 8)
    }
    #[doc = "Bit 10 - Set 1 to enable mspi func clock"]
    #[inline(always)]
    pub fn mspi_func_clk_en(&mut self) -> MSPI_FUNC_CLK_EN_W<MSPI_CLK_CONF_SPEC> {
        MSPI_FUNC_CLK_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set 0 to reset axi_clock domain of mspi module"]
    #[inline(always)]
    pub fn mspi_axi_rst_en(&mut self) -> MSPI_AXI_RST_EN_W<MSPI_CLK_CONF_SPEC> {
        MSPI_AXI_RST_EN_W::new(self, 11)
    }
}
#[doc = "MSPI_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspi_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspi_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSPI_CLK_CONF_SPEC;
impl crate::RegisterSpec for MSPI_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mspi_clk_conf::R`](R) reader structure"]
impl crate::Readable for MSPI_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mspi_clk_conf::W`](W) writer structure"]
impl crate::Writable for MSPI_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSPI_CLK_CONF to value 0x0400"]
impl crate::Resettable for MSPI_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0400;
}
