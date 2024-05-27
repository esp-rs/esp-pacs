///Register `LOG_DATA_3` reader
pub type R = crate::R<LOG_DATA_3_SPEC>;
///Register `LOG_DATA_3` writer
pub type W = crate::W<LOG_DATA_3_SPEC>;
///Field `LOG_DATA_3` reader - check data3
pub type LOG_DATA_3_R = crate::FieldReader<u32>;
///Field `LOG_DATA_3` writer - check data3
pub type LOG_DATA_3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - check data3
    #[inline(always)]
    pub fn log_data_3(&self) -> LOG_DATA_3_R {
        LOG_DATA_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_DATA_3")
            .field("log_data_3", &self.log_data_3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - check data3
    #[inline(always)]
    #[must_use]
    pub fn log_data_3(&mut self) -> LOG_DATA_3_W<LOG_DATA_3_SPEC> {
        LOG_DATA_3_W::new(self, 0)
    }
}
/**log check data register

You can [`read`](crate::generic::Reg::read) this register and get [`log_data_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOG_DATA_3_SPEC;
impl crate::RegisterSpec for LOG_DATA_3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`log_data_3::R`](R) reader structure
impl crate::Readable for LOG_DATA_3_SPEC {}
///`write(|w| ..)` method takes [`log_data_3::W`](W) writer structure
impl crate::Writable for LOG_DATA_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LOG_DATA_3 to value 0
impl crate::Resettable for LOG_DATA_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
