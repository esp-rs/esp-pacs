#[doc = "Register `ESC_CONF2` reader"]
pub type R = crate::R<ESC_CONF2_SPEC>;
#[doc = "Register `ESC_CONF2` writer"]
pub type W = crate::W<ESC_CONF2_SPEC>;
#[doc = "Field `ESC_SEQ1` reader - a"]
pub type ESC_SEQ1_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ1` writer - a"]
pub type ESC_SEQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ESC_SEQ1_CHAR0` reader - a"]
pub type ESC_SEQ1_CHAR0_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ1_CHAR0` writer - a"]
pub type ESC_SEQ1_CHAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ESC_SEQ1_CHAR1` reader - a"]
pub type ESC_SEQ1_CHAR1_R = crate::FieldReader;
#[doc = "Field `ESC_SEQ1_CHAR1` writer - a"]
pub type ESC_SEQ1_CHAR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - a"]
    #[inline(always)]
    pub fn esc_seq1(&self) -> ESC_SEQ1_R {
        ESC_SEQ1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - a"]
    #[inline(always)]
    pub fn esc_seq1_char0(&self) -> ESC_SEQ1_CHAR0_R {
        ESC_SEQ1_CHAR0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - a"]
    #[inline(always)]
    pub fn esc_seq1_char1(&self) -> ESC_SEQ1_CHAR1_R {
        ESC_SEQ1_CHAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESC_CONF2")
            .field("esc_seq1", &format_args!("{}", self.esc_seq1().bits()))
            .field(
                "esc_seq1_char0",
                &format_args!("{}", self.esc_seq1_char0().bits()),
            )
            .field(
                "esc_seq1_char1",
                &format_args!("{}", self.esc_seq1_char1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ESC_CONF2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - a"]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq1(&mut self) -> ESC_SEQ1_W<ESC_CONF2_SPEC> {
        ESC_SEQ1_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - a"]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq1_char0(&mut self) -> ESC_SEQ1_CHAR0_W<ESC_CONF2_SPEC> {
        ESC_SEQ1_CHAR0_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - a"]
    #[inline(always)]
    #[must_use]
    pub fn esc_seq1_char1(&mut self) -> ESC_SEQ1_CHAR1_W<ESC_CONF2_SPEC> {
        ESC_SEQ1_CHAR1_W::new(self, 16)
    }
}
#[doc = "a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`esc_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`esc_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESC_CONF2_SPEC;
impl crate::RegisterSpec for ESC_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esc_conf2::R`](R) reader structure"]
impl crate::Readable for ESC_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`esc_conf2::W`](W) writer structure"]
impl crate::Writable for ESC_CONF2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESC_CONF2 to value 0x00de_db11"]
impl crate::Resettable for ESC_CONF2_SPEC {
    const RESET_VALUE: u32 = 0x00de_db11;
}
