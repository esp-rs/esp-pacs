#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `SLVENA` reader - 1: allow the slave to operate on i2c or i3c bus. 0: the slave will ignore the bus. This should be not set until registers such as PARTNO, IDEXT and the like are set 1st -if used- since they impact data to the master"]
pub type SLVENA_R = crate::BitReader;
#[doc = "Field `SLVENA` writer - 1: allow the slave to operate on i2c or i3c bus. 0: the slave will ignore the bus. This should be not set until registers such as PARTNO, IDEXT and the like are set 1st -if used- since they impact data to the master"]
pub type SLVENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - 1:the slave will NACK all requests to it except CCC broadcast. This should be used with caution as the Master may determine the slave is missing if overused."]
pub type NACK_R = crate::BitReader;
#[doc = "Field `NACK` writer - 1:the slave will NACK all requests to it except CCC broadcast. This should be used with caution as the Master may determine the slave is missing if overused."]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHSS` reader - 1: the START and STOP sticky STATUS bits will only be set if MATCHED is set..This allows START and STOP to be used to detect end of a message to /from this slave."]
pub type MATCHSS_R = crate::BitReader;
#[doc = "Field `MATCHSS` writer - 1: the START and STOP sticky STATUS bits will only be set if MATCHED is set..This allows START and STOP to be used to detect end of a message to /from this slave."]
pub type MATCHSS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0IGNORE` reader - If 1, the Slave will not detect S0 or S1 errors and so not lock up waiting on an Exit Pattern. This should only be used when the bus will not use HDR."]
pub type S0IGNORE_R = crate::BitReader;
#[doc = "Field `S0IGNORE` writer - If 1, the Slave will not detect S0 or S1 errors and so not lock up waiting on an Exit Pattern. This should only be used when the bus will not use HDR."]
pub type S0IGNORE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDROK` reader - NA"]
pub type DDROK_R = crate::BitReader;
#[doc = "Field `DDROK` writer - NA"]
pub type DDROK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDRAND` reader - NA"]
pub type IDRAND_R = crate::BitReader;
#[doc = "Field `IDRAND` writer - NA"]
pub type IDRAND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFFLINE` reader - NA"]
pub type OFFLINE_R = crate::BitReader;
#[doc = "Field `OFFLINE` writer - NA"]
pub type OFFLINE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BAMATCH` reader - Bus Available condition match value for current ???Slow clock???. This provides the count of the slow clock to count out 1us (or more) to allow an IBI to drive SDA Low when the Master is not doing so. The max width , and so max value, is controlled by the block. Only if enabled for events such IBI or MR or HJ, and if enabled to provide this as a register. With is limited to CLK_SLOW_BITS"]
pub type BAMATCH_R = crate::FieldReader;
#[doc = "Field `BAMATCH` writer - Bus Available condition match value for current ???Slow clock???. This provides the count of the slow clock to count out 1us (or more) to allow an IBI to drive SDA Low when the Master is not doing so. The max width , and so max value, is controlled by the block. Only if enabled for events such IBI or MR or HJ, and if enabled to provide this as a register. With is limited to CLK_SLOW_BITS"]
pub type BAMATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SADDR` reader - If allowed by the block:sets i2c 7 bits static address,else should be 0. If enabled to use one and to be provided by SW. Block may provide in HW as well."]
pub type SADDR_R = crate::FieldReader;
#[doc = "Field `SADDR` writer - If allowed by the block:sets i2c 7 bits static address,else should be 0. If enabled to use one and to be provided by SW. Block may provide in HW as well."]
pub type SADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 1: allow the slave to operate on i2c or i3c bus. 0: the slave will ignore the bus. This should be not set until registers such as PARTNO, IDEXT and the like are set 1st -if used- since they impact data to the master"]
    #[inline(always)]
    pub fn slvena(&self) -> SLVENA_R {
        SLVENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:the slave will NACK all requests to it except CCC broadcast. This should be used with caution as the Master may determine the slave is missing if overused."]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: the START and STOP sticky STATUS bits will only be set if MATCHED is set..This allows START and STOP to be used to detect end of a message to /from this slave."]
    #[inline(always)]
    pub fn matchss(&self) -> MATCHSS_R {
        MATCHSS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If 1, the Slave will not detect S0 or S1 errors and so not lock up waiting on an Exit Pattern. This should only be used when the bus will not use HDR."]
    #[inline(always)]
    pub fn s0ignore(&self) -> S0IGNORE_R {
        S0IGNORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ddrok(&self) -> DDROK_R {
        DDROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn idrand(&self) -> IDRAND_R {
        IDRAND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn offline(&self) -> OFFLINE_R {
        OFFLINE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bus Available condition match value for current ???Slow clock???. This provides the count of the slow clock to count out 1us (or more) to allow an IBI to drive SDA Low when the Master is not doing so. The max width , and so max value, is controlled by the block. Only if enabled for events such IBI or MR or HJ, and if enabled to provide this as a register. With is limited to CLK_SLOW_BITS"]
    #[inline(always)]
    pub fn bamatch(&self) -> BAMATCH_R {
        BAMATCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 25:31 - If allowed by the block:sets i2c 7 bits static address,else should be 0. If enabled to use one and to be provided by SW. Block may provide in HW as well."]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("slvena", &self.slvena())
            .field("nack", &self.nack())
            .field("matchss", &self.matchss())
            .field("s0ignore", &self.s0ignore())
            .field("ddrok", &self.ddrok())
            .field("idrand", &self.idrand())
            .field("offline", &self.offline())
            .field("bamatch", &self.bamatch())
            .field("saddr", &self.saddr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1: allow the slave to operate on i2c or i3c bus. 0: the slave will ignore the bus. This should be not set until registers such as PARTNO, IDEXT and the like are set 1st -if used- since they impact data to the master"]
    #[inline(always)]
    pub fn slvena(&mut self) -> SLVENA_W<CONFIG_SPEC> {
        SLVENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:the slave will NACK all requests to it except CCC broadcast. This should be used with caution as the Master may determine the slave is missing if overused."]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<CONFIG_SPEC> {
        NACK_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: the START and STOP sticky STATUS bits will only be set if MATCHED is set..This allows START and STOP to be used to detect end of a message to /from this slave."]
    #[inline(always)]
    pub fn matchss(&mut self) -> MATCHSS_W<CONFIG_SPEC> {
        MATCHSS_W::new(self, 2)
    }
    #[doc = "Bit 3 - If 1, the Slave will not detect S0 or S1 errors and so not lock up waiting on an Exit Pattern. This should only be used when the bus will not use HDR."]
    #[inline(always)]
    pub fn s0ignore(&mut self) -> S0IGNORE_W<CONFIG_SPEC> {
        S0IGNORE_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ddrok(&mut self) -> DDROK_W<CONFIG_SPEC> {
        DDROK_W::new(self, 4)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn idrand(&mut self) -> IDRAND_W<CONFIG_SPEC> {
        IDRAND_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn offline(&mut self) -> OFFLINE_W<CONFIG_SPEC> {
        OFFLINE_W::new(self, 9)
    }
    #[doc = "Bits 16:23 - Bus Available condition match value for current ???Slow clock???. This provides the count of the slow clock to count out 1us (or more) to allow an IBI to drive SDA Low when the Master is not doing so. The max width , and so max value, is controlled by the block. Only if enabled for events such IBI or MR or HJ, and if enabled to provide this as a register. With is limited to CLK_SLOW_BITS"]
    #[inline(always)]
    pub fn bamatch(&mut self) -> BAMATCH_W<CONFIG_SPEC> {
        BAMATCH_W::new(self, 16)
    }
    #[doc = "Bits 25:31 - If allowed by the block:sets i2c 7 bits static address,else should be 0. If enabled to use one and to be provided by SW. Block may provide in HW as well."]
    #[inline(always)]
    pub fn saddr(&mut self) -> SADDR_W<CONFIG_SPEC> {
        SADDR_W::new(self, 25)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x002f_0001"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x002f_0001;
}
