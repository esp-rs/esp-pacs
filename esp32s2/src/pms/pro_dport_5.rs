#[doc = "Register `PRO_DPORT_5` reader"]
pub type R = crate::R<PRO_DPORT_5_SPEC>;
#[doc = "Register `PRO_DPORT_5` writer"]
pub type W = crate::W<PRO_DPORT_5_SPEC>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_3` reader - Configure read-protection address 3."]
pub type PRO_DPORT_RESERVE_FIFO_3_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_3` writer - Configure read-protection address 3."]
pub type PRO_DPORT_RESERVE_FIFO_3_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Configure read-protection address 3."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_3(&self) -> PRO_DPORT_RESERVE_FIFO_3_R {
        PRO_DPORT_RESERVE_FIFO_3_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_5")
            .field(
                "pro_dport_reserve_fifo_3",
                &self.pro_dport_reserve_fifo_3().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DPORT_5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:17 - Configure read-protection address 3."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_reserve_fifo_3(&mut self) -> PRO_DPORT_RESERVE_FIFO_3_W<PRO_DPORT_5_SPEC> {
        PRO_DPORT_RESERVE_FIFO_3_W::new(self, 0)
    }
}
#[doc = "PeriBus1 permission control register 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DPORT_5_SPEC;
impl crate::RegisterSpec for PRO_DPORT_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dport_5::R`](R) reader structure"]
impl crate::Readable for PRO_DPORT_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dport_5::W`](W) writer structure"]
impl crate::Writable for PRO_DPORT_5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DPORT_5 to value 0"]
impl crate::Resettable for PRO_DPORT_5_SPEC {
    const RESET_VALUE: u32 = 0;
}
