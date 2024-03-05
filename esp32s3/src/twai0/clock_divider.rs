#[doc = "Register `CLOCK_DIVIDER` reader"]
pub type R = crate::R<CLOCK_DIVIDER_SPEC>;
#[doc = "Register `CLOCK_DIVIDER` writer"]
pub type W = crate::W<CLOCK_DIVIDER_SPEC>;
#[doc = "Field `CD` reader - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
pub type CD_R = crate::FieldReader;
#[doc = "Field `CD` writer - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
pub type CD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLOCK_OFF` reader - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
pub type CLOCK_OFF_R = crate::BitReader;
#[doc = "Field `CLOCK_OFF` writer - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
pub type CLOCK_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
    #[inline(always)]
    pub fn clock_off(&self) -> CLOCK_OFF_R {
        CLOCK_OFF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK_DIVIDER")
            .field("cd", &format_args!("{}", self.cd().bits()))
            .field("clock_off", &format_args!("{}", self.clock_off().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLOCK_DIVIDER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
    #[inline(always)]
    #[must_use]
    pub fn cd(&mut self) -> CD_W<CLOCK_DIVIDER_SPEC> {
        CD_W::new(self, 0)
    }
    #[doc = "Bit 8 - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
    #[inline(always)]
    #[must_use]
    pub fn clock_off(&mut self) -> CLOCK_OFF_W<CLOCK_DIVIDER_SPEC> {
        CLOCK_OFF_W::new(self, 8)
    }
}
#[doc = "Clock Divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_divider::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_divider::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_DIVIDER_SPEC;
impl crate::RegisterSpec for CLOCK_DIVIDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_divider::R`](R) reader structure"]
impl crate::Readable for CLOCK_DIVIDER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock_divider::W`](W) writer structure"]
impl crate::Writable for CLOCK_DIVIDER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCK_DIVIDER to value 0"]
impl crate::Resettable for CLOCK_DIVIDER_SPEC {
    const RESET_VALUE: u32 = 0;
}
