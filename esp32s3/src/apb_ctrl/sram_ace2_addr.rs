///Register `SRAM_ACE2_ADDR` reader
pub type R = crate::R<SRAM_ACE2_ADDR_SPEC>;
///Register `SRAM_ACE2_ADDR` writer
pub type W = crate::W<SRAM_ACE2_ADDR_SPEC>;
///Field `S` reader - ******* Description ***********
pub type S_R = crate::FieldReader<u32>;
///Field `S` writer - ******* Description ***********
pub type S_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ******* Description ***********
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_ACE2_ADDR")
            .field("s", &self.s())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ******* Description ***********
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<SRAM_ACE2_ADDR_SPEC> {
        S_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ace2_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ace2_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_ACE2_ADDR_SPEC;
impl crate::RegisterSpec for SRAM_ACE2_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sram_ace2_addr::R`](R) reader structure
impl crate::Readable for SRAM_ACE2_ADDR_SPEC {}
///`write(|w| ..)` method takes [`sram_ace2_addr::W`](W) writer structure
impl crate::Writable for SRAM_ACE2_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SRAM_ACE2_ADDR to value 0x2000_0000
impl crate::Resettable for SRAM_ACE2_ADDR_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
