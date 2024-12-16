#[doc = "Register `SAR_HALL_CTRL` reader"]
pub type R = crate::R<SAR_HALL_CTRL_SPEC>;
#[doc = "Register `SAR_HALL_CTRL` writer"]
pub type W = crate::W<SAR_HALL_CTRL_SPEC>;
#[doc = "Field `XPD_HALL` reader - Power on hall sensor and connect to VP and VN"]
pub type XPD_HALL_R = crate::BitReader;
#[doc = "Field `XPD_HALL` writer - Power on hall sensor and connect to VP and VN"]
pub type XPD_HALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_HALL_FORCE` reader - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub type XPD_HALL_FORCE_R = crate::BitReader;
#[doc = "Field `XPD_HALL_FORCE` writer - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
pub type XPD_HALL_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALL_PHASE` reader - Reverse phase of hall sensor"]
pub type HALL_PHASE_R = crate::BitReader;
#[doc = "Field `HALL_PHASE` writer - Reverse phase of hall sensor"]
pub type HALL_PHASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALL_PHASE_FORCE` reader - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub type HALL_PHASE_FORCE_R = crate::BitReader;
#[doc = "Field `HALL_PHASE_FORCE` writer - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
pub type HALL_PHASE_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    pub fn xpd_hall(&self) -> XPD_HALL_R {
        XPD_HALL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn xpd_hall_force(&self) -> XPD_HALL_FORCE_R {
        XPD_HALL_FORCE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    pub fn hall_phase(&self) -> HALL_PHASE_R {
        HALL_PHASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn hall_phase_force(&self) -> HALL_PHASE_FORCE_R {
        HALL_PHASE_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_HALL_CTRL")
            .field("xpd_hall", &self.xpd_hall())
            .field("xpd_hall_force", &self.xpd_hall_force())
            .field("hall_phase", &self.hall_phase())
            .field("hall_phase_force", &self.hall_phase_force())
            .finish()
    }
}
impl W {
    #[doc = "Bit 28 - Power on hall sensor and connect to VP and VN"]
    #[inline(always)]
    pub fn xpd_hall(&mut self) -> XPD_HALL_W<SAR_HALL_CTRL_SPEC> {
        XPD_HALL_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1: XPD HALL is controlled by SW. 0: XPD HALL is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn xpd_hall_force(&mut self) -> XPD_HALL_FORCE_W<SAR_HALL_CTRL_SPEC> {
        XPD_HALL_FORCE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Reverse phase of hall sensor"]
    #[inline(always)]
    pub fn hall_phase(&mut self) -> HALL_PHASE_W<SAR_HALL_CTRL_SPEC> {
        HALL_PHASE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: HALL PHASE is controlled by SW 0: HALL PHASE is controlled by FSM in ULP-coprocessor"]
    #[inline(always)]
    pub fn hall_phase_force(&mut self) -> HALL_PHASE_FORCE_W<SAR_HALL_CTRL_SPEC> {
        HALL_PHASE_FORCE_W::new(self, 31)
    }
}
#[doc = "no public\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_hall_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_hall_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_HALL_CTRL_SPEC;
impl crate::RegisterSpec for SAR_HALL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_hall_ctrl::R`](R) reader structure"]
impl crate::Readable for SAR_HALL_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_hall_ctrl::W`](W) writer structure"]
impl crate::Writable for SAR_HALL_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_HALL_CTRL to value 0xa000_0000"]
impl crate::Resettable for SAR_HALL_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xa000_0000;
}
