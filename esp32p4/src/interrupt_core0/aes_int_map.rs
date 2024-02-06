#[doc = "Register `AES_INT_MAP` reader"]
pub type R = crate::R<AES_INT_MAP_SPEC>;
#[doc = "Register `AES_INT_MAP` writer"]
pub type W = crate::W<AES_INT_MAP_SPEC>;
#[doc = "Field `CORE0_AES_INT_MAP` reader - NA"]
pub type CORE0_AES_INT_MAP_R = crate::FieldReader;
#[doc = "Field `CORE0_AES_INT_MAP` writer - NA"]
pub type CORE0_AES_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    pub fn core0_aes_int_map(&self) -> CORE0_AES_INT_MAP_R {
        CORE0_AES_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES_INT_MAP")
            .field(
                "core0_aes_int_map",
                &format_args!("{}", self.core0_aes_int_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AES_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn core0_aes_int_map(&mut self) -> CORE0_AES_INT_MAP_W<AES_INT_MAP_SPEC> {
        CORE0_AES_INT_MAP_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aes_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AES_INT_MAP_SPEC;
impl crate::RegisterSpec for AES_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_int_map::R`](R) reader structure"]
impl crate::Readable for AES_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aes_int_map::W`](W) writer structure"]
impl crate::Writable for AES_INT_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_INT_MAP to value 0"]
impl crate::Resettable for AES_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
