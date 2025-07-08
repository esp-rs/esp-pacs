#[doc = "Register `UART2_SCLK_CONF` reader"]
pub type R = crate::R<UART2_SCLK_CONF_SPEC>;
#[doc = "Register `UART2_SCLK_CONF` writer"]
pub type W = crate::W<UART2_SCLK_CONF_SPEC>;
#[doc = "Field `UART2_SCLK_DIV_A` reader - The denominator of the frequency divider factor of the uart2 function clock."]
pub type UART2_SCLK_DIV_A_R = crate::FieldReader;
#[doc = "Field `UART2_SCLK_DIV_A` writer - The denominator of the frequency divider factor of the uart2 function clock."]
pub type UART2_SCLK_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `UART2_SCLK_DIV_B` reader - The numerator of the frequency divider factor of the uart2 function clock."]
pub type UART2_SCLK_DIV_B_R = crate::FieldReader;
#[doc = "Field `UART2_SCLK_DIV_B` writer - The numerator of the frequency divider factor of the uart2 function clock."]
pub type UART2_SCLK_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `UART2_SCLK_DIV_NUM` reader - The integral part of the frequency divider factor of the uart2 function clock."]
pub type UART2_SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `UART2_SCLK_DIV_NUM` writer - The integral part of the frequency divider factor of the uart2 function clock."]
pub type UART2_SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART2_SCLK_SEL` reader - Configures the clock source of UART2.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
pub type UART2_SCLK_SEL_R = crate::FieldReader;
#[doc = "Field `UART2_SCLK_SEL` writer - Configures the clock source of UART2.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
pub type UART2_SCLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART2_SCLK_EN` reader - Set 1 to enable uart2 function clock"]
pub type UART2_SCLK_EN_R = crate::BitReader;
#[doc = "Field `UART2_SCLK_EN` writer - Set 1 to enable uart2 function clock"]
pub type UART2_SCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the uart2 function clock."]
    #[inline(always)]
    pub fn uart2_sclk_div_a(&self) -> UART2_SCLK_DIV_A_R {
        UART2_SCLK_DIV_A_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the uart2 function clock."]
    #[inline(always)]
    pub fn uart2_sclk_div_b(&self) -> UART2_SCLK_DIV_B_R {
        UART2_SCLK_DIV_B_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the uart2 function clock."]
    #[inline(always)]
    pub fn uart2_sclk_div_num(&self) -> UART2_SCLK_DIV_NUM_R {
        UART2_SCLK_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - Configures the clock source of UART2.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
    #[inline(always)]
    pub fn uart2_sclk_sel(&self) -> UART2_SCLK_SEL_R {
        UART2_SCLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable uart2 function clock"]
    #[inline(always)]
    pub fn uart2_sclk_en(&self) -> UART2_SCLK_EN_R {
        UART2_SCLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART2_SCLK_CONF")
            .field("uart2_sclk_div_a", &self.uart2_sclk_div_a())
            .field("uart2_sclk_div_b", &self.uart2_sclk_div_b())
            .field("uart2_sclk_div_num", &self.uart2_sclk_div_num())
            .field("uart2_sclk_sel", &self.uart2_sclk_sel())
            .field("uart2_sclk_en", &self.uart2_sclk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the uart2 function clock."]
    #[inline(always)]
    pub fn uart2_sclk_div_a(&mut self) -> UART2_SCLK_DIV_A_W<UART2_SCLK_CONF_SPEC> {
        UART2_SCLK_DIV_A_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the uart2 function clock."]
    #[inline(always)]
    pub fn uart2_sclk_div_b(&mut self) -> UART2_SCLK_DIV_B_W<UART2_SCLK_CONF_SPEC> {
        UART2_SCLK_DIV_B_W::new(self, 6)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the uart2 function clock."]
    #[inline(always)]
    pub fn uart2_sclk_div_num(&mut self) -> UART2_SCLK_DIV_NUM_W<UART2_SCLK_CONF_SPEC> {
        UART2_SCLK_DIV_NUM_W::new(self, 12)
    }
    #[doc = "Bits 20:21 - Configures the clock source of UART2.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
    #[inline(always)]
    pub fn uart2_sclk_sel(&mut self) -> UART2_SCLK_SEL_W<UART2_SCLK_CONF_SPEC> {
        UART2_SCLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - Set 1 to enable uart2 function clock"]
    #[inline(always)]
    pub fn uart2_sclk_en(&mut self) -> UART2_SCLK_EN_W<UART2_SCLK_CONF_SPEC> {
        UART2_SCLK_EN_W::new(self, 22)
    }
}
#[doc = "UART2_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_sclk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_sclk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART2_SCLK_CONF_SPEC;
impl crate::RegisterSpec for UART2_SCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart2_sclk_conf::R`](R) reader structure"]
impl crate::Readable for UART2_SCLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart2_sclk_conf::W`](W) writer structure"]
impl crate::Writable for UART2_SCLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART2_SCLK_CONF to value 0x0040_0000"]
impl crate::Resettable for UART2_SCLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0040_0000;
}
