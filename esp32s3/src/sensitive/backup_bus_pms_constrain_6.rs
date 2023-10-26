#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_6` reader"]
pub type R = crate::R<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Register `BACKUP_BUS_PMS_CONSTRAIN_6` writer"]
pub type W = crate::W<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L` reader - BackUp access rtcfast_l permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L` writer - BackUp access rtcfast_l permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H` reader - BackUp access rtcfast_h permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_R = crate::FieldReader;
#[doc = "Field `BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H` writer - BackUp access rtcfast_h permission."]
pub type BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - BackUp access rtcfast_l permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtcfast_l(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_R {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - BackUp access rtcfast_h permission."]
    #[inline(always)]
    pub fn backup_bus_pms_constrain_rtcfast_h(&self) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_R {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BACKUP_BUS_PMS_CONSTRAIN_6")
            .field(
                "backup_bus_pms_constrain_rtcfast_l",
                &format_args!("{}", self.backup_bus_pms_constrain_rtcfast_l().bits()),
            )
            .field(
                "backup_bus_pms_constrain_rtcfast_h",
                &format_args!("{}", self.backup_bus_pms_constrain_rtcfast_h().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - BackUp access rtcfast_l permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_rtcfast_l(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_W<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC, 0> {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_L_W::new(self)
    }
    #[doc = "Bits 3:5 - BackUp access rtcfast_h permission."]
    #[inline(always)]
    #[must_use]
    pub fn backup_bus_pms_constrain_rtcfast_h(
        &mut self,
    ) -> BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_W<BACKUP_BUS_PMS_CONSTRAIN_6_SPEC, 3> {
        BACKUP_BUS_PMS_CONSTRAIN_RTCFAST_H_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "BackUp access peripherals permission configuration register 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`backup_bus_pms_constrain_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_bus_pms_constrain_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BACKUP_BUS_PMS_CONSTRAIN_6_SPEC;
impl crate::RegisterSpec for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`backup_bus_pms_constrain_6::R`](R) reader structure"]
impl crate::Readable for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`backup_bus_pms_constrain_6::W`](W) writer structure"]
impl crate::Writable for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BACKUP_BUS_PMS_CONSTRAIN_6 to value 0x3f"]
impl crate::Resettable for BACKUP_BUS_PMS_CONSTRAIN_6_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
