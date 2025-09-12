#[doc = "Register `SRAM_ACE3_ADDR` reader"]
pub type R = crate::R<SRAM_ACE3_ADDR_SPEC>;
#[doc = "Register `SRAM_ACE3_ADDR` writer"]
pub type W = crate::W<SRAM_ACE3_ADDR_SPEC>;
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
        f.debug_struct("SRAM_ACE3_ADDR")
            .field("s", &self.s())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn s(&mut self) -> S_W<'_, SRAM_ACE3_ADDR_SPEC> {
        S_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_ace3_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_ace3_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_ACE3_ADDR_SPEC;
impl crate::RegisterSpec for SRAM_ACE3_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ace3_addr::R`](R) reader structure"]
impl crate::Readable for SRAM_ACE3_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ace3_addr::W`](W) writer structure"]
impl crate::Writable for SRAM_ACE3_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_ACE3_ADDR to value 0x3000_0000"]
impl crate::Resettable for SRAM_ACE3_ADDR_SPEC {
    const RESET_VALUE: u32 = 0x3000_0000;
}
