#[doc = "Register `UPDATE` reader"]
pub type R = crate::R<UPDATE_SPEC>;
#[doc = "Register `UPDATE` writer"]
pub type W = crate::W<UPDATE_SPEC>;
#[doc = "Field `UPDATE` reader - Write any value will trigger a timer 0 time-base counter value update (timer 0 current value will be stored in registers above)"]
pub type UPDATE_R = crate::BitReader;
#[doc = "Field `UPDATE` writer - Write any value will trigger a timer 0 time-base counter value update (timer 0 current value will be stored in registers above)"]
pub type UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Write any value will trigger a timer 0 time-base counter value update (timer 0 current value will be stored in registers above)"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UPDATE")
            .field("update", &self.update())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Write any value will trigger a timer 0 time-base counter value update (timer 0 current value will be stored in registers above)"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W<UPDATE_SPEC> {
        UPDATE_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`update::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`update::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPDATE_SPEC;
impl crate::RegisterSpec for UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`update::R`](R) reader structure"]
impl crate::Readable for UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`update::W`](W) writer structure"]
impl crate::Writable for UPDATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UPDATE to value 0"]
impl crate::Resettable for UPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
