#[doc = "Register `UART_CLKDIV` reader"]
pub type R = crate::R<UART_CLKDIV_SPEC>;
#[doc = "Register `UART_CLKDIV` writer"]
pub type W = crate::W<UART_CLKDIV_SPEC>;
#[doc = "Field `uart_clkdiv` reader - BAUDRATE = UART_CLK_FREQ / UART_CLKDIV"]
pub type UART_CLKDIV_R = crate::FieldReader<u32>;
#[doc = "Field `uart_clkdiv` writer - BAUDRATE = UART_CLK_FREQ / UART_CLKDIV"]
pub type UART_CLKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - BAUDRATE = UART_CLK_FREQ / UART_CLKDIV"]
    #[inline(always)]
    pub fn uart_clkdiv(&self) -> UART_CLKDIV_R {
        UART_CLKDIV_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_CLKDIV")
            .field(
                "uart_clkdiv",
                &format_args!("{}", self.uart_clkdiv().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_CLKDIV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - BAUDRATE = UART_CLK_FREQ / UART_CLKDIV"]
    #[inline(always)]
    #[must_use]
    pub fn uart_clkdiv(&mut self) -> UART_CLKDIV_W<UART_CLKDIV_SPEC> {
        UART_CLKDIV_W::new(self, 0)
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
#[doc = "UART CLK DIV REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_clkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_clkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_CLKDIV_SPEC;
impl crate::RegisterSpec for UART_CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_clkdiv::R`](R) reader structure"]
impl crate::Readable for UART_CLKDIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_clkdiv::W`](W) writer structure"]
impl crate::Writable for UART_CLKDIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_CLKDIV to value 0"]
impl crate::Resettable for UART_CLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
