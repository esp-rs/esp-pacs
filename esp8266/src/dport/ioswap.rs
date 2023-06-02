#[doc = "Register `IOSWAP` reader"]
pub struct R(crate::R<IOSWAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOSWAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOSWAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOSWAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOSWAP` writer"]
pub struct W(crate::W<IOSWAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOSWAP_SPEC>;
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
impl From<crate::W<IOSWAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOSWAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart` reader - Swap UART"]
pub type UART_R = crate::BitReader;
#[doc = "Field `uart` writer - Swap UART"]
pub type UART_W<'a, const O: u8> = crate::BitWriter<'a, IOSWAP_SPEC, O>;
#[doc = "Field `spi` reader - Swap SPI"]
pub type SPI_R = crate::BitReader;
#[doc = "Field `spi` writer - Swap SPI"]
pub type SPI_W<'a, const O: u8> = crate::BitWriter<'a, IOSWAP_SPEC, O>;
#[doc = "Field `uart0` reader - Swap UART0 pins (u0rxd &lt;-> u0cts), (u0txd &lt;-> u0rts)"]
pub type UART0_R = crate::BitReader;
#[doc = "Field `uart0` writer - Swap UART0 pins (u0rxd &lt;-> u0cts), (u0txd &lt;-> u0rts)"]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, IOSWAP_SPEC, O>;
#[doc = "Field `uart1` reader - Swap UART1 pins (u1rxd &lt;-> u1cts), (u1txd &lt;-> u1rts)"]
pub type UART1_R = crate::BitReader;
#[doc = "Field `uart1` writer - Swap UART1 pins (u1rxd &lt;-> u1cts), (u1txd &lt;-> u1rts)"]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, IOSWAP_SPEC, O>;
#[doc = "Field `hspi` reader - Set HSPI with higher priority"]
pub type HSPI_R = crate::BitReader;
#[doc = "Field `hspi` writer - Set HSPI with higher priority"]
pub type HSPI_W<'a, const O: u8> = crate::BitWriter<'a, IOSWAP_SPEC, O>;
#[doc = "Field `double_hspi` reader - Set two SPI masters on HSPI"]
pub type DOUBLE_HSPI_R = crate::BitReader;
#[doc = "Field `double_hspi` writer - Set two SPI masters on HSPI"]
pub type DOUBLE_HSPI_W<'a, const O: u8> = crate::BitWriter<'a, IOSWAP_SPEC, O>;
#[doc = "Field `double_cspi` reader - Set two SPI masters on CSPI"]
pub type DOUBLE_CSPI_R = crate::BitReader;
#[doc = "Field `double_cspi` writer - Set two SPI masters on CSPI"]
pub type DOUBLE_CSPI_W<'a, const O: u8> = crate::BitWriter<'a, IOSWAP_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Swap UART"]
    #[inline(always)]
    pub fn uart(&self) -> UART_R {
        UART_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Swap SPI"]
    #[inline(always)]
    pub fn spi(&self) -> SPI_R {
        SPI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Swap UART0 pins (u0rxd &lt;-> u0cts), (u0txd &lt;-> u0rts)"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Swap UART1 pins (u1rxd &lt;-> u1cts), (u1txd &lt;-> u1rts)"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Set HSPI with higher priority"]
    #[inline(always)]
    pub fn hspi(&self) -> HSPI_R {
        HSPI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set two SPI masters on HSPI"]
    #[inline(always)]
    pub fn double_hspi(&self) -> DOUBLE_HSPI_R {
        DOUBLE_HSPI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set two SPI masters on CSPI"]
    #[inline(always)]
    pub fn double_cspi(&self) -> DOUBLE_CSPI_R {
        DOUBLE_CSPI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOSWAP")
            .field("uart", &format_args!("{}", self.uart().bit()))
            .field("spi", &format_args!("{}", self.spi().bit()))
            .field("uart0", &format_args!("{}", self.uart0().bit()))
            .field("uart1", &format_args!("{}", self.uart1().bit()))
            .field("hspi", &format_args!("{}", self.hspi().bit()))
            .field("double_hspi", &format_args!("{}", self.double_hspi().bit()))
            .field("double_cspi", &format_args!("{}", self.double_cspi().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IOSWAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Swap UART"]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UART_W<0> {
        UART_W::new(self)
    }
    #[doc = "Bit 1 - Swap SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spi(&mut self) -> SPI_W<1> {
        SPI_W::new(self)
    }
    #[doc = "Bit 2 - Swap UART0 pins (u0rxd &lt;-> u0cts), (u0txd &lt;-> u0rts)"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<2> {
        UART0_W::new(self)
    }
    #[doc = "Bit 3 - Swap UART1 pins (u1rxd &lt;-> u1cts), (u1txd &lt;-> u1rts)"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<3> {
        UART1_W::new(self)
    }
    #[doc = "Bit 5 - Set HSPI with higher priority"]
    #[inline(always)]
    #[must_use]
    pub fn hspi(&mut self) -> HSPI_W<5> {
        HSPI_W::new(self)
    }
    #[doc = "Bit 6 - Set two SPI masters on HSPI"]
    #[inline(always)]
    #[must_use]
    pub fn double_hspi(&mut self) -> DOUBLE_HSPI_W<6> {
        DOUBLE_HSPI_W::new(self)
    }
    #[doc = "Bit 7 - Set two SPI masters on CSPI"]
    #[inline(always)]
    #[must_use]
    pub fn double_cspi(&mut self) -> DOUBLE_CSPI_W<7> {
        DOUBLE_CSPI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO Swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioswap](index.html) module"]
pub struct IOSWAP_SPEC;
impl crate::RegisterSpec for IOSWAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioswap::R](R) reader structure"]
impl crate::Readable for IOSWAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioswap::W](W) writer structure"]
impl crate::Writable for IOSWAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOSWAP to value 0"]
impl crate::Resettable for IOSWAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
