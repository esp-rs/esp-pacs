#[doc = "Register `FLASH_ACE3_ADDR` reader"]
pub type R = crate::R<FLASH_ACE3_ADDR_SPEC>;
#[doc = "Register `FLASH_ACE3_ADDR` writer"]
pub type W = crate::W<FLASH_ACE3_ADDR_SPEC>;
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
        f.debug_struct("FLASH_ACE3_ADDR")
            .field("s", &self.s())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<FLASH_ACE3_ADDR_SPEC> {
        S_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_ace3_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_ace3_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_ACE3_ADDR_SPEC;
impl crate::RegisterSpec for FLASH_ACE3_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_ace3_addr::R`](R) reader structure"]
impl crate::Readable for FLASH_ACE3_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_ace3_addr::W`](W) writer structure"]
impl crate::Writable for FLASH_ACE3_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_ACE3_ADDR to value 0x3000_0000"]
impl crate::Resettable for FLASH_ACE3_ADDR_SPEC {
    const RESET_VALUE: u32 = 0x3000_0000;
}
