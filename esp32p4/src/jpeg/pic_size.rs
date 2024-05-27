///Register `PIC_SIZE` reader
pub type R = crate::R<PIC_SIZE_SPEC>;
///Register `PIC_SIZE` writer
pub type W = crate::W<PIC_SIZE_SPEC>;
///Field `VA` reader - configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
pub type VA_R = crate::FieldReader<u16>;
///Field `VA` writer - configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
pub type VA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `HA` reader - configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
pub type HA_R = crate::FieldReader<u16>;
///Field `HA` writer - configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
pub type HA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIC_SIZE")
            .field("va", &self.va())
            .field("ha", &self.ha())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - configure picture's height. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    #[must_use]
    pub fn va(&mut self) -> VA_W<PIC_SIZE_SPEC> {
        VA_W::new(self, 0)
    }
    ///Bits 16:31 - configure picture's width. when encode, the max configurable bits is 14, when decode, the max configurable bits is 16
    #[inline(always)]
    #[must_use]
    pub fn ha(&mut self) -> HA_W<PIC_SIZE_SPEC> {
        HA_W::new(self, 16)
    }
}
/**Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`pic_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pic_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIC_SIZE_SPEC;
impl crate::RegisterSpec for PIC_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pic_size::R`](R) reader structure
impl crate::Readable for PIC_SIZE_SPEC {}
///`write(|w| ..)` method takes [`pic_size::W`](W) writer structure
impl crate::Writable for PIC_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PIC_SIZE to value 0x0280_01e0
impl crate::Resettable for PIC_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x0280_01e0;
}
