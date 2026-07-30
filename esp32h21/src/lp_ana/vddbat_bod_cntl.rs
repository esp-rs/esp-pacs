#[doc = "Register `VDDBAT_BOD_CNTL` reader"]
pub type R = crate::R<VDDBAT_BOD_CNTL_SPEC>;
#[doc = "Register `VDDBAT_BOD_CNTL` writer"]
pub type W = crate::W<VDDBAT_BOD_CNTL_SPEC>;
#[doc = "Field `VDDBAT_UNDERVOLTAGE_FLAG` reader - need_des"]
pub type VDDBAT_UNDERVOLTAGE_FLAG_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGER` reader - need_des"]
pub type VDDBAT_CHARGER_R = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGER` writer - need_des"]
pub type VDDBAT_CHARGER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_CNT_CLR` reader - need_des"]
pub type VDDBAT_CNT_CLR_R = crate::BitReader;
#[doc = "Field `VDDBAT_CNT_CLR` writer - need_des"]
pub type VDDBAT_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_UPVOLTAGE_TARGET` reader - need_des"]
pub type VDDBAT_UPVOLTAGE_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `VDDBAT_UPVOLTAGE_TARGET` writer - need_des"]
pub type VDDBAT_UPVOLTAGE_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `VDDBAT_UNDERVOLTAGE_TARGET` reader - need_des"]
pub type VDDBAT_UNDERVOLTAGE_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `VDDBAT_UNDERVOLTAGE_TARGET` writer - need_des"]
pub type VDDBAT_UNDERVOLTAGE_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage_flag(&self) -> VDDBAT_UNDERVOLTAGE_FLAG_R {
        VDDBAT_UNDERVOLTAGE_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn vddbat_charger(&self) -> VDDBAT_CHARGER_R {
        VDDBAT_CHARGER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn vddbat_cnt_clr(&self) -> VDDBAT_CNT_CLR_R {
        VDDBAT_CNT_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    pub fn vddbat_upvoltage_target(&self) -> VDDBAT_UPVOLTAGE_TARGET_R {
        VDDBAT_UPVOLTAGE_TARGET_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage_target(&self) -> VDDBAT_UNDERVOLTAGE_TARGET_R {
        VDDBAT_UNDERVOLTAGE_TARGET_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VDDBAT_BOD_CNTL")
            .field("vddbat_undervoltage_flag", &self.vddbat_undervoltage_flag())
            .field("vddbat_charger", &self.vddbat_charger())
            .field("vddbat_cnt_clr", &self.vddbat_cnt_clr())
            .field("vddbat_upvoltage_target", &self.vddbat_upvoltage_target())
            .field(
                "vddbat_undervoltage_target",
                &self.vddbat_undervoltage_target(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn vddbat_charger(&mut self) -> VDDBAT_CHARGER_W<'_, VDDBAT_BOD_CNTL_SPEC> {
        VDDBAT_CHARGER_W::new(self, 10)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn vddbat_cnt_clr(&mut self) -> VDDBAT_CNT_CLR_W<'_, VDDBAT_BOD_CNTL_SPEC> {
        VDDBAT_CNT_CLR_W::new(self, 11)
    }
    #[doc = "Bits 12:21 - need_des"]
    #[inline(always)]
    pub fn vddbat_upvoltage_target(
        &mut self,
    ) -> VDDBAT_UPVOLTAGE_TARGET_W<'_, VDDBAT_BOD_CNTL_SPEC> {
        VDDBAT_UPVOLTAGE_TARGET_W::new(self, 12)
    }
    #[doc = "Bits 22:31 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage_target(
        &mut self,
    ) -> VDDBAT_UNDERVOLTAGE_TARGET_W<'_, VDDBAT_BOD_CNTL_SPEC> {
        VDDBAT_UNDERVOLTAGE_TARGET_W::new(self, 22)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`vddbat_bod_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vddbat_bod_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VDDBAT_BOD_CNTL_SPEC;
impl crate::RegisterSpec for VDDBAT_BOD_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vddbat_bod_cntl::R`](R) reader structure"]
impl crate::Readable for VDDBAT_BOD_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vddbat_bod_cntl::W`](W) writer structure"]
impl crate::Writable for VDDBAT_BOD_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDDBAT_BOD_CNTL to value 0xffc0_0000"]
impl crate::Resettable for VDDBAT_BOD_CNTL_SPEC {
    const RESET_VALUE: u32 = 0xffc0_0000;
}
