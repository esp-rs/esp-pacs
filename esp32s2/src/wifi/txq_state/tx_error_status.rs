#[doc = "Register `TX_ERROR_STATUS` reader"]
pub type R = crate::R<TX_ERROR_STATUS_SPEC>;
#[doc = "Register `TX_ERROR_STATUS` writer"]
pub type W = crate::W<TX_ERROR_STATUS_SPEC>;
#[doc = "Field `SLOT_COLLISION(0-4)` reader - Collision status bit for a slot"]
pub type SLOT_COLLISION_R = crate::BitReader;
#[doc = "Field `SLOT_COLLISION(0-4)` writer - Collision status bit for a slot"]
pub type SLOT_COLLISION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOT_TIMEOUT(0-4)` reader - Timeout status bit for a slot"]
pub type SLOT_TIMEOUT_R = crate::BitReader;
#[doc = "Field `SLOT_TIMEOUT(0-4)` writer - Timeout status bit for a slot"]
pub type SLOT_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Collision status bit for a slot"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SLOT_COLLISION0` field.</div>"]
    #[inline(always)]
    pub fn slot_collision(&self, n: u8) -> SLOT_COLLISION_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SLOT_COLLISION_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Collision status bit for a slot"]
    #[inline(always)]
    pub fn slot_collision_iter(&self) -> impl Iterator<Item = SLOT_COLLISION_R> + '_ {
        (0..5).map(move |n| SLOT_COLLISION_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Collision status bit for a slot"]
    #[inline(always)]
    pub fn slot_collision0(&self) -> SLOT_COLLISION_R {
        SLOT_COLLISION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision status bit for a slot"]
    #[inline(always)]
    pub fn slot_collision1(&self) -> SLOT_COLLISION_R {
        SLOT_COLLISION_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Collision status bit for a slot"]
    #[inline(always)]
    pub fn slot_collision2(&self) -> SLOT_COLLISION_R {
        SLOT_COLLISION_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Collision status bit for a slot"]
    #[inline(always)]
    pub fn slot_collision3(&self) -> SLOT_COLLISION_R {
        SLOT_COLLISION_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Collision status bit for a slot"]
    #[inline(always)]
    pub fn slot_collision4(&self) -> SLOT_COLLISION_R {
        SLOT_COLLISION_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Timeout status bit for a slot"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SLOT_TIMEOUT0` field.</div>"]
    #[inline(always)]
    pub fn slot_timeout(&self, n: u8) -> SLOT_TIMEOUT_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SLOT_TIMEOUT_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Timeout status bit for a slot"]
    #[inline(always)]
    pub fn slot_timeout_iter(&self) -> impl Iterator<Item = SLOT_TIMEOUT_R> + '_ {
        (0..5).map(move |n| SLOT_TIMEOUT_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - Timeout status bit for a slot"]
    #[inline(always)]
    pub fn slot_timeout0(&self) -> SLOT_TIMEOUT_R {
        SLOT_TIMEOUT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timeout status bit for a slot"]
    #[inline(always)]
    pub fn slot_timeout1(&self) -> SLOT_TIMEOUT_R {
        SLOT_TIMEOUT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout status bit for a slot"]
    #[inline(always)]
    pub fn slot_timeout2(&self) -> SLOT_TIMEOUT_R {
        SLOT_TIMEOUT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timeout status bit for a slot"]
    #[inline(always)]
    pub fn slot_timeout3(&self) -> SLOT_TIMEOUT_R {
        SLOT_TIMEOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timeout status bit for a slot"]
    #[inline(always)]
    pub fn slot_timeout4(&self) -> SLOT_TIMEOUT_R {
        SLOT_TIMEOUT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_ERROR_STATUS")
            .field("slot_collision0", &self.slot_collision0())
            .field("slot_collision1", &self.slot_collision1())
            .field("slot_collision2", &self.slot_collision2())
            .field("slot_collision3", &self.slot_collision3())
            .field("slot_collision4", &self.slot_collision4())
            .field("slot_timeout0", &self.slot_timeout0())
            .field("slot_timeout1", &self.slot_timeout1())
            .field("slot_timeout2", &self.slot_timeout2())
            .field("slot_timeout3", &self.slot_timeout3())
            .field("slot_timeout4", &self.slot_timeout4())
            .finish()
    }
}
impl W {
    #[doc = "Collision status bit for a slot"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SLOT_COLLISION0` field.</div>"]
    #[inline(always)]
    pub fn slot_collision(&mut self, n: u8) -> SLOT_COLLISION_W<'_, TX_ERROR_STATUS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SLOT_COLLISION_W::new(self, n)
    }
    #[doc = "Bit 0 - Collision status bit for a slot"]
    #[inline(always)]
    pub fn slot_collision0(&mut self) -> SLOT_COLLISION_W<'_, TX_ERROR_STATUS_SPEC> {
        SLOT_COLLISION_W::new(self, 0)
    }
    #[doc = "Bit 1 - Collision status bit for a slot"]
    #[inline(always)]
    pub fn slot_collision1(&mut self) -> SLOT_COLLISION_W<'_, TX_ERROR_STATUS_SPEC> {
        SLOT_COLLISION_W::new(self, 1)
    }
    #[doc = "Bit 2 - Collision status bit for a slot"]
    #[inline(always)]
    pub fn slot_collision2(&mut self) -> SLOT_COLLISION_W<'_, TX_ERROR_STATUS_SPEC> {
        SLOT_COLLISION_W::new(self, 2)
    }
    #[doc = "Bit 3 - Collision status bit for a slot"]
    #[inline(always)]
    pub fn slot_collision3(&mut self) -> SLOT_COLLISION_W<'_, TX_ERROR_STATUS_SPEC> {
        SLOT_COLLISION_W::new(self, 3)
    }
    #[doc = "Bit 4 - Collision status bit for a slot"]
    #[inline(always)]
    pub fn slot_collision4(&mut self) -> SLOT_COLLISION_W<'_, TX_ERROR_STATUS_SPEC> {
        SLOT_COLLISION_W::new(self, 4)
    }
    #[doc = "Timeout status bit for a slot"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `SLOT_TIMEOUT0` field.</div>"]
    #[inline(always)]
    pub fn slot_timeout(&mut self, n: u8) -> SLOT_TIMEOUT_W<'_, TX_ERROR_STATUS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        SLOT_TIMEOUT_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Timeout status bit for a slot"]
    #[inline(always)]
    pub fn slot_timeout0(&mut self) -> SLOT_TIMEOUT_W<'_, TX_ERROR_STATUS_SPEC> {
        SLOT_TIMEOUT_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timeout status bit for a slot"]
    #[inline(always)]
    pub fn slot_timeout1(&mut self) -> SLOT_TIMEOUT_W<'_, TX_ERROR_STATUS_SPEC> {
        SLOT_TIMEOUT_W::new(self, 17)
    }
    #[doc = "Bit 18 - Timeout status bit for a slot"]
    #[inline(always)]
    pub fn slot_timeout2(&mut self) -> SLOT_TIMEOUT_W<'_, TX_ERROR_STATUS_SPEC> {
        SLOT_TIMEOUT_W::new(self, 18)
    }
    #[doc = "Bit 19 - Timeout status bit for a slot"]
    #[inline(always)]
    pub fn slot_timeout3(&mut self) -> SLOT_TIMEOUT_W<'_, TX_ERROR_STATUS_SPEC> {
        SLOT_TIMEOUT_W::new(self, 19)
    }
    #[doc = "Bit 20 - Timeout status bit for a slot"]
    #[inline(always)]
    pub fn slot_timeout4(&mut self) -> SLOT_TIMEOUT_W<'_, TX_ERROR_STATUS_SPEC> {
        SLOT_TIMEOUT_W::new(self, 20)
    }
}
#[doc = "Error status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_error_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_error_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_ERROR_STATUS_SPEC;
impl crate::RegisterSpec for TX_ERROR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_error_status::R`](R) reader structure"]
impl crate::Readable for TX_ERROR_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_error_status::W`](W) writer structure"]
impl crate::Writable for TX_ERROR_STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_ERROR_STATUS to value 0"]
impl crate::Resettable for TX_ERROR_STATUS_SPEC {}
