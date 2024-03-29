#[doc = "Register `TOUCH_APPROACH` reader"]
pub type R = crate::R<TOUCH_APPROACH_SPEC>;
#[doc = "Register `TOUCH_APPROACH` writer"]
pub type W = crate::W<TOUCH_APPROACH_SPEC>;
#[doc = "Field `PAD0` reader - need_des"]
pub type PAD0_R = crate::FieldReader;
#[doc = "Field `PAD0` writer - need_des"]
pub type PAD0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PAD1` reader - need_des"]
pub type PAD1_R = crate::FieldReader;
#[doc = "Field `PAD1` writer - need_des"]
pub type PAD1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PAD2` reader - need_des"]
pub type PAD2_R = crate::FieldReader;
#[doc = "Field `PAD2` writer - need_des"]
pub type PAD2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_SLP_APPROACH_EN` reader - need_des"]
pub type TOUCH_SLP_APPROACH_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_SLP_APPROACH_EN` writer - need_des"]
pub type TOUCH_SLP_APPROACH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - need_des"]
    #[inline(always)]
    pub fn pad0(&self) -> PAD0_R {
        PAD0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - need_des"]
    #[inline(always)]
    pub fn pad1(&self) -> PAD1_R {
        PAD1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - need_des"]
    #[inline(always)]
    pub fn pad2(&self) -> PAD2_R {
        PAD2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn touch_slp_approach_en(&self) -> TOUCH_SLP_APPROACH_EN_R {
        TOUCH_SLP_APPROACH_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_APPROACH")
            .field("pad0", &format_args!("{}", self.pad0().bits()))
            .field("pad1", &format_args!("{}", self.pad1().bits()))
            .field("pad2", &format_args!("{}", self.pad2().bits()))
            .field(
                "touch_slp_approach_en",
                &format_args!("{}", self.touch_slp_approach_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_APPROACH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pad0(&mut self) -> PAD0_W<TOUCH_APPROACH_SPEC> {
        PAD0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pad1(&mut self) -> PAD1_W<TOUCH_APPROACH_SPEC> {
        PAD1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pad2(&mut self) -> PAD2_W<TOUCH_APPROACH_SPEC> {
        PAD2_W::new(self, 8)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_slp_approach_en(&mut self) -> TOUCH_SLP_APPROACH_EN_W<TOUCH_APPROACH_SPEC> {
        TOUCH_SLP_APPROACH_EN_W::new(self, 12)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_approach::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_approach::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_APPROACH_SPEC;
impl crate::RegisterSpec for TOUCH_APPROACH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_approach::R`](R) reader structure"]
impl crate::Readable for TOUCH_APPROACH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_approach::W`](W) writer structure"]
impl crate::Writable for TOUCH_APPROACH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_APPROACH to value 0x0fff"]
impl crate::Resettable for TOUCH_APPROACH_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
