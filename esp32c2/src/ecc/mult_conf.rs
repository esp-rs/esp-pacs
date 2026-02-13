#[doc = "Register `MULT_CONF` reader"]
pub type R = crate::R<MULT_CONF_SPEC>;
#[doc = "Register `MULT_CONF` writer"]
pub type W = crate::W<MULT_CONF_SPEC>;
#[doc = "Field `START` reader - Configures whether to start calculation of ECC Accelerator."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Configures whether to start calculation of ECC Accelerator."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` writer - Configures whether to reset ECC Accelerator."]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Configures the key length mode bit of ECC Accelerator.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KEY_LENGTH {
    #[doc = "0: P-192 elliptic curve"]
    P192 = 0,
    #[doc = "1: P-256 elliptic curve"]
    P256 = 1,
}
impl From<KEY_LENGTH> for bool {
    #[inline(always)]
    fn from(variant: KEY_LENGTH) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `KEY_LENGTH` reader - Configures the key length mode bit of ECC Accelerator."]
pub type KEY_LENGTH_R = crate::BitReader<KEY_LENGTH>;
impl KEY_LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> KEY_LENGTH {
        match self.bits {
            false => KEY_LENGTH::P192,
            true => KEY_LENGTH::P256,
        }
    }
    #[doc = "P-192 elliptic curve"]
    #[inline(always)]
    pub fn is_p192(&self) -> bool {
        *self == KEY_LENGTH::P192
    }
    #[doc = "P-256 elliptic curve"]
    #[inline(always)]
    pub fn is_p256(&self) -> bool {
        *self == KEY_LENGTH::P256
    }
}
#[doc = "Field `KEY_LENGTH` writer - Configures the key length mode bit of ECC Accelerator."]
pub type KEY_LENGTH_W<'a, REG> = crate::BitWriter<'a, REG, KEY_LENGTH>;
impl<'a, REG> KEY_LENGTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "P-192 elliptic curve"]
    #[inline(always)]
    pub fn p192(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_LENGTH::P192)
    }
    #[doc = "P-256 elliptic curve"]
    #[inline(always)]
    pub fn p256(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_LENGTH::P256)
    }
}
#[doc = "Field `SECURITY_MODE` reader - Configures the security mode of ECC Accelerator."]
pub type SECURITY_MODE_R = crate::BitReader;
#[doc = "Field `SECURITY_MODE` writer - Configures the security mode of ECC Accelerator."]
pub type SECURITY_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - Configures whether to force on register clock gate."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Configures whether to force on register clock gate."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_MODE` reader - Configures the work mode of ECC Accelerator."]
pub type WORK_MODE_R = crate::FieldReader;
#[doc = "Field `WORK_MODE` writer - Configures the work mode of ECC Accelerator."]
pub type WORK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VERIFICATION_RESULT` reader - Represents the verification result of ECC Accelerator, valid only when calculation is done."]
pub type VERIFICATION_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Configures whether to start calculation of ECC Accelerator."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Configures the key length mode bit of ECC Accelerator."]
    #[inline(always)]
    pub fn key_length(&self) -> KEY_LENGTH_R {
        KEY_LENGTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures the security mode of ECC Accelerator."]
    #[inline(always)]
    pub fn security_mode(&self) -> SECURITY_MODE_R {
        SECURITY_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether to force on register clock gate."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Configures the work mode of ECC Accelerator."]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Represents the verification result of ECC Accelerator, valid only when calculation is done."]
    #[inline(always)]
    pub fn verification_result(&self) -> VERIFICATION_RESULT_R {
        VERIFICATION_RESULT_R::new(((self.bits >> 8) & 1) != 0)
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
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to start calculation of ECC Accelerator."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, MULT_CONF_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to reset ECC Accelerator."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<'_, MULT_CONF_SPEC> {
        RESET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures the key length mode bit of ECC Accelerator."]
    #[inline(always)]
    pub fn key_length(&mut self) -> KEY_LENGTH_W<'_, MULT_CONF_SPEC> {
        KEY_LENGTH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures the security mode of ECC Accelerator."]
    #[inline(always)]
    pub fn security_mode(&mut self) -> SECURITY_MODE_W<'_, MULT_CONF_SPEC> {
        SECURITY_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether to force on register clock gate."]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, MULT_CONF_SPEC> {
        CLK_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Configures the work mode of ECC Accelerator."]
    #[inline(always)]
    pub fn work_mode(&mut self) -> WORK_MODE_W<'_, MULT_CONF_SPEC> {
        WORK_MODE_W::new(self, 5)
    }
}
#[doc = "I2S RX configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_CONF_SPEC;
impl crate::RegisterSpec for MULT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mult_conf::R`](R) reader structure"]
impl crate::Readable for MULT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mult_conf::W`](W) writer structure"]
impl crate::Writable for MULT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MULT_CONF to value 0"]
impl crate::Resettable for MULT_CONF_SPEC {}
