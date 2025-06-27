#[doc = "Register `FLASH_ACE2_ATTR` reader"]
pub type R = crate::R<FLASH_ACE2_ATTR_SPEC>;
#[doc = "Register `FLASH_ACE2_ATTR` writer"]
pub type W = crate::W<FLASH_ACE2_ATTR_SPEC>;
#[doc = "Field `FLASH_ACE2_ATTR` reader - ******* Description ***********"]
pub type FLASH_ACE2_ATTR_R = crate::FieldReader<u16>;
#[doc = "Field `FLASH_ACE2_ATTR` writer - ******* Description ***********"]
pub type FLASH_ACE2_ATTR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - ******* Description ***********"]
    #[inline(always)]
    pub fn flash_ace2_attr(&self) -> FLASH_ACE2_ATTR_R {
        FLASH_ACE2_ATTR_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_ACE2_ATTR")
            .field("flash_ace2_attr", &self.flash_ace2_attr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - ******* Description ***********"]
    #[inline(always)]
    pub fn flash_ace2_attr(&mut self) -> FLASH_ACE2_ATTR_W<FLASH_ACE2_ATTR_SPEC> {
        FLASH_ACE2_ATTR_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_ace2_attr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_ace2_attr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_ACE2_ATTR_SPEC;
impl crate::RegisterSpec for FLASH_ACE2_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_ace2_attr::R`](R) reader structure"]
impl crate::Readable for FLASH_ACE2_ATTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_ace2_attr::W`](W) writer structure"]
impl crate::Writable for FLASH_ACE2_ATTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_ACE2_ATTR to value 0xff"]
impl crate::Resettable for FLASH_ACE2_ATTR_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
