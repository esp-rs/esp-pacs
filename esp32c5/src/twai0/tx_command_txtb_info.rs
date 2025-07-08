#[doc = "Register `TX_COMMAND_TXTB_INFO` reader"]
pub type R = crate::R<TX_COMMAND_TXTB_INFO_SPEC>;
#[doc = "Register `TX_COMMAND_TXTB_INFO` writer"]
pub type W = crate::W<TX_COMMAND_TXTB_INFO_SPEC>;
#[doc = "Field `TXCE` writer - Issues \"set empty\" command."]
pub type TXCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCR` writer - Issues \"set ready\" command."]
pub type TXCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCA` writer - Issues \"set abort\" command."]
pub type TXCA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXB1` writer - Command is issued to TXT Buffer 1."]
pub type TXB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXB2` writer - Command is issued to TXT Buffer 2."]
pub type TXB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXB3` writer - Command is issued to TXT Buffer 3. If number of TXT Buffers is less than 3, this field is reserved and has no Function."]
pub type TXB3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXB4` writer - Command is issued to TXT Buffer 4. If number of TXT Buffers is less than 4, this field is reserved and has no Function."]
pub type TXB4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXB5` writer - Command is issued to TXT Buffer 5. If number of TXT Buffers is less than 5, this field is reserved and has no Function."]
pub type TXB5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXB6` writer - Command is issued to TXT Buffer 6. If number of TXT Buffers is less than 6, this field is reserved and has no Function."]
pub type TXB6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXB7` writer - Command is issued to TXT Buffer 7. If number of TXT Buffers is less than 7, this field is reserved and has no Function."]
pub type TXB7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXB8` writer - Command is issued to TXT Buffer 8. If number of TXT Buffers is less than 8, this field is reserved and has no Function."]
pub type TXB8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXT_BUFFER_COUNT` reader - Number of TXT buffers present in CTU CAN FD. Lowest buffer is always 1. Highest buffer is at index equal to number of present buffers."]
pub type TXT_BUFFER_COUNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 16:19 - Number of TXT buffers present in CTU CAN FD. Lowest buffer is always 1. Highest buffer is at index equal to number of present buffers."]
    #[inline(always)]
    pub fn txt_buffer_count(&self) -> TXT_BUFFER_COUNT_R {
        TXT_BUFFER_COUNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_COMMAND_TXTB_INFO")
            .field("txt_buffer_count", &self.txt_buffer_count())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Issues \"set empty\" command."]
    #[inline(always)]
    pub fn txce(&mut self) -> TXCE_W<TX_COMMAND_TXTB_INFO_SPEC> {
        TXCE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Issues \"set ready\" command."]
    #[inline(always)]
    pub fn txcr(&mut self) -> TXCR_W<TX_COMMAND_TXTB_INFO_SPEC> {
        TXCR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Issues \"set abort\" command."]
    #[inline(always)]
    pub fn txca(&mut self) -> TXCA_W<TX_COMMAND_TXTB_INFO_SPEC> {
        TXCA_W::new(self, 2)
    }
    #[doc = "Bit 8 - Command is issued to TXT Buffer 1."]
    #[inline(always)]
    pub fn txb1(&mut self) -> TXB1_W<TX_COMMAND_TXTB_INFO_SPEC> {
        TXB1_W::new(self, 8)
    }
    #[doc = "Bit 9 - Command is issued to TXT Buffer 2."]
    #[inline(always)]
    pub fn txb2(&mut self) -> TXB2_W<TX_COMMAND_TXTB_INFO_SPEC> {
        TXB2_W::new(self, 9)
    }
    #[doc = "Bit 10 - Command is issued to TXT Buffer 3. If number of TXT Buffers is less than 3, this field is reserved and has no Function."]
    #[inline(always)]
    pub fn txb3(&mut self) -> TXB3_W<TX_COMMAND_TXTB_INFO_SPEC> {
        TXB3_W::new(self, 10)
    }
    #[doc = "Bit 11 - Command is issued to TXT Buffer 4. If number of TXT Buffers is less than 4, this field is reserved and has no Function."]
    #[inline(always)]
    pub fn txb4(&mut self) -> TXB4_W<TX_COMMAND_TXTB_INFO_SPEC> {
        TXB4_W::new(self, 11)
    }
    #[doc = "Bit 12 - Command is issued to TXT Buffer 5. If number of TXT Buffers is less than 5, this field is reserved and has no Function."]
    #[inline(always)]
    pub fn txb5(&mut self) -> TXB5_W<TX_COMMAND_TXTB_INFO_SPEC> {
        TXB5_W::new(self, 12)
    }
    #[doc = "Bit 13 - Command is issued to TXT Buffer 6. If number of TXT Buffers is less than 6, this field is reserved and has no Function."]
    #[inline(always)]
    pub fn txb6(&mut self) -> TXB6_W<TX_COMMAND_TXTB_INFO_SPEC> {
        TXB6_W::new(self, 13)
    }
    #[doc = "Bit 14 - Command is issued to TXT Buffer 7. If number of TXT Buffers is less than 7, this field is reserved and has no Function."]
    #[inline(always)]
    pub fn txb7(&mut self) -> TXB7_W<TX_COMMAND_TXTB_INFO_SPEC> {
        TXB7_W::new(self, 14)
    }
    #[doc = "Bit 15 - Command is issued to TXT Buffer 8. If number of TXT Buffers is less than 8, this field is reserved and has no Function."]
    #[inline(always)]
    pub fn txb8(&mut self) -> TXB8_W<TX_COMMAND_TXTB_INFO_SPEC> {
        TXB8_W::new(self, 15)
    }
}
#[doc = "TWAI FD TXT buffer command & information register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_command_txtb_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_command_txtb_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_COMMAND_TXTB_INFO_SPEC;
impl crate::RegisterSpec for TX_COMMAND_TXTB_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_command_txtb_info::R`](R) reader structure"]
impl crate::Readable for TX_COMMAND_TXTB_INFO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_command_txtb_info::W`](W) writer structure"]
impl crate::Writable for TX_COMMAND_TXTB_INFO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_COMMAND_TXTB_INFO to value 0x0004_0000"]
impl crate::Resettable for TX_COMMAND_TXTB_INFO_SPEC {
    const RESET_VALUE: u32 = 0x0004_0000;
}
