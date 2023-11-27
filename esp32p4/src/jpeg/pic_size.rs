#[doc = "Register `PIC_SIZE` reader"]
pub type R = crate::R<PIC_SIZE_SPEC>;
#[doc = "Register `PIC_SIZE` writer"]
pub type W = crate::W<PIC_SIZE_SPEC>;
#[doc = "Field `VA` reader - configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16"]
pub type VA_R = crate::FieldReader<u16>;
#[doc = "Field `VA` writer - configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16"]
pub type VA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HA` reader - configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16"]
pub type HA_R = crate::FieldReader<u16>;
#[doc = "Field `HA` writer - configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16"]
pub type HA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16"]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16"]
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIC_SIZE")
            .field("va", &format_args!("{}", self.va().bits()))
            .field("ha", &format_args!("{}", self.ha().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIC_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16"]
    #[inline(always)]
    #[must_use]
    pub fn va(&mut self) -> VA_W<PIC_SIZE_SPEC> {
        VA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16"]
    #[inline(always)]
    #[must_use]
    pub fn ha(&mut self) -> HA_W<PIC_SIZE_SPEC> {
        HA_W::new(self, 16)
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
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pic_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pic_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIC_SIZE_SPEC;
impl crate::RegisterSpec for PIC_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pic_size::R`](R) reader structure"]
impl crate::Readable for PIC_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pic_size::W`](W) writer structure"]
impl crate::Writable for PIC_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIC_SIZE to value 0x0280_01e0"]
impl crate::Resettable for PIC_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0280_01e0;
}
