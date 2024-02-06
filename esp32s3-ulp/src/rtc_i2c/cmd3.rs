#[doc = "Register `CMD3` reader"]
pub type R = crate::R<CMD3_SPEC>;
#[doc = "Register `CMD3` writer"]
pub type W = crate::W<CMD3_SPEC>;
#[doc = "Field `COMMAND3` reader - command3"]
pub type COMMAND3_R = crate::FieldReader<u16>;
#[doc = "Field `COMMAND3` writer - command3"]
pub type COMMAND3_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `COMMAND3_DONE` reader - command3_done"]
pub type COMMAND3_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:13 - command3"]
    #[inline(always)]
    pub fn command3(&self) -> COMMAND3_R {
        COMMAND3_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - command3_done"]
    #[inline(always)]
    pub fn command3_done(&self) -> COMMAND3_DONE_R {
        COMMAND3_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD3")
            .field("command3", &format_args!("{}", self.command3().bits()))
            .field(
                "command3_done",
                &format_args!("{}", self.command3_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - command3"]
    #[inline(always)]
    #[must_use]
    pub fn command3(&mut self) -> COMMAND3_W<CMD3_SPEC> {
        COMMAND3_W::new(self, 0)
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
#[doc = "i2c commond3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD3_SPEC;
impl crate::RegisterSpec for CMD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd3::R`](R) reader structure"]
impl crate::Readable for CMD3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd3::W`](W) writer structure"]
impl crate::Writable for CMD3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD3 to value 0x0101"]
impl crate::Resettable for CMD3_SPEC {
    const RESET_VALUE: u32 = 0x0101;
}
