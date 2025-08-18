#[doc = "Register `EXT1` reader"]
pub type R = crate::R<EXT1_SPEC>;
#[doc = "Register `EXT1` writer"]
pub type W = crate::W<EXT1_SPEC>;
#[doc = "Field `T_ERASE_TIME` reader - erase flash delay time by system clock."]
pub type T_ERASE_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `T_ERASE_TIME` writer - erase flash delay time by system clock."]
pub type T_ERASE_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `T_ERASE_SHIFT` reader - erase flash delay time shift."]
pub type T_ERASE_SHIFT_R = crate::FieldReader;
#[doc = "Field `T_ERASE_SHIFT` writer - erase flash delay time shift."]
pub type T_ERASE_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_ERASE_ENA` reader - erase flash delay enable."]
pub type T_ERASE_ENA_R = crate::BitReader;
#[doc = "Field `T_ERASE_ENA` writer - erase flash delay enable."]
pub type T_ERASE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - erase flash delay time by system clock."]
    #[inline(always)]
    pub fn t_erase_time(&self) -> T_ERASE_TIME_R {
        T_ERASE_TIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - erase flash delay time shift."]
    #[inline(always)]
    pub fn t_erase_shift(&self) -> T_ERASE_SHIFT_R {
        T_ERASE_SHIFT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - erase flash delay enable."]
    #[inline(always)]
    pub fn t_erase_ena(&self) -> T_ERASE_ENA_R {
        T_ERASE_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT1")
            .field("t_erase_time", &self.t_erase_time())
            .field("t_erase_shift", &self.t_erase_shift())
            .field("t_erase_ena", &self.t_erase_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - erase flash delay time by system clock."]
    #[inline(always)]
    pub fn t_erase_time(&mut self) -> T_ERASE_TIME_W<'_, EXT1_SPEC> {
        T_ERASE_TIME_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - erase flash delay time shift."]
    #[inline(always)]
    pub fn t_erase_shift(&mut self) -> T_ERASE_SHIFT_W<'_, EXT1_SPEC> {
        T_ERASE_SHIFT_W::new(self, 16)
    }
    #[doc = "Bit 31 - erase flash delay enable."]
    #[inline(always)]
    pub fn t_erase_ena(&mut self) -> T_ERASE_ENA_W<'_, EXT1_SPEC> {
        T_ERASE_ENA_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ext1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT1_SPEC;
impl crate::RegisterSpec for EXT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext1::R`](R) reader structure"]
impl crate::Readable for EXT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext1::W`](W) writer structure"]
impl crate::Writable for EXT1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT1 to value 0x800f_0000"]
impl crate::Resettable for EXT1_SPEC {
    const RESET_VALUE: u32 = 0x800f_0000;
}
