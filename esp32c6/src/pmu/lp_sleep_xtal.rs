///Register `LP_SLEEP_XTAL` reader
pub type R = crate::R<LP_SLEEP_XTAL_SPEC>;
///Register `LP_SLEEP_XTAL` writer
pub type W = crate::W<LP_SLEEP_XTAL_SPEC>;
///Field `LP_SLEEP_XPD_XTAL` reader - need_des
pub type LP_SLEEP_XPD_XTAL_R = crate::BitReader;
///Field `LP_SLEEP_XPD_XTAL` writer - need_des
pub type LP_SLEEP_XPD_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn lp_sleep_xpd_xtal(&self) -> LP_SLEEP_XPD_XTAL_R {
        LP_SLEEP_XPD_XTAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_SLEEP_XTAL")
            .field("lp_sleep_xpd_xtal", &self.lp_sleep_xpd_xtal())
            .finish()
    }
}
impl W {
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_xpd_xtal(&mut self) -> LP_SLEEP_XPD_XTAL_W<LP_SLEEP_XTAL_SPEC> {
        LP_SLEEP_XPD_XTAL_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_sleep_xtal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_xtal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_SLEEP_XTAL_SPEC;
impl crate::RegisterSpec for LP_SLEEP_XTAL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_sleep_xtal::R`](R) reader structure
impl crate::Readable for LP_SLEEP_XTAL_SPEC {}
///`write(|w| ..)` method takes [`lp_sleep_xtal::W`](W) writer structure
impl crate::Writable for LP_SLEEP_XTAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_SLEEP_XTAL to value 0x8000_0000
impl crate::Resettable for LP_SLEEP_XTAL_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
