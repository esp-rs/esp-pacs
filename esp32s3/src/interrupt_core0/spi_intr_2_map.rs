#[doc = "Register `SPI_INTR_2_MAP` reader"]
pub struct R(crate::R<SPI_INTR_2_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_INTR_2_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_INTR_2_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_INTR_2_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_INTR_2_MAP` writer"]
pub struct W(crate::W<SPI_INTR_2_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_INTR_2_MAP_SPEC>;
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
impl From<crate::W<SPI_INTR_2_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_INTR_2_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_INTR_2_MAP` reader - this register used to map spi_intr_2 interrupt to one of core0's external interrupt"]
pub type SPI_INTR_2_MAP_R = crate::FieldReader;
#[doc = "Field `SPI_INTR_2_MAP` writer - this register used to map spi_intr_2 interrupt to one of core0's external interrupt"]
pub type SPI_INTR_2_MAP_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_INTR_2_MAP_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - this register used to map spi_intr_2 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn spi_intr_2_map(&self) -> SPI_INTR_2_MAP_R {
        SPI_INTR_2_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_INTR_2_MAP")
            .field(
                "spi_intr_2_map",
                &format_args!("{}", self.spi_intr_2_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_INTR_2_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map spi_intr_2 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn spi_intr_2_map(&mut self) -> SPI_INTR_2_MAP_W<0> {
        SPI_INTR_2_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_intr_2 interrupt configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_intr_2_map](index.html) module"]
pub struct SPI_INTR_2_MAP_SPEC;
impl crate::RegisterSpec for SPI_INTR_2_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_intr_2_map::R](R) reader structure"]
impl crate::Readable for SPI_INTR_2_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_intr_2_map::W](W) writer structure"]
impl crate::Writable for SPI_INTR_2_MAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_INTR_2_MAP to value 0x10"]
impl crate::Resettable for SPI_INTR_2_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
