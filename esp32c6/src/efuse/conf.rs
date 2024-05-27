///Register `CONF` reader
pub type R = crate::R<CONF_SPEC>;
///Register `CONF` writer
pub type W = crate::W<CONF_SPEC>;
///Field `OP_CODE` reader - 0x5A5A: programming operation command 0x5AA5: read operation command.
pub type OP_CODE_R = crate::FieldReader<u16>;
///Field `OP_CODE` writer - 0x5A5A: programming operation command 0x5AA5: read operation command.
pub type OP_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - 0x5A5A: programming operation command 0x5AA5: read operation command.
    #[inline(always)]
    pub fn op_code(&self) -> OP_CODE_R {
        OP_CODE_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("op_code", &self.op_code())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - 0x5A5A: programming operation command 0x5AA5: read operation command.
    #[inline(always)]
    #[must_use]
    pub fn op_code(&mut self) -> OP_CODE_W<CONF_SPEC> {
        OP_CODE_W::new(self, 0)
    }
}
/**eFuse operation mode configuraiton register

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`conf::R`](R) reader structure
impl crate::Readable for CONF_SPEC {}
///`write(|w| ..)` method takes [`conf::W`](W) writer structure
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONF to value 0
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
