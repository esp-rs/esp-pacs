#[doc = "Register `SRAM_ACE2_SIZE` reader"]
pub type R = crate::R<SRAM_ACE2_SIZE_SPEC>;
#[doc = "Register `SRAM_ACE2_SIZE` writer"]
pub type W = crate::W<SRAM_ACE2_SIZE_SPEC>;
#[doc = "Field `SRAM_ACE2_SIZE` reader - "]
pub type SRAM_ACE2_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `SRAM_ACE2_SIZE` writer - "]
pub type SRAM_ACE2_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sram_ace2_size(&self) -> SRAM_ACE2_SIZE_R {
        SRAM_ACE2_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_ACE2_SIZE")
            .field(
                "sram_ace2_size",
                &format_args!("{}", self.sram_ace2_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_ACE2_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn sram_ace2_size(&mut self) -> SRAM_ACE2_SIZE_W<SRAM_ACE2_SIZE_SPEC> {
        SRAM_ACE2_SIZE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace2_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace2_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_ACE2_SIZE_SPEC;
impl crate::RegisterSpec for SRAM_ACE2_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ace2_size::R`](R) reader structure"]
impl crate::Readable for SRAM_ACE2_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ace2_size::W`](W) writer structure"]
impl crate::Writable for SRAM_ACE2_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_ACE2_SIZE to value 0x1000"]
impl crate::Resettable for SRAM_ACE2_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
