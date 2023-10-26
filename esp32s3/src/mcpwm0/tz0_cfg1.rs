#[doc = "Register `TZ0_CFG1` reader"]
pub type R = crate::R<TZ0_CFG1_SPEC>;
#[doc = "Register `TZ0_CFG1` writer"]
pub type W = crate::W<TZ0_CFG1_SPEC>;
#[doc = "Field `TZ0_CLR_OST` reader - a rising edge will clear on going one-shot mode action"]
pub type TZ0_CLR_OST_R = crate::BitReader;
#[doc = "Field `TZ0_CLR_OST` writer - a rising edge will clear on going one-shot mode action"]
pub type TZ0_CLR_OST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TZ0_CBCPULSE` reader - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
pub type TZ0_CBCPULSE_R = crate::FieldReader;
#[doc = "Field `TZ0_CBCPULSE` writer - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
pub type TZ0_CBCPULSE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TZ0_FORCE_CBC` reader - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ0_FORCE_CBC_R = crate::BitReader;
#[doc = "Field `TZ0_FORCE_CBC` writer - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ0_FORCE_CBC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TZ0_FORCE_OST` reader - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ0_FORCE_OST_R = crate::BitReader;
#[doc = "Field `TZ0_FORCE_OST` writer - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ0_FORCE_OST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn tz0_clr_ost(&self) -> TZ0_CLR_OST_R {
        TZ0_CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
    #[inline(always)]
    pub fn tz0_cbcpulse(&self) -> TZ0_CBCPULSE_R {
        TZ0_CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn tz0_force_cbc(&self) -> TZ0_FORCE_CBC_R {
        TZ0_FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn tz0_force_ost(&self) -> TZ0_FORCE_OST_R {
        TZ0_FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TZ0_CFG1")
            .field("tz0_clr_ost", &format_args!("{}", self.tz0_clr_ost().bit()))
            .field(
                "tz0_cbcpulse",
                &format_args!("{}", self.tz0_cbcpulse().bits()),
            )
            .field(
                "tz0_force_cbc",
                &format_args!("{}", self.tz0_force_cbc().bit()),
            )
            .field(
                "tz0_force_ost",
                &format_args!("{}", self.tz0_force_ost().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TZ0_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_clr_ost(&mut self) -> TZ0_CLR_OST_W<TZ0_CFG1_SPEC, 0> {
        TZ0_CLR_OST_W::new(self)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_cbcpulse(&mut self) -> TZ0_CBCPULSE_W<TZ0_CFG1_SPEC, 1> {
        TZ0_CBCPULSE_W::new(self)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_force_cbc(&mut self) -> TZ0_FORCE_CBC_W<TZ0_CFG1_SPEC, 3> {
        TZ0_FORCE_CBC_W::new(self)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    #[must_use]
    pub fn tz0_force_ost(&mut self) -> TZ0_FORCE_OST_W<TZ0_CFG1_SPEC, 4> {
        TZ0_FORCE_OST_W::new(self)
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
#[doc = "Software triggers for fault handler actions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tz0_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tz0_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZ0_CFG1_SPEC;
impl crate::RegisterSpec for TZ0_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tz0_cfg1::R`](R) reader structure"]
impl crate::Readable for TZ0_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tz0_cfg1::W`](W) writer structure"]
impl crate::Writable for TZ0_CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TZ0_CFG1 to value 0"]
impl crate::Resettable for TZ0_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
