///Register `SEC_KEY2` reader
pub type R = crate::R<SEC_KEY2_SPEC>;
///Register `SEC_KEY2` writer
pub type W = crate::W<SEC_KEY2_SPEC>;
///Field `SEC_KEY2` reader -
pub type SEC_KEY2_R = crate::FieldReader<u32>;
///Field `SEC_KEY2` writer -
pub type SEC_KEY2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn sec_key2(&self) -> SEC_KEY2_R {
        SEC_KEY2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_KEY2")
            .field("sec_key2", &self.sec_key2())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn sec_key2(&mut self) -> SEC_KEY2_W<SEC_KEY2_SPEC> {
        SEC_KEY2_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sec_key2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_key2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SEC_KEY2_SPEC;
impl crate::RegisterSpec for SEC_KEY2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sec_key2::R`](R) reader structure
impl crate::Readable for SEC_KEY2_SPEC {}
///`write(|w| ..)` method takes [`sec_key2::W`](W) writer structure
impl crate::Writable for SEC_KEY2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SEC_KEY2 to value 0
impl crate::Resettable for SEC_KEY2_SPEC {
    const RESET_VALUE: u32 = 0;
}
