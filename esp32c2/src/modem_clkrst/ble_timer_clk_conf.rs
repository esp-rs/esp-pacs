///Register `BLE_TIMER_CLK_CONF` reader
pub type R = crate::R<BLE_TIMER_CLK_CONF_SPEC>;
///Register `BLE_TIMER_CLK_CONF` writer
pub type W = crate::W<BLE_TIMER_CLK_CONF_SPEC>;
///Field `BLETIMER_USE_XTAL` reader - .
pub type BLETIMER_USE_XTAL_R = crate::BitReader;
///Field `BLETIMER_USE_XTAL` writer - .
pub type BLETIMER_USE_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLETIMER_CLK_IS_ACTIVE` reader - .
pub type BLETIMER_CLK_IS_ACTIVE_R = crate::BitReader;
///Field `BLETIMER_CLK_IS_ACTIVE` writer - .
pub type BLETIMER_CLK_IS_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - .
    #[inline(always)]
    pub fn bletimer_use_xtal(&self) -> BLETIMER_USE_XTAL_R {
        BLETIMER_USE_XTAL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - .
    #[inline(always)]
    pub fn bletimer_clk_is_active(&self) -> BLETIMER_CLK_IS_ACTIVE_R {
        BLETIMER_CLK_IS_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLE_TIMER_CLK_CONF")
            .field("bletimer_use_xtal", &self.bletimer_use_xtal())
            .field("bletimer_clk_is_active", &self.bletimer_clk_is_active())
            .finish()
    }
}
impl W {
    ///Bit 0 - .
    #[inline(always)]
    #[must_use]
    pub fn bletimer_use_xtal(&mut self) -> BLETIMER_USE_XTAL_W<BLE_TIMER_CLK_CONF_SPEC> {
        BLETIMER_USE_XTAL_W::new(self, 0)
    }
    ///Bit 1 - .
    #[inline(always)]
    #[must_use]
    pub fn bletimer_clk_is_active(&mut self) -> BLETIMER_CLK_IS_ACTIVE_W<BLE_TIMER_CLK_CONF_SPEC> {
        BLETIMER_CLK_IS_ACTIVE_W::new(self, 1)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ble_timer_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ble_timer_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLE_TIMER_CLK_CONF_SPEC;
impl crate::RegisterSpec for BLE_TIMER_CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ble_timer_clk_conf::R`](R) reader structure
impl crate::Readable for BLE_TIMER_CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`ble_timer_clk_conf::W`](W) writer structure
impl crate::Writable for BLE_TIMER_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BLE_TIMER_CLK_CONF to value 0x03
impl crate::Resettable for BLE_TIMER_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
