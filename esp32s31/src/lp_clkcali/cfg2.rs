#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2_SPEC>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2_SPEC>;
#[doc = "Field `DIV_WAIT_PWR_GOOD` reader - need_des"]
pub type DIV_WAIT_PWR_GOOD_R = crate::FieldReader<u16>;
#[doc = "Field `DIV_WAIT_PWR_GOOD` writer - need_des"]
pub type DIV_WAIT_PWR_GOOD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DIV_SLP_VAL` reader - need_des"]
pub type DIV_SLP_VAL_R = crate::FieldReader<u16>;
#[doc = "Field `DIV_SLP_VAL` writer - need_des"]
pub type DIV_SLP_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DIV_TIMER_EN` reader - need_des"]
pub type DIV_TIMER_EN_R = crate::BitReader;
#[doc = "Field `DIV_TIMER_EN` writer - need_des"]
pub type DIV_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - need_des"]
    #[inline(always)]
    pub fn div_wait_pwr_good(&self) -> DIV_WAIT_PWR_GOOD_R {
        DIV_WAIT_PWR_GOOD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 15:30 - need_des"]
    #[inline(always)]
    pub fn div_slp_val(&self) -> DIV_SLP_VAL_R {
        DIV_SLP_VAL_R::new(((self.bits >> 15) & 0xffff) as u16)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn div_timer_en(&self) -> DIV_TIMER_EN_R {
        DIV_TIMER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG2")
            .field("div_wait_pwr_good", &self.div_wait_pwr_good())
            .field("div_slp_val", &self.div_slp_val())
            .field("div_timer_en", &self.div_timer_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - need_des"]
    #[inline(always)]
    pub fn div_wait_pwr_good(&mut self) -> DIV_WAIT_PWR_GOOD_W<'_, CFG2_SPEC> {
        DIV_WAIT_PWR_GOOD_W::new(self, 0)
    }
    #[doc = "Bits 15:30 - need_des"]
    #[inline(always)]
    pub fn div_slp_val(&mut self) -> DIV_SLP_VAL_W<'_, CFG2_SPEC> {
        DIV_SLP_VAL_W::new(self, 15)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn div_timer_en(&mut self) -> DIV_TIMER_EN_W<'_, CFG2_SPEC> {
        DIV_TIMER_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG2 to value 0x80ff"]
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: u32 = 0x80ff;
}
