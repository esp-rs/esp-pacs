#[doc = "Register `CTRL_32K_CONF` reader"]
pub type R = crate::R<CTRL_32K_CONF_SPEC>;
#[doc = "Register `CTRL_32K_CONF` writer"]
pub type W = crate::W<CTRL_32K_CONF_SPEC>;
#[doc = "Field `_32K_SEL` reader - Configures the 32KHz clock for TIMER_GROUP.\\\\ 0 (default): RC32K_CLK\\\\ 1: XTAL32K_CLK\\\\ 2: OSC_SLOW_CLK\\\\ 3: RC_SLOW_CLK\\\\ 4: RC_FAST_CLK\\\\"]
pub type _32K_SEL_R = crate::FieldReader;
#[doc = "Field `_32K_SEL` writer - Configures the 32KHz clock for TIMER_GROUP.\\\\ 0 (default): RC32K_CLK\\\\ 1: XTAL32K_CLK\\\\ 2: OSC_SLOW_CLK\\\\ 3: RC_SLOW_CLK\\\\ 4: RC_FAST_CLK\\\\"]
pub type _32K_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FOSC_TICK_NUM` reader - When PCR_32K_SEL set as 4, This field PCR_FOSC_TICK_NUM is used to set the divider number of fosc."]
pub type FOSC_TICK_NUM_R = crate::FieldReader;
#[doc = "Field `FOSC_TICK_NUM` writer - When PCR_32K_SEL set as 4, This field PCR_FOSC_TICK_NUM is used to set the divider number of fosc."]
pub type FOSC_TICK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Configures the 32KHz clock for TIMER_GROUP.\\\\ 0 (default): RC32K_CLK\\\\ 1: XTAL32K_CLK\\\\ 2: OSC_SLOW_CLK\\\\ 3: RC_SLOW_CLK\\\\ 4: RC_FAST_CLK\\\\"]
    #[inline(always)]
    pub fn _32k_sel(&self) -> _32K_SEL_R {
        _32K_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - When PCR_32K_SEL set as 4, This field PCR_FOSC_TICK_NUM is used to set the divider number of fosc."]
    #[inline(always)]
    pub fn fosc_tick_num(&self) -> FOSC_TICK_NUM_R {
        FOSC_TICK_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_32K_CONF")
            .field("_32k_sel", &self._32k_sel())
            .field("fosc_tick_num", &self.fosc_tick_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures the 32KHz clock for TIMER_GROUP.\\\\ 0 (default): RC32K_CLK\\\\ 1: XTAL32K_CLK\\\\ 2: OSC_SLOW_CLK\\\\ 3: RC_SLOW_CLK\\\\ 4: RC_FAST_CLK\\\\"]
    #[inline(always)]
    pub fn _32k_sel(&mut self) -> _32K_SEL_W<CTRL_32K_CONF_SPEC> {
        _32K_SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - When PCR_32K_SEL set as 4, This field PCR_FOSC_TICK_NUM is used to set the divider number of fosc."]
    #[inline(always)]
    pub fn fosc_tick_num(&mut self) -> FOSC_TICK_NUM_W<CTRL_32K_CONF_SPEC> {
        FOSC_TICK_NUM_W::new(self, 8)
    }
}
#[doc = "32KHz clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_32k_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_32k_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_32K_CONF_SPEC;
impl crate::RegisterSpec for CTRL_32K_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_32k_conf::R`](R) reader structure"]
impl crate::Readable for CTRL_32K_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_32k_conf::W`](W) writer structure"]
impl crate::Writable for CTRL_32K_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL_32K_CONF to value 0x0700"]
impl crate::Resettable for CTRL_32K_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0700;
}
