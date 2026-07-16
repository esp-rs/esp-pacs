#[doc = "Register `LPPERI_CTRL` reader"]
pub type R = crate::R<LPPERI_CTRL_SPEC>;
#[doc = "Register `LPPERI_CTRL` writer"]
pub type W = crate::W<LPPERI_CTRL_SPEC>;
#[doc = "Field `LPPERI_SW_FORCE_ON` reader - 1: software wake lpperi"]
pub type LPPERI_SW_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LPPERI_SW_FORCE_ON` writer - 1: software wake lpperi"]
pub type LPPERI_SW_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPPERI_SW_REQ` writer - software req pulse for following mode register"]
pub type LPPERI_SW_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1: software wake lpperi"]
    #[inline(always)]
    pub fn lpperi_sw_force_on(&self) -> LPPERI_SW_FORCE_ON_R {
        LPPERI_SW_FORCE_ON_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPPERI_CTRL")
            .field("lpperi_sw_force_on", &self.lpperi_sw_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: software wake lpperi"]
    #[inline(always)]
    pub fn lpperi_sw_force_on(&mut self) -> LPPERI_SW_FORCE_ON_W<'_, LPPERI_CTRL_SPEC> {
        LPPERI_SW_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - software req pulse for following mode register"]
    #[inline(always)]
    pub fn lpperi_sw_req(&mut self) -> LPPERI_SW_REQ_W<'_, LPPERI_CTRL_SPEC> {
        LPPERI_SW_REQ_W::new(self, 1)
    }
}
#[doc = "ctrl register for lpperi power control\n\nYou can [`read`](crate::Reg::read) this register and get [`lpperi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpperi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPPERI_CTRL_SPEC;
impl crate::RegisterSpec for LPPERI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpperi_ctrl::R`](R) reader structure"]
impl crate::Readable for LPPERI_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpperi_ctrl::W`](W) writer structure"]
impl crate::Writable for LPPERI_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPPERI_CTRL to value 0x01"]
impl crate::Resettable for LPPERI_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
