///Register `MULT_CONF` reader
pub type R = crate::R<MULT_CONF_SPEC>;
///Register `MULT_CONF` writer
pub type W = crate::W<MULT_CONF_SPEC>;
///Field `START` reader - Set this bit to reset receiver
pub type START_R = crate::BitReader;
///Field `START` writer - Set this bit to reset receiver
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESET` writer - Set this bit to reset Rx AFIFO
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEY_LENGTH` reader - Set this bit to start receiving data
pub type KEY_LENGTH_R = crate::BitReader;
///Field `KEY_LENGTH` writer - Set this bit to start receiving data
pub type KEY_LENGTH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SECURITY_MODE` reader - Set this bit to enable slave receiver mode
pub type SECURITY_MODE_R = crate::BitReader;
///Field `SECURITY_MODE` writer - Set this bit to enable slave receiver mode
pub type SECURITY_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_EN` reader - clk gate
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - clk gate
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WORK_MODE` reader - Reserved
pub type WORK_MODE_R = crate::FieldReader;
///Field `WORK_MODE` writer - Reserved
pub type WORK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `VERIFICATION_RESULT` reader - Reserve
pub type VERIFICATION_RESULT_R = crate::BitReader;
impl R {
    ///Bit 0 - Set this bit to reset receiver
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Set this bit to start receiving data
    #[inline(always)]
    pub fn key_length(&self) -> KEY_LENGTH_R {
        KEY_LENGTH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set this bit to enable slave receiver mode
    #[inline(always)]
    pub fn security_mode(&self) -> SECURITY_MODE_R {
        SECURITY_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - clk gate
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - Reserved
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Reserve
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
    ///Bit 0 - Set this bit to reset receiver
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<MULT_CONF_SPEC> {
        START_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to reset Rx AFIFO
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<MULT_CONF_SPEC> {
        RESET_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to start receiving data
    #[inline(always)]
    #[must_use]
    pub fn key_length(&mut self) -> KEY_LENGTH_W<MULT_CONF_SPEC> {
        KEY_LENGTH_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to enable slave receiver mode
    #[inline(always)]
    #[must_use]
    pub fn security_mode(&mut self) -> SECURITY_MODE_W<MULT_CONF_SPEC> {
        SECURITY_MODE_W::new(self, 3)
    }
    ///Bit 4 - clk gate
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<MULT_CONF_SPEC> {
        CLK_EN_W::new(self, 4)
    }
    ///Bits 5:7 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn work_mode(&mut self) -> WORK_MODE_W<MULT_CONF_SPEC> {
        WORK_MODE_W::new(self, 5)
    }
}
/**I2S RX configure register

You can [`read`](crate::generic::Reg::read) this register and get [`mult_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mult_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MULT_CONF_SPEC;
impl crate::RegisterSpec for MULT_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mult_conf::R`](R) reader structure
impl crate::Readable for MULT_CONF_SPEC {}
///`write(|w| ..)` method takes [`mult_conf::W`](W) writer structure
impl crate::Writable for MULT_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MULT_CONF to value 0
impl crate::Resettable for MULT_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
