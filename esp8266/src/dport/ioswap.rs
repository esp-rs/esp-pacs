#[doc = "Register `IOSWAP` reader"]
pub type R = crate::R<IOSWAP_SPEC>;
#[doc = "Register `IOSWAP` writer"]
pub type W = crate::W<IOSWAP_SPEC>;
#[doc = "Field `uart` reader - Swap UART"]
pub type UART_R = crate::BitReader;
#[doc = "Field `uart` writer - Swap UART"]
pub type UART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spi` reader - Swap SPI"]
pub type SPI_R = crate::BitReader;
#[doc = "Field `spi` writer - Swap SPI"]
pub type SPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart0` reader - Swap UART0 pins (u0rxd &lt;-> u0cts), (u0txd &lt;-> u0rts)"]
pub type UART0_R = crate::BitReader;
#[doc = "Field `uart0` writer - Swap UART0 pins (u0rxd &lt;-> u0cts), (u0txd &lt;-> u0rts)"]
pub type UART0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uart1` reader - Swap UART1 pins (u1rxd &lt;-> u1cts), (u1txd &lt;-> u1rts)"]
pub type UART1_R = crate::BitReader;
#[doc = "Field `uart1` writer - Swap UART1 pins (u1rxd &lt;-> u1cts), (u1txd &lt;-> u1rts)"]
pub type UART1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hspi` reader - Set HSPI with higher priority"]
pub type HSPI_R = crate::BitReader;
#[doc = "Field `hspi` writer - Set HSPI with higher priority"]
pub type HSPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `double_hspi` reader - Set two SPI masters on HSPI"]
pub type DOUBLE_HSPI_R = crate::BitReader;
#[doc = "Field `double_hspi` writer - Set two SPI masters on HSPI"]
pub type DOUBLE_HSPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `double_cspi` reader - Set two SPI masters on CSPI"]
pub type DOUBLE_CSPI_R = crate::BitReader;
#[doc = "Field `double_cspi` writer - Set two SPI masters on CSPI"]
pub type DOUBLE_CSPI_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Swap UART"]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UART_W<IOSWAP_SPEC> {
        UART_W::new(self, 0)
    }
    #[doc = "Bit 1 - Swap SPI"]
    #[inline(always)]
    #[must_use]
    pub fn spi(&mut self) -> SPI_W<IOSWAP_SPEC> {
        SPI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Swap UART0 pins (u0rxd &lt;-> u0cts), (u0txd &lt;-> u0rts)"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<IOSWAP_SPEC> {
        UART0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Swap UART1 pins (u1rxd &lt;-> u1cts), (u1txd &lt;-> u1rts)"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<IOSWAP_SPEC> {
        UART1_W::new(self, 3)
    }
    #[doc = "Bit 5 - Set HSPI with higher priority"]
    #[inline(always)]
    #[must_use]
    pub fn hspi(&mut self) -> HSPI_W<IOSWAP_SPEC> {
        HSPI_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set two SPI masters on HSPI"]
    #[inline(always)]
    #[must_use]
    pub fn double_hspi(&mut self) -> DOUBLE_HSPI_W<IOSWAP_SPEC> {
        DOUBLE_HSPI_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set two SPI masters on CSPI"]
    #[inline(always)]
    #[must_use]
    pub fn double_cspi(&mut self) -> DOUBLE_CSPI_W<IOSWAP_SPEC> {
        DOUBLE_CSPI_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IO Swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioswap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioswap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOSWAP_SPEC;
impl crate::RegisterSpec for IOSWAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioswap::R`](R) reader structure"]
impl crate::Readable for IOSWAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioswap::W`](W) writer structure"]
impl crate::Writable for IOSWAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IOSWAP to value 0"]
impl crate::Resettable for IOSWAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
