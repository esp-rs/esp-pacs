#[doc = "Register `CALI2` reader"]
pub type R = crate::R<CALI2_SPEC>;
#[doc = "Register `CALI2` writer"]
pub type W = crate::W<CALI2_SPEC>;
#[doc = "Field `LP_CALI_DIV_WAIT_PWR_GOOD` reader - "]
pub type LP_CALI_DIV_WAIT_PWR_GOOD_R = crate::FieldReader<u16>;
#[doc = "Field `LP_CALI_DIV_WAIT_PWR_GOOD` writer - "]
pub type LP_CALI_DIV_WAIT_PWR_GOOD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `LP_CALI_DIV_SLP_VAL` reader - "]
pub type LP_CALI_DIV_SLP_VAL_R = crate::FieldReader<u16>;
#[doc = "Field `LP_CALI_DIV_SLP_VAL` writer - "]
pub type LP_CALI_DIV_SLP_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LP_CALI_DIV_TIMER_EN` reader - "]
pub type LP_CALI_DIV_TIMER_EN_R = crate::BitReader;
#[doc = "Field `LP_CALI_DIV_TIMER_EN` writer - "]
pub type LP_CALI_DIV_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn lp_cali_div_wait_pwr_good(&self) -> LP_CALI_DIV_WAIT_PWR_GOOD_R {
        LP_CALI_DIV_WAIT_PWR_GOOD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 15:30"]
    #[inline(always)]
    pub fn lp_cali_div_slp_val(&self) -> LP_CALI_DIV_SLP_VAL_R {
        LP_CALI_DIV_SLP_VAL_R::new(((self.bits >> 15) & 0xffff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lp_cali_div_timer_en(&self) -> LP_CALI_DIV_TIMER_EN_R {
        LP_CALI_DIV_TIMER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALI2")
            .field(
                "lp_cali_div_wait_pwr_good",
                &self.lp_cali_div_wait_pwr_good(),
            )
            .field("lp_cali_div_slp_val", &self.lp_cali_div_slp_val())
            .field("lp_cali_div_timer_en", &self.lp_cali_div_timer_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn lp_cali_div_wait_pwr_good(&mut self) -> LP_CALI_DIV_WAIT_PWR_GOOD_W<'_, CALI2_SPEC> {
        LP_CALI_DIV_WAIT_PWR_GOOD_W::new(self, 0)
    }
    #[doc = "Bits 15:30"]
    #[inline(always)]
    pub fn lp_cali_div_slp_val(&mut self) -> LP_CALI_DIV_SLP_VAL_W<'_, CALI2_SPEC> {
        LP_CALI_DIV_SLP_VAL_W::new(self, 15)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn lp_cali_div_timer_en(&mut self) -> LP_CALI_DIV_TIMER_EN_W<'_, CALI2_SPEC> {
        LP_CALI_DIV_TIMER_EN_W::new(self, 31)
    }
}
#[doc = "LP clock calibration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cali2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cali2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALI2_SPEC;
impl crate::RegisterSpec for CALI2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cali2::R`](R) reader structure"]
impl crate::Readable for CALI2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cali2::W`](W) writer structure"]
impl crate::Writable for CALI2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALI2 to value 0x0080_00ff"]
impl crate::Resettable for CALI2_SPEC {
    const RESET_VALUE: u32 = 0x0080_00ff;
}
