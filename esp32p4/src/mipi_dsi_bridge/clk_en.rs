///Register `CLK_EN` reader
pub type R = crate::R<CLK_EN_SPEC>;
///Register `CLK_EN` writer
pub type W = crate::W<CLK_EN_SPEC>;
///Field `CLK_EN` reader - this bit configures force_on of dsi_bridge register clock gate
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - this bit configures force_on of dsi_bridge register clock gate
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - this bit configures force_on of dsi_bridge register clock gate
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_EN")
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - this bit configures force_on of dsi_bridge register clock gate
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CLK_EN_SPEC> {
        CLK_EN_W::new(self, 0)
    }
}
/**dsi bridge clk control register

You can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLK_EN_SPEC;
impl crate::RegisterSpec for CLK_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clk_en::R`](R) reader structure
impl crate::Readable for CLK_EN_SPEC {}
///`write(|w| ..)` method takes [`clk_en::W`](W) writer structure
impl crate::Writable for CLK_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLK_EN to value 0
impl crate::Resettable for CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
