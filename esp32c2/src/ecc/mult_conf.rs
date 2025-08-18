#[doc = "Register `MULT_CONF` reader"]
pub type R = crate::R<MULT_CONF_SPEC>;
#[doc = "Register `MULT_CONF` writer"]
pub type W = crate::W<MULT_CONF_SPEC>;
#[doc = "Field `START` reader - Set this bit to reset receiver"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Set this bit to reset receiver"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` writer - Set this bit to reset Rx AFIFO"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY_LENGTH` reader - Set this bit to start receiving data"]
pub type KEY_LENGTH_R = crate::BitReader;
#[doc = "Field `KEY_LENGTH` writer - Set this bit to start receiving data"]
pub type KEY_LENGTH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECURITY_MODE` reader - Set this bit to enable slave receiver mode"]
pub type SECURITY_MODE_R = crate::BitReader;
#[doc = "Field `SECURITY_MODE` writer - Set this bit to enable slave receiver mode"]
pub type SECURITY_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - clk gate"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - clk gate"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_MODE` reader - Reserved"]
pub type WORK_MODE_R = crate::FieldReader;
#[doc = "Field `WORK_MODE` writer - Reserved"]
pub type WORK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VERIFICATION_RESULT` reader - Reserve"]
pub type VERIFICATION_RESULT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set this bit to reset receiver"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to start receiving data"]
    #[inline(always)]
    pub fn key_length(&self) -> KEY_LENGTH_R {
        KEY_LENGTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable slave receiver mode"]
    #[inline(always)]
    pub fn security_mode(&self) -> SECURITY_MODE_R {
        SECURITY_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk gate"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Reserve"]
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
    #[doc = "Bit 0 - Set this bit to reset receiver"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, MULT_CONF_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to reset Rx AFIFO"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<'_, MULT_CONF_SPEC> {
        RESET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to start receiving data"]
    #[inline(always)]
    pub fn key_length(&mut self) -> KEY_LENGTH_W<'_, MULT_CONF_SPEC> {
        KEY_LENGTH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable slave receiver mode"]
    #[inline(always)]
    pub fn security_mode(&mut self) -> SECURITY_MODE_W<'_, MULT_CONF_SPEC> {
        SECURITY_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - clk gate"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, MULT_CONF_SPEC> {
        CLK_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Reserved"]
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
