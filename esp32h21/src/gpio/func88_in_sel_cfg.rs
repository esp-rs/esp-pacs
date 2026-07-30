#[doc = "Register `FUNC88_IN_SEL_CFG` reader"]
pub type R = crate::R<FUNC88_IN_SEL_CFG_SPEC>;
#[doc = "Register `FUNC88_IN_SEL_CFG` writer"]
pub type W = crate::W<FUNC88_IN_SEL_CFG_SPEC>;
#[doc = "Field `FUNC88_IN_SEL` reader - set this value: s=0-29: connect GPIO\\[s\\] to this port. s=0x20: set this port always high level. s=0x30: set this port always low level."]
pub type FUNC88_IN_SEL_R = crate::FieldReader;
#[doc = "Field `FUNC88_IN_SEL` writer - set this value: s=0-29: connect GPIO\\[s\\] to this port. s=0x20: set this port always high level. s=0x30: set this port always low level."]
pub type FUNC88_IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FUNC88_IN_INV_SEL` reader - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC88_IN_INV_SEL_R = crate::BitReader;
#[doc = "Field `FUNC88_IN_INV_SEL` writer - set this bit to invert input signal. 1:invert. 0:not invert."]
pub type FUNC88_IN_INV_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIG88_IN_SEL` reader - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG88_IN_SEL_R = crate::BitReader;
#[doc = "Field `SIG88_IN_SEL` writer - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
pub type SIG88_IN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - set this value: s=0-29: connect GPIO\\[s\\] to this port. s=0x20: set this port always high level. s=0x30: set this port always low level."]
    #[inline(always)]
    pub fn func88_in_sel(&self) -> FUNC88_IN_SEL_R {
        FUNC88_IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn func88_in_inv_sel(&self) -> FUNC88_IN_INV_SEL_R {
        FUNC88_IN_INV_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sig88_in_sel(&self) -> SIG88_IN_SEL_R {
        SIG88_IN_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC88_IN_SEL_CFG")
            .field("func88_in_sel", &self.func88_in_sel())
            .field("func88_in_inv_sel", &self.func88_in_inv_sel())
            .field("sig88_in_sel", &self.sig88_in_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - set this value: s=0-29: connect GPIO\\[s\\] to this port. s=0x20: set this port always high level. s=0x30: set this port always low level."]
    #[inline(always)]
    pub fn func88_in_sel(&mut self) -> FUNC88_IN_SEL_W<'_, FUNC88_IN_SEL_CFG_SPEC> {
        FUNC88_IN_SEL_W::new(self, 0)
    }
    #[doc = "Bit 6 - set this bit to invert input signal. 1:invert. 0:not invert."]
    #[inline(always)]
    pub fn func88_in_inv_sel(&mut self) -> FUNC88_IN_INV_SEL_W<'_, FUNC88_IN_SEL_CFG_SPEC> {
        FUNC88_IN_INV_SEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - set this bit to bypass GPIO. 1:do not bypass GPIO. 0:bypass GPIO."]
    #[inline(always)]
    pub fn sig88_in_sel(&mut self) -> SIG88_IN_SEL_W<'_, FUNC88_IN_SEL_CFG_SPEC> {
        SIG88_IN_SEL_W::new(self, 7)
    }
}
#[doc = "GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func88_in_sel_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func88_in_sel_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC88_IN_SEL_CFG_SPEC;
impl crate::RegisterSpec for FUNC88_IN_SEL_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func88_in_sel_cfg::R`](R) reader structure"]
impl crate::Readable for FUNC88_IN_SEL_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func88_in_sel_cfg::W`](W) writer structure"]
impl crate::Writable for FUNC88_IN_SEL_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC88_IN_SEL_CFG to value 0x30"]
impl crate::Resettable for FUNC88_IN_SEL_CFG_SPEC {
    const RESET_VALUE: u32 = 0x30;
}
