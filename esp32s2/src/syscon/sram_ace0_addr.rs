#[doc = "Register `SRAM_ACE0_ADDR` reader"]
pub type R = crate::R<SRAM_ACE0_ADDR_SPEC>;
#[doc = "Register `SRAM_ACE0_ADDR` writer"]
pub type W = crate::W<SRAM_ACE0_ADDR_SPEC>;
#[doc = "Field `S` reader - "]
pub type S_R = crate::FieldReader<u32>;
#[doc = "Field `S` writer - "]
pub type S_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_ACE0_ADDR")
            .field("s", &format_args!("{}", self.s().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_ACE0_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<SRAM_ACE0_ADDR_SPEC> {
        S_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace0_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace0_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_ACE0_ADDR_SPEC;
impl crate::RegisterSpec for SRAM_ACE0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ace0_addr::R`](R) reader structure"]
impl crate::Readable for SRAM_ACE0_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ace0_addr::W`](W) writer structure"]
impl crate::Writable for SRAM_ACE0_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_ACE0_ADDR to value 0"]
impl crate::Resettable for SRAM_ACE0_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
