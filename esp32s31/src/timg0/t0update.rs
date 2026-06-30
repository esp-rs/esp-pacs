#[doc = "Register `T0UPDATE` reader"]
pub type R = crate::R<T0UPDATE_SPEC>;
#[doc = "Register `T0UPDATE` writer"]
pub type W = crate::W<T0UPDATE_SPEC>;
#[doc = "Field `T_UPDATE` reader - Configures to latch the counter value. \\\\ 0: Latch \\\\ 1: Latch \\\\"]
pub type T_UPDATE_R = crate::BitReader;
#[doc = "Field `T_UPDATE` writer - Configures to latch the counter value. \\\\ 0: Latch \\\\ 1: Latch \\\\"]
pub type T_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Configures to latch the counter value. \\\\ 0: Latch \\\\ 1: Latch \\\\"]
    #[inline(always)]
    pub fn t_update(&self) -> T_UPDATE_R {
        T_UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0UPDATE")
            .field("t_update", &self.t_update())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Configures to latch the counter value. \\\\ 0: Latch \\\\ 1: Latch \\\\"]
    #[inline(always)]
    pub fn t_update(&mut self) -> T_UPDATE_W<'_, T0UPDATE_SPEC> {
        T_UPDATE_W::new(self, 31)
    }
}
#[doc = "Write to copy current timer value to TIMGn_T0LO_REG or TIMGn_T0HI_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`t0update::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0update::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0UPDATE_SPEC;
impl crate::RegisterSpec for T0UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0update::R`](R) reader structure"]
impl crate::Readable for T0UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t0update::W`](W) writer structure"]
impl crate::Writable for T0UPDATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets T0UPDATE to value 0"]
impl crate::Resettable for T0UPDATE_SPEC {}
