#[doc = "Register `VALUE_UPDATE` reader"]
pub type R = crate::R<VALUE_UPDATE_SPEC>;
#[doc = "Register `VALUE_UPDATE` writer"]
pub type W = crate::W<VALUE_UPDATE_SPEC>;
#[doc = "Field `VALUE_UPDATE` writer - needs field desc"]
pub type VALUE_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS` reader - needs field desc"]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `BYPASS` writer - needs field desc"]
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - needs field desc"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VALUE_UPDATE")
            .field("bypass", &self.bypass())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn value_update(&mut self) -> VALUE_UPDATE_W<VALUE_UPDATE_SPEC> {
        VALUE_UPDATE_W::new(self, 0)
    }
    #[doc = "Bit 1 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<VALUE_UPDATE_SPEC> {
        BYPASS_W::new(self, 1)
    }
}
#[doc = "needs field desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value_update::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`value_update::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VALUE_UPDATE_SPEC;
impl crate::RegisterSpec for VALUE_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value_update::R`](R) reader structure"]
impl crate::Readable for VALUE_UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`value_update::W`](W) writer structure"]
impl crate::Writable for VALUE_UPDATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VALUE_UPDATE to value 0"]
impl crate::Resettable for VALUE_UPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
