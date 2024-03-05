#[doc = "Register `FH1_CFG1` reader"]
pub type R = crate::R<FH1_CFG1_SPEC>;
#[doc = "Register `FH1_CFG1` writer"]
pub type W = crate::W<FH1_CFG1_SPEC>;
#[doc = "Field `TZ1_CLR_OST` reader - a rising edge will clear on going one-shot mode action"]
pub type TZ1_CLR_OST_R = crate::BitReader;
#[doc = "Field `TZ1_CLR_OST` writer - a rising edge will clear on going one-shot mode action"]
pub type TZ1_CLR_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ1_CBCPULSE` reader - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP"]
pub type TZ1_CBCPULSE_R = crate::FieldReader;
#[doc = "Field `TZ1_CBCPULSE` writer - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP"]
pub type TZ1_CBCPULSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TZ1_FORCE_CBC` reader - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ1_FORCE_CBC_R = crate::BitReader;
#[doc = "Field `TZ1_FORCE_CBC` writer - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ1_FORCE_CBC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZ1_FORCE_OST` reader - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ1_FORCE_OST_R = crate::BitReader;
#[doc = "Field `TZ1_FORCE_OST` writer - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ1_FORCE_OST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn tz1_clr_ost(&self) -> TZ1_CLR_OST_R {
        TZ1_CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP"]
    #[inline(always)]
    pub fn tz1_cbcpulse(&self) -> TZ1_CBCPULSE_R {
        TZ1_CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn tz1_force_cbc(&self) -> TZ1_FORCE_CBC_R {
        TZ1_FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn tz1_force_ost(&self) -> TZ1_FORCE_OST_R {
        TZ1_FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH1_CFG1")
            .field("tz1_clr_ost", &format_args!("{}", self.tz1_clr_ost().bit()))
            .field(
                "tz1_cbcpulse",
                &format_args!("{}", self.tz1_cbcpulse().bits()),
            )
            .field(
                "tz1_force_cbc",
                &format_args!("{}", self.tz1_force_cbc().bit()),
            )
            .field(
                "tz1_force_ost",
                &format_args!("{}", self.tz1_force_ost().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FH1_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz1_clr_ost(&mut self) -> TZ1_CLR_OST_W<FH1_CFG1_SPEC> {
        TZ1_CLR_OST_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. When bit0 is set to 1: TEZ, when bit1 is set to 1:TEP"]
    #[inline(always)]
    #[must_use]
    pub fn tz1_cbcpulse(&mut self) -> TZ1_CBCPULSE_W<FH1_CFG1_SPEC> {
        TZ1_CBCPULSE_W::new(self, 1)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz1_force_cbc(&mut self) -> TZ1_FORCE_CBC_W<FH1_CFG1_SPEC> {
        TZ1_FORCE_CBC_W::new(self, 3)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz1_force_ost(&mut self) -> TZ1_FORCE_OST_W<FH1_CFG1_SPEC> {
        TZ1_FORCE_OST_W::new(self, 4)
    }
}
#[doc = "Software triggers for fault handler actions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh1_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh1_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FH1_CFG1_SPEC;
impl crate::RegisterSpec for FH1_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fh1_cfg1::R`](R) reader structure"]
impl crate::Readable for FH1_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fh1_cfg1::W`](W) writer structure"]
impl crate::Writable for FH1_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FH1_CFG1 to value 0"]
impl crate::Resettable for FH1_CFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
