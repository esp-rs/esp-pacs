///Register `CPU_PERI_CLK_EN` reader
pub type R = crate::R<CPU_PERI_CLK_EN_SPEC>;
///Register `CPU_PERI_CLK_EN` writer
pub type W = crate::W<CPU_PERI_CLK_EN_SPEC>;
///Field `CLK_EN_DEDICATED_GPIO` reader - Set this bit to enable clock of DEDICATED GPIO module.
pub type CLK_EN_DEDICATED_GPIO_R = crate::BitReader;
///Field `CLK_EN_DEDICATED_GPIO` writer - Set this bit to enable clock of DEDICATED GPIO module.
pub type CLK_EN_DEDICATED_GPIO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 7 - Set this bit to enable clock of DEDICATED GPIO module.
    #[inline(always)]
    pub fn clk_en_dedicated_gpio(&self) -> CLK_EN_DEDICATED_GPIO_R {
        CLK_EN_DEDICATED_GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERI_CLK_EN")
            .field("clk_en_dedicated_gpio", &self.clk_en_dedicated_gpio())
            .finish()
    }
}
impl W {
    ///Bit 7 - Set this bit to enable clock of DEDICATED GPIO module.
    #[inline(always)]
    #[must_use]
    pub fn clk_en_dedicated_gpio(&mut self) -> CLK_EN_DEDICATED_GPIO_W<CPU_PERI_CLK_EN_SPEC> {
        CLK_EN_DEDICATED_GPIO_W::new(self, 7)
    }
}
/**CPU peripheral clock enable register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPU_PERI_CLK_EN_SPEC;
impl crate::RegisterSpec for CPU_PERI_CLK_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cpu_peri_clk_en::R`](R) reader structure
impl crate::Readable for CPU_PERI_CLK_EN_SPEC {}
///`write(|w| ..)` method takes [`cpu_peri_clk_en::W`](W) writer structure
impl crate::Writable for CPU_PERI_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPU_PERI_CLK_EN to value 0
impl crate::Resettable for CPU_PERI_CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
