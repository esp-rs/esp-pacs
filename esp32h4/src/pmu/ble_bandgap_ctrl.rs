#[doc = "Register `BLE_BANDGAP_CTRL` reader"]
pub type R = crate::R<BLE_BANDGAP_CTRL_SPEC>;
#[doc = "Register `BLE_BANDGAP_CTRL` writer"]
pub type W = crate::W<BLE_BANDGAP_CTRL_SPEC>;
#[doc = "Field `EXT_OCODE` reader - need_des"]
pub type EXT_OCODE_R = crate::FieldReader;
#[doc = "Field `EXT_OCODE` writer - need_des"]
pub type EXT_OCODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXT_FORCE_OCODE` reader - need_des"]
pub type EXT_FORCE_OCODE_R = crate::BitReader;
#[doc = "Field `EXT_FORCE_OCODE` writer - need_des"]
pub type EXT_FORCE_OCODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    pub fn ext_ocode(&self) -> EXT_OCODE_R {
        EXT_OCODE_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn ext_force_ocode(&self) -> EXT_FORCE_OCODE_R {
        EXT_FORCE_OCODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLE_BANDGAP_CTRL")
            .field("ext_ocode", &self.ext_ocode())
            .field("ext_force_ocode", &self.ext_force_ocode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 23:30 - need_des"]
    #[inline(always)]
    pub fn ext_ocode(&mut self) -> EXT_OCODE_W<'_, BLE_BANDGAP_CTRL_SPEC> {
        EXT_OCODE_W::new(self, 23)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn ext_force_ocode(&mut self) -> EXT_FORCE_OCODE_W<'_, BLE_BANDGAP_CTRL_SPEC> {
        EXT_FORCE_OCODE_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ble_bandgap_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ble_bandgap_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLE_BANDGAP_CTRL_SPEC;
impl crate::RegisterSpec for BLE_BANDGAP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ble_bandgap_ctrl::R`](R) reader structure"]
impl crate::Readable for BLE_BANDGAP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ble_bandgap_ctrl::W`](W) writer structure"]
impl crate::Writable for BLE_BANDGAP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLE_BANDGAP_CTRL to value 0x3c00_0000"]
impl crate::Resettable for BLE_BANDGAP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x3c00_0000;
}
