///Register `TEST_CONF` reader
pub type R = crate::R<TEST_CONF_SPEC>;
///Register `TEST_CONF` writer
pub type W = crate::W<TEST_CONF_SPEC>;
///Field `CLK_EN` reader -
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer -
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_DEBUG_ENA` reader -
pub type CLK_DEBUG_ENA_R = crate::BitReader;
///Field `CLK_DEBUG_ENA` writer -
pub type CLK_DEBUG_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn clk_debug_ena(&self) -> CLK_DEBUG_ENA_R {
        CLK_DEBUG_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST_CONF")
            .field("clk_en", &self.clk_en())
            .field("clk_debug_ena", &self.clk_debug_ena())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<TEST_CONF_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    #[must_use]
    pub fn clk_debug_ena(&mut self) -> CLK_DEBUG_ENA_W<TEST_CONF_SPEC> {
        CLK_DEBUG_ENA_W::new(self, 1)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`test_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TEST_CONF_SPEC;
impl crate::RegisterSpec for TEST_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`test_conf::R`](R) reader structure
impl crate::Readable for TEST_CONF_SPEC {}
///`write(|w| ..)` method takes [`test_conf::W`](W) writer structure
impl crate::Writable for TEST_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TEST_CONF to value 0
impl crate::Resettable for TEST_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
