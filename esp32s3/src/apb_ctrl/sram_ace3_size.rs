#[doc = "Register `SRAM_ACE3_SIZE` reader"]
pub type R = crate::R<SRAM_ACE3_SIZE_SPEC>;
#[doc = "Register `SRAM_ACE3_SIZE` writer"]
pub type W = crate::W<SRAM_ACE3_SIZE_SPEC>;
#[doc = "Field `SRAM_ACE3_SIZE` reader - ******* Description ***********"]
pub type SRAM_ACE3_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `SRAM_ACE3_SIZE` writer - ******* Description ***********"]
pub type SRAM_ACE3_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ******* Description ***********"]
    #[inline(always)]
    pub fn sram_ace3_size(&self) -> SRAM_ACE3_SIZE_R {
        SRAM_ACE3_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_ACE3_SIZE")
            .field(
                "sram_ace3_size",
                &format_args!("{}", self.sram_ace3_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_ACE3_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ace3_size(&mut self) -> SRAM_ACE3_SIZE_W<SRAM_ACE3_SIZE_SPEC> {
        SRAM_ACE3_SIZE_W::new(self, 0)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace3_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace3_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_ACE3_SIZE_SPEC;
impl crate::RegisterSpec for SRAM_ACE3_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ace3_size::R`](R) reader structure"]
impl crate::Readable for SRAM_ACE3_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ace3_size::W`](W) writer structure"]
impl crate::Writable for SRAM_ACE3_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_ACE3_SIZE to value 0x1000"]
impl crate::Resettable for SRAM_ACE3_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
