///Register `IFS_COUNTER` reader
pub type R = crate::R<IFS_COUNTER_SPEC>;
///Register `IFS_COUNTER` writer
pub type W = crate::W<IFS_COUNTER_SPEC>;
///Field `IFS_COUNTER` reader -
pub type IFS_COUNTER_R = crate::FieldReader<u16>;
///Field `IFS_COUNTER` writer -
pub type IFS_COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `EN` reader -
pub type EN_R = crate::BitReader;
///Field `EN` writer -
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9
    #[inline(always)]
    pub fn ifs_counter(&self) -> IFS_COUNTER_R {
        IFS_COUNTER_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 16
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IFS_COUNTER")
            .field("ifs_counter", &self.ifs_counter())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bits 0:9
    #[inline(always)]
    #[must_use]
    pub fn ifs_counter(&mut self) -> IFS_COUNTER_W<IFS_COUNTER_SPEC> {
        IFS_COUNTER_W::new(self, 0)
    }
    ///Bit 16
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<IFS_COUNTER_SPEC> {
        EN_W::new(self, 16)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ifs_counter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs_counter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IFS_COUNTER_SPEC;
impl crate::RegisterSpec for IFS_COUNTER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ifs_counter::R`](R) reader structure
impl crate::Readable for IFS_COUNTER_SPEC {}
///`write(|w| ..)` method takes [`ifs_counter::W`](W) writer structure
impl crate::Writable for IFS_COUNTER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFS_COUNTER to value 0
impl crate::Resettable for IFS_COUNTER_SPEC {
    const RESET_VALUE: u32 = 0;
}
