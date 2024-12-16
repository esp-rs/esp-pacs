#[doc = "Register `SDA_SAMPLE_TIME` reader"]
pub type R = crate::R<SDA_SAMPLE_TIME_SPEC>;
#[doc = "Register `SDA_SAMPLE_TIME` writer"]
pub type W = crate::W<SDA_SAMPLE_TIME_SPEC>;
#[doc = "Field `REG_SDA_OD_SAMPLE_TIME` reader - It is used to adjust sda sample point when scl high under open drain speed"]
pub type REG_SDA_OD_SAMPLE_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `REG_SDA_OD_SAMPLE_TIME` writer - It is used to adjust sda sample point when scl high under open drain speed"]
pub type REG_SDA_OD_SAMPLE_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `REG_SDA_PP_SAMPLE_TIME` reader - It is used to adjust sda sample point when scl high under push pull speed"]
pub type REG_SDA_PP_SAMPLE_TIME_R = crate::FieldReader;
#[doc = "Field `REG_SDA_PP_SAMPLE_TIME` writer - It is used to adjust sda sample point when scl high under push pull speed"]
pub type REG_SDA_PP_SAMPLE_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:8 - It is used to adjust sda sample point when scl high under open drain speed"]
    #[inline(always)]
    pub fn reg_sda_od_sample_time(&self) -> REG_SDA_OD_SAMPLE_TIME_R {
        REG_SDA_OD_SAMPLE_TIME_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:13 - It is used to adjust sda sample point when scl high under push pull speed"]
    #[inline(always)]
    pub fn reg_sda_pp_sample_time(&self) -> REG_SDA_PP_SAMPLE_TIME_R {
        REG_SDA_PP_SAMPLE_TIME_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_SAMPLE_TIME")
            .field("reg_sda_od_sample_time", &self.reg_sda_od_sample_time())
            .field("reg_sda_pp_sample_time", &self.reg_sda_pp_sample_time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - It is used to adjust sda sample point when scl high under open drain speed"]
    #[inline(always)]
    pub fn reg_sda_od_sample_time(&mut self) -> REG_SDA_OD_SAMPLE_TIME_W<SDA_SAMPLE_TIME_SPEC> {
        REG_SDA_OD_SAMPLE_TIME_W::new(self, 0)
    }
    #[doc = "Bits 9:13 - It is used to adjust sda sample point when scl high under push pull speed"]
    #[inline(always)]
    pub fn reg_sda_pp_sample_time(&mut self) -> REG_SDA_PP_SAMPLE_TIME_W<SDA_SAMPLE_TIME_SPEC> {
        REG_SDA_PP_SAMPLE_TIME_W::new(self, 9)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sda_sample_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sda_sample_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDA_SAMPLE_TIME_SPEC;
impl crate::RegisterSpec for SDA_SAMPLE_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_sample_time::R`](R) reader structure"]
impl crate::Readable for SDA_SAMPLE_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sda_sample_time::W`](W) writer structure"]
impl crate::Writable for SDA_SAMPLE_TIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDA_SAMPLE_TIME to value 0"]
impl crate::Resettable for SDA_SAMPLE_TIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
