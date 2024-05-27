///Register `BLE_TIMER_INTR_MAP` reader
pub type R = crate::R<BLE_TIMER_INTR_MAP_SPEC>;
///Register `BLE_TIMER_INTR_MAP` writer
pub type W = crate::W<BLE_TIMER_INTR_MAP_SPEC>;
///Field `BLE_TIMER_INTR_MAP` reader - Need add description
pub type BLE_TIMER_INTR_MAP_R = crate::FieldReader;
///Field `BLE_TIMER_INTR_MAP` writer - Need add description
pub type BLE_TIMER_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Need add description
    #[inline(always)]
    pub fn ble_timer_intr_map(&self) -> BLE_TIMER_INTR_MAP_R {
        BLE_TIMER_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLE_TIMER_INTR_MAP")
            .field("ble_timer_intr_map", &self.ble_timer_intr_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn ble_timer_intr_map(&mut self) -> BLE_TIMER_INTR_MAP_W<BLE_TIMER_INTR_MAP_SPEC> {
        BLE_TIMER_INTR_MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`ble_timer_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ble_timer_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLE_TIMER_INTR_MAP_SPEC;
impl crate::RegisterSpec for BLE_TIMER_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ble_timer_intr_map::R`](R) reader structure
impl crate::Readable for BLE_TIMER_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`ble_timer_intr_map::W`](W) writer structure
impl crate::Writable for BLE_TIMER_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BLE_TIMER_INTR_MAP to value 0
impl crate::Resettable for BLE_TIMER_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
