#[doc = "Register `TIME_UPDATE` reader"]
pub type R = crate::R<TIME_UPDATE_SPEC>;
#[doc = "Register `TIME_UPDATE` writer"]
pub type W = crate::W<TIME_UPDATE_SPEC>;
#[doc = "Field `TIME_VALID` reader - To indicate the register is updated"]
pub type TIME_VALID_R = crate::BitReader;
#[doc = "Field `TIME_UPDATE` writer - Set 1: to update register with RTC timer"]
pub type TIME_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - To indicate the register is updated"]
    #[inline(always)]
    pub fn time_valid(&self) -> TIME_VALID_R {
        TIME_VALID_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_UPDATE")
            .field("time_valid", &self.time_valid())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Set 1: to update register with RTC timer"]
    #[inline(always)]
    #[must_use]
    pub fn time_update(&mut self) -> TIME_UPDATE_W<TIME_UPDATE_SPEC> {
        TIME_UPDATE_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_update::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_update::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_UPDATE_SPEC;
impl crate::RegisterSpec for TIME_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_update::R`](R) reader structure"]
impl crate::Readable for TIME_UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time_update::W`](W) writer structure"]
impl crate::Writable for TIME_UPDATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME_UPDATE to value 0"]
impl crate::Resettable for TIME_UPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
