#[doc = "Register `BLE_WAKEUP_SRC` reader"]
pub type R = crate::R<BLE_WAKEUP_SRC_SPEC>;
#[doc = "Register `BLE_WAKEUP_SRC` writer"]
pub type W = crate::W<BLE_WAKEUP_SRC_SPEC>;
#[doc = "Field `BLE_WAKEUP_SOURCE_EN` reader - wakeup enable signal for ble"]
pub type BLE_WAKEUP_SOURCE_EN_R = crate::FieldReader<u32>;
#[doc = "Field `BLE_WAKEUP_SOURCE_EN` writer - wakeup enable signal for ble"]
pub type BLE_WAKEUP_SOURCE_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - wakeup enable signal for ble"]
    #[inline(always)]
    pub fn ble_wakeup_source_en(&self) -> BLE_WAKEUP_SOURCE_EN_R {
        BLE_WAKEUP_SOURCE_EN_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLE_WAKEUP_SRC")
            .field("ble_wakeup_source_en", &self.ble_wakeup_source_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - wakeup enable signal for ble"]
    #[inline(always)]
    pub fn ble_wakeup_source_en(&mut self) -> BLE_WAKEUP_SOURCE_EN_W<'_, BLE_WAKEUP_SRC_SPEC> {
        BLE_WAKEUP_SOURCE_EN_W::new(self, 0)
    }
}
#[doc = "wakeup source register for ble\n\nYou can [`read`](crate::Reg::read) this register and get [`ble_wakeup_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_wakeup_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLE_WAKEUP_SRC_SPEC;
impl crate::RegisterSpec for BLE_WAKEUP_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ble_wakeup_src::R`](R) reader structure"]
impl crate::Readable for BLE_WAKEUP_SRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ble_wakeup_src::W`](W) writer structure"]
impl crate::Writable for BLE_WAKEUP_SRC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLE_WAKEUP_SRC to value 0"]
impl crate::Resettable for BLE_WAKEUP_SRC_SPEC {}
