#[doc = "Register `BUS_CLK_UPDATE` reader"]
pub type R = crate::R<BUS_CLK_UPDATE_SPEC>;
#[doc = "Register `BUS_CLK_UPDATE` writer"]
pub type W = crate::W<BUS_CLK_UPDATE_SPEC>;
#[doc = "Field `BUS_CLOCK_UPDATE` reader - Configures whether or not to update configurations for CPU_CLK division, AHB_CLK division and HP_ROOT_CLK clock source selection.\\\\ 0: Not update configurations\\\\ 1: Update configurations\\\\ This bit is automatically cleared when configurations have been updated.\\\\"]
pub type BUS_CLOCK_UPDATE_R = crate::BitReader;
#[doc = "Field `BUS_CLOCK_UPDATE` writer - Configures whether or not to update configurations for CPU_CLK division, AHB_CLK division and HP_ROOT_CLK clock source selection.\\\\ 0: Not update configurations\\\\ 1: Update configurations\\\\ This bit is automatically cleared when configurations have been updated.\\\\"]
pub type BUS_CLOCK_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to update configurations for CPU_CLK division, AHB_CLK division and HP_ROOT_CLK clock source selection.\\\\ 0: Not update configurations\\\\ 1: Update configurations\\\\ This bit is automatically cleared when configurations have been updated.\\\\"]
    #[inline(always)]
    pub fn bus_clock_update(&self) -> BUS_CLOCK_UPDATE_R {
        BUS_CLOCK_UPDATE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_CLK_UPDATE")
            .field("bus_clock_update", &self.bus_clock_update())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to update configurations for CPU_CLK division, AHB_CLK division and HP_ROOT_CLK clock source selection.\\\\ 0: Not update configurations\\\\ 1: Update configurations\\\\ This bit is automatically cleared when configurations have been updated.\\\\"]
    #[inline(always)]
    pub fn bus_clock_update(&mut self) -> BUS_CLOCK_UPDATE_W<'_, BUS_CLK_UPDATE_SPEC> {
        BUS_CLOCK_UPDATE_W::new(self, 0)
    }
}
#[doc = "Configuration register for applying updated high-performance system clock sources\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_clk_update::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_clk_update::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_CLK_UPDATE_SPEC;
impl crate::RegisterSpec for BUS_CLK_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_clk_update::R`](R) reader structure"]
impl crate::Readable for BUS_CLK_UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_clk_update::W`](W) writer structure"]
impl crate::Writable for BUS_CLK_UPDATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_CLK_UPDATE to value 0"]
impl crate::Resettable for BUS_CLK_UPDATE_SPEC {}
