///Register `APP_VECBASE_SET` reader
pub type R = crate::R<APP_VECBASE_SET_SPEC>;
///Register `APP_VECBASE_SET` writer
pub type W = crate::W<APP_VECBASE_SET_SPEC>;
///Field `APP_OUT_VECBASE` reader -
pub type APP_OUT_VECBASE_R = crate::FieldReader<u32>;
///Field `APP_OUT_VECBASE` writer -
pub type APP_OUT_VECBASE_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:21
    #[inline(always)]
    pub fn app_out_vecbase(&self) -> APP_OUT_VECBASE_R {
        APP_OUT_VECBASE_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_VECBASE_SET")
            .field("app_out_vecbase", &self.app_out_vecbase())
            .finish()
    }
}
impl W {
    ///Bits 0:21
    #[inline(always)]
    #[must_use]
    pub fn app_out_vecbase(&mut self) -> APP_OUT_VECBASE_W<APP_VECBASE_SET_SPEC> {
        APP_OUT_VECBASE_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`app_vecbase_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_vecbase_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct APP_VECBASE_SET_SPEC;
impl crate::RegisterSpec for APP_VECBASE_SET_SPEC {
    type Ux = u32;
}
///`read()` method returns [`app_vecbase_set::R`](R) reader structure
impl crate::Readable for APP_VECBASE_SET_SPEC {}
///`write(|w| ..)` method takes [`app_vecbase_set::W`](W) writer structure
impl crate::Writable for APP_VECBASE_SET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets APP_VECBASE_SET to value 0
impl crate::Resettable for APP_VECBASE_SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
