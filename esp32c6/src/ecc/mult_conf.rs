#[doc = "Register `MULT_CONF` reader"]
pub type R = crate::R<MULT_CONF_SPEC>;
#[doc = "Register `MULT_CONF` writer"]
pub type W = crate::W<MULT_CONF_SPEC>;
#[doc = "Field `START` reader - Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` writer - Write 1 to reset ECC Accelerator."]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_LENGTH` reader - The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256."]
pub type KEY_LENGTH_R = crate::BitReader;
#[doc = "Field `KEY_LENGTH` writer - The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256."]
pub type KEY_LENGTH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECURITY_MODE` reader - Reserved"]
pub type SECURITY_MODE_R = crate::BitReader;
#[doc = "Field `SECURITY_MODE` writer - Reserved"]
pub type SECURITY_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Write 1 to force on register clock gate."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Write 1 to force on register clock gate."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_MODE` reader - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Division mode. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Reserved. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode."]
pub type WORK_MODE_R = crate::FieldReader;
#[doc = "Field `WORK_MODE` writer - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Division mode. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Reserved. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode."]
pub type WORK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VERIFICATION_RESULT` reader - The verification result bit of ECC Accelerator, only valid when calculation is done."]
pub type VERIFICATION_RESULT_R = crate::BitReader;
#[doc = "Field `MEM_CLOCK_GATE_FORCE_ON` reader - ECC memory clock gate force on register"]
pub type MEM_CLOCK_GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLOCK_GATE_FORCE_ON` writer - ECC memory clock gate force on register"]
pub type MEM_CLOCK_GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn security_mode(&self) -> SECURITY_MODE_R {
        SECURITY_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to force on register clock gate."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Division mode. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Reserved. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode."]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - The verification result bit of ECC Accelerator, only valid when calculation is done."]
    #[inline(always)]
    pub fn verification_result(&self) -> VERIFICATION_RESULT_R {
        VERIFICATION_RESULT_R::new(((self.bits >> 8) & 1) != 0)
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
            .field("start", &self.start())
            .field("key_length", &self.key_length())
            .field("security_mode", &self.security_mode())
            .field("clk_en", &self.clk_en())
            .field("work_mode", &self.work_mode())
            .field("verification_result", &self.verification_result())
            .field("mem_clock_gate_force_on", &self.mem_clock_gate_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start caculation of ECC Accelerator. This bit will be self-cleared after the caculatrion is done."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<MULT_CONF_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to reset ECC Accelerator."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<MULT_CONF_SPEC> {
        RESET_W::new(self, 1)
    }
    #[doc = "Bit 2 - The key length mode bit of ECC Accelerator. 0: P-192. 1: P-256."]
    #[inline(always)]
    pub fn key_length(&mut self) -> KEY_LENGTH_W<MULT_CONF_SPEC> {
        KEY_LENGTH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn security_mode(&mut self) -> SECURITY_MODE_W<MULT_CONF_SPEC> {
        SECURITY_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to force on register clock gate."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<MULT_CONF_SPEC> {
        CLK_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - The work mode bits of ECC Accelerator. 0: Point Mult Mode. 1: Division mode. 2: Point verification mode. 3: Point Verif+mult mode. 4: Jacobian Point Mult Mode. 5: Reserved. 6: Jacobian Point Verification Mode. 7: Point Verif + Jacobian Mult Mode."]
    #[inline(always)]
    pub fn work_mode(&mut self) -> WORK_MODE_W<MULT_CONF_SPEC> {
        WORK_MODE_W::new(self, 5)
    }
    #[doc = "Bit 31 - ECC memory clock gate force on register"]
    #[inline(always)]
    pub fn mem_clock_gate_force_on(&mut self) -> MEM_CLOCK_GATE_FORCE_ON_W<MULT_CONF_SPEC> {
        MEM_CLOCK_GATE_FORCE_ON_W::new(self, 31)
    }
}
#[doc = "ECC configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_CONF_SPEC;
impl crate::RegisterSpec for MULT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mult_conf::R`](R) reader structure"]
impl crate::Readable for MULT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mult_conf::W`](W) writer structure"]
impl crate::Writable for MULT_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MULT_CONF to value 0x8000_0000"]
impl crate::Resettable for MULT_CONF_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
