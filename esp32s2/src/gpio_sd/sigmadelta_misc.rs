#[doc = "Register `SIGMADELTA_MISC` reader"]
pub struct R(crate::R<SIGMADELTA_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA_MISC` writer"]
pub struct W(crate::W<SIGMADELTA_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SIGMADELTA_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNCTION_CLK_EN` reader - Clock enable bit of sigma delta modulation."]
pub type FUNCTION_CLK_EN_R = crate::BitReader;
#[doc = "Field `FUNCTION_CLK_EN` writer - Clock enable bit of sigma delta modulation."]
pub type FUNCTION_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, SIGMADELTA_MISC_SPEC, O>;
#[doc = "Field `SPI_SWAP` reader - Reserved."]
pub type SPI_SWAP_R = crate::BitReader;
#[doc = "Field `SPI_SWAP` writer - Reserved."]
pub type SPI_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, SIGMADELTA_MISC_SPEC, O>;
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - Clock enable bit of sigma delta modulation."]
    #[inline(always)]
    #[must_use]
    pub fn function_clk_en(&mut self) -> FUNCTION_CLK_EN_W<30> {
        FUNCTION_CLK_EN_W::new(self)
    }
    #[doc = "Bit 31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn spi_swap(&mut self) -> SPI_SWAP_W<31> {
        SPI_SWAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MISC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta_misc](index.html) module"]
pub struct SIGMADELTA_MISC_SPEC;
impl crate::RegisterSpec for SIGMADELTA_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta_misc::R](R) reader structure"]
impl crate::Readable for SIGMADELTA_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta_misc::W](W) writer structure"]
impl crate::Writable for SIGMADELTA_MISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA_MISC to value 0"]
impl crate::Resettable for SIGMADELTA_MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
