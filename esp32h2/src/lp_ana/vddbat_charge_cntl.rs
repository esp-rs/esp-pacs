#[doc = "Register `VDDBAT_CHARGE_CNTL` reader"]
pub type R = crate::R<VDDBAT_CHARGE_CNTL_SPEC>;
#[doc = "Register `VDDBAT_CHARGE_CNTL` writer"]
pub type W = crate::W<VDDBAT_CHARGE_CNTL_SPEC>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_FLAG` reader - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_FLAG_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_CHARGER` reader - need_des"]
pub type VDDBAT_CHARGE_CHARGER_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_CHARGER` writer - need_des"]
pub type VDDBAT_CHARGE_CHARGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_CHARGE_CNT_CLR` reader - need_des"]
pub type VDDBAT_CHARGE_CNT_CLR_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_CNT_CLR` writer - need_des"]
pub type VDDBAT_CHARGE_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE_TARGET` reader - need_des"]
pub type VDDBAT_CHARGE_UPVOLTAGE_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE_TARGET` writer - need_des"]
pub type VDDBAT_CHARGE_UPVOLTAGE_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_TARGET` reader - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE_TARGET` writer - need_des"]
pub type VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
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
                &self.vddbat_charge_undervoltage_flag(),
            )
            .field("vddbat_charge_charger", &self.vddbat_charge_charger())
            .field("vddbat_charge_cnt_clr", &self.vddbat_charge_cnt_clr())
            .field(
                "vddbat_charge_upvoltage_target",
                &self.vddbat_charge_upvoltage_target(),
            )
            .field(
                "vddbat_charge_undervoltage_target",
                &self.vddbat_charge_undervoltage_target(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_charger(&mut self) -> VDDBAT_CHARGE_CHARGER_W<VDDBAT_CHARGE_CNTL_SPEC> {
        VDDBAT_CHARGE_CHARGER_W::new(self, 10)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_cnt_clr(&mut self) -> VDDBAT_CHARGE_CNT_CLR_W<VDDBAT_CHARGE_CNTL_SPEC> {
        VDDBAT_CHARGE_CNT_CLR_W::new(self, 11)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_upvoltage_target(
        &mut self,
    ) -> VDDBAT_CHARGE_UPVOLTAGE_TARGET_W<VDDBAT_CHARGE_CNTL_SPEC> {
        VDDBAT_CHARGE_UPVOLTAGE_TARGET_W::new(self, 12)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn vddbat_charge_undervoltage_target(
        &mut self,
    ) -> VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_W<VDDBAT_CHARGE_CNTL_SPEC> {
        VDDBAT_CHARGE_UNDERVOLTAGE_TARGET_W::new(self, 22)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VDDBAT_CHARGE_CNTL to value 0xffc0_0000"]
impl crate::Resettable for VDDBAT_CHARGE_CNTL_SPEC {
    const RESET_VALUE: u32 = 0xffc0_0000;
}
