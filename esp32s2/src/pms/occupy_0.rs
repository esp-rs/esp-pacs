#[doc = "Register `OCCUPY_0` reader"]
pub type R = crate::R<OCCUPY_0_SPEC>;
#[doc = "Register `OCCUPY_0` writer"]
pub type W = crate::W<OCCUPY_0_SPEC>;
#[doc = "Field `OCCUPY_LOCK` reader - Lock register. Setting to 1 locks occupy permission control registers."]
pub type OCCUPY_LOCK_R = crate::BitReader;
#[doc = "Field `OCCUPY_LOCK` writer - Lock register. Setting to 1 locks occupy permission control registers."]
pub type OCCUPY_LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
            .field("occupy_lock", &format_args!("{}", self.occupy_lock().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OCCUPY_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks occupy permission control registers."]
    #[inline(always)]
    #[must_use]
    pub fn occupy_lock(&mut self) -> OCCUPY_LOCK_W<OCCUPY_0_SPEC, 0> {
        OCCUPY_LOCK_W::new(self)
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
#[doc = "Occupy permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`occupy_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`occupy_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCCUPY_0_SPEC;
impl crate::RegisterSpec for OCCUPY_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`occupy_0::R`](R) reader structure"]
impl crate::Readable for OCCUPY_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`occupy_0::W`](W) writer structure"]
impl crate::Writable for OCCUPY_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCCUPY_0 to value 0"]
impl crate::Resettable for OCCUPY_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
