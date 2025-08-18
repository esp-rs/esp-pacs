#[doc = "Register `HP_MODEM_BIAS` reader"]
pub type R = crate::R<HP_MODEM_BIAS_SPEC>;
#[doc = "Register `HP_MODEM_BIAS` writer"]
pub type W = crate::W<HP_MODEM_BIAS_SPEC>;
#[doc = "Field `HP_MODEM_XPD_TRX` reader - need_des"]
pub type HP_MODEM_XPD_TRX_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_TRX` writer - need_des"]
pub type HP_MODEM_XPD_TRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_XPD_BIAS` reader - need_des"]
pub type HP_MODEM_XPD_BIAS_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_BIAS` writer - need_des"]
pub type HP_MODEM_XPD_BIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_PD_CUR` reader - need_des"]
pub type HP_MODEM_PD_CUR_R = crate::BitReader;
#[doc = "Field `HP_MODEM_PD_CUR` writer - need_des"]
pub type HP_MODEM_PD_CUR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - need_des"]
pub type SLEEP_R = crate::BitReader;
#[doc = "Field `SLEEP` writer - need_des"]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_trx(&self) -> HP_MODEM_XPD_TRX_R {
        HP_MODEM_XPD_TRX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_bias(&self) -> HP_MODEM_XPD_BIAS_R {
        HP_MODEM_XPD_BIAS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_modem_pd_cur(&self) -> HP_MODEM_PD_CUR_R {
        HP_MODEM_PD_CUR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_BIAS")
            .field("hp_modem_xpd_trx", &self.hp_modem_xpd_trx())
            .field("hp_modem_xpd_bias", &self.hp_modem_xpd_bias())
            .field("hp_modem_pd_cur", &self.hp_modem_pd_cur())
            .field("sleep", &self.sleep())
            .finish()
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_trx(&mut self) -> HP_MODEM_XPD_TRX_W<'_, HP_MODEM_BIAS_SPEC> {
        HP_MODEM_XPD_TRX_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_bias(&mut self) -> HP_MODEM_XPD_BIAS_W<'_, HP_MODEM_BIAS_SPEC> {
        HP_MODEM_XPD_BIAS_W::new(self, 25)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_modem_pd_cur(&mut self) -> HP_MODEM_PD_CUR_W<'_, HP_MODEM_BIAS_SPEC> {
        HP_MODEM_PD_CUR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<'_, HP_MODEM_BIAS_SPEC> {
        SLEEP_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_modem_bias::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_bias::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_BIAS_SPEC;
impl crate::RegisterSpec for HP_MODEM_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_modem_bias::R`](R) reader structure"]
impl crate::Readable for HP_MODEM_BIAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_modem_bias::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_BIAS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_BIAS to value 0x0100_0000"]
impl crate::Resettable for HP_MODEM_BIAS_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
