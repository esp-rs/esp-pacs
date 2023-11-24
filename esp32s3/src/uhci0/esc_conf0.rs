#[doc = "Register `ESC_CONF0` reader"]
pub type R = crate::R<ESC_CONF0_SPEC>;
#[doc = "Register `ESC_CONF0` writer"]
pub type W = crate::W<ESC_CONF0_SPEC>;
#[doc = "Field `SEPER_CHAR` reader - This register is used to define the separate char that need to be encoded, default is 0xc0."]
pub type SEPER_CHAR_R = crate::FieldReader;
#[doc = "Field `SEPER_CHAR` writer - This register is used to define the separate char that need to be encoded, default is 0xc0."]
pub type SEPER_CHAR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEPER_ESC_CHAR0` reader - This register is used to define the first char of slip escape sequence when encoding the separate char, default is 0xdb."]
pub type SEPER_ESC_CHAR0_R = crate::FieldReader;
#[doc = "Field `SEPER_ESC_CHAR0` writer - This register is used to define the first char of slip escape sequence when encoding the separate char, default is 0xdb."]
pub type SEPER_ESC_CHAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEPER_ESC_CHAR1` reader - This register is used to define the second char of slip escape sequence when encoding the separate char, default is 0xdc."]
pub type SEPER_ESC_CHAR1_R = crate::FieldReader;
#[doc = "Field `SEPER_ESC_CHAR1` writer - This register is used to define the second char of slip escape sequence when encoding the separate char, default is 0xdc."]
pub type SEPER_ESC_CHAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register is used to define the separate char that need to be encoded, default is 0xc0."]
    #[inline(always)]
    pub fn seper_char(&self) -> SEPER_CHAR_R {
        SEPER_CHAR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register is used to define the first char of slip escape sequence when encoding the separate char, default is 0xdb."]
    #[inline(always)]
    pub fn seper_esc_char0(&self) -> SEPER_ESC_CHAR0_R {
        SEPER_ESC_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register is used to define the second char of slip escape sequence when encoding the separate char, default is 0xdc."]
    #[inline(always)]
    pub fn seper_esc_char1(&self) -> SEPER_ESC_CHAR1_R {
        SEPER_ESC_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESC_CONF0")
            .field("seper_char", &format_args!("{}", self.seper_char().bits()))
            .field(
                "seper_esc_char0",
                &format_args!("{}", self.seper_esc_char0().bits()),
            )
            .field(
                "seper_esc_char1",
                &format_args!("{}", self.seper_esc_char1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ESC_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to define the separate char that need to be encoded, default is 0xc0."]
    #[inline(always)]
    #[must_use]
    pub fn seper_char(&mut self) -> SEPER_CHAR_W<ESC_CONF0_SPEC> {
        SEPER_CHAR_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - This register is used to define the first char of slip escape sequence when encoding the separate char, default is 0xdb."]
    #[inline(always)]
    #[must_use]
    pub fn seper_esc_char0(&mut self) -> SEPER_ESC_CHAR0_W<ESC_CONF0_SPEC> {
        SEPER_ESC_CHAR0_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - This register is used to define the second char of slip escape sequence when encoding the separate char, default is 0xdc."]
    #[inline(always)]
    #[must_use]
    pub fn seper_esc_char1(&mut self) -> SEPER_ESC_CHAR1_W<ESC_CONF0_SPEC> {
        SEPER_ESC_CHAR1_W::new(self, 16)
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
#[doc = "Escape sequence configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esc_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESC_CONF0_SPEC;
impl crate::RegisterSpec for ESC_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esc_conf0::R`](R) reader structure"]
impl crate::Readable for ESC_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`esc_conf0::W`](W) writer structure"]
impl crate::Writable for ESC_CONF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESC_CONF0 to value 0x00dc_dbc0"]
impl crate::Resettable for ESC_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x00dc_dbc0;
}
