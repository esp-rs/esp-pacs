#[doc = "Register `FUNC63_IN_SEL_CFG` reader"]
pub type R = crate::R<FUNC63_IN_SEL_CFG_SPEC>;
#[doc = "Register `FUNC63_IN_SEL_CFG` writer"]
pub type W = crate::W<FUNC63_IN_SEL_CFG_SPEC>;
#[doc = "Field `FUNC63_IN_SEL` reader - set this value: s=0-29: connect GPIO\\[s\\] to this port. s=0x20: set this port always high level. s=0x30: set this port always low level."]
pub type FUNC63_IN_SEL_R = crate::FieldReader;
#[doc = "Field `FUNC63_IN_SEL` writer - set this value: s=0-29: connect GPIO\\[s\\] to this port. s=0x20: set this port always high level. s=0x30: set this port always low level."]
pub type FUNC63_IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FUNC63_IN_INV_SEL` reader - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC63_IN_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC63_IN_INV_SEL` writer - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC63_IN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIG63_IN_SEL` reader - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG63_IN_SEL_R = crate::BitReader;
#[doc = "Field `SIG63_IN_SEL` writer - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG63_IN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - set this value: s=0-29: connect GPIO\\[s\\] to this port. s=0x20: set this port always high level. s=0x30: set this port always low level."]
    #[inline(always)]
    pub fn func63_in_sel(&self) -> FUNC63_IN_SEL_R {
        FUNC63_IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn func63_in_inv_sel(&self) -> FUNC63_IN_INV_SEL_R {
        FUNC63_IN_INV_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sig63_in_sel(&self) -> SIG63_IN_SEL_R {
        SIG63_IN_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC63_IN_SEL_CFG")
            .field("func63_in_sel", &self.func63_in_sel())
            .field("func63_in_inv_sel", &self.func63_in_inv_sel())
            .field("sig63_in_sel", &self.sig63_in_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - set this value: s=0-29: connect GPIO\\[s\\] to this port. s=0x20: set this port always high level. s=0x30: set this port always low level."]
    #[inline(always)]
    pub fn func63_in_sel(&mut self) -> FUNC63_IN_SEL_W<'_, FUNC63_IN_SEL_CFG_SPEC> {
        FUNC63_IN_SEL_W::new(self, 0)
    }
    #[doc = "Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn func63_in_inv_sel(&mut self) -> FUNC63_IN_INV_SEL_W<'_, FUNC63_IN_SEL_CFG_SPEC> {
        FUNC63_IN_INV_SEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sig63_in_sel(&mut self) -> SIG63_IN_SEL_W<'_, FUNC63_IN_SEL_CFG_SPEC> {
        SIG63_IN_SEL_W::new(self, 7)
    }
}
#[doc = "GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func63_in_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func63_in_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC63_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC63_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func63_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC63_IN_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func63_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC63_IN_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC63_IN_SEL_CFG to value 0x30"]
impl crate::Resettable for FUNC63_IN_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x30;
}
