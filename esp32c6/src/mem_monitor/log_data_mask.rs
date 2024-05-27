///Register `LOG_DATA_MASK` reader
pub type R = crate::R<LOG_DATA_MASK_SPEC>;
///Register `LOG_DATA_MASK` writer
pub type W = crate::W<LOG_DATA_MASK_SPEC>;
///Field `LOG_DATA_MASK` reader - byte mask enable, BIT0 mask the first byte of MEM_MONITOR_LOG_CHECK_DATA, and BIT1 mask second byte, and so on.
pub type LOG_DATA_MASK_R = crate::FieldReader;
///Field `LOG_DATA_MASK` writer - byte mask enable, BIT0 mask the first byte of MEM_MONITOR_LOG_CHECK_DATA, and BIT1 mask second byte, and so on.
pub type LOG_DATA_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - byte mask enable, BIT0 mask the first byte of MEM_MONITOR_LOG_CHECK_DATA, and BIT1 mask second byte, and so on.
    #[inline(always)]
    pub fn log_data_mask(&self) -> LOG_DATA_MASK_R {
        LOG_DATA_MASK_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_DATA_MASK")
            .field("log_data_mask", &self.log_data_mask())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - byte mask enable, BIT0 mask the first byte of MEM_MONITOR_LOG_CHECK_DATA, and BIT1 mask second byte, and so on.
    #[inline(always)]
    #[must_use]
    pub fn log_data_mask(&mut self) -> LOG_DATA_MASK_W<LOG_DATA_MASK_SPEC> {
        LOG_DATA_MASK_W::new(self, 0)
    }
}
/**check data mask register

You can [`read`](crate::generic::Reg::read) this register and get [`log_data_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOG_DATA_MASK_SPEC;
impl crate::RegisterSpec for LOG_DATA_MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`log_data_mask::R`](R) reader structure
impl crate::Readable for LOG_DATA_MASK_SPEC {}
///`write(|w| ..)` method takes [`log_data_mask::W`](W) writer structure
impl crate::Writable for LOG_DATA_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LOG_DATA_MASK to value 0
impl crate::Resettable for LOG_DATA_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
