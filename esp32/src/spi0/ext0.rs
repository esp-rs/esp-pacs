#[doc = "Register `EXT0` reader"]
pub type R = crate::R<EXT0_SPEC>;
#[doc = "Register `EXT0` writer"]
pub type W = crate::W<EXT0_SPEC>;
#[doc = "Field `T_PP_TIME` reader - page program delay time by system clock."]
pub type T_PP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `T_PP_TIME` writer - page program delay time by system clock."]
pub type T_PP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `T_PP_SHIFT` reader - page program delay time shift ."]
pub type T_PP_SHIFT_R = crate::FieldReader;
#[doc = "Field `T_PP_SHIFT` writer - page program delay time shift ."]
pub type T_PP_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_PP_ENA` reader - page program delay enable."]
pub type T_PP_ENA_R = crate::BitReader;
#[doc = "Field `T_PP_ENA` writer - page program delay enable."]
pub type T_PP_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - page program delay time by system clock."]
    #[inline(always)]
    pub fn t_pp_time(&self) -> T_PP_TIME_R {
        T_PP_TIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - page program delay time shift ."]
    #[inline(always)]
    pub fn t_pp_shift(&self) -> T_PP_SHIFT_R {
        T_PP_SHIFT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - page program delay enable."]
    #[inline(always)]
    pub fn t_pp_ena(&self) -> T_PP_ENA_R {
        T_PP_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT0")
            .field("t_pp_time", &self.t_pp_time())
            .field("t_pp_shift", &self.t_pp_shift())
            .field("t_pp_ena", &self.t_pp_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - page program delay time by system clock."]
    #[inline(always)]
    pub fn t_pp_time(&mut self) -> T_PP_TIME_W<'_, EXT0_SPEC> {
        T_PP_TIME_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - page program delay time shift ."]
    #[inline(always)]
    pub fn t_pp_shift(&mut self) -> T_PP_SHIFT_W<'_, EXT0_SPEC> {
        T_PP_SHIFT_W::new(self, 16)
    }
    #[doc = "Bit 31 - page program delay enable."]
    #[inline(always)]
    pub fn t_pp_ena(&mut self) -> T_PP_ENA_W<'_, EXT0_SPEC> {
        T_PP_ENA_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ext0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT0_SPEC;
impl crate::RegisterSpec for EXT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext0::R`](R) reader structure"]
impl crate::Readable for EXT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext0::W`](W) writer structure"]
impl crate::Writable for EXT0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT0 to value 0x800a_0050"]
impl crate::Resettable for EXT0_SPEC {
    const RESET_VALUE: u32 = 0x800a_0050;
}
