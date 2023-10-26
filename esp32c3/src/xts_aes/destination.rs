#[doc = "Register `DESTINATION` reader"]
pub type R = crate::R<DESTINATION_SPEC>;
#[doc = "Register `DESTINATION` writer"]
pub type W = crate::W<DESTINATION_SPEC>;
#[doc = "Field `DESTINATION` reader - This bit stores the destination. 0: flash(default). 1: reserved."]
pub type DESTINATION_R = crate::BitReader;
#[doc = "Field `DESTINATION` writer - This bit stores the destination. 0: flash(default). 1: reserved."]
pub type DESTINATION_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - This bit stores the destination. 0: flash(default). 1: reserved."]
    #[inline(always)]
    pub fn destination(&self) -> DESTINATION_R {
        DESTINATION_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DESTINATION")
            .field("destination", &format_args!("{}", self.destination().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DESTINATION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit stores the destination. 0: flash(default). 1: reserved."]
    #[inline(always)]
    #[must_use]
    pub fn destination(&mut self) -> DESTINATION_W<DESTINATION_SPEC, 0> {
        DESTINATION_W::new(self)
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
#[doc = "XTS-AES destination register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`destination::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`destination::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESTINATION_SPEC;
impl crate::RegisterSpec for DESTINATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`destination::R`](R) reader structure"]
impl crate::Readable for DESTINATION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`destination::W`](W) writer structure"]
impl crate::Writable for DESTINATION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTINATION to value 0"]
impl crate::Resettable for DESTINATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
