#[doc = "Register `J0_%s` reader"]
pub type R = crate::R<J0__SPEC>;
#[doc = "Register `J0_%s` writer"]
pub type W = crate::W<J0__SPEC>;
#[doc = "Field `J0` reader - This register stores the %sth 32-bit piece of 128-bit J0"]
pub type J0_R = crate::FieldReader<u32>;
#[doc = "Field `J0` writer - This register stores the %sth 32-bit piece of 128-bit J0"]
pub type J0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit J0"]
    #[inline(always)]
    pub fn j0(&self) -> J0_R {
        J0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("J0_")
            .field("j0", &format_args!("{}", self.j0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<J0__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit J0"]
    #[inline(always)]
    #[must_use]
    pub fn j0(&mut self) -> J0_W<J0__SPEC, 0> {
        J0_W::new(self)
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
#[doc = "J0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`j0_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`j0_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct J0__SPEC;
impl crate::RegisterSpec for J0__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`j0_::R`](R) reader structure"]
impl crate::Readable for J0__SPEC {}
#[doc = "`write(|w| ..)` method takes [`j0_::W`](W) writer structure"]
impl crate::Writable for J0__SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets J0_%s to value 0"]
impl crate::Resettable for J0__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
