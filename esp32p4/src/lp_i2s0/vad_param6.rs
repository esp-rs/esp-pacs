///Register `VAD_PARAM6` reader
pub type R = crate::R<VAD_PARAM6_SPEC>;
///Register `VAD_PARAM6` writer
pub type W = crate::W<VAD_PARAM6_SPEC>;
///Field `PARAM_NOISE_STD_FS_THSL` reader - Feature_sum threshold to determine noise_std max value when vad_tag=1, equal to ((noise_std_max)>>11)^2*5
pub type PARAM_NOISE_STD_FS_THSL_R = crate::FieldReader<u16>;
///Field `PARAM_NOISE_STD_FS_THSL` writer - Feature_sum threshold to determine noise_std max value when vad_tag=1, equal to ((noise_std_max)>>11)^2*5
pub type PARAM_NOISE_STD_FS_THSL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PARAM_NOISE_STD_FS_THSH` reader - Feature_sum threshold to determine noise_std max value when vad_tag=0, equal to ((noise_std_max)>>11)^2*5
pub type PARAM_NOISE_STD_FS_THSH_R = crate::FieldReader<u16>;
///Field `PARAM_NOISE_STD_FS_THSH` writer - Feature_sum threshold to determine noise_std max value when vad_tag=0, equal to ((noise_std_max)>>11)^2*5
pub type PARAM_NOISE_STD_FS_THSH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Feature_sum threshold to determine noise_std max value when vad_tag=1, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    pub fn param_noise_std_fs_thsl(&self) -> PARAM_NOISE_STD_FS_THSL_R {
        PARAM_NOISE_STD_FS_THSL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Feature_sum threshold to determine noise_std max value when vad_tag=0, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    pub fn param_noise_std_fs_thsh(&self) -> PARAM_NOISE_STD_FS_THSH_R {
        PARAM_NOISE_STD_FS_THSH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_PARAM6")
            .field("param_noise_std_fs_thsl", &self.param_noise_std_fs_thsl())
            .field("param_noise_std_fs_thsh", &self.param_noise_std_fs_thsh())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Feature_sum threshold to determine noise_std max value when vad_tag=1, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    #[must_use]
    pub fn param_noise_std_fs_thsl(&mut self) -> PARAM_NOISE_STD_FS_THSL_W<VAD_PARAM6_SPEC> {
        PARAM_NOISE_STD_FS_THSL_W::new(self, 0)
    }
    ///Bits 16:31 - Feature_sum threshold to determine noise_std max value when vad_tag=0, equal to ((noise_std_max)>>11)^2*5
    #[inline(always)]
    #[must_use]
    pub fn param_noise_std_fs_thsh(&mut self) -> PARAM_NOISE_STD_FS_THSH_W<VAD_PARAM6_SPEC> {
        PARAM_NOISE_STD_FS_THSH_W::new(self, 16)
    }
}
/**I2S VAD Parameter register

You can [`read`](crate::generic::Reg::read) this register and get [`vad_param6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vad_param6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VAD_PARAM6_SPEC;
impl crate::RegisterSpec for VAD_PARAM6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`vad_param6::R`](R) reader structure
impl crate::Readable for VAD_PARAM6_SPEC {}
///`write(|w| ..)` method takes [`vad_param6::W`](W) writer structure
impl crate::Writable for VAD_PARAM6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VAD_PARAM6 to value 0xb400_7d00
impl crate::Resettable for VAD_PARAM6_SPEC {
    const RESET_VALUE: u32 = 0xb400_7d00;
}
