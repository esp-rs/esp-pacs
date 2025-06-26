#[doc = "Register `Core_1_STATUSTABLE_CURRENT` reader"]
pub type R = crate::R<CORE_1_STATUSTABLE_CURRENT_SPEC>;
#[doc = "Register `Core_1_STATUSTABLE_CURRENT` writer"]
pub type W = crate::W<CORE_1_STATUSTABLE_CURRENT_SPEC>;
#[doc = "Field `CORE_1_STATUSTABLE_CURRENT` reader - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1"]
pub type CORE_1_STATUSTABLE_CURRENT_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_1_STATUSTABLE_CURRENT` writer - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1"]
pub type CORE_1_STATUSTABLE_CURRENT_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 1:13 - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1"]
    #[inline(always)]
    pub fn core_1_statustable_current(&self) -> CORE_1_STATUSTABLE_CURRENT_R {
        CORE_1_STATUSTABLE_CURRENT_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_STATUSTABLE_CURRENT")
            .field(
                "core_1_statustable_current",
                &self.core_1_statustable_current(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 1:13 - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1"]
    #[inline(always)]
    pub fn core_1_statustable_current(
        &mut self,
    ) -> CORE_1_STATUSTABLE_CURRENT_W<CORE_1_STATUSTABLE_CURRENT_SPEC> {
        CORE_1_STATUSTABLE_CURRENT_W::new(self, 1)
    }
}
#[doc = "Status register of statustable current\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_statustable_current::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_statustable_current::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_STATUSTABLE_CURRENT_SPEC;
impl crate::RegisterSpec for CORE_1_STATUSTABLE_CURRENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_statustable_current::R`](R) reader structure"]
impl crate::Readable for CORE_1_STATUSTABLE_CURRENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_statustable_current::W`](W) writer structure"]
impl crate::Writable for CORE_1_STATUSTABLE_CURRENT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Core_1_STATUSTABLE_CURRENT to value 0"]
impl crate::Resettable for CORE_1_STATUSTABLE_CURRENT_SPEC {}
