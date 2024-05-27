///Register `CLKM_CONF` reader
pub type R = crate::R<CLKM_CONF_SPEC>;
///Register `CLKM_CONF` writer
pub type W = crate::W<CLKM_CONF_SPEC>;
///Field `CLKM_DIV_NUM` reader - Integral DIG_ADC clock divider value
pub type CLKM_DIV_NUM_R = crate::FieldReader;
///Field `CLKM_DIV_NUM` writer - Integral DIG_ADC clock divider value
pub type CLKM_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CLKM_DIV_B` reader - Fractional clock divider numerator value
pub type CLKM_DIV_B_R = crate::FieldReader;
///Field `CLKM_DIV_B` writer - Fractional clock divider numerator value
pub type CLKM_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CLKM_DIV_A` reader - Fractional clock divider denominator value
pub type CLKM_DIV_A_R = crate::FieldReader;
///Field `CLKM_DIV_A` writer - Fractional clock divider denominator value
pub type CLKM_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `CLK_SEL` reader - 1: select APLL. 2: select APB_CLK. Other values: disable clock.
pub type CLK_SEL_R = crate::FieldReader;
///Field `CLK_SEL` writer - 1: select APLL. 2: select APB_CLK. Other values: disable clock.
pub type CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:7 - Integral DIG_ADC clock divider value
    #[inline(always)]
    pub fn clkm_div_num(&self) -> CLKM_DIV_NUM_R {
        CLKM_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:13 - Fractional clock divider numerator value
    #[inline(always)]
    pub fn clkm_div_b(&self) -> CLKM_DIV_B_R {
        CLKM_DIV_B_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:19 - Fractional clock divider denominator value
    #[inline(always)]
    pub fn clkm_div_a(&self) -> CLKM_DIV_A_R {
        CLKM_DIV_A_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    ///Bits 21:22 - 1: select APLL. 2: select APB_CLK. Other values: disable clock.
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 21) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKM_CONF")
            .field("clkm_div_num", &self.clkm_div_num())
            .field("clkm_div_b", &self.clkm_div_b())
            .field("clkm_div_a", &self.clkm_div_a())
            .field("clk_sel", &self.clk_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Integral DIG_ADC clock divider value
    #[inline(always)]
    #[must_use]
    pub fn clkm_div_num(&mut self) -> CLKM_DIV_NUM_W<CLKM_CONF_SPEC> {
        CLKM_DIV_NUM_W::new(self, 0)
    }
    ///Bits 8:13 - Fractional clock divider numerator value
    #[inline(always)]
    #[must_use]
    pub fn clkm_div_b(&mut self) -> CLKM_DIV_B_W<CLKM_CONF_SPEC> {
        CLKM_DIV_B_W::new(self, 8)
    }
    ///Bits 14:19 - Fractional clock divider denominator value
    #[inline(always)]
    #[must_use]
    pub fn clkm_div_a(&mut self) -> CLKM_DIV_A_W<CLKM_CONF_SPEC> {
        CLKM_DIV_A_W::new(self, 14)
    }
    ///Bits 21:22 - 1: select APLL. 2: select APB_CLK. Other values: disable clock.
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<CLKM_CONF_SPEC> {
        CLK_SEL_W::new(self, 21)
    }
}
/**Configure DIG ADC clock

You can [`read`](crate::generic::Reg::read) this register and get [`clkm_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkm_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLKM_CONF_SPEC;
impl crate::RegisterSpec for CLKM_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clkm_conf::R`](R) reader structure
impl crate::Readable for CLKM_CONF_SPEC {}
///`write(|w| ..)` method takes [`clkm_conf::W`](W) writer structure
impl crate::Writable for CLKM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLKM_CONF to value 0x04
impl crate::Resettable for CLKM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
