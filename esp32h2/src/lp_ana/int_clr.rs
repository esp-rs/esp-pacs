///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `VDDBAT_CHARGE_UPVOLTAGE` writer - need_des
pub type VDDBAT_CHARGE_UPVOLTAGE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `VDDBAT_CHARGE_UNDERVOLTAGE` writer - need_des
pub type VDDBAT_CHARGE_UNDERVOLTAGE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `VDDBAT_UPVOLTAGE` writer - need_des
pub type VDDBAT_UPVOLTAGE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `VDDBAT_UNDERVOLTAGE` writer - need_des
pub type VDDBAT_UNDERVOLTAGE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `BOD_MODE0` writer - need_des
pub type BOD_MODE0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 27 - need_des
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_upvoltage(&mut self) -> VDDBAT_CHARGE_UPVOLTAGE_W<INT_CLR_SPEC> {
        VDDBAT_CHARGE_UPVOLTAGE_W::new(self, 27)
    }
    ///Bit 28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_undervoltage(&mut self) -> VDDBAT_CHARGE_UNDERVOLTAGE_W<INT_CLR_SPEC> {
        VDDBAT_CHARGE_UNDERVOLTAGE_W::new(self, 28)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn vddbat_upvoltage(&mut self) -> VDDBAT_UPVOLTAGE_W<INT_CLR_SPEC> {
        VDDBAT_UPVOLTAGE_W::new(self, 29)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn vddbat_undervoltage(&mut self) -> VDDBAT_UNDERVOLTAGE_W<INT_CLR_SPEC> {
        VDDBAT_UNDERVOLTAGE_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn bod_mode0(&mut self) -> BOD_MODE0_W<INT_CLR_SPEC> {
        BOD_MODE0_W::new(self, 31)
    }
}
/**need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf800_0000;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
