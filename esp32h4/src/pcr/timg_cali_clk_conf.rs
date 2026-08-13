#[doc = "Register `TIMG_CALI_CLK_CONF` reader"]
pub type R = crate::R<TIMG_CALI_CLK_CONF_SPEC>;
#[doc = "Register `TIMG_CALI_CLK_CONF` writer"]
pub type W = crate::W<TIMG_CALI_CLK_CONF_SPEC>;
#[doc = "Field `TIMG_CALI_CLK_SEL` reader - Configures the 32KHz clock for TIMER_GROUP.\\\\ 0 (default): RC32K_CLK\\\\ 1: XTAL32K_CLK\\\\ 2: OSC_SLOW_CLK\\\\ 3: RC_SLOW_CLK\\\\ 4: TIMG_SECURE_CLK\\\\"]
pub type TIMG_CALI_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `TIMG_CALI_CLK_SEL` writer - Configures the 32KHz clock for TIMER_GROUP.\\\\ 0 (default): RC32K_CLK\\\\ 1: XTAL32K_CLK\\\\ 2: OSC_SLOW_CLK\\\\ 3: RC_SLOW_CLK\\\\ 4: TIMG_SECURE_CLK\\\\"]
pub type TIMG_CALI_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TIMG_SECURE_CLK_SEL` reader - Configures the clock source for the TIMG_SECURE_CLK.\\\\ 0 (default):CPU_CLK\\\\ 1: AHB_CLK\\\\ 2: APB_CLK\\\\ 3: sec function clock\\\\ 4: mspi function clock\\\\ 5: iomux function clock\\\\ 6: parl io rx function clock\\\\ 7: parl io tx function clock\\\\ 8: spi2 function clock\\\\ 9: spi3 function clock\\\\"]
pub type TIMG_SECURE_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `TIMG_SECURE_CLK_SEL` writer - Configures the clock source for the TIMG_SECURE_CLK.\\\\ 0 (default):CPU_CLK\\\\ 1: AHB_CLK\\\\ 2: APB_CLK\\\\ 3: sec function clock\\\\ 4: mspi function clock\\\\ 5: iomux function clock\\\\ 6: parl io rx function clock\\\\ 7: parl io tx function clock\\\\ 8: spi2 function clock\\\\ 9: spi3 function clock\\\\"]
pub type TIMG_SECURE_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIMG_SECURE_CLK_DIV_NUM` reader - When PCR_TIMG_CALI_CLK_SEL set as 4, This field PCR_TIMG_SECURE_CLK_DIV_NUM is used to set the divider number of TIMG_SECURE_CLK."]
pub type TIMG_SECURE_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `TIMG_SECURE_CLK_DIV_NUM` writer - When PCR_TIMG_CALI_CLK_SEL set as 4, This field PCR_TIMG_SECURE_CLK_DIV_NUM is used to set the divider number of TIMG_SECURE_CLK."]
pub type TIMG_SECURE_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Configures the 32KHz clock for TIMER_GROUP.\\\\ 0 (default): RC32K_CLK\\\\ 1: XTAL32K_CLK\\\\ 2: OSC_SLOW_CLK\\\\ 3: RC_SLOW_CLK\\\\ 4: TIMG_SECURE_CLK\\\\"]
    #[inline(always)]
    pub fn timg_cali_clk_sel(&self) -> TIMG_CALI_CLK_SEL_R {
        TIMG_CALI_CLK_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Configures the clock source for the TIMG_SECURE_CLK.\\\\ 0 (default):CPU_CLK\\\\ 1: AHB_CLK\\\\ 2: APB_CLK\\\\ 3: sec function clock\\\\ 4: mspi function clock\\\\ 5: iomux function clock\\\\ 6: parl io rx function clock\\\\ 7: parl io tx function clock\\\\ 8: spi2 function clock\\\\ 9: spi3 function clock\\\\"]
    #[inline(always)]
    pub fn timg_secure_clk_sel(&self) -> TIMG_SECURE_CLK_SEL_R {
        TIMG_SECURE_CLK_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - When PCR_TIMG_CALI_CLK_SEL set as 4, This field PCR_TIMG_SECURE_CLK_DIV_NUM is used to set the divider number of TIMG_SECURE_CLK."]
    #[inline(always)]
    pub fn timg_secure_clk_div_num(&self) -> TIMG_SECURE_CLK_DIV_NUM_R {
        TIMG_SECURE_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMG_CALI_CLK_CONF")
            .field("timg_cali_clk_sel", &self.timg_cali_clk_sel())
            .field("timg_secure_clk_sel", &self.timg_secure_clk_sel())
            .field("timg_secure_clk_div_num", &self.timg_secure_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures the 32KHz clock for TIMER_GROUP.\\\\ 0 (default): RC32K_CLK\\\\ 1: XTAL32K_CLK\\\\ 2: OSC_SLOW_CLK\\\\ 3: RC_SLOW_CLK\\\\ 4: TIMG_SECURE_CLK\\\\"]
    #[inline(always)]
    pub fn timg_cali_clk_sel(&mut self) -> TIMG_CALI_CLK_SEL_W<'_, TIMG_CALI_CLK_CONF_SPEC> {
        TIMG_CALI_CLK_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configures the clock source for the TIMG_SECURE_CLK.\\\\ 0 (default):CPU_CLK\\\\ 1: AHB_CLK\\\\ 2: APB_CLK\\\\ 3: sec function clock\\\\ 4: mspi function clock\\\\ 5: iomux function clock\\\\ 6: parl io rx function clock\\\\ 7: parl io tx function clock\\\\ 8: spi2 function clock\\\\ 9: spi3 function clock\\\\"]
    #[inline(always)]
    pub fn timg_secure_clk_sel(&mut self) -> TIMG_SECURE_CLK_SEL_W<'_, TIMG_CALI_CLK_CONF_SPEC> {
        TIMG_SECURE_CLK_SEL_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - When PCR_TIMG_CALI_CLK_SEL set as 4, This field PCR_TIMG_SECURE_CLK_DIV_NUM is used to set the divider number of TIMG_SECURE_CLK."]
    #[inline(always)]
    pub fn timg_secure_clk_div_num(
        &mut self,
    ) -> TIMG_SECURE_CLK_DIV_NUM_W<'_, TIMG_CALI_CLK_CONF_SPEC> {
        TIMG_SECURE_CLK_DIV_NUM_W::new(self, 8)
    }
}
#[doc = "timergrout calibrate clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timg_cali_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg_cali_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMG_CALI_CLK_CONF_SPEC;
impl crate::RegisterSpec for TIMG_CALI_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timg_cali_clk_conf::R`](R) reader structure"]
impl crate::Readable for TIMG_CALI_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timg_cali_clk_conf::W`](W) writer structure"]
impl crate::Writable for TIMG_CALI_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMG_CALI_CLK_CONF to value 0x0770"]
impl crate::Resettable for TIMG_CALI_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0770;
}
