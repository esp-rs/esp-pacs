///Register `FLASH_ACE0_ATTR` reader
pub type R = crate::R<FLASH_ACE0_ATTR_SPEC>;
///Register `FLASH_ACE0_ATTR` writer
pub type W = crate::W<FLASH_ACE0_ATTR_SPEC>;
///Field `FLASH_ACE0_ATTR` reader -
pub type FLASH_ACE0_ATTR_R = crate::FieldReader;
///Field `FLASH_ACE0_ATTR` writer -
pub type FLASH_ACE0_ATTR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2
    #[inline(always)]
    pub fn flash_ace0_attr(&self) -> FLASH_ACE0_ATTR_R {
        FLASH_ACE0_ATTR_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_ACE0_ATTR")
            .field("flash_ace0_attr", &self.flash_ace0_attr())
            .finish()
    }
}
impl W {
    ///Bits 0:2
    #[inline(always)]
    #[must_use]
    pub fn flash_ace0_attr(&mut self) -> FLASH_ACE0_ATTR_W<FLASH_ACE0_ATTR_SPEC> {
        FLASH_ACE0_ATTR_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`flash_ace0_attr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flash_ace0_attr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FLASH_ACE0_ATTR_SPEC;
impl crate::RegisterSpec for FLASH_ACE0_ATTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`flash_ace0_attr::R`](R) reader structure
impl crate::Readable for FLASH_ACE0_ATTR_SPEC {}
///`write(|w| ..)` method takes [`flash_ace0_attr::W`](W) writer structure
impl crate::Writable for FLASH_ACE0_ATTR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_ACE0_ATTR to value 0x07
impl crate::Resettable for FLASH_ACE0_ATTR_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
