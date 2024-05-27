///Register `DATA` reader
pub type R = crate::R<DATA_SPEC>;
///Register `DATA` writer
pub type W = crate::W<DATA_SPEC>;
///Field `ICM_REG_DATA` reader - NA
pub type ICM_REG_DATA_R = crate::FieldReader<u32>;
///Field `ICM_REG_DATA` writer - NA
pub type ICM_REG_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - NA
    #[inline(always)]
    pub fn icm_reg_data(&self) -> ICM_REG_DATA_R {
        ICM_REG_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA")
            .field("icm_reg_data", &self.icm_reg_data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - NA
    #[inline(always)]
    #[must_use]
    pub fn icm_reg_data(&mut self) -> ICM_REG_DATA_W<DATA_SPEC> {
        ICM_REG_DATA_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`data::R`](R) reader structure
impl crate::Readable for DATA_SPEC {}
///`write(|w| ..)` method takes [`data::W`](W) writer structure
impl crate::Writable for DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA to value 0
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
