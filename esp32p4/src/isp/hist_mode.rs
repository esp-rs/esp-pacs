///Register `HIST_MODE` reader
pub type R = crate::R<HIST_MODE_SPEC>;
///Register `HIST_MODE` writer
pub type W = crate::W<HIST_MODE_SPEC>;
///Field `HIST_MODE` reader - this field configures statistic mode. 0: RAW_B, 1: RAW_GB, 2: RAW_GR 3: RAW_R, 4: RGB, 5:YUV_Y, 6:YUV_U, 7:YUV_V
pub type HIST_MODE_R = crate::FieldReader;
///Field `HIST_MODE` writer - this field configures statistic mode. 0: RAW_B, 1: RAW_GB, 2: RAW_GR 3: RAW_R, 4: RGB, 5:YUV_Y, 6:YUV_U, 7:YUV_V
pub type HIST_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - this field configures statistic mode. 0: RAW_B, 1: RAW_GB, 2: RAW_GR 3: RAW_R, 4: RGB, 5:YUV_Y, 6:YUV_U, 7:YUV_V
    #[inline(always)]
    pub fn hist_mode(&self) -> HIST_MODE_R {
        HIST_MODE_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_MODE")
            .field("hist_mode", &self.hist_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - this field configures statistic mode. 0: RAW_B, 1: RAW_GB, 2: RAW_GR 3: RAW_R, 4: RGB, 5:YUV_Y, 6:YUV_U, 7:YUV_V
    #[inline(always)]
    #[must_use]
    pub fn hist_mode(&mut self) -> HIST_MODE_W<HIST_MODE_SPEC> {
        HIST_MODE_W::new(self, 0)
    }
}
/**histogram mode control register

You can [`read`](crate::generic::Reg::read) this register and get [`hist_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HIST_MODE_SPEC;
impl crate::RegisterSpec for HIST_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hist_mode::R`](R) reader structure
impl crate::Readable for HIST_MODE_SPEC {}
///`write(|w| ..)` method takes [`hist_mode::W`](W) writer structure
impl crate::Writable for HIST_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HIST_MODE to value 0x04
impl crate::Resettable for HIST_MODE_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
