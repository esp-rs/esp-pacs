///Register `ESC_CONF%s` reader
pub type R = crate::R<ESC_CONF_SPEC>;
///Register `ESC_CONF%s` writer
pub type W = crate::W<ESC_CONF_SPEC>;
///Field `SEPER_CHAR` reader - This register is used to define separators to encode data packets. The default value is 0xC0.
pub type SEPER_CHAR_R = crate::FieldReader;
///Field `SEPER_CHAR` writer - This register is used to define separators to encode data packets. The default value is 0xC0.
pub type SEPER_CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SEPER_ESC_CHAR0` reader - This register is used to define the first character of SLIP escape sequence. The default value is 0xDB.
pub type SEPER_ESC_CHAR0_R = crate::FieldReader;
///Field `SEPER_ESC_CHAR0` writer - This register is used to define the first character of SLIP escape sequence. The default value is 0xDB.
pub type SEPER_ESC_CHAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SEPER_ESC_CHAR1` reader - This register is used to define the second character of SLIP escape sequence. The default value is 0xDC.
pub type SEPER_ESC_CHAR1_R = crate::FieldReader;
///Field `SEPER_ESC_CHAR1` writer - This register is used to define the second character of SLIP escape sequence. The default value is 0xDC.
pub type SEPER_ESC_CHAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - This register is used to define separators to encode data packets. The default value is 0xC0.
    #[inline(always)]
    pub fn seper_char(&self) -> SEPER_CHAR_R {
        SEPER_CHAR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - This register is used to define the first character of SLIP escape sequence. The default value is 0xDB.
    #[inline(always)]
    pub fn seper_esc_char0(&self) -> SEPER_ESC_CHAR0_R {
        SEPER_ESC_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - This register is used to define the second character of SLIP escape sequence. The default value is 0xDC.
    #[inline(always)]
    pub fn seper_esc_char1(&self) -> SEPER_ESC_CHAR1_R {
        SEPER_ESC_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESC_CONF")
            .field("seper_char", &self.seper_char())
            .field("seper_esc_char0", &self.seper_esc_char0())
            .field("seper_esc_char1", &self.seper_esc_char1())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - This register is used to define separators to encode data packets. The default value is 0xC0.
    #[inline(always)]
    #[must_use]
    pub fn seper_char(&mut self) -> SEPER_CHAR_W<ESC_CONF_SPEC> {
        SEPER_CHAR_W::new(self, 0)
    }
    ///Bits 8:15 - This register is used to define the first character of SLIP escape sequence. The default value is 0xDB.
    #[inline(always)]
    #[must_use]
    pub fn seper_esc_char0(&mut self) -> SEPER_ESC_CHAR0_W<ESC_CONF_SPEC> {
        SEPER_ESC_CHAR0_W::new(self, 8)
    }
    ///Bits 16:23 - This register is used to define the second character of SLIP escape sequence. The default value is 0xDC.
    #[inline(always)]
    #[must_use]
    pub fn seper_esc_char1(&mut self) -> SEPER_ESC_CHAR1_W<ESC_CONF_SPEC> {
        SEPER_ESC_CHAR1_W::new(self, 16)
    }
}
/**Escape sequence configuration register %s

You can [`read`](crate::generic::Reg::read) this register and get [`esc_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esc_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ESC_CONF_SPEC;
impl crate::RegisterSpec for ESC_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`esc_conf::R`](R) reader structure
impl crate::Readable for ESC_CONF_SPEC {}
///`write(|w| ..)` method takes [`esc_conf::W`](W) writer structure
impl crate::Writable for ESC_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ESC_CONF%s to value 0x00dc_dbc0
impl crate::Resettable for ESC_CONF_SPEC {
    const RESET_VALUE: u32 = 0x00dc_dbc0;
}
