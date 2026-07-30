#[doc = "Register `FH2_CFG1` reader"]
pub type R = crate::R<FH2_CFG1_SPEC>;
#[doc = "Register `FH2_CFG1` writer"]
pub type W = crate::W<FH2_CFG1_SPEC>;
#[doc = "Field `TZ2_CLR_OST` reader - a rising edge will clear on going one-shot mode action"]
pub type TZ2_CLR_OST_R = crate::BitReader;
#[doc = "Field `TZ2_CLR_OST` writer - a rising edge will clear on going one-shot mode action"]
pub type TZ2_CLR_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ2_CBCPULSE` reader - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP"]
pub type TZ2_CBCPULSE_R = crate::FieldReader;
#[doc = "Field `TZ2_CBCPULSE` writer - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP"]
pub type TZ2_CBCPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZ2_FORCE_CBC` reader - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ2_FORCE_CBC_R = crate::BitReader;
#[doc = "Field `TZ2_FORCE_CBC` writer - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ2_FORCE_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ2_FORCE_OST` reader - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ2_FORCE_OST_R = crate::BitReader;
#[doc = "Field `TZ2_FORCE_OST` writer - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ2_FORCE_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn tz2_clr_ost(&self) -> TZ2_CLR_OST_R {
        TZ2_CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP"]
    #[inline(always)]
    pub fn tz2_cbcpulse(&self) -> TZ2_CBCPULSE_R {
        TZ2_CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn tz2_force_cbc(&self) -> TZ2_FORCE_CBC_R {
        TZ2_FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn tz2_force_ost(&self) -> TZ2_FORCE_OST_R {
        TZ2_FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH2_CFG1")
            .field("tz2_clr_ost", &self.tz2_clr_ost())
            .field("tz2_cbcpulse", &self.tz2_cbcpulse())
            .field("tz2_force_cbc", &self.tz2_force_cbc())
            .field("tz2_force_ost", &self.tz2_force_ost())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn tz2_clr_ost(&mut self) -> TZ2_CLR_OST_W<'_, FH2_CFG1_SPEC> {
        TZ2_CLR_OST_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP"]
    #[inline(always)]
    pub fn tz2_cbcpulse(&mut self) -> TZ2_CBCPULSE_W<'_, FH2_CFG1_SPEC> {
        TZ2_CBCPULSE_W::new(self, 1)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn tz2_force_cbc(&mut self) -> TZ2_FORCE_CBC_W<'_, FH2_CFG1_SPEC> {
        TZ2_FORCE_CBC_W::new(self, 3)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn tz2_force_ost(&mut self) -> TZ2_FORCE_OST_W<'_, FH2_CFG1_SPEC> {
        TZ2_FORCE_OST_W::new(self, 4)
    }
}
#[doc = "Software triggers for fault handler actions\n\nYou can [`read`](crate::Reg::read) this register and get [`fh2_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fh2_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH2_CFG1_SPEC;
impl crate::RegisterSpec for FH2_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh2_cfg1::R`](R) reader structure"]
impl crate::Readable for FH2_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fh2_cfg1::W`](W) writer structure"]
impl crate::Writable for FH2_CFG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FH2_CFG1 to value 0"]
impl crate::Resettable for FH2_CFG1_SPEC {}
