///Register `HP_SLEEP_BIAS` reader
pub type R = crate::R<HP_SLEEP_BIAS_SPEC>;
///Register `HP_SLEEP_BIAS` writer
pub type W = crate::W<HP_SLEEP_BIAS_SPEC>;
///Field `HP_SLEEP_XPD_TRX` reader - need_des
pub type HP_SLEEP_XPD_TRX_R = crate::BitReader;
///Field `HP_SLEEP_XPD_TRX` writer - need_des
pub type HP_SLEEP_XPD_TRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_SLEEP_XPD_BIAS` reader - need_des
pub type HP_SLEEP_XPD_BIAS_R = crate::BitReader;
///Field `HP_SLEEP_XPD_BIAS` writer - need_des
pub type HP_SLEEP_XPD_BIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HP_SLEEP_PD_CUR` reader - need_des
pub type HP_SLEEP_PD_CUR_R = crate::BitReader;
///Field `HP_SLEEP_PD_CUR` writer - need_des
pub type HP_SLEEP_PD_CUR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEEP` reader - need_des
pub type SLEEP_R = crate::BitReader;
///Field `SLEEP` writer - need_des
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 24 - need_des
    #[inline(always)]
    pub fn hp_sleep_xpd_trx(&self) -> HP_SLEEP_XPD_TRX_R {
        HP_SLEEP_XPD_TRX_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - need_des
    #[inline(always)]
    pub fn hp_sleep_xpd_bias(&self) -> HP_SLEEP_XPD_BIAS_R {
        HP_SLEEP_XPD_BIAS_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    pub fn hp_sleep_pd_cur(&self) -> HP_SLEEP_PD_CUR_R {
        HP_SLEEP_PD_CUR_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_BIAS")
            .field("hp_sleep_xpd_trx", &self.hp_sleep_xpd_trx())
            .field("hp_sleep_xpd_bias", &self.hp_sleep_xpd_bias())
            .field("hp_sleep_pd_cur", &self.hp_sleep_pd_cur())
            .field("sleep", &self.sleep())
            .finish()
    }
}
impl W {
    ///Bit 24 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_xpd_trx(&mut self) -> HP_SLEEP_XPD_TRX_W<HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_XPD_TRX_W::new(self, 24)
    }
    ///Bit 25 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_xpd_bias(&mut self) -> HP_SLEEP_XPD_BIAS_W<HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_XPD_BIAS_W::new(self, 25)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_pd_cur(&mut self) -> HP_SLEEP_PD_CUR_W<HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_PD_CUR_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<HP_SLEEP_BIAS_SPEC> {
        SLEEP_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_bias::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_bias::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HP_SLEEP_BIAS_SPEC;
impl crate::RegisterSpec for HP_SLEEP_BIAS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hp_sleep_bias::R`](R) reader structure
impl crate::Readable for HP_SLEEP_BIAS_SPEC {}
///`write(|w| ..)` method takes [`hp_sleep_bias::W`](W) writer structure
impl crate::Writable for HP_SLEEP_BIAS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HP_SLEEP_BIAS to value 0x0100_0000
impl crate::Resettable for HP_SLEEP_BIAS_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
