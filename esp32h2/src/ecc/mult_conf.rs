#[doc = "Register `MULT_CONF` reader"]
pub struct R(crate::R<MULT_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULT_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULT_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULT_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MULT_CONF` writer"]
pub struct W(crate::W<MULT_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MULT_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MULT_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MULT_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, MULT_CONF_SPEC, O>;
#[doc = "Field `RESET` writer - Write 1 to reset ECC Accelerator."]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, MULT_CONF_SPEC, O>;
#[doc = "Field `KEY_LENGTH` reader - The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256."]
pub type KEY_LENGTH_R = crate::BitReader;
#[doc = "Field `KEY_LENGTH` writer - The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256."]
pub type KEY_LENGTH_W<'a, const O: u8> = crate::BitWriter<'a, MULT_CONF_SPEC, O>;
#[doc = "Field `MOD_BASE` reader - The mod base of mod operation, only valid in work_mode 8-11. 0: n(order of curve). 1: p(mod base of curve)"]
pub type MOD_BASE_R = crate::BitReader;
#[doc = "Field `MOD_BASE` writer - The mod base of mod operation, only valid in work_mode 8-11. 0: n(order of curve). 1: p(mod base of curve)"]
pub type MOD_BASE_W<'a, const O: u8> = crate::BitWriter<'a, MULT_CONF_SPEC, O>;
#[doc = "Field `WORK_MODE` reader - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Reserved. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Point Add Mode. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode. 8: mod addition. 9. mod substraction. 10: mod multiplication. 11: mod division."]
pub type WORK_MODE_R = crate::FieldReader;
#[doc = "Field `WORK_MODE` writer - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Reserved. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Point Add Mode. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode. 8: mod addition. 9. mod substraction. 10: mod multiplication. 11: mod division."]
pub type WORK_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, MULT_CONF_SPEC, 4, O>;
#[doc = "Field `SECURITY_MODE` reader - Reserved"]
pub type SECURITY_MODE_R = crate::BitReader;
#[doc = "Field `SECURITY_MODE` writer - Reserved"]
pub type SECURITY_MODE_W<'a, const O: u8> = crate::BitWriter<'a, MULT_CONF_SPEC, O>;
#[doc = "Field `VERIFICATION_RESULT` reader - The verification result bit of ECC Accelerator, only valid when calculation is done."]
pub type VERIFICATION_RESULT_R = crate::BitReader;
#[doc = "Field `CLK_EN` reader - Write 1 to force on register clock gate."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Write 1 to force on register clock gate."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, MULT_CONF_SPEC, O>;
#[doc = "Field `MEM_CLOCK_GATE_FORCE_ON` reader - ECC memory clock gate force on register"]
pub type MEM_CLOCK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLOCK_GATE_FORCE_ON` writer - ECC memory clock gate force on register"]
pub type MEM_CLOCK_GATE_FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, MULT_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256."]
    #[inline(always)]
    pub fn key_length(&self) -> KEY_LENGTH_R {
        KEY_LENGTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The mod base of mod operation, only valid in work_mode 8-11. 0: n(order of curve). 1: p(mod base of curve)"]
    #[inline(always)]
    pub fn mod_base(&self) -> MOD_BASE_R {
        MOD_BASE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Reserved. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Point Add Mode. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode. 8: mod addition. 9. mod substraction. 10: mod multiplication. 11: mod division."]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn security_mode(&self) -> SECURITY_MODE_R {
        SECURITY_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 29 - The verification result bit of ECC Accelerator, only valid when calculation is done."]
    #[inline(always)]
    pub fn verification_result(&self) -> VERIFICATION_RESULT_R {
        VERIFICATION_RESULT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write 1 to force on register clock gate."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC memory clock gate force on register"]
    #[inline(always)]
    pub fn mem_clock_gate_force_on(&self) -> MEM_CLOCK_GATE_FORCE_ON_R {
        MEM_CLOCK_GATE_FORCE_ON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_CONF")
            .field("start", &format_args!("{}", self.start().bit()))
            .field("key_length", &format_args!("{}", self.key_length().bit()))
            .field("mod_base", &format_args!("{}", self.mod_base().bit()))
            .field("work_mode", &format_args!("{}", self.work_mode().bits()))
            .field(
                "security_mode",
                &format_args!("{}", self.security_mode().bit()),
            )
            .field(
                "verification_result",
                &format_args!("{}", self.verification_result().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field(
                "mem_clock_gate_force_on",
                &format_args!("{}", self.mem_clock_gate_force_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MULT_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Write 1 to reset ECC Accelerator."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<1> {
        RESET_W::new(self)
    }
    #[doc = "Bit 2 - The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256."]
    #[inline(always)]
    #[must_use]
    pub fn key_length(&mut self) -> KEY_LENGTH_W<2> {
        KEY_LENGTH_W::new(self)
    }
    #[doc = "Bit 3 - The mod base of mod operation, only valid in work_mode 8-11. 0: n(order of curve). 1: p(mod base of curve)"]
    #[inline(always)]
    #[must_use]
    pub fn mod_base(&mut self) -> MOD_BASE_W<3> {
        MOD_BASE_W::new(self)
    }
    #[doc = "Bits 4:7 - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Reserved. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Point Add Mode. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode. 8: mod addition. 9. mod substraction. 10: mod multiplication. 11: mod division."]
    #[inline(always)]
    #[must_use]
    pub fn work_mode(&mut self) -> WORK_MODE_W<4> {
        WORK_MODE_W::new(self)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn security_mode(&mut self) -> SECURITY_MODE_W<8> {
        SECURITY_MODE_W::new(self)
    }
    #[doc = "Bit 30 - Write 1 to force on register clock gate."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<30> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 31 - ECC memory clock gate force on register"]
    #[inline(always)]
    #[must_use]
    pub fn mem_clock_gate_force_on(&mut self) -> MEM_CLOCK_GATE_FORCE_ON_W<31> {
        MEM_CLOCK_GATE_FORCE_ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECC configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mult_conf](index.html) module"]
pub struct MULT_CONF_SPEC;
impl crate::RegisterSpec for MULT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mult_conf::R](R) reader structure"]
impl crate::Readable for MULT_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mult_conf::W](W) writer structure"]
impl crate::Writable for MULT_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MULT_CONF to value 0x8000_0000"]
impl crate::Resettable for MULT_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
