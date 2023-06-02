#[doc = "Register `SPI_PIN` reader"]
pub struct R(crate::R<SPI_PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_PIN` writer"]
pub struct W(crate::W<SPI_PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_PIN_SPEC>;
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
impl From<crate::W<SPI_PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_cs0_dis` reader - 1: disable CS0; 0: spi_cs signal is from/to CS0 pin"]
pub type SPI_CS0_DIS_R = crate::BitReader;
#[doc = "Field `spi_cs0_dis` writer - 1: disable CS0; 0: spi_cs signal is from/to CS0 pin"]
pub type SPI_CS0_DIS_W<'a, const O: u8> = crate::BitWriter<'a, SPI_PIN_SPEC, O>;
#[doc = "Field `spi_cs1_dis` reader - 1: disable CS1; 0: spi_cs signal is from/to CS1 pin"]
pub type SPI_CS1_DIS_R = crate::BitReader;
#[doc = "Field `spi_cs1_dis` writer - 1: disable CS1; 0: spi_cs signal is from/to CS1 pin"]
pub type SPI_CS1_DIS_W<'a, const O: u8> = crate::BitWriter<'a, SPI_PIN_SPEC, O>;
#[doc = "Field `spi_cs2_dis` reader - 1: disable CS2; 0: spi_cs signal is from/to CS2 pin"]
pub type SPI_CS2_DIS_R = crate::BitReader;
#[doc = "Field `spi_cs2_dis` writer - 1: disable CS2; 0: spi_cs signal is from/to CS2 pin"]
pub type SPI_CS2_DIS_W<'a, const O: u8> = crate::BitWriter<'a, SPI_PIN_SPEC, O>;
#[doc = "Field `spi_idle_edge` reader - In the master mode, 1: high when idle; 0: low when idle"]
pub type SPI_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `spi_idle_edge` writer - In the master mode, 1: high when idle; 0: low when idle"]
pub type SPI_IDLE_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_PIN_SPEC, O>;
impl R {
    #[doc = "Bit 0 - 1: disable CS0; 0: spi_cs signal is from/to CS0 pin"]
    #[inline(always)]
    pub fn spi_cs0_dis(&self) -> SPI_CS0_DIS_R {
        SPI_CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: disable CS1; 0: spi_cs signal is from/to CS1 pin"]
    #[inline(always)]
    pub fn spi_cs1_dis(&self) -> SPI_CS1_DIS_R {
        SPI_CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: disable CS2; 0: spi_cs signal is from/to CS2 pin"]
    #[inline(always)]
    pub fn spi_cs2_dis(&self) -> SPI_CS2_DIS_R {
        SPI_CS2_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 29 - In the master mode, 1: high when idle; 0: low when idle"]
    #[inline(always)]
    pub fn spi_idle_edge(&self) -> SPI_IDLE_EDGE_R {
        SPI_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_PIN")
            .field("spi_cs2_dis", &format_args!("{}", self.spi_cs2_dis().bit()))
            .field("spi_cs1_dis", &format_args!("{}", self.spi_cs1_dis().bit()))
            .field("spi_cs0_dis", &format_args!("{}", self.spi_cs0_dis().bit()))
            .field(
                "spi_idle_edge",
                &format_args!("{}", self.spi_idle_edge().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_PIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 1: disable CS0; 0: spi_cs signal is from/to CS0 pin"]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs0_dis(&mut self) -> SPI_CS0_DIS_W<0> {
        SPI_CS0_DIS_W::new(self)
    }
    #[doc = "Bit 1 - 1: disable CS1; 0: spi_cs signal is from/to CS1 pin"]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs1_dis(&mut self) -> SPI_CS1_DIS_W<1> {
        SPI_CS1_DIS_W::new(self)
    }
    #[doc = "Bit 2 - 1: disable CS2; 0: spi_cs signal is from/to CS2 pin"]
    #[inline(always)]
    #[must_use]
    pub fn spi_cs2_dis(&mut self) -> SPI_CS2_DIS_W<2> {
        SPI_CS2_DIS_W::new(self)
    }
    #[doc = "Bit 29 - In the master mode, 1: high when idle; 0: low when idle"]
    #[inline(always)]
    #[must_use]
    pub fn spi_idle_edge(&mut self) -> SPI_IDLE_EDGE_W<29> {
        SPI_IDLE_EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1: disable CS2; 0: spi_cs signal is from/to CS2 pin\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_pin](index.html) module"]
pub struct SPI_PIN_SPEC;
impl crate::RegisterSpec for SPI_PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_pin::R](R) reader structure"]
impl crate::Readable for SPI_PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_pin::W](W) writer structure"]
impl crate::Writable for SPI_PIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_PIN to value 0"]
impl crate::Resettable for SPI_PIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
