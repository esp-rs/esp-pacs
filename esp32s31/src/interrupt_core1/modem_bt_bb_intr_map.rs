#[doc = "Register `MODEM_BT_BB_INTR_MAP` reader"]
pub type R = crate::R<MODEM_BT_BB_INTR_MAP_SPEC>;
#[doc = "Register `MODEM_BT_BB_INTR_MAP` writer"]
pub type W = crate::W<MODEM_BT_BB_INTR_MAP_SPEC>;
#[doc = "Field `MAP` reader - "]
pub type MAP_R = crate::FieldReader;
#[doc = "Field `MAP` writer - "]
pub type MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PASS_LEVEL` reader - "]
pub type PASS_LEVEL_R = crate::FieldReader;
#[doc = "Field `PASS_LEVEL` writer - "]
pub type PASS_LEVEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pass_level(&self) -> PASS_LEVEL_R {
        PASS_LEVEL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_BT_BB_INTR_MAP")
            .field("map", &self.map())
            .field("pass_level", &self.pass_level())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn map(&mut self) -> MAP_W<'_, MODEM_BT_BB_INTR_MAP_SPEC> {
        MAP_W::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pass_level(&mut self) -> PASS_LEVEL_W<'_, MODEM_BT_BB_INTR_MAP_SPEC> {
        PASS_LEVEL_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_bt_bb_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_bt_bb_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_BT_BB_INTR_MAP_SPEC;
impl crate::RegisterSpec for MODEM_BT_BB_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_bt_bb_intr_map::R`](R) reader structure"]
impl crate::Readable for MODEM_BT_BB_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_bt_bb_intr_map::W`](W) writer structure"]
impl crate::Writable for MODEM_BT_BB_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEM_BT_BB_INTR_MAP to value 0"]
impl crate::Resettable for MODEM_BT_BB_INTR_MAP_SPEC {}
