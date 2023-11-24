#[doc = "Register `UART_AUTOBAUD` reader"]
pub type R = crate::R<UART_AUTOBAUD_SPEC>;
#[doc = "Register `UART_AUTOBAUD` writer"]
pub type W = crate::W<UART_AUTOBAUD_SPEC>;
#[doc = "Field `autobaud_en` reader - Set this bit to enable baudrate detect"]
pub type AUTOBAUD_EN_R = crate::BitReader;
#[doc = "Field `autobaud_en` writer - Set this bit to enable baudrate detect"]
pub type AUTOBAUD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `glitch_filt` reader - "]
pub type GLITCH_FILT_R = crate::FieldReader;
#[doc = "Field `glitch_filt` writer - "]
pub type GLITCH_FILT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable baudrate detect"]
    #[inline(always)]
    pub fn autobaud_en(&self) -> AUTOBAUD_EN_R {
        AUTOBAUD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn glitch_filt(&self) -> GLITCH_FILT_R {
        GLITCH_FILT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_AUTOBAUD")
            .field(
                "glitch_filt",
                &format_args!("{}", self.glitch_filt().bits()),
            )
            .field("autobaud_en", &format_args!("{}", self.autobaud_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_AUTOBAUD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable baudrate detect"]
    #[inline(always)]
    #[must_use]
    pub fn autobaud_en(&mut self) -> AUTOBAUD_EN_W<UART_AUTOBAUD_SPEC> {
        AUTOBAUD_EN_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn glitch_filt(&mut self) -> GLITCH_FILT_W<UART_AUTOBAUD_SPEC> {
        GLITCH_FILT_W::new(self, 8)
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
#[doc = "UART BAUDRATE DETECT REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_autobaud::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_autobaud::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_AUTOBAUD_SPEC;
impl crate::RegisterSpec for UART_AUTOBAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_autobaud::R`](R) reader structure"]
impl crate::Readable for UART_AUTOBAUD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_autobaud::W`](W) writer structure"]
impl crate::Writable for UART_AUTOBAUD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_AUTOBAUD to value 0"]
impl crate::Resettable for UART_AUTOBAUD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
