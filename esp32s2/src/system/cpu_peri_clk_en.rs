#[doc = "Register `CPU_PERI_CLK_EN` reader"]
pub type R = crate::R<CPU_PERI_CLK_EN_SPEC>;
#[doc = "Register `CPU_PERI_CLK_EN` writer"]
pub type W = crate::W<CPU_PERI_CLK_EN_SPEC>;
#[doc = "Field `DEDICATED_GPIO_CLK_EN` reader - Set this bit to enable clock of DEDICATED GPIO module."]
pub type DEDICATED_GPIO_CLK_EN_R = crate::BitReader;
#[doc = "Field `DEDICATED_GPIO_CLK_EN` writer - Set this bit to enable clock of DEDICATED GPIO module."]
pub type DEDICATED_GPIO_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - Set this bit to enable clock of DEDICATED GPIO module."]
    #[inline(always)]
    pub fn dedicated_gpio_clk_en(&self) -> DEDICATED_GPIO_CLK_EN_R {
        DEDICATED_GPIO_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERI_CLK_EN")
            .field("dedicated_gpio_clk_en", &self.dedicated_gpio_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - Set this bit to enable clock of DEDICATED GPIO module."]
    #[inline(always)]
    pub fn dedicated_gpio_clk_en(&mut self) -> DEDICATED_GPIO_CLK_EN_W<'_, CPU_PERI_CLK_EN_SPEC> {
        DEDICATED_GPIO_CLK_EN_W::new(self, 7)
    }
}
#[doc = "CPU peripheral clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_peri_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_peri_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_PERI_CLK_EN_SPEC;
impl crate::RegisterSpec for CPU_PERI_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_peri_clk_en::R`](R) reader structure"]
impl crate::Readable for CPU_PERI_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_peri_clk_en::W`](W) writer structure"]
impl crate::Writable for CPU_PERI_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_PERI_CLK_EN to value 0"]
impl crate::Resettable for CPU_PERI_CLK_EN_SPEC {}
