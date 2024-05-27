///Register `REG_MAP1` reader
pub type R = crate::R<REG_MAP1_SPEC>;
///Register `REG_MAP1` writer
pub type W = crate::W<REG_MAP1_SPEC>;
///Field `MAP1` reader - x
pub type MAP1_R = crate::FieldReader<u32>;
///Field `MAP1` writer - x
pub type MAP1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - x
    #[inline(always)]
    pub fn map1(&self) -> MAP1_R {
        MAP1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_MAP1")
            .field("map1", &self.map1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - x
    #[inline(always)]
    #[must_use]
    pub fn map1(&mut self) -> MAP1_W<REG_MAP1_SPEC> {
        MAP1_W::new(self, 0)
    }
}
/**x

You can [`read`](crate::generic::Reg::read) this register and get [`reg_map1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_map1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct REG_MAP1_SPEC;
impl crate::RegisterSpec for REG_MAP1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`reg_map1::R`](R) reader structure
impl crate::Readable for REG_MAP1_SPEC {}
///`write(|w| ..)` method takes [`reg_map1::W`](W) writer structure
impl crate::Writable for REG_MAP1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REG_MAP1 to value 0
impl crate::Resettable for REG_MAP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
