#[doc = "Register `AUTOBAUD` reader"]
pub type R = crate::R<AUTOBAUD_SPEC>;
#[doc = "Register `AUTOBAUD` writer"]
pub type W = crate::W<AUTOBAUD_SPEC>;
#[doc = "Field `EN` reader - This is the enable bit for baud rate detection."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - This is the enable bit for baud rate detection."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_FILT` reader - When input pulse width is lower than this value, the pulse is ignored. This register is used in autobaud detection."]
pub type GLITCH_FILT_R = crate::FieldReader;
#[doc = "Field `GLITCH_FILT` writer - When input pulse width is lower than this value, the pulse is ignored. This register is used in autobaud detection."]
pub type GLITCH_FILT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - This is the enable bit for baud rate detection."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - When input pulse width is lower than this value, the pulse is ignored. This register is used in autobaud detection."]
    #[inline(always)]
    pub fn glitch_filt(&self) -> GLITCH_FILT_R {
        GLITCH_FILT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOBAUD")
            .field("en", &format_args!("{}", self.en().bit()))
            .field(
                "glitch_filt",
                &format_args!("{}", self.glitch_filt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AUTOBAUD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit for baud rate detection."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<AUTOBAUD_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - When input pulse width is lower than this value, the pulse is ignored. This register is used in autobaud detection."]
    #[inline(always)]
    #[must_use]
    pub fn glitch_filt(&mut self) -> GLITCH_FILT_W<AUTOBAUD_SPEC> {
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
#[doc = "Autobaud configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`autobaud::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`autobaud::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUTOBAUD_SPEC;
impl crate::RegisterSpec for AUTOBAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autobaud::R`](R) reader structure"]
impl crate::Readable for AUTOBAUD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`autobaud::W`](W) writer structure"]
impl crate::Writable for AUTOBAUD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUTOBAUD to value 0x1000"]
impl crate::Resettable for AUTOBAUD_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
