///Register `FLASH_ACE1_SIZE` reader
pub type R = crate::R<FLASH_ACE1_SIZE_SPEC>;
///Register `FLASH_ACE1_SIZE` writer
pub type W = crate::W<FLASH_ACE1_SIZE_SPEC>;
///Field `FLASH_ACE1_SIZE` reader - ******* Description ***********
pub type FLASH_ACE1_SIZE_R = crate::FieldReader<u16>;
///Field `FLASH_ACE1_SIZE` writer - ******* Description ***********
pub type FLASH_ACE1_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - ******* Description ***********
    #[inline(always)]
    pub fn flash_ace1_size(&self) -> FLASH_ACE1_SIZE_R {
        FLASH_ACE1_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_ACE1_SIZE")
            .field("flash_ace1_size", &self.flash_ace1_size())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - ******* Description ***********
    #[inline(always)]
    #[must_use]
    pub fn flash_ace1_size(&mut self) -> FLASH_ACE1_SIZE_W<FLASH_ACE1_SIZE_SPEC> {
        FLASH_ACE1_SIZE_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flash_ace1_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace1_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_ACE1_SIZE_SPEC;
impl crate::RegisterSpec for FLASH_ACE1_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`flash_ace1_size::R`](R) reader structure
impl crate::Readable for FLASH_ACE1_SIZE_SPEC {}
///`write(|w| ..)` method takes [`flash_ace1_size::W`](W) writer structure
impl crate::Writable for FLASH_ACE1_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_ACE1_SIZE to value 0x1000
impl crate::Resettable for FLASH_ACE1_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
