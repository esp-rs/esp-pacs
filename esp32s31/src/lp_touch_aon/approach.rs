#[doc = "Register `APPROACH` reader"]
pub type R = crate::R<APPROACH_SPEC>;
#[doc = "Register `APPROACH` writer"]
pub type W = crate::W<APPROACH_SPEC>;
#[doc = "Field `TOUCH_APPROACH_PAD0` reader - need_des"]
pub type TOUCH_APPROACH_PAD0_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_PAD0` writer - need_des"]
pub type TOUCH_APPROACH_PAD0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_APPROACH_PAD1` reader - need_des"]
pub type TOUCH_APPROACH_PAD1_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_PAD1` writer - need_des"]
pub type TOUCH_APPROACH_PAD1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_APPROACH_PAD2` reader - need_des"]
pub type TOUCH_APPROACH_PAD2_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_PAD2` writer - need_des"]
pub type TOUCH_APPROACH_PAD2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_SLP_APPROACH_EN` reader - need_des"]
pub type TOUCH_SLP_APPROACH_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_SLP_APPROACH_EN` writer - need_des"]
pub type TOUCH_SLP_APPROACH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - need_des"]
    #[inline(always)]
    pub fn touch_approach_pad0(&self) -> TOUCH_APPROACH_PAD0_R {
        TOUCH_APPROACH_PAD0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - need_des"]
    #[inline(always)]
    pub fn touch_approach_pad1(&self) -> TOUCH_APPROACH_PAD1_R {
        TOUCH_APPROACH_PAD1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - need_des"]
    #[inline(always)]
    pub fn touch_approach_pad2(&self) -> TOUCH_APPROACH_PAD2_R {
        TOUCH_APPROACH_PAD2_R::new(((self.bits >> 8) & 0x0f) as u8)
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
        f.debug_struct("APPROACH")
            .field("touch_approach_pad0", &self.touch_approach_pad0())
            .field("touch_approach_pad1", &self.touch_approach_pad1())
            .field("touch_approach_pad2", &self.touch_approach_pad2())
            .field("touch_slp_approach_en", &self.touch_slp_approach_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - need_des"]
    #[inline(always)]
    pub fn touch_approach_pad0(&mut self) -> TOUCH_APPROACH_PAD0_W<'_, APPROACH_SPEC> {
        TOUCH_APPROACH_PAD0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - need_des"]
    #[inline(always)]
    pub fn touch_approach_pad1(&mut self) -> TOUCH_APPROACH_PAD1_W<'_, APPROACH_SPEC> {
        TOUCH_APPROACH_PAD1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - need_des"]
    #[inline(always)]
    pub fn touch_approach_pad2(&mut self) -> TOUCH_APPROACH_PAD2_W<'_, APPROACH_SPEC> {
        TOUCH_APPROACH_PAD2_W::new(self, 8)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn touch_slp_approach_en(&mut self) -> TOUCH_SLP_APPROACH_EN_W<'_, APPROACH_SPEC> {
        TOUCH_SLP_APPROACH_EN_W::new(self, 12)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`approach::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`approach::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPROACH_SPEC;
impl crate::RegisterSpec for APPROACH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`approach::R`](R) reader structure"]
impl crate::Readable for APPROACH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`approach::W`](W) writer structure"]
impl crate::Writable for APPROACH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APPROACH to value 0x0fff"]
impl crate::Resettable for APPROACH_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
