#[doc = "Register `AUTOBAUD` reader"]
pub type R = crate::R<AUTOBAUD_SPEC>;
#[doc = "Register `AUTOBAUD` writer"]
pub type W = crate::W<AUTOBAUD_SPEC>;
#[doc = "Field `EN` reader - This is the enable bit for detecting baudrate."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - This is the enable bit for detecting baudrate."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_FILT` reader - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
pub type GLITCH_FILT_R = crate::FieldReader;
#[doc = "Field `GLITCH_FILT` writer - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
pub type GLITCH_FILT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - This is the enable bit for detecting baudrate."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
    #[inline(always)]
    pub fn glitch_filt(&self) -> GLITCH_FILT_R {
        GLITCH_FILT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUTOBAUD")
            .field("en", &self.en())
            .field("glitch_filt", &self.glitch_filt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This is the enable bit for detecting baudrate."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, AUTOBAUD_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - when input pulse width is lower then this value igore this pulse.this register is used in autobaud detect process."]
    #[inline(always)]
    pub fn glitch_filt(&mut self) -> GLITCH_FILT_W<'_, AUTOBAUD_SPEC> {
        GLITCH_FILT_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`autobaud::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autobaud::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AUTOBAUD_SPEC;
impl crate::RegisterSpec for AUTOBAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autobaud::R`](R) reader structure"]
impl crate::Readable for AUTOBAUD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`autobaud::W`](W) writer structure"]
impl crate::Writable for AUTOBAUD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTOBAUD to value 0x1000"]
impl crate::Resettable for AUTOBAUD_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
