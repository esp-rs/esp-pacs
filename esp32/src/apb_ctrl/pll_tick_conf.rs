///Register `PLL_TICK_CONF` reader
pub type R = crate::R<PLL_TICK_CONF_SPEC>;
///Register `PLL_TICK_CONF` writer
pub type W = crate::W<PLL_TICK_CONF_SPEC>;
///Field `PLL_TICK_NUM` reader -
pub type PLL_TICK_NUM_R = crate::FieldReader;
///Field `PLL_TICK_NUM` writer -
pub type PLL_TICK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn pll_tick_num(&self) -> PLL_TICK_NUM_R {
        PLL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_TICK_CONF")
            .field("pll_tick_num", &self.pll_tick_num())
            .finish()
    }
}
impl W {
    ///Bits 0:7
    #[inline(always)]
    #[must_use]
    pub fn pll_tick_num(&mut self) -> PLL_TICK_NUM_W<PLL_TICK_CONF_SPEC> {
        PLL_TICK_NUM_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pll_tick_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_tick_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PLL_TICK_CONF_SPEC;
impl crate::RegisterSpec for PLL_TICK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pll_tick_conf::R`](R) reader structure
impl crate::Readable for PLL_TICK_CONF_SPEC {}
///`write(|w| ..)` method takes [`pll_tick_conf::W`](W) writer structure
impl crate::Writable for PLL_TICK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PLL_TICK_CONF to value 0x4f
impl crate::Resettable for PLL_TICK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x4f;
}
