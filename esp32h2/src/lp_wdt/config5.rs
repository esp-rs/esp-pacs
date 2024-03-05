#[doc = "Register `CONFIG5` reader"]
pub type R = crate::R<CONFIG5_SPEC>;
#[doc = "Register `CONFIG5` writer"]
pub type W = crate::W<CONFIG5_SPEC>;
#[doc = "Field `CHIP_RESET_TARGET` reader - need_des"]
pub type CHIP_RESET_TARGET_R = crate::FieldReader;
#[doc = "Field `CHIP_RESET_TARGET` writer - need_des"]
pub type CHIP_RESET_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHIP_RESET_EN` reader - need_des"]
pub type CHIP_RESET_EN_R = crate::BitReader;
#[doc = "Field `CHIP_RESET_EN` writer - need_des"]
pub type CHIP_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHIP_RESET_KEY` reader - need_des"]
pub type CHIP_RESET_KEY_R = crate::FieldReader;
#[doc = "Field `CHIP_RESET_KEY` writer - need_des"]
pub type CHIP_RESET_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn chip_reset_target(&self) -> CHIP_RESET_TARGET_R {
        CHIP_RESET_TARGET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn chip_reset_en(&self) -> CHIP_RESET_EN_R {
        CHIP_RESET_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - need_des"]
    #[inline(always)]
    pub fn chip_reset_key(&self) -> CHIP_RESET_KEY_R {
        CHIP_RESET_KEY_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG5")
            .field(
                "chip_reset_target",
                &format_args!("{}", self.chip_reset_target().bits()),
            )
            .field(
                "chip_reset_en",
                &format_args!("{}", self.chip_reset_en().bit()),
            )
            .field(
                "chip_reset_key",
                &format_args!("{}", self.chip_reset_key().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONFIG5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn chip_reset_target(&mut self) -> CHIP_RESET_TARGET_W<CONFIG5_SPEC> {
        CHIP_RESET_TARGET_W::new(self, 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn chip_reset_en(&mut self) -> CHIP_RESET_EN_W<CONFIG5_SPEC> {
        CHIP_RESET_EN_W::new(self, 8)
    }
    #[doc = "Bits 9:16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn chip_reset_key(&mut self) -> CHIP_RESET_KEY_W<CONFIG5_SPEC> {
        CHIP_RESET_KEY_W::new(self, 9)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG5_SPEC;
impl crate::RegisterSpec for CONFIG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config5::R`](R) reader structure"]
impl crate::Readable for CONFIG5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config5::W`](W) writer structure"]
impl crate::Writable for CONFIG5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG5 to value 0xff"]
impl crate::Resettable for CONFIG5_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
