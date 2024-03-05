#[doc = "Register `REDUNDANT_ECO_CTRL` reader"]
pub type R = crate::R<REDUNDANT_ECO_CTRL_SPEC>;
#[doc = "Register `REDUNDANT_ECO_CTRL` writer"]
pub type W = crate::W<REDUNDANT_ECO_CTRL_SPEC>;
#[doc = "Field `REDUNDANT_ECO_DRIVE` reader - reg_redundant_eco_drive"]
pub type REDUNDANT_ECO_DRIVE_R = crate::BitReader;
#[doc = "Field `REDUNDANT_ECO_DRIVE` writer - reg_redundant_eco_drive"]
pub type REDUNDANT_ECO_DRIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDUNDANT_ECO_RESULT` reader - reg_redundant_eco_result"]
pub type REDUNDANT_ECO_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - reg_redundant_eco_drive"]
    #[inline(always)]
    pub fn redundant_eco_drive(&self) -> REDUNDANT_ECO_DRIVE_R {
        REDUNDANT_ECO_DRIVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_redundant_eco_result"]
    #[inline(always)]
    pub fn redundant_eco_result(&self) -> REDUNDANT_ECO_RESULT_R {
        REDUNDANT_ECO_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REDUNDANT_ECO_CTRL")
            .field(
                "redundant_eco_drive",
                &format_args!("{}", self.redundant_eco_drive().bit()),
            )
            .field(
                "redundant_eco_result",
                &format_args!("{}", self.redundant_eco_result().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REDUNDANT_ECO_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_redundant_eco_drive"]
    #[inline(always)]
    #[must_use]
    pub fn redundant_eco_drive(&mut self) -> REDUNDANT_ECO_DRIVE_W<REDUNDANT_ECO_CTRL_SPEC> {
        REDUNDANT_ECO_DRIVE_W::new(self, 0)
    }
}
#[doc = "eco register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`redundant_eco_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redundant_eco_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REDUNDANT_ECO_CTRL_SPEC;
impl crate::RegisterSpec for REDUNDANT_ECO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`redundant_eco_ctrl::R`](R) reader structure"]
impl crate::Readable for REDUNDANT_ECO_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`redundant_eco_ctrl::W`](W) writer structure"]
impl crate::Writable for REDUNDANT_ECO_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REDUNDANT_ECO_CTRL to value 0"]
impl crate::Resettable for REDUNDANT_ECO_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
