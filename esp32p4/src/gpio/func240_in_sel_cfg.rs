#[doc = "Register `FUNC240_IN_SEL_CFG` reader"]
pub type R = crate::R<FUNC240_IN_SEL_CFG_SPEC>;
#[doc = "Register `FUNC240_IN_SEL_CFG` writer"]
pub type W = crate::W<FUNC240_IN_SEL_CFG_SPEC>;
#[doc = "Field `FUNC240_IN_SEL` reader - set this value: s=0-56: connect GPIO\\[s\\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level."]
pub type FUNC240_IN_SEL_R = crate::FieldReader;
#[doc = "Field `FUNC240_IN_SEL` writer - set this value: s=0-56: connect GPIO\\[s\\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level."]
pub type FUNC240_IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FUNC240_IN_INV_SEL` reader - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC240_IN_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC240_IN_INV_SEL` writer - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC240_IN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIG240_IN_SEL` reader - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG240_IN_SEL_R = crate::BitReader;
#[doc = "Field `SIG240_IN_SEL` writer - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG240_IN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - set this value: s=0-56: connect GPIO\\[s\\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level."]
    #[inline(always)]
    pub fn func240_in_sel(&self) -> FUNC240_IN_SEL_R {
        FUNC240_IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn func240_in_inv_sel(&self) -> FUNC240_IN_INV_SEL_R {
        FUNC240_IN_INV_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sig240_in_sel(&self) -> SIG240_IN_SEL_R {
        SIG240_IN_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC240_IN_SEL_CFG")
            .field(
                "func240_in_sel",
                &format_args!("{}", self.func240_in_sel().bits()),
            )
            .field(
                "func240_in_inv_sel",
                &format_args!("{}", self.func240_in_inv_sel().bit()),
            )
            .field(
                "sig240_in_sel",
                &format_args!("{}", self.sig240_in_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FUNC240_IN_SEL_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - set this value: s=0-56: connect GPIO\\[s\\] to this port. s=0x3F: set this port always high level. s=0x3E: set this port always low level."]
    #[inline(always)]
    #[must_use]
    pub fn func240_in_sel(&mut self) -> FUNC240_IN_SEL_W<FUNC240_IN_SEL_CFG_SPEC> {
        FUNC240_IN_SEL_W::new(self, 0)
    }
    #[doc = "Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    #[must_use]
    pub fn func240_in_inv_sel(&mut self) -> FUNC240_IN_INV_SEL_W<FUNC240_IN_SEL_CFG_SPEC> {
        FUNC240_IN_INV_SEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn sig240_in_sel(&mut self) -> SIG240_IN_SEL_W<FUNC240_IN_SEL_CFG_SPEC> {
        SIG240_IN_SEL_W::new(self, 7)
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
#[doc = "GPIO input function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func240_in_sel_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func240_in_sel_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC240_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC240_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func240_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC240_IN_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func240_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC240_IN_SEL_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNC240_IN_SEL_CFG to value 0x3e"]
impl crate::Resettable for FUNC240_IN_SEL_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x3e;
}
