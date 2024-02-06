#[doc = "Register `SAR_MEAS_WAIT2` reader"]
pub type R = crate::R<SAR_MEAS_WAIT2_SPEC>;
#[doc = "Register `SAR_MEAS_WAIT2` writer"]
pub type W = crate::W<SAR_MEAS_WAIT2_SPEC>;
#[doc = "Field `FORCE_XPD_SAR_SW` reader - "]
pub type FORCE_XPD_SAR_SW_R = crate::BitReader;
#[doc = "Field `FORCE_XPD_SAR_SW` writer - "]
pub type FORCE_XPD_SAR_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_AMP_WAIT3` reader - "]
pub type SAR_AMP_WAIT3_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_AMP_WAIT3` writer - "]
pub type SAR_AMP_WAIT3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FORCE_XPD_AMP` reader - "]
pub type FORCE_XPD_AMP_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_AMP` writer - "]
pub type FORCE_XPD_AMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORCE_XPD_SAR` reader - "]
pub type FORCE_XPD_SAR_R = crate::FieldReader;
#[doc = "Field `FORCE_XPD_SAR` writer - "]
pub type FORCE_XPD_SAR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR2_RSTB_WAIT` reader - "]
pub type SAR2_RSTB_WAIT_R = crate::FieldReader;
#[doc = "Field `SAR2_RSTB_WAIT` writer - "]
pub type SAR2_RSTB_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn force_xpd_sar_sw(&self) -> FORCE_XPD_SAR_SW_R {
        FORCE_XPD_SAR_SW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sar_amp_wait3(&self) -> SAR_AMP_WAIT3_R {
        SAR_AMP_WAIT3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn force_xpd_amp(&self) -> FORCE_XPD_AMP_R {
        FORCE_XPD_AMP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn force_xpd_sar(&self) -> FORCE_XPD_SAR_R {
        FORCE_XPD_SAR_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    pub fn sar2_rstb_wait(&self) -> SAR2_RSTB_WAIT_R {
        SAR2_RSTB_WAIT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_MEAS_WAIT2")
            .field(
                "force_xpd_sar_sw",
                &format_args!("{}", self.force_xpd_sar_sw().bit()),
            )
            .field(
                "sar_amp_wait3",
                &format_args!("{}", self.sar_amp_wait3().bits()),
            )
            .field(
                "force_xpd_amp",
                &format_args!("{}", self.force_xpd_amp().bits()),
            )
            .field(
                "force_xpd_sar",
                &format_args!("{}", self.force_xpd_sar().bits()),
            )
            .field(
                "sar2_rstb_wait",
                &format_args!("{}", self.sar2_rstb_wait().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_MEAS_WAIT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_sar_sw(&mut self) -> FORCE_XPD_SAR_SW_W<SAR_MEAS_WAIT2_SPEC> {
        FORCE_XPD_SAR_SW_W::new(self, 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn sar_amp_wait3(&mut self) -> SAR_AMP_WAIT3_W<SAR_MEAS_WAIT2_SPEC> {
        SAR_AMP_WAIT3_W::new(self, 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_amp(&mut self) -> FORCE_XPD_AMP_W<SAR_MEAS_WAIT2_SPEC> {
        FORCE_XPD_AMP_W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn force_xpd_sar(&mut self) -> FORCE_XPD_SAR_W<SAR_MEAS_WAIT2_SPEC> {
        FORCE_XPD_SAR_W::new(self, 18)
    }
    #[doc = "Bits 20:27"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_rstb_wait(&mut self) -> SAR2_RSTB_WAIT_W<SAR_MEAS_WAIT2_SPEC> {
        SAR2_RSTB_WAIT_W::new(self, 20)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_meas_wait2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_meas_wait2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_MEAS_WAIT2_SPEC;
impl crate::RegisterSpec for SAR_MEAS_WAIT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_meas_wait2::R`](R) reader structure"]
impl crate::Readable for SAR_MEAS_WAIT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_meas_wait2::W`](W) writer structure"]
impl crate::Writable for SAR_MEAS_WAIT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_MEAS_WAIT2 to value 0x0020_000a"]
impl crate::Resettable for SAR_MEAS_WAIT2_SPEC {
    const RESET_VALUE: u32 = 0x0020_000a;
}
