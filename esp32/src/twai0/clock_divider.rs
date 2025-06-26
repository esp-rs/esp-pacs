#[doc = "Register `CLOCK_DIVIDER` reader"]
pub type R = crate::R<CLOCK_DIVIDER_SPEC>;
#[doc = "Register `CLOCK_DIVIDER` writer"]
pub type W = crate::W<CLOCK_DIVIDER_SPEC>;
#[doc = "Field `CD` reader - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
pub type CD_R = crate::FieldReader;
#[doc = "Field `CD` writer - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
pub type CD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLOCK_OFF` reader - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
pub type CLOCK_OFF_R = crate::BitReader;
#[doc = "Field `CLOCK_OFF` writer - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
pub type CLOCK_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_MODE` reader - This bit can be configured under reset mode. 1: Extended mode, compatiable with CAN2.0B; 0: Basic mode"]
pub type EXT_MODE_R = crate::BitReader;
#[doc = "Field `EXT_MODE` writer - This bit can be configured under reset mode. 1: Extended mode, compatiable with CAN2.0B; 0: Basic mode"]
pub type EXT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
    #[inline(always)]
    pub fn clock_off(&self) -> CLOCK_OFF_R {
        CLOCK_OFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit can be configured under reset mode. 1: Extended mode, compatiable with CAN2.0B; 0: Basic mode"]
    #[inline(always)]
    pub fn ext_mode(&self) -> EXT_MODE_R {
        EXT_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCK_DIVIDER")
            .field("cd", &self.cd())
            .field("clock_off", &self.clock_off())
            .field("ext_mode", &self.ext_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - These bits are used to configure frequency dividing coefficients of the external CLKOUT pin."]
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W<CLOCK_DIVIDER_SPEC> {
        CD_W::new(self, 0)
    }
    #[doc = "Bit 3 - This bit can be configured under reset mode. 1: Disable the external CLKOUT pin; 0: Enable the external CLKOUT pin"]
    #[inline(always)]
    pub fn clock_off(&mut self) -> CLOCK_OFF_W<CLOCK_DIVIDER_SPEC> {
        CLOCK_OFF_W::new(self, 3)
    }
    #[doc = "Bit 7 - This bit can be configured under reset mode. 1: Extended mode, compatiable with CAN2.0B; 0: Basic mode"]
    #[inline(always)]
    pub fn ext_mode(&mut self) -> EXT_MODE_W<CLOCK_DIVIDER_SPEC> {
        EXT_MODE_W::new(self, 7)
    }
}
#[doc = "Clock Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_divider::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_divider::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_DIVIDER_SPEC;
impl crate::RegisterSpec for CLOCK_DIVIDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_divider::R`](R) reader structure"]
impl crate::Readable for CLOCK_DIVIDER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock_divider::W`](W) writer structure"]
impl crate::Writable for CLOCK_DIVIDER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCK_DIVIDER to value 0"]
impl crate::Resettable for CLOCK_DIVIDER_SPEC {}
