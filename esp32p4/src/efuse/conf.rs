#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `OP_CODE` reader - 0x5A5A: programming operation command 0x5AA5: read operation command."]
pub type OP_CODE_R = crate::FieldReader<u16>;
#[doc = "Field `OP_CODE` writer - 0x5A5A: programming operation command 0x5AA5: read operation command."]
pub type OP_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CFG_ECDSA_BLK` reader - Configures which block to use for ECDSA key output."]
pub type CFG_ECDSA_BLK_R = crate::FieldReader;
#[doc = "Field `CFG_ECDSA_BLK` writer - Configures which block to use for ECDSA key output."]
pub type CFG_ECDSA_BLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - 0x5A5A: programming operation command 0x5AA5: read operation command."]
    #[inline(always)]
    pub fn op_code(&self) -> OP_CODE_R {
        OP_CODE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Configures which block to use for ECDSA key output."]
    #[inline(always)]
    pub fn cfg_ecdsa_blk(&self) -> CFG_ECDSA_BLK_R {
        CFG_ECDSA_BLK_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("op_code", &format_args!("{}", self.op_code().bits()))
            .field(
                "cfg_ecdsa_blk",
                &format_args!("{}", self.cfg_ecdsa_blk().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - 0x5A5A: programming operation command 0x5AA5: read operation command."]
    #[inline(always)]
    #[must_use]
    pub fn op_code(&mut self) -> OP_CODE_W<CONF_SPEC> {
        OP_CODE_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Configures which block to use for ECDSA key output."]
    #[inline(always)]
    #[must_use]
    pub fn cfg_ecdsa_blk(&mut self) -> CFG_ECDSA_BLK_W<CONF_SPEC> {
        CFG_ECDSA_BLK_W::new(self, 16)
    }
}
#[doc = "eFuse operation mode configuraiton register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
