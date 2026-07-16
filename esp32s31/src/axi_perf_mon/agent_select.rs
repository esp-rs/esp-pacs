#[doc = "Register `AGENT_SELECT` reader"]
pub type R = crate::R<AGENT_SELECT_SPEC>;
#[doc = "Register `AGENT_SELECT` writer"]
pub type W = crate::W<AGENT_SELECT_SPEC>;
#[doc = "Field `AGENT_SELECT` reader - Select Agent in slot to be monitored, 4 bits means one agent number"]
pub type AGENT_SELECT_R = crate::FieldReader<u32>;
#[doc = "Field `AGENT_SELECT` writer - Select Agent in slot to be monitored, 4 bits means one agent number"]
pub type AGENT_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Select Agent in slot to be monitored, 4 bits means one agent number"]
    #[inline(always)]
    pub fn agent_select(&self) -> AGENT_SELECT_R {
        AGENT_SELECT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AGENT_SELECT")
            .field("agent_select", &self.agent_select())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Select Agent in slot to be monitored, 4 bits means one agent number"]
    #[inline(always)]
    pub fn agent_select(&mut self) -> AGENT_SELECT_W<'_, AGENT_SELECT_SPEC> {
        AGENT_SELECT_W::new(self, 0)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`agent_select::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agent_select::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AGENT_SELECT_SPEC;
impl crate::RegisterSpec for AGENT_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agent_select::R`](R) reader structure"]
impl crate::Readable for AGENT_SELECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`agent_select::W`](W) writer structure"]
impl crate::Writable for AGENT_SELECT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AGENT_SELECT to value 0"]
impl crate::Resettable for AGENT_SELECT_SPEC {}
