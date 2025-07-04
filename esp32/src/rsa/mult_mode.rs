#[doc = "Register `MULT_MODE` reader"]
pub type R = crate::R<MULT_MODE_SPEC>;
#[doc = "Register `MULT_MODE` writer"]
pub type W = crate::W<MULT_MODE_SPEC>;
#[doc = "Field `MULT_MODE` reader - This register contains the mode of modular multiplication and multiplication."]
pub type MULT_MODE_R = crate::FieldReader;
#[doc = "Field `MULT_MODE` writer - This register contains the mode of modular multiplication and multiplication."]
pub type MULT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - This register contains the mode of modular multiplication and multiplication."]
    #[inline(always)]
    pub fn mult_mode(&self) -> MULT_MODE_R {
        MULT_MODE_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_MODE")
            .field("mult_mode", &self.mult_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - This register contains the mode of modular multiplication and multiplication."]
    #[inline(always)]
    pub fn mult_mode(&mut self) -> MULT_MODE_W<MULT_MODE_SPEC> {
        MULT_MODE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_MODE_SPEC;
impl crate::RegisterSpec for MULT_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mult_mode::R`](R) reader structure"]
impl crate::Readable for MULT_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mult_mode::W`](W) writer structure"]
impl crate::Writable for MULT_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULT_MODE to value 0"]
impl crate::Resettable for MULT_MODE_SPEC {}
