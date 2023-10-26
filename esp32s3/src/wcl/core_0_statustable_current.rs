#[doc = "Register `Core_0_STATUSTABLE_CURRENT` reader"]
pub type R = crate::R<CORE_0_STATUSTABLE_CURRENT_SPEC>;
#[doc = "Register `Core_0_STATUSTABLE_CURRENT` writer"]
pub type W = crate::W<CORE_0_STATUSTABLE_CURRENT_SPEC>;
#[doc = "Field `CORE_0_STATUSTABLE_CURRENT` reader - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1,bit2 represents the current field of STATUSTABLE2"]
pub type CORE_0_STATUSTABLE_CURRENT_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_0_STATUSTABLE_CURRENT` writer - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1,bit2 represents the current field of STATUSTABLE2"]
pub type CORE_0_STATUSTABLE_CURRENT_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 13, O, u16>;
impl R {
    #[doc = "Bits 1:13 - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1,bit2 represents the current field of STATUSTABLE2"]
    #[inline(always)]
    pub fn core_0_statustable_current(&self) -> CORE_0_STATUSTABLE_CURRENT_R {
        CORE_0_STATUSTABLE_CURRENT_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_STATUSTABLE_CURRENT")
            .field(
                "core_0_statustable_current",
                &format_args!("{}", self.core_0_statustable_current().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_STATUSTABLE_CURRENT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 1:13 - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1,bit2 represents the current field of STATUSTABLE2"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_statustable_current(
        &mut self,
    ) -> CORE_0_STATUSTABLE_CURRENT_W<CORE_0_STATUSTABLE_CURRENT_SPEC, 1> {
        CORE_0_STATUSTABLE_CURRENT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status register of statustable current\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_statustable_current::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_statustable_current::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_STATUSTABLE_CURRENT_SPEC;
impl crate::RegisterSpec for CORE_0_STATUSTABLE_CURRENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_statustable_current::R`](R) reader structure"]
impl crate::Readable for CORE_0_STATUSTABLE_CURRENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_statustable_current::W`](W) writer structure"]
impl crate::Writable for CORE_0_STATUSTABLE_CURRENT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_0_STATUSTABLE_CURRENT to value 0"]
impl crate::Resettable for CORE_0_STATUSTABLE_CURRENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
