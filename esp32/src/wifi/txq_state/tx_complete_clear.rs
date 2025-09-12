#[doc = "Register `TX_COMPLETE_CLEAR` reader"]
pub type R = crate::R<TX_COMPLETE_CLEAR_SPEC>;
#[doc = "Register `TX_COMPLETE_CLEAR` writer"]
pub type W = crate::W<TX_COMPLETE_CLEAR_SPEC>;
#[doc = "Field `SLOT(0-4)` reader - Clear bit for a slot"]
pub type SLOT_R = crate::BitReader;
#[doc = "Field `SLOT(0-4)` writer - Clear bit for a slot"]
pub type SLOT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Clear bit for a slot"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SLOT0` field.</div>"]
    #[inline(always)]
    pub fn slot(&self, n: u8) -> SLOT_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SLOT_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Clear bit for a slot"]
    #[inline(always)]
    pub fn slot_iter(&self) -> impl Iterator<Item = SLOT_R> + '_ {
        (0..5).map(move |n| SLOT_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Clear bit for a slot"]
    #[inline(always)]
    pub fn slot0(&self) -> SLOT_R {
        SLOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear bit for a slot"]
    #[inline(always)]
    pub fn slot1(&self) -> SLOT_R {
        SLOT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear bit for a slot"]
    #[inline(always)]
    pub fn slot2(&self) -> SLOT_R {
        SLOT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear bit for a slot"]
    #[inline(always)]
    pub fn slot3(&self) -> SLOT_R {
        SLOT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear bit for a slot"]
    #[inline(always)]
    pub fn slot4(&self) -> SLOT_R {
        SLOT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_COMPLETE_CLEAR")
            .field("slot0", &self.slot0())
            .field("slot1", &self.slot1())
            .field("slot2", &self.slot2())
            .field("slot3", &self.slot3())
            .field("slot4", &self.slot4())
            .finish()
    }
}
impl W {
    #[doc = "Clear bit for a slot"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SLOT0` field.</div>"]
    #[inline(always)]
    pub fn slot(&mut self, n: u8) -> SLOT_W<'_, TX_COMPLETE_CLEAR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SLOT_W::new(self, n)
    }
    #[doc = "Bit 0 - Clear bit for a slot"]
    #[inline(always)]
    pub fn slot0(&mut self) -> SLOT_W<'_, TX_COMPLETE_CLEAR_SPEC> {
        SLOT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear bit for a slot"]
    #[inline(always)]
    pub fn slot1(&mut self) -> SLOT_W<'_, TX_COMPLETE_CLEAR_SPEC> {
        SLOT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear bit for a slot"]
    #[inline(always)]
    pub fn slot2(&mut self) -> SLOT_W<'_, TX_COMPLETE_CLEAR_SPEC> {
        SLOT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit for a slot"]
    #[inline(always)]
    pub fn slot3(&mut self) -> SLOT_W<'_, TX_COMPLETE_CLEAR_SPEC> {
        SLOT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear bit for a slot"]
    #[inline(always)]
    pub fn slot4(&mut self) -> SLOT_W<'_, TX_COMPLETE_CLEAR_SPEC> {
        SLOT_W::new(self, 4)
    }
}
#[doc = "Clear the completion status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_complete_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_complete_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_COMPLETE_CLEAR_SPEC;
impl crate::RegisterSpec for TX_COMPLETE_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_complete_clear::R`](R) reader structure"]
impl crate::Readable for TX_COMPLETE_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_complete_clear::W`](W) writer structure"]
impl crate::Writable for TX_COMPLETE_CLEAR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_COMPLETE_CLEAR to value 0"]
impl crate::Resettable for TX_COMPLETE_CLEAR_SPEC {}
