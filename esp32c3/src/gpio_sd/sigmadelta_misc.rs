#[doc = "Register `SIGMADELTA_MISC` reader"]
pub type R = crate::R<SIGMADELTA_MISC_SPEC>;
#[doc = "Register `SIGMADELTA_MISC` writer"]
pub type W = crate::W<SIGMADELTA_MISC_SPEC>;
#[doc = "Field `FUNCTION_CLK_EN` reader - Clock enable bit of sigma delta modulation."]
pub type FUNCTION_CLK_EN_R = crate::BitReader;
#[doc = "Field `FUNCTION_CLK_EN` writer - Clock enable bit of sigma delta modulation."]
pub type FUNCTION_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SWAP` reader - Reserved."]
pub type SPI_SWAP_R = crate::BitReader;
#[doc = "Field `SPI_SWAP` writer - Reserved."]
pub type SPI_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Clock enable bit of sigma delta modulation."]
    #[inline(always)]
    pub fn function_clk_en(&self) -> FUNCTION_CLK_EN_R {
        FUNCTION_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved."]
    #[inline(always)]
    pub fn spi_swap(&self) -> SPI_SWAP_R {
        SPI_SWAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA_MISC")
            .field(
                "function_clk_en",
                &format_args!("{}", self.function_clk_en().bit()),
            )
            .field("spi_swap", &format_args!("{}", self.spi_swap().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA_MISC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 30 - Clock enable bit of sigma delta modulation."]
    #[inline(always)]
    #[must_use]
    pub fn function_clk_en(&mut self) -> FUNCTION_CLK_EN_W<SIGMADELTA_MISC_SPEC> {
        FUNCTION_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn spi_swap(&mut self) -> SPI_SWAP_W<SIGMADELTA_MISC_SPEC> {
        SPI_SWAP_W::new(self, 31)
    }
}
#[doc = "MISC Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta_misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta_misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA_MISC_SPEC;
impl crate::RegisterSpec for SIGMADELTA_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta_misc::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA_MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta_misc::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA_MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGMADELTA_MISC to value 0"]
impl crate::Resettable for SIGMADELTA_MISC_SPEC {
    const RESET_VALUE: u32 = 0;
}
