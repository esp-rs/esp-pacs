#[doc = "Register `PAUSE_EN` reader"]
pub type R = crate::R<PAUSE_EN_SPEC>;
#[doc = "Register `PAUSE_EN` writer"]
pub type W = crate::W<PAUSE_EN_SPEC>;
#[doc = "Field `PAUSE_EN` reader - need_des"]
pub type PAUSE_EN_R = crate::BitReader;
#[doc = "Field `PAUSE_EN` writer - need_des"]
pub type PAUSE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn pause_en(&self) -> PAUSE_EN_R {
        PAUSE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAUSE_EN")
            .field("pause_en", &self.pause_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn pause_en(&mut self) -> PAUSE_EN_W<'_, PAUSE_EN_SPEC> {
        PAUSE_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pause_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pause_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAUSE_EN_SPEC;
impl crate::RegisterSpec for PAUSE_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pause_en::R`](R) reader structure"]
impl crate::Readable for PAUSE_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pause_en::W`](W) writer structure"]
impl crate::Writable for PAUSE_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAUSE_EN to value 0x8000_0000"]
impl crate::Resettable for PAUSE_EN_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
