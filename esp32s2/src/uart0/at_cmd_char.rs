///Register `AT_CMD_CHAR` reader
pub type R = crate::R<AT_CMD_CHAR_SPEC>;
///Register `AT_CMD_CHAR` writer
pub type W = crate::W<AT_CMD_CHAR_SPEC>;
///Field `AT_CMD_CHAR` reader - This register is used to configure the content of AT_CMD character.
pub type AT_CMD_CHAR_R = crate::FieldReader;
///Field `AT_CMD_CHAR` writer - This register is used to configure the content of AT_CMD character.
pub type AT_CMD_CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CHAR_NUM` reader - This register is used to configure the number of continuous AT_CMD characters received by the receiver.
pub type CHAR_NUM_R = crate::FieldReader;
///Field `CHAR_NUM` writer - This register is used to configure the number of continuous AT_CMD characters received by the receiver.
pub type CHAR_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - This register is used to configure the content of AT_CMD character.
    #[inline(always)]
    pub fn at_cmd_char(&self) -> AT_CMD_CHAR_R {
        AT_CMD_CHAR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - This register is used to configure the number of continuous AT_CMD characters received by the receiver.
    #[inline(always)]
    pub fn char_num(&self) -> CHAR_NUM_R {
        CHAR_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AT_CMD_CHAR")
            .field("at_cmd_char", &self.at_cmd_char())
            .field("char_num", &self.char_num())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - This register is used to configure the content of AT_CMD character.
    #[inline(always)]
    #[must_use]
    pub fn at_cmd_char(&mut self) -> AT_CMD_CHAR_W<AT_CMD_CHAR_SPEC> {
        AT_CMD_CHAR_W::new(self, 0)
    }
    ///Bits 8:15 - This register is used to configure the number of continuous AT_CMD characters received by the receiver.
    #[inline(always)]
    #[must_use]
    pub fn char_num(&mut self) -> CHAR_NUM_W<AT_CMD_CHAR_SPEC> {
        CHAR_NUM_W::new(self, 8)
    }
}
/**AT escape sequence selection configuration

You can [`read`](crate::generic::Reg::read) this register and get [`at_cmd_char::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`at_cmd_char::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AT_CMD_CHAR_SPEC;
impl crate::RegisterSpec for AT_CMD_CHAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`at_cmd_char::R`](R) reader structure
impl crate::Readable for AT_CMD_CHAR_SPEC {}
///`write(|w| ..)` method takes [`at_cmd_char::W`](W) writer structure
impl crate::Writable for AT_CMD_CHAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AT_CMD_CHAR to value 0x032b
impl crate::Resettable for AT_CMD_CHAR_SPEC {
    const RESET_VALUE: u32 = 0x032b;
}
