#[doc = "Register `OCCUPY_0` reader"]
pub type R = crate::R<OCCUPY_0_SPEC>;
#[doc = "Register `OCCUPY_0` writer"]
pub type W = crate::W<OCCUPY_0_SPEC>;
#[doc = "Field `OCCUPY_LOCK` reader - Lock register. Setting to 1 locks occupy permission control registers."]
pub type OCCUPY_LOCK_R = crate::BitReader;
#[doc = "Field `OCCUPY_LOCK` writer - Lock register. Setting to 1 locks occupy permission control registers."]
pub type OCCUPY_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks occupy permission control registers."]
    #[inline(always)]
    pub fn occupy_lock(&self) -> OCCUPY_LOCK_R {
        OCCUPY_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCCUPY_0")
            .field("occupy_lock", &self.occupy_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks occupy permission control registers."]
    #[inline(always)]
    pub fn occupy_lock(&mut self) -> OCCUPY_LOCK_W<OCCUPY_0_SPEC> {
        OCCUPY_LOCK_W::new(self, 0)
    }
}
#[doc = "Occupy permission control register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`occupy_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`occupy_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCCUPY_0_SPEC;
impl crate::RegisterSpec for OCCUPY_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`occupy_0::R`](R) reader structure"]
impl crate::Readable for OCCUPY_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`occupy_0::W`](W) writer structure"]
impl crate::Writable for OCCUPY_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OCCUPY_0 to value 0"]
impl crate::Resettable for OCCUPY_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
