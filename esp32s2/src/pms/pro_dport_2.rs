#[doc = "Register `PRO_DPORT_2` reader"]
pub type R = crate::R<PRO_DPORT_2_SPEC>;
#[doc = "Register `PRO_DPORT_2` writer"]
pub type W = crate::W<PRO_DPORT_2_SPEC>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_0` reader - Configure read-protection address 0."]
pub type PRO_DPORT_RESERVE_FIFO_0_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_0` writer - Configure read-protection address 0."]
pub type PRO_DPORT_RESERVE_FIFO_0_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Configure read-protection address 0."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_0(&self) -> PRO_DPORT_RESERVE_FIFO_0_R {
        PRO_DPORT_RESERVE_FIFO_0_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_2")
            .field(
                "pro_dport_reserve_fifo_0",
                &self.pro_dport_reserve_fifo_0().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DPORT_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:17 - Configure read-protection address 0."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_reserve_fifo_0(&mut self) -> PRO_DPORT_RESERVE_FIFO_0_W<PRO_DPORT_2_SPEC> {
        PRO_DPORT_RESERVE_FIFO_0_W::new(self, 0)
    }
}
#[doc = "PeriBus1 permission control register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DPORT_2_SPEC;
impl crate::RegisterSpec for PRO_DPORT_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dport_2::R`](R) reader structure"]
impl crate::Readable for PRO_DPORT_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dport_2::W`](W) writer structure"]
impl crate::Writable for PRO_DPORT_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DPORT_2 to value 0"]
impl crate::Resettable for PRO_DPORT_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
