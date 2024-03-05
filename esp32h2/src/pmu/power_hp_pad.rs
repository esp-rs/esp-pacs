#[doc = "Register `POWER_HP_PAD` reader"]
pub type R = crate::R<POWER_HP_PAD_SPEC>;
#[doc = "Register `POWER_HP_PAD` writer"]
pub type W = crate::W<POWER_HP_PAD_SPEC>;
#[doc = "Field `FORCE_HP_PAD_NO_ISO_ALL` reader - need_des"]
pub type FORCE_HP_PAD_NO_ISO_ALL_R = crate::BitReader;
#[doc = "Field `FORCE_HP_PAD_NO_ISO_ALL` writer - need_des"]
pub type FORCE_HP_PAD_NO_ISO_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_PAD_ISO_ALL` reader - need_des"]
pub type FORCE_HP_PAD_ISO_ALL_R = crate::BitReader;
#[doc = "Field `FORCE_HP_PAD_ISO_ALL` writer - need_des"]
pub type FORCE_HP_PAD_ISO_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_hp_pad_no_iso_all(&self) -> FORCE_HP_PAD_NO_ISO_ALL_R {
        FORCE_HP_PAD_NO_ISO_ALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_hp_pad_iso_all(&self) -> FORCE_HP_PAD_ISO_ALL_R {
        FORCE_HP_PAD_ISO_ALL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_HP_PAD")
            .field(
                "force_hp_pad_no_iso_all",
                &format_args!("{}", self.force_hp_pad_no_iso_all().bit()),
            )
            .field(
                "force_hp_pad_iso_all",
                &format_args!("{}", self.force_hp_pad_iso_all().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_HP_PAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_pad_no_iso_all(&mut self) -> FORCE_HP_PAD_NO_ISO_ALL_W<POWER_HP_PAD_SPEC> {
        FORCE_HP_PAD_NO_ISO_ALL_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn force_hp_pad_iso_all(&mut self) -> FORCE_HP_PAD_ISO_ALL_W<POWER_HP_PAD_SPEC> {
        FORCE_HP_PAD_ISO_ALL_W::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_hp_pad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_hp_pad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_HP_PAD_SPEC;
impl crate::RegisterSpec for POWER_HP_PAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_hp_pad::R`](R) reader structure"]
impl crate::Readable for POWER_HP_PAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_hp_pad::W`](W) writer structure"]
impl crate::Writable for POWER_HP_PAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_HP_PAD to value 0"]
impl crate::Resettable for POWER_HP_PAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
