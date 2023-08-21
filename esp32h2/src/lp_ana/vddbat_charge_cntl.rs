#[doc = "Register `VDDBAT_CHARGE_CNTL` reader"]
pub type R = crate::R<VDDBAT_CHARGE_CNTL_SPEC>;
#[doc = "Register `VDDBAT_CHARGE_CNTL` writer"]
pub type W = crate::W<VDDBAT_CHARGE_CNTL_SPEC>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_FLAG` reader - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_FLAG_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_CHARGER` reader - need_des"]
pub type VDDBAT_CHARGE_CHARGER_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_CHARGER` writer - need_des"]
pub type VDDBAT_CHARGE_CHARGER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VDDBAT_CHARGE_CNT_CLR` reader - need_des"]
pub type VDDBAT_CHARGE_CNT_CLR_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_CNT_CLR` writer - need_des"]
pub type VDDBAT_CHARGE_CNT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE_TARGET` reader - need_des"]
pub type VDDBAT_CHARGE_UPVOLTAGE_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE_TARGET` writer - need_des"]
pub type VDDBAT_CHARGE_UPVOLTAGE_TARGET_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_TARGET` reader - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_TARGET` writer - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage_flag(&self) -> VDDBAT_CHARGE_UNDERVOLTAGE_FLAG_R {
        VDDBAT_CHARGE_UNDERVOLTAGE_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_charger(&self) -> VDDBAT_CHARGE_CHARGER_R {
        VDDBAT_CHARGE_CHARGER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_cnt_clr(&self) -> VDDBAT_CHARGE_CNT_CLR_R {
        VDDBAT_CHARGE_CNT_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_upvoltage_target(&self) -> VDDBAT_CHARGE_UPVOLTAGE_TARGET_R {
        VDDBAT_CHARGE_UPVOLTAGE_TARGET_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage_target(&self) -> VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_R {
        VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDDBAT_CHARGE_CNTL")
            .field(
                "vddbat_charge_undervoltage_flag",
                &format_args!("{}", self.vddbat_charge_undervoltage_flag().bit()),
            )
            .field(
                "vddbat_charge_charger",
                &format_args!("{}", self.vddbat_charge_charger().bit()),
            )
            .field(
                "vddbat_charge_cnt_clr",
                &format_args!("{}", self.vddbat_charge_cnt_clr().bit()),
            )
            .field(
                "vddbat_charge_upvoltage_target",
                &format_args!("{}", self.vddbat_charge_upvoltage_target().bits()),
            )
            .field(
                "vddbat_charge_undervoltage_target",
                &format_args!("{}", self.vddbat_charge_undervoltage_target().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VDDBAT_CHARGE_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_charger(
        &mut self,
    ) -> VDDBAT_CHARGE_CHARGER_W<VDDBAT_CHARGE_CNTL_SPEC, 10> {
        VDDBAT_CHARGE_CHARGER_W::new(self)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_cnt_clr(
        &mut self,
    ) -> VDDBAT_CHARGE_CNT_CLR_W<VDDBAT_CHARGE_CNTL_SPEC, 11> {
        VDDBAT_CHARGE_CNT_CLR_W::new(self)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_upvoltage_target(
        &mut self,
    ) -> VDDBAT_CHARGE_UPVOLTAGE_TARGET_W<VDDBAT_CHARGE_CNTL_SPEC, 12> {
        VDDBAT_CHARGE_UPVOLTAGE_TARGET_W::new(self)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_undervoltage_target(
        &mut self,
    ) -> VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_W<VDDBAT_CHARGE_CNTL_SPEC, 22> {
        VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vddbat_charge_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vddbat_charge_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDDBAT_CHARGE_CNTL_SPEC;
impl crate::RegisterSpec for VDDBAT_CHARGE_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vddbat_charge_cntl::R`](R) reader structure"]
impl crate::Readable for VDDBAT_CHARGE_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vddbat_charge_cntl::W`](W) writer structure"]
impl crate::Writable for VDDBAT_CHARGE_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VDDBAT_CHARGE_CNTL to value 0xffc0_0000"]
impl crate::Resettable for VDDBAT_CHARGE_CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffc0_0000;
}
