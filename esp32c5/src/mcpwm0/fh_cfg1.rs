#[doc = "Register `FH%s_CFG1` reader"]
pub type R = crate::R<FH_CFG1_SPEC>;
#[doc = "Register `FH%s_CFG1` writer"]
pub type W = crate::W<FH_CFG1_SPEC>;
#[doc = "Field `TZ_CLR_OST` reader - Configures the generation of software one-shot mode action clear. A toggle (software negate its value) triggers a clear for on going one-shot mode action."]
pub type TZ_CLR_OST_R = crate::BitReader;
#[doc = "Field `TZ_CLR_OST` writer - Configures the generation of software one-shot mode action clear. A toggle (software negate its value) triggers a clear for on going one-shot mode action."]
pub type TZ_CLR_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_CBCPULSE` reader - Configures the refresh moment selection of cycle-by-cycle mode action.\\\\0: Select nothing, will not refresh\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP"]
pub type TZ_CBCPULSE_R = crate::FieldReader;
#[doc = "Field `TZ_CBCPULSE` writer - Configures the refresh moment selection of cycle-by-cycle mode action.\\\\0: Select nothing, will not refresh\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP"]
pub type TZ_CBCPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZ_FORCE_CBC` reader - Configures the generation of software cycle-by-cycle mode action. A toggle (software negate its value) triggers a cycle-by-cycle mode action."]
pub type TZ_FORCE_CBC_R = crate::BitReader;
#[doc = "Field `TZ_FORCE_CBC` writer - Configures the generation of software cycle-by-cycle mode action. A toggle (software negate its value) triggers a cycle-by-cycle mode action."]
pub type TZ_FORCE_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ_FORCE_OST` reader - Configures the generation of software one-shot mode action. A toggle (software negate its value) triggers a one-shot mode action."]
pub type TZ_FORCE_OST_R = crate::BitReader;
#[doc = "Field `TZ_FORCE_OST` writer - Configures the generation of software one-shot mode action. A toggle (software negate its value) triggers a one-shot mode action."]
pub type TZ_FORCE_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the generation of software one-shot mode action clear. A toggle (software negate its value) triggers a clear for on going one-shot mode action."]
    #[inline(always)]
    pub fn tz_clr_ost(&self) -> TZ_CLR_OST_R {
        TZ_CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Configures the refresh moment selection of cycle-by-cycle mode action.\\\\0: Select nothing, will not refresh\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP"]
    #[inline(always)]
    pub fn tz_cbcpulse(&self) -> TZ_CBCPULSE_R {
        TZ_CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Configures the generation of software cycle-by-cycle mode action. A toggle (software negate its value) triggers a cycle-by-cycle mode action."]
    #[inline(always)]
    pub fn tz_force_cbc(&self) -> TZ_FORCE_CBC_R {
        TZ_FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures the generation of software one-shot mode action. A toggle (software negate its value) triggers a one-shot mode action."]
    #[inline(always)]
    pub fn tz_force_ost(&self) -> TZ_FORCE_OST_R {
        TZ_FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH_CFG1")
            .field("tz_clr_ost", &self.tz_clr_ost())
            .field("tz_cbcpulse", &self.tz_cbcpulse())
            .field("tz_force_cbc", &self.tz_force_cbc())
            .field("tz_force_ost", &self.tz_force_ost())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the generation of software one-shot mode action clear. A toggle (software negate its value) triggers a clear for on going one-shot mode action."]
    #[inline(always)]
    pub fn tz_clr_ost(&mut self) -> TZ_CLR_OST_W<FH_CFG1_SPEC> {
        TZ_CLR_OST_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Configures the refresh moment selection of cycle-by-cycle mode action.\\\\0: Select nothing, will not refresh\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP"]
    #[inline(always)]
    pub fn tz_cbcpulse(&mut self) -> TZ_CBCPULSE_W<FH_CFG1_SPEC> {
        TZ_CBCPULSE_W::new(self, 1)
    }
    #[doc = "Bit 3 - Configures the generation of software cycle-by-cycle mode action. A toggle (software negate its value) triggers a cycle-by-cycle mode action."]
    #[inline(always)]
    pub fn tz_force_cbc(&mut self) -> TZ_FORCE_CBC_W<FH_CFG1_SPEC> {
        TZ_FORCE_CBC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures the generation of software one-shot mode action. A toggle (software negate its value) triggers a one-shot mode action."]
    #[inline(always)]
    pub fn tz_force_ost(&mut self) -> TZ_FORCE_OST_W<FH_CFG1_SPEC> {
        TZ_FORCE_OST_W::new(self, 4)
    }
}
#[doc = "Software triggers for fault handler actions configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`fh_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fh_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH_CFG1_SPEC;
impl crate::RegisterSpec for FH_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh_cfg1::R`](R) reader structure"]
impl crate::Readable for FH_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fh_cfg1::W`](W) writer structure"]
impl crate::Writable for FH_CFG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FH%s_CFG1 to value 0"]
impl crate::Resettable for FH_CFG1_SPEC {}
