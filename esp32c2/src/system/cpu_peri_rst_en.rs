///Register `CPU_PERI_RST_EN` reader
pub type R = crate::R<CPU_PERI_RST_EN_SPEC>;
///Register `CPU_PERI_RST_EN` writer
pub type W = crate::W<CPU_PERI_RST_EN_SPEC>;
///Field `RST_EN_ASSIST_DEBUG` reader - Set 1 to let assist_debug module reset
pub type RST_EN_ASSIST_DEBUG_R = crate::BitReader;
///Field `RST_EN_ASSIST_DEBUG` writer - Set 1 to let assist_debug module reset
pub type RST_EN_ASSIST_DEBUG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_EN_DEDICATED_GPIO` reader - Set 1 to let dedicated_gpio module reset
pub type RST_EN_DEDICATED_GPIO_R = crate::BitReader;
///Field `RST_EN_DEDICATED_GPIO` writer - Set 1 to let dedicated_gpio module reset
pub type RST_EN_DEDICATED_GPIO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 6 - Set 1 to let assist_debug module reset
    #[inline(always)]
    pub fn rst_en_assist_debug(&self) -> RST_EN_ASSIST_DEBUG_R {
        RST_EN_ASSIST_DEBUG_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Set 1 to let dedicated_gpio module reset
    #[inline(always)]
    pub fn rst_en_dedicated_gpio(&self) -> RST_EN_DEDICATED_GPIO_R {
        RST_EN_DEDICATED_GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_PERI_RST_EN")
            .field("rst_en_assist_debug", &self.rst_en_assist_debug())
            .field("rst_en_dedicated_gpio", &self.rst_en_dedicated_gpio())
            .finish()
    }
}
impl W {
    ///Bit 6 - Set 1 to let assist_debug module reset
    #[inline(always)]
    #[must_use]
    pub fn rst_en_assist_debug(&mut self) -> RST_EN_ASSIST_DEBUG_W<CPU_PERI_RST_EN_SPEC> {
        RST_EN_ASSIST_DEBUG_W::new(self, 6)
    }
    ///Bit 7 - Set 1 to let dedicated_gpio module reset
    #[inline(always)]
    #[must_use]
    pub fn rst_en_dedicated_gpio(&mut self) -> RST_EN_DEDICATED_GPIO_W<CPU_PERI_RST_EN_SPEC> {
        RST_EN_DEDICATED_GPIO_W::new(self, 7)
    }
}
/**cpu_peripheral reset register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_rst_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_rst_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPU_PERI_RST_EN_SPEC;
impl crate::RegisterSpec for CPU_PERI_RST_EN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cpu_peri_rst_en::R`](R) reader structure
impl crate::Readable for CPU_PERI_RST_EN_SPEC {}
///`write(|w| ..)` method takes [`cpu_peri_rst_en::W`](W) writer structure
impl crate::Writable for CPU_PERI_RST_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPU_PERI_RST_EN to value 0xc0
impl crate::Resettable for CPU_PERI_RST_EN_SPEC {
    const RESET_VALUE: u32 = 0xc0;
}
