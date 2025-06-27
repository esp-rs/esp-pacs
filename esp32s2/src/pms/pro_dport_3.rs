#[doc = "Register `PRO_DPORT_3` reader"]
pub type R = crate::R<PRO_DPORT_3_SPEC>;
#[doc = "Register `PRO_DPORT_3` writer"]
pub type W = crate::W<PRO_DPORT_3_SPEC>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_1` reader - Configure read-protection address 1."]
pub type PRO_DPORT_RESERVE_FIFO_1_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_1` writer - Configure read-protection address 1."]
pub type PRO_DPORT_RESERVE_FIFO_1_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Configure read-protection address 1."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_1(&self) -> PRO_DPORT_RESERVE_FIFO_1_R {
        PRO_DPORT_RESERVE_FIFO_1_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_3")
            .field("pro_dport_reserve_fifo_1", &self.pro_dport_reserve_fifo_1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17 - Configure read-protection address 1."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_1(&mut self) -> PRO_DPORT_RESERVE_FIFO_1_W<PRO_DPORT_3_SPEC> {
        PRO_DPORT_RESERVE_FIFO_1_W::new(self, 0)
    }
}
#[doc = "PeriBus1 permission control register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dport_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dport_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DPORT_3_SPEC;
impl crate::RegisterSpec for PRO_DPORT_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dport_3::R`](R) reader structure"]
impl crate::Readable for PRO_DPORT_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dport_3::W`](W) writer structure"]
impl crate::Writable for PRO_DPORT_3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_DPORT_3 to value 0"]
impl crate::Resettable for PRO_DPORT_3_SPEC {}
